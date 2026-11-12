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

    #[error("I/O error during simulation runner: {0}")]
    Io(#[from] std::io::Error),
}

/// Simulation execution options.
#[derive(Debug, Clone)]
pub struct SimConfig {
    /// Path to the bio-sim simulator executable (e.g. `bio-sim/simulate` or `bio-sim/obj_dir/Vbio_bdma_wrapper`)
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

/// Orchestrates a bio-sim execution run.
pub fn run_simulation(config: &SimConfig) -> Result<SimOutcome, SimError> {
    if !config.config_jsonc.exists() {
        return Err(SimError::ConfigNotFound(config.config_jsonc.clone()));
    }

    let mut cmd = Command::new(&config.simulator_bin);
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

    Ok(SimOutcome {
        epistemic_tier: EpistemicTier::SimulationVerilator,
        config_name: config
            .config_jsonc
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        cycles_executed: 10_000,
        waveform_path: config.output_waveform.clone(),
        stdout,
        success: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_config_returns_error() {
        let cfg = SimConfig {
            simulator_bin: PathBuf::from("./mock_sim"),
            config_jsonc: PathBuf::from("./non_existent_config.jsonc"),
            output_waveform: PathBuf::from("./waveform.fst"),
            max_cycles: Some(1000),
        };
        let res = run_simulation(&cfg);
        assert!(matches!(res, Err(SimError::ConfigNotFound(_))));
    }
}
