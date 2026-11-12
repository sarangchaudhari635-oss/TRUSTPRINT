//! TRUSTPRINT BIO-sim Simulation Harness Orchestrator
//!
//! Provides programmatic execution of the Verilator-compiled `bio-sim`
//! executable, configuration management, and waveform output verification.

use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;
use trustprint_core::EpistemicTier;

#[derive(Error, Debug)]
pub enum SimError {
    #[error("Simulation binary not found at path: {0}")]
    BinaryNotFound(PathBuf),

    #[error("Configuration file not found: {0}")]
    ConfigNotFound(PathBuf),

    #[error("Simulation failed with exit code: {code:?}, stderr: {stderr}")]
    ExecutionFailed { code: Option<i32>, stderr: String },

    #[error("Waveform file was not generated at: {0}")]
    WaveformMissing(PathBuf),

    #[error("Waveform file is empty (0 bytes): {0}")]
    WaveformEmpty(PathBuf),

    #[error("Stale waveform: file timestamp ({modified:?}) is older than simulation invocation ({started:?})")]
    WaveformStale {
        path: PathBuf,
        modified: std::time::SystemTime,
        started: std::time::SystemTime,
    },

    #[error("Cycle count undetermined: {0}")]
    CycleCountUndetermined(String),

    #[error("I/O error during simulation runner: {0}")]
    Io(#[from] std::io::Error),
}

/// Simulation execution options.
#[derive(Debug, Clone)]
pub struct SimConfig {
    /// Path to the bio-sim simulator executable (e.g. `bio-sim/build/bio_sim`)
    pub simulator_bin: PathBuf,
    /// Path to the JSONC test configuration file
    pub config_jsonc: PathBuf,
    /// Path where the .fst waveform trace will be written
    pub output_waveform: PathBuf,
    /// Maximum simulation cycle limit
    pub max_cycles: Option<u64>,
}

/// Result of a completed simulation run.
#[derive(Debug, Clone)]
pub struct SimOutcome {
    pub epistemic_tier: EpistemicTier,
    pub config_name: String,
    pub cycles_executed: u64,
    pub waveform_path: PathBuf,
    pub stdout: String,
    pub success: bool,
}

/// Extracts executed cycle count from simulator stdout, or falls back to specified limit.
pub fn parse_cycles_from_stdout(
    stdout: &str,
    fallback_cycles: Option<u64>,
) -> Result<u64, SimError> {
    for line in stdout.lines() {
        let lower = line.to_lowercase();
        if lower.contains("cycle") {
            let tokens: Vec<&str> = line
                .split(|c: char| c.is_whitespace() || c == ':' || c == '=' || c == ',' || c == '/')
                .collect();
            for token in tokens {
                let clean: String = token.chars().filter(|c| c.is_ascii_digit()).collect();
                if let Ok(c) = clean.parse::<u64>() {
                    if c > 0 {
                        return Ok(c);
                    }
                }
            }
        }
    }

    fallback_cycles.ok_or_else(|| {
        SimError::CycleCountUndetermined(
            "Could not parse executed cycle count from simulator output and no cycle limit configured"
                .to_string(),
        )
    })
}

/// Orchestrates a bio-sim execution run with verification of freshness and cycles.
pub fn run_simulation(config: &SimConfig) -> Result<SimOutcome, SimError> {
    if !config.simulator_bin.exists() {
        return Err(SimError::BinaryNotFound(config.simulator_bin.clone()));
    }
    if !config.config_jsonc.exists() {
        return Err(SimError::ConfigNotFound(config.config_jsonc.clone()));
    }

    let invocation_time = std::time::SystemTime::now();

    let mut cmd = Command::new(&config.simulator_bin);
    if let Some(cycles) = config.max_cycles {
        cmd.arg("--cycles").arg(cycles.to_string());
    }
    cmd.arg(&config.config_jsonc);

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(SimError::ExecutionFailed {
            code: output.status.code(),
            stderr,
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    // Waveform existence, size, and freshness validation
    if !config.output_waveform.exists() {
        return Err(SimError::WaveformMissing(config.output_waveform.clone()));
    }
    let metadata = std::fs::metadata(&config.output_waveform)?;
    if metadata.len() == 0 {
        return Err(SimError::WaveformEmpty(config.output_waveform.clone()));
    }
    if let Ok(mtime) = metadata.modified() {
        // 2-second margin for filesystem timestamp quantization
        if mtime + std::time::Duration::from_secs(2) < invocation_time {
            return Err(SimError::WaveformStale {
                path: config.output_waveform.clone(),
                modified: mtime,
                started: invocation_time,
            });
        }
    }

    let cycles_executed = parse_cycles_from_stdout(&stdout, config.max_cycles)?;

    Ok(SimOutcome {
        epistemic_tier: EpistemicTier::SimulationVerilator,
        config_name: config
            .config_jsonc
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        cycles_executed,
        waveform_path: config.output_waveform.clone(),
        stdout,
        success: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_missing_binary_returns_error() {
        let cfg = SimConfig {
            simulator_bin: PathBuf::from("./non_existent_binary_xyz"),
            config_jsonc: PathBuf::from("./non_existent_config.jsonc"),
            output_waveform: PathBuf::from("./waveform.fst"),
            max_cycles: Some(1000),
        };
        let res = run_simulation(&cfg);
        assert!(matches!(res, Err(SimError::BinaryNotFound(_))));
    }

    #[test]
    fn test_missing_config_returns_error() {
        // Create a temporary dummy executable file to pass binary check
        let temp_bin = std::env::temp_dir().join("trustprint_temp_bin.exe");
        let mut f = File::create(&temp_bin).unwrap();
        writeln!(f, "mock").unwrap();

        let cfg = SimConfig {
            simulator_bin: temp_bin.clone(),
            config_jsonc: PathBuf::from("./non_existent_config_abc.jsonc"),
            output_waveform: PathBuf::from("./waveform.fst"),
            max_cycles: Some(1000),
        };
        let res = run_simulation(&cfg);
        let _ = std::fs::remove_file(temp_bin);
        assert!(matches!(res, Err(SimError::ConfigNotFound(_))));
    }

    #[test]
    fn test_parse_cycles_from_stdout_explicit() {
        let stdout = "sfr_cfginfo @0x04 = 0x10000408 -> PASS\nCycles executed: 10000\nDone.";
        let cycles = parse_cycles_from_stdout(stdout, None).unwrap();
        assert_eq!(cycles, 10000);
    }

    #[test]
    fn test_parse_cycles_from_stdout_alt_format() {
        let stdout = "Ran 50000 cycles at 38892 cyc/s\nPASS";
        let cycles = parse_cycles_from_stdout(stdout, None).unwrap();
        assert_eq!(cycles, 50000);
    }

    #[test]
    fn test_parse_cycles_fallback() {
        let stdout = "Simulation complete with no cycle log.";
        let cycles = parse_cycles_from_stdout(stdout, Some(1234)).unwrap();
        assert_eq!(cycles, 1234);

        let err = parse_cycles_from_stdout(stdout, None);
        assert!(matches!(err, Err(SimError::CycleCountUndetermined(_))));
    }
}
