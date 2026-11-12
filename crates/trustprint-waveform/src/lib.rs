//! TRUSTPRINT Waveform Capture & Conformance Analysis Engine
//!
//! Ingests digital transition streams (VCD / JSON / FST), extracts signal edge epochs,
//! calculates statistical jitter and duty cycle metrics, and checks conformance
//! against expected RTL timing specifications.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::path::Path;
use thiserror::Error;
use trustprint_core::{ConformanceVerdict, TimingMetrics};

#[derive(Error, Debug)]
pub enum WaveformError {
    #[error("Signal '{0}' not found in waveform trace")]
    SignalNotFound(String),

    #[error("Insufficient edge transitions detected: found {found}, minimum required {required}")]
    InsufficientEdges { found: usize, required: usize },

    #[error("Non-monotonic timestamp sequence: timestamp {current_ps} ps is earlier than preceding {previous_ps} ps")]
    NonMonotonicTimestamps { previous_ps: u64, current_ps: u64 },

    #[error(
        "Zero period encountered between consecutive rising edges at timestamp {timestamp_ps} ps"
    )]
    ZeroPeriod { timestamp_ps: u64 },

    #[error("Failed to parse waveform data: {0}")]
    ParseError(String),

    #[error("I/O error during waveform analysis: {0}")]
    Io(#[from] std::io::Error),
}

/// Timescale unit definition for digital traces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimescaleUnit {
    Picoseconds,
    Nanoseconds,
    Microseconds,
}

impl TimescaleUnit {
    /// Multiplier to convert a value in this unit to picoseconds (ps).
    pub fn to_picoseconds(&self, val: u64) -> u64 {
        match self {
            Self::Picoseconds => val,
            Self::Nanoseconds => val.saturating_mul(1_000),
            Self::Microseconds => val.saturating_mul(1_000_000),
        }
    }
}

/// A single discrete logic state transition event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeEvent {
    /// Timestamp in picoseconds (ps)
    pub timestamp_ps: u64,
    /// Boolean logic state (true = 1, false = 0)
    pub state: bool,
}

/// Configuration tolerances for waveform conformance verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConformanceTolerance {
    pub signal_name: String,
    pub expected_period_ns: f64,
    pub max_period_delta_percent: f64,
    pub expected_duty_cycle_percent: f64,
    pub max_duty_cycle_delta_percent: f64,
    pub max_allowed_jitter_p2p_ps: f64,
    pub max_allowed_jitter_rms_ps: f64,
}

impl Default for ConformanceTolerance {
    fn default() -> Self {
        Self {
            signal_name: "gpio_out[21]".to_string(),
            expected_period_ns: 108.528,
            max_period_delta_percent: 2.0, // +/- 2%
            expected_duty_cycle_percent: 14.47,
            max_duty_cycle_delta_percent: 3.0, // +/- 3%
            max_allowed_jitter_p2p_ps: 100.0,  // Max 100 ps peak-to-peak jitter
            max_allowed_jitter_rms_ps: 50.0,   // Max 50 ps RMS jitter
        }
    }
}

pub struct WaveformAnalyzer;

impl WaveformAnalyzer {
    /// Validates an input edge event sequence for monotonicity and duplicate states.
    pub fn validate_and_dedup_edges(edges: &[EdgeEvent]) -> Result<Vec<EdgeEvent>, WaveformError> {
        if edges.is_empty() {
            return Ok(Vec::new());
        }

        let mut cleaned: Vec<EdgeEvent> = Vec::with_capacity(edges.len());
        let mut prev_ts = 0u64;

        for (i, &edge) in edges.iter().enumerate() {
            if i > 0 && edge.timestamp_ps < prev_ts {
                return Err(WaveformError::NonMonotonicTimestamps {
                    previous_ps: prev_ts,
                    current_ps: edge.timestamp_ps,
                });
            }
            prev_ts = edge.timestamp_ps;

            // Skip consecutive identical states at identical or later timestamps
            if let Some(last) = cleaned.last() {
                if last.state == edge.state {
                    continue;
                }
            }
            cleaned.push(edge);
        }

        Ok(cleaned)
    }

    /// Analyzes a time-ordered sequence of edge events for a single digital net.
    pub fn analyze_edges(
        edges: &[EdgeEvent],
        tolerance: &ConformanceTolerance,
        raw_artifact_bytes: Option<&[u8]>,
    ) -> Result<TimingMetrics, WaveformError> {
        let valid_edges = Self::validate_and_dedup_edges(edges)?;

        if valid_edges.len() < 4 {
            return Err(WaveformError::InsufficientEdges {
                found: valid_edges.len(),
                required: 4,
            });
        }

        // Separate into rising and falling edges
        let mut rising_times: Vec<u64> = Vec::new();
        let mut falling_times: Vec<u64> = Vec::new();

        for edge in &valid_edges {
            if edge.state {
                rising_times.push(edge.timestamp_ps);
            } else {
                falling_times.push(edge.timestamp_ps);
            }
        }

        if rising_times.len() < 2 {
            return Err(WaveformError::InsufficientEdges {
                found: rising_times.len(),
                required: 2,
            });
        }

        // Calculate cycle periods (rising edge to next rising edge)
        let mut periods_ps: Vec<f64> = Vec::with_capacity(rising_times.len() - 1);
        for i in 0..rising_times.len() - 1 {
            let dt = rising_times[i + 1].saturating_sub(rising_times[i]);
            if dt == 0 {
                return Err(WaveformError::ZeroPeriod {
                    timestamp_ps: rising_times[i],
                });
            }
            periods_ps.push(dt as f64);
        }

        let n = periods_ps.len() as f64;
        let sum_period: f64 = periods_ps.iter().sum();
        let mean_period_ps = sum_period / n;
        let mean_period_ns = mean_period_ps / 1000.0;

        // Calculate jitter (peak-to-peak and RMS)
        let mut min_period = f64::MAX;
        let mut max_period = f64::MIN;
        let mut variance_sum = 0.0;

        for &p in &periods_ps {
            if p < min_period {
                min_period = p;
            }
            if p > max_period {
                max_period = p;
            }
            let diff = p - mean_period_ps;
            variance_sum += diff * diff;
        }

        let jitter_p2p_ps = max_period - min_period;
        let jitter_rms_ps = (variance_sum / n).sqrt();

        // Calculate duty cycle using paired forward-scanned rising/falling edges
        let mut duty_cycle_sum = 0.0;
        let mut duty_count = 0;
        let mut fall_idx = 0;

        for (i, &t_rise) in rising_times.iter().enumerate() {
            while fall_idx < falling_times.len() && falling_times[fall_idx] <= t_rise {
                fall_idx += 1;
            }

            if fall_idx < falling_times.len() {
                let t_fall = falling_times[fall_idx];
                let high_time = (t_fall - t_rise) as f64;
                if i < periods_ps.len() && periods_ps[i] > 0.0 {
                    duty_cycle_sum += (high_time / periods_ps[i]) * 100.0;
                    duty_count += 1;
                }
            }
        }

        let duty_cycle_percent = if duty_count > 0 {
            duty_cycle_sum / (duty_count as f64)
        } else {
            50.0
        };

        // Evaluate Conformance Verdict
        let period_delta_percent = ((mean_period_ns - tolerance.expected_period_ns).abs()
            / tolerance.expected_period_ns)
            * 100.0;

        let duty_delta_percent = (duty_cycle_percent - tolerance.expected_duty_cycle_percent).abs();

        let period_ok = period_delta_percent <= tolerance.max_period_delta_percent;
        let duty_ok = duty_delta_percent <= tolerance.max_duty_cycle_delta_percent;
        let jitter_ok = jitter_p2p_ps <= tolerance.max_allowed_jitter_p2p_ps;

        let (verdict, verdict_reason) = if period_ok && duty_ok && jitter_ok {
            (
                ConformanceVerdict::Pass,
                format!(
                    "Period delta {:.4}% <= {:.2}%, Duty delta {:.2}% <= {:.2}%, Jitter {:.2}ps <= {:.2}ps",
                    period_delta_percent,
                    tolerance.max_period_delta_percent,
                    duty_delta_percent,
                    tolerance.max_duty_cycle_delta_percent,
                    jitter_p2p_ps,
                    tolerance.max_allowed_jitter_p2p_ps
                ),
            )
        } else {
            let mut reasons = Vec::new();
            if !period_ok {
                reasons.push(format!(
                    "Period deviation ({:.4}%) exceeds limit ({:.2}%)",
                    period_delta_percent, tolerance.max_period_delta_percent
                ));
            }
            if !duty_ok {
                reasons.push(format!(
                    "Duty cycle deviation ({:.2}%) exceeds limit ({:.2}%)",
                    duty_delta_percent, tolerance.max_duty_cycle_delta_percent
                ));
            }
            if !jitter_ok {
                reasons.push(format!(
                    "Jitter ({:.2}ps) exceeds max allowed ({:.2}ps)",
                    jitter_p2p_ps, tolerance.max_allowed_jitter_p2p_ps
                ));
            }
            (ConformanceVerdict::Fail, reasons.join("; "))
        };

        // Compute artifact SHA-512 hash if bytes were provided
        let artifact_hash = raw_artifact_bytes.map(|b| {
            let mut hasher = Sha512::new();
            hasher.update(b);
            let result = hasher.finalize();
            result
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        });

        Ok(TimingMetrics {
            signal_name: tolerance.signal_name.clone(),
            expected_period_ns: tolerance.expected_period_ns,
            observed_period_mean_ns: mean_period_ns,
            period_delta_percent,
            expected_duty_cycle_percent: tolerance.expected_duty_cycle_percent,
            duty_cycle_percent,
            duty_delta_percent,
            jitter_peak_to_peak_ps: jitter_p2p_ps,
            jitter_rms_ps,
            cycles_analyzed: periods_ps.len() as u64,
            artifact_hash_sha512: artifact_hash,
            verdict,
            verdict_reason,
        })
    }

    /// Ingests an extracted transition JSON file from disk.
    pub fn parse_edge_file<P: AsRef<Path>>(path: P) -> Result<Vec<EdgeEvent>, WaveformError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_edge_json(&content)
    }

    /// Ingests transitions from JSON format: `[{"timestamp_ps": 1000, "state": true}, ...]`.
    pub fn parse_edge_json(json_str: &str) -> Result<Vec<EdgeEvent>, WaveformError> {
        #[derive(Deserialize)]
        struct RawEdge {
            #[serde(alias = "timestamp", alias = "time_ps", alias = "timestamp_ps")]
            timestamp_ps: u64,
            #[serde(alias = "new_state", alias = "val", alias = "value", alias = "state")]
            state: serde_json::Value,
        }

        let raw: Vec<RawEdge> =
            serde_json::from_str(json_str).map_err(|e| WaveformError::ParseError(e.to_string()))?;

        let edges = raw
            .into_iter()
            .map(|r| {
                let st = match r.state {
                    serde_json::Value::Bool(b) => b,
                    serde_json::Value::Number(n) => n.as_u64().unwrap_or(0) != 0,
                    serde_json::Value::String(s) => {
                        s.trim() == "1" || s.trim().to_lowercase() == "true"
                    }
                    _ => false,
                };
                EdgeEvent {
                    timestamp_ps: r.timestamp_ps,
                    state: st,
                }
            })
            .collect();

        Ok(edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_square_wave_conformance() {
        // 10 MHz clock = 100 ns period = 100,000 ps. 50% duty cycle = 50,000 ps high.
        let mut edges = Vec::new();
        for i in 0..10 {
            let t = i * 100_000;
            edges.push(EdgeEvent {
                timestamp_ps: t,
                state: true,
            });
            edges.push(EdgeEvent {
                timestamp_ps: t + 50_000,
                state: false,
            });
        }

        let tol = ConformanceTolerance {
            signal_name: "test_clk".to_string(),
            expected_period_ns: 100.0,
            max_period_delta_percent: 1.0,
            expected_duty_cycle_percent: 50.0,
            max_duty_cycle_delta_percent: 1.0,
            max_allowed_jitter_p2p_ps: 10.0,
            max_allowed_jitter_rms_ps: 10.0,
        };

        let metrics = WaveformAnalyzer::analyze_edges(&edges, &tol, None).unwrap();
        assert_eq!(metrics.verdict, ConformanceVerdict::Pass);
        assert!((metrics.observed_period_mean_ns - 100.0).abs() < 1e-4);
        assert!((metrics.duty_cycle_percent - 50.0).abs() < 1e-4);
        assert_eq!(metrics.jitter_peak_to_peak_ps, 0.0);
    }

    #[test]
    fn test_non_monotonic_timestamps_rejected() {
        let edges = vec![
            EdgeEvent {
                timestamp_ps: 100,
                state: true,
            },
            EdgeEvent {
                timestamp_ps: 200,
                state: false,
            },
            EdgeEvent {
                timestamp_ps: 150,
                state: true,
            }, // Backwards jump!
            EdgeEvent {
                timestamp_ps: 300,
                state: false,
            },
        ];
        let tol = ConformanceTolerance::default();
        let err = WaveformAnalyzer::analyze_edges(&edges, &tol, None);
        assert!(matches!(
            err,
            Err(WaveformError::NonMonotonicTimestamps { .. })
        ));
    }

    #[test]
    fn test_duplicate_transitions_cleaned() {
        let edges = vec![
            EdgeEvent {
                timestamp_ps: 100,
                state: true,
            },
            EdgeEvent {
                timestamp_ps: 100,
                state: true,
            }, // duplicate
            EdgeEvent {
                timestamp_ps: 200,
                state: false,
            },
            EdgeEvent {
                timestamp_ps: 300,
                state: true,
            },
            EdgeEvent {
                timestamp_ps: 400,
                state: false,
            },
            EdgeEvent {
                timestamp_ps: 500,
                state: true,
            },
        ];
        let cleaned = WaveformAnalyzer::validate_and_dedup_edges(&edges).unwrap();
        assert_eq!(cleaned.len(), 5);
    }

    #[test]
    fn test_insufficient_edges_rejected() {
        let edges = vec![
            EdgeEvent {
                timestamp_ps: 100,
                state: true,
            },
            EdgeEvent {
                timestamp_ps: 200,
                state: false,
            },
        ];
        let tol = ConformanceTolerance::default();
        let err = WaveformAnalyzer::analyze_edges(&edges, &tol, None);
        assert!(matches!(err, Err(WaveformError::InsufficientEdges { .. })));
    }

    #[test]
    fn test_json_edge_parsing() {
        let json = r#"[
            {"timestamp_ps": 1000, "state": true},
            {"timestamp_ps": 2000, "state": 0},
            {"timestamp_ps": 3000, "state": "1"},
            {"timestamp_ps": 4000, "state": false}
        ]"#;
        let edges = WaveformAnalyzer::parse_edge_json(json).unwrap();
        assert_eq!(edges.len(), 4);
        assert!(edges[0].state);
        assert!(!edges[1].state);
        assert!(edges[2].state);
        assert!(!edges[3].state);
    }
}
