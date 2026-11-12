//! TRUSTPRINT Waveform Capture & Conformance Analysis Engine
//!
//! Ingests digital transition streams (VCD / FST), extracts signal edge epochs,
//! calculates statistical jitter and duty cycle metrics, and checks conformance
//! against expected RTL timing specifications.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use trustprint_core::{ConformanceVerdict, TimingMetrics};

#[derive(Error, Debug)]
pub enum WaveformError {
    #[error("Signal '{0}' not found in waveform trace")]
    SignalNotFound(String),

    #[error("Insufficient edge transitions detected: found {found}, minimum required {required}")]
    InsufficientEdges { found: usize, required: usize },

    #[error("Failed to parse waveform data: {0}")]
    ParseError(String),

    #[error("I/O error during waveform analysis: {0}")]
    Io(#[from] std::io::Error),
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
    pub expected_period_ns: f64,
    pub max_period_delta_percent: f64,
    pub expected_duty_cycle_percent: f64,
    pub max_duty_cycle_delta_percent: f64,
    pub max_allowed_jitter_p2p_ps: f64,
}

impl Default for ConformanceTolerance {
    fn default() -> Self {
        Self {
            expected_period_ns: 500.0,
            max_period_delta_percent: 2.0, // +/- 2%
            expected_duty_cycle_percent: 50.0,
            max_duty_cycle_delta_percent: 5.0, // +/- 5%
            max_allowed_jitter_p2p_ps: 500.0,  // Max 500 ps peak-to-peak jitter
        }
    }
}

pub struct WaveformAnalyzer;

impl WaveformAnalyzer {
    /// Analyzes a time-ordered sequence of edge events for a single digital net.
    pub fn analyze_edges(
        edges: &[EdgeEvent],
        tolerance: &ConformanceTolerance,
    ) -> Result<TimingMetrics, WaveformError> {
        if edges.len() < 4 {
            return Err(WaveformError::InsufficientEdges {
                found: edges.len(),
                required: 4,
            });
        }

        // Separate into rising and falling edges
        let mut rising_times: Vec<u64> = Vec::new();
        let mut falling_times: Vec<u64> = Vec::new();

        for edge in edges {
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
            let dt = (rising_times[i + 1] - rising_times[i]) as f64;
            periods_ps.push(dt);
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

        // Calculate duty cycle using paired rising/falling edges
        // Each pulse starts at rising_times[i] and ends at the immediate next falling_time > rising_times[i]
        let mut duty_cycle_sum = 0.0;
        let mut duty_count = 0;
        let mut fall_idx = 0;

        for i in 0..rising_times.len() {
            let t_rise = rising_times[i];
            // Advance fall_idx until we find the falling edge after this rising edge
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

        let duty_delta_percent =
            (duty_cycle_percent - tolerance.expected_duty_cycle_percent).abs();

        let verdict = if period_delta_percent <= tolerance.max_period_delta_percent
            && duty_delta_percent <= tolerance.max_duty_cycle_delta_percent
            && jitter_p2p_ps <= tolerance.max_allowed_jitter_p2p_ps
        {
            ConformanceVerdict::Pass
        } else {
            ConformanceVerdict::Fail
        };

        Ok(TimingMetrics {
            expected_period_ns: tolerance.expected_period_ns,
            observed_period_mean_ns: mean_period_ns,
            jitter_peak_to_peak_ps: jitter_p2p_ps,
            jitter_rms_ps,
            duty_cycle_percent,
            cycles_analyzed: periods_ps.len() as u64,
            verdict,
        })
    }

    /// Parses raw transition monitor log output from `bio-sim`
    /// Lines format: `[mon] cyc=58 (2257820 ps) gpio_out[21]: 0 -> 1`
    pub fn parse_monitor_log(log_text: &str, target_signal: &str) -> Vec<EdgeEvent> {
        let mut edges = Vec::new();

        for line in log_text.lines() {
            if line.contains("[mon]") && line.contains(target_signal) {
                // Extract timestamp: inside `(...) ps)`
                if let Some(p1) = line.find('(') {
                    if let Some(p2) = line.find(" ps)") {
                        if let Ok(ts) = line[p1 + 1..p2].trim().parse::<u64>() {
                            // Extract state transition `-> X`
                            if let Some(arrow_pos) = line.find("->") {
                                let new_state_str = line[arrow_pos + 2..].trim();
                                let state = new_state_str.starts_with('1');
                                edges.push(EdgeEvent {
                                    timestamp_ps: ts,
                                    state,
                                });
                            }
                        }
                    }
                }
            }
        }

        edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_square_wave_conformance() {
        let mut edges = Vec::new();
        // 1 MHz square wave: period = 1000 ns = 1,000,000 ps
        for i in 0..10 {
            let t_rise = i * 1_000_000;
            let t_fall = t_rise + 500_000;
            edges.push(EdgeEvent {
                timestamp_ps: t_rise,
                state: true,
            });
            edges.push(EdgeEvent {
                timestamp_ps: t_fall,
                state: false,
            });
        }

        let tol = ConformanceTolerance {
            expected_period_ns: 1000.0,
            max_period_delta_percent: 1.0,
            expected_duty_cycle_percent: 50.0,
            max_duty_cycle_delta_percent: 2.0,
            max_allowed_jitter_p2p_ps: 100.0,
        };

        let metrics = WaveformAnalyzer::analyze_edges(&edges, &tol).unwrap();
        assert_eq!(metrics.verdict, ConformanceVerdict::Pass);
        assert!((metrics.observed_period_mean_ns - 1000.0).abs() < 0.001);
        assert!((metrics.duty_cycle_percent - 50.0).abs() < 0.001);
        assert_eq!(metrics.jitter_peak_to_peak_ps, 0.0);
    }

    #[test]
    fn test_parse_monitor_log_lines() {
        let log = r#"
        [mon] cyc=58 (2257820 ps)  gpio_out[21]: 0 -> 1
        [mon] cyc=202 (2669372 ps)  gpio_out[21]: 1 -> 0
        [mon] cyc=369 (3146658 ps)  gpio_out[21]: 0 -> 1
        [mon] cyc=513 (3558210 ps)  gpio_out[21]: 1 -> 0
        "#;

        let edges = WaveformAnalyzer::parse_monitor_log(log, "gpio_out[21]");
        assert_eq!(edges.len(), 4);
        assert_eq!(edges[0].timestamp_ps, 2257820);
        assert!(edges[0].state);
        assert_eq!(edges[1].timestamp_ps, 2669372);
        assert!(!edges[1].state);
    }
}
