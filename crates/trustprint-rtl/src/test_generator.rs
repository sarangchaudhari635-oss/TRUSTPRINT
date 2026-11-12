//! Automated Test Vector and Simulation Configuration Generator
//!
//! Analyzes extracted RTL module interfaces and generates cycle-accurate
//! simulation test configurations (`.jsonc`) for the `bio-sim` Verilator harness.

use crate::parser::{ModuleInterface, PortDirection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterPoke {
    pub name: Option<String>,
    pub offset: Option<String>,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalMonitor {
    pub signal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStartConfig {
    pub cores: Vec<u32>,
    pub restart: bool,
    pub clkdiv_restart: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunLimitConfig {
    pub max_cycles: u64,
    pub stop_on_trap: bool,
}

/// A complete simulation test configuration matching `bio-sim` format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioSimTestConfig {
    pub fclk_mhz: u32,
    pub sw: String,
    pub load_core: u32,
    pub registers: Vec<RegisterPoke>,
    pub start: CoreStartConfig,
    pub monitor: Vec<SignalMonitor>,
    pub run: RunLimitConfig,
}

pub struct TestGenerator;

impl TestGenerator {
    /// Generates a default `bio-sim` test configuration from a parsed module interface.
    pub fn generate_config(
        iface: &ModuleInterface,
        firmware_name: &str,
        target_core: u32,
        max_cycles: u64,
    ) -> BioSimTestConfig {
        // Detect clock frequency or default to 350 MHz / 700 MHz
        let fclk_mhz = if iface.ports.iter().any(|p| p.name == "fclk" && p.is_clock) {
            700
        } else {
            350
        };

        // Populate monitored signals from output ports
        let mut monitor = Vec::new();
        for port in &iface.ports {
            if port.direction == PortDirection::Output {
                if port.name == "gpio_out" || port.name == "gpio_dir" {
                    // Monitor individual bit 21 or bit 0
                    monitor.push(SignalMonitor {
                        signal: port.name.clone(),
                        bit: Some(21),
                    });
                } else if port.name == "irq" {
                    monitor.push(SignalMonitor {
                        signal: port.name.clone(),
                        bit: None,
                    });
                }
            }
        }

        // If no matching ports found, add default GPIO probes
        if monitor.is_empty() {
            monitor.push(SignalMonitor {
                signal: "gpio_out".to_string(),
                bit: Some(0),
            });
            monitor.push(SignalMonitor {
                signal: "irq".to_string(),
                bit: None,
            });
        }

        // Populate initial register configuration
        let mut registers = Vec::new();
        if iface.registers.iter().any(|r| r.name == "sfr_config") {
            registers.push(RegisterPoke {
                name: Some("sfr_config".to_string()),
                offset: None,
                value: "0x000".to_string(),
                port: None,
            });
        }
        if iface.registers.iter().any(|r| r.name == "sfr_qdiv0") {
            registers.push(RegisterPoke {
                name: Some("sfr_qdiv0".to_string()),
                offset: None,
                value: "0x00010000".to_string(),
                port: None,
            });
        }

        BioSimTestConfig {
            fclk_mhz,
            sw: firmware_name.to_string(),
            load_core: target_core,
            registers,
            start: CoreStartConfig {
                cores: vec![target_core],
                restart: true,
                clkdiv_restart: true,
            },
            monitor,
            run: RunLimitConfig {
                max_cycles,
                stop_on_trap: true,
            },
        }
    }

    /// Serializes configuration to a pretty JSONC string.
    pub fn to_jsonc(config: &BioSimTestConfig) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::RtlParser;

    #[test]
    fn test_generate_config_from_bio_rtl() {
        let verilog = r#"
        module bio (
            input wire fclk,
            output wire [31:0] gpio_out,
            output wire [3:0] irq
        );
            apb_cr #(.A('h08), .DW(6)) sfr_config (.cr(), .prdata32(), .*);
            apb_cr #(.A('h50), .DW(32)) sfr_qdiv0 (.cr(), .prdata32(), .*);
        endmodule
        "#;

        let iface = RtlParser::parse_str(verilog, "bio.sv").unwrap();
        let cfg = TestGenerator::generate_config(&iface, "blink", 0, 5000);

        assert_eq!(cfg.fclk_mhz, 700);
        assert_eq!(cfg.sw, "blink");
        assert_eq!(cfg.load_core, 0);
        assert_eq!(cfg.run.max_cycles, 5000);
        assert_eq!(cfg.registers.len(), 2);
        assert!(cfg.monitor.iter().any(|m| m.signal == "gpio_out"));

        let json = TestGenerator::to_jsonc(&cfg).unwrap();
        assert!(json.contains("\"fclk_mhz\": 700"));
        assert!(json.contains("\"sw\": \"blink\""));
    }
}

