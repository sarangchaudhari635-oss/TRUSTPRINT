//! TRUSTPRINT Core Types & Epistemic Verification Boundaries
//!
//! This crate defines the canonical data models, epistemic verification tiers,
//! hardware metadata descriptors, and error handling types for the TRUSTPRINT platform.

use serde::{Deserialize, Serialize};

/// The epistemic verification tier of an observation or artifact.
///
/// In TRUSTPRINT, every assertion strictly distinguishes what it proves
/// and what it cannot prove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EpistemicTier {
    /// Static structural analysis of open-source Verilog/SystemVerilog RTL.
    /// Proves: Module topologies, port widths, and declared registers.
    /// Cannot prove: Physical silicon equivalence or absence of dormant gates.
    #[serde(rename = "DESIGN_RTL")]
    DesignRtl,

    /// Cycle-accurate software simulation in Verilator (bio-sim).
    /// Proves: RTL behavioral correctness under specific test vectors.
    /// Cannot prove: Real-world physical silicon characteristics or fabrication defects.
    #[serde(rename = "SIMULATION_VERILATOR")]
    SimulationVerilator,

    /// Hardware-in-the-loop deterministic probing via the 4x 700 MHz BIO coprocessor.
    /// Proves: Physical pin transition timing and state-machine conformance.
    /// Cannot prove: Transistor-level netlist equivalence of internal unrouted nodes.
    #[serde(rename = "PHYSICAL_IO_BEHAVIOURAL")]
    PhysicalIoBehavioural,

    /// Non-destructive Short-Wave Infrared (~1050nm) backside optical die inspection.
    /// Proves: Macroscopic existence of functional blocks (SRAM, ReRAM, CPU, security mesh).
    /// Cannot prove: Sub-micron gate connectivity or individual standard-cell modifications.
    #[serde(rename = "OPTICAL_IRIS")]
    OpticalIris,

    /// Hardware-anchored cryptographic attestation (TRNG + SHA-512 + Ed25519).
    /// Proves: Non-repudiation and uncorrupted provenance from a specific silicon device.
    /// Cannot prove: That the device's physical environment was unmanipulated before test.
    #[serde(rename = "CRYPTOGRAPHIC_ATTESTATION")]
    CryptographicAttestation,
}

impl std::fmt::Display for EpistemicTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DesignRtl => write!(f, "DESIGN_RTL (Source-Level Structural Specification)"),
            Self::SimulationVerilator => write!(f, "SIMULATION_VERILATOR (Cycle-Accurate Emulation)"),
            Self::PhysicalIoBehavioural => write!(f, "PHYSICAL_IO_BEHAVIOURAL (700 MHz Silicon Probing)"),
            Self::OpticalIris => write!(f, "OPTICAL_IRIS (1050nm SWIR Die Microscopy)"),
            Self::CryptographicAttestation => write!(f, "CRYPTOGRAPHIC_ATTESTATION (Hardware Root of Trust)"),
        }
    }
}

/// Target hardware metadata describing the physical Baochip-1x SoC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetHardwareInfo {
    pub soc_part: String,
    pub soc_name: String,
    pub process_node: String,
    pub board_model: String,
    pub cpu_core: String,
    pub cpu_freq_hz: u64,
    pub bio_cores: u32,
    pub bio_freq_hz: u64,
    pub sram_size_bytes: usize,
    pub reram_size_bytes: usize,
}

impl Default for TargetHardwareInfo {
    fn default() -> Self {
        Self {
            soc_part: "BAO1X2S4F-WA".to_string(),
            soc_name: "Baochip-1x".to_string(),
            process_node: "TSMC 22nm ULL".to_string(),
            board_model: "Dabao v3c".to_string(),
            cpu_core: "VexRiscv RV32-IMAC (Sv39 MMU)".to_string(),
            cpu_freq_hz: 350_000_000,
            bio_cores: 4,
            bio_freq_hz: 700_000_000,
            sram_size_bytes: 2 * 1024 * 1024,   // 2 MiB
            reram_size_bytes: 4 * 1024 * 1024,  // 4 MiB
        }
    }
}

/// Conformance test verdict result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConformanceVerdict {
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "INCONCLUSIVE")]
    Inconclusive,
}

/// Parametric timing measurements computed from simulation or physical traces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimingMetrics {
    pub expected_period_ns: f64,
    pub observed_period_mean_ns: f64,
    pub jitter_peak_to_peak_ps: f64,
    pub jitter_rms_ps: f64,
    pub duty_cycle_percent: f64,
    pub cycles_analyzed: u64,
    pub verdict: ConformanceVerdict,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epistemic_tier_serialization() {
        let tier = EpistemicTier::PhysicalIoBehavioural;
        let json = serde_json::to_string(&tier).unwrap();
        assert_eq!(json, "\"PHYSICAL_IO_BEHAVIOURAL\"");
        let deserialized: EpistemicTier = serde_json::from_str(&json).unwrap();
        assert_eq!(tier, deserialized);
    }

    #[test]
    fn test_target_hardware_default() {
        let hw = TargetHardwareInfo::default();
        assert_eq!(hw.soc_name, "Baochip-1x");
        assert_eq!(hw.bio_freq_hz, 700_000_000);
        assert_eq!(hw.sram_size_bytes, 2097152);
    }
}

