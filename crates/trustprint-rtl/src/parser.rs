use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RtlError {
    #[error("Failed to read RTL source file at {0}: {1}")]
    Io(String, #[source] std::io::Error),

    #[error("No valid Verilog module declaration found in file: {0}")]
    NoModuleFound(String),
}

/// Direction of an extracted hardware port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortDirection {
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "output")]
    Output,
    #[serde(rename = "inout")]
    Inout,
}

/// A parsed port declaration from a Verilog module header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortDeclaration {
    pub name: String,
    pub direction: PortDirection,
    pub bit_width: u32,
    pub is_clock: bool,
    pub is_reset: bool,
}

/// A parsed hardware parameter from a module header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleParameter {
    pub name: String,
    pub default_value: String,
}

/// A parsed memory-mapped register definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterDefinition {
    pub name: String,
    pub offset_hex: String,
    pub bit_width: Option<u32>,
    pub reset_value_hex: Option<String>,
    pub kind: String,
}

/// Complete structural interface extracted from a Verilog module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleInterface {
    pub module_name: String,
    pub source_path: String,
    pub parameters: Vec<ModuleParameter>,
    pub ports: Vec<PortDeclaration>,
    pub registers: Vec<RegisterDefinition>,
}

pub struct RtlParser;

impl RtlParser {
    /// Parses a Verilog/SystemVerilog source string and extracts the module interface.
    pub fn parse_str(content: &str, source_path: &str) -> Result<ModuleInterface, RtlError> {
        let clean_content = Self::strip_comments(content);
        
        // Find module name
        let module_name = Self::extract_module_name(&clean_content)
            .ok_or_else(|| RtlError::NoModuleFound(source_path.to_string()))?;

        // Extract parameters
        let parameters = Self::extract_parameters(&clean_content);

        // Extract ports
        let ports = Self::extract_ports(&clean_content);

        // Extract registers/defines
        let registers = Self::extract_registers(&clean_content);

        Ok(ModuleInterface {
            module_name,
            source_path: source_path.to_string(),
            parameters,
            ports,
            registers,
        })
    }

    /// Reads and parses an RTL file from disk.
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<ModuleInterface, RtlError> {
        let path_ref = path.as_ref();
        let content = std::fs::read_to_string(path_ref)
            .map_err(|e| RtlError::Io(path_ref.display().to_string(), e))?;
        Self::parse_str(&content, &path_ref.display().to_string())
    }

    fn strip_comments(s: &str) -> String {
        let mut result = String::new();
        let mut in_block_comment = false;
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if in_block_comment {
                if c == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    in_block_comment = false;
                }
            } else if c == '/' && chars.peek() == Some(&'*') {
                chars.next();
                in_block_comment = true;
            } else if c == '/' && chars.peek() == Some(&'/') {
                // Single line comment: skip until newline
                while let Some(&next_c) = chars.peek() {
                    if next_c == '\n' {
                        break;
                    }
                    chars.next();
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    fn extract_module_name(s: &str) -> Option<String> {
        for line in s.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("module ") {
                let parts: Vec<&str> = trimmed["module ".len()..]
                    .split(|c: char| c == '(' || c == '#' || c == ';' || c.is_whitespace())
                    .filter(|p| !p.is_empty())
                    .collect();
                if let Some(name) = parts.first() {
                    return Some(name.to_string());
                }
            }
        }
        None
    }

    fn extract_parameters(s: &str) -> Vec<ModuleParameter> {
        let mut params = Vec::new();
        for line in s.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("parameter ") || trimmed.starts_with("localparam ") {
                let rest = trimmed
                    .trim_start_matches("parameter ")
                    .trim_start_matches("localparam ")
                    .trim_end_matches(';')
                    .trim_end_matches(',');
                if let Some((k, v)) = rest.split_once('=') {
                    let name = k.split_whitespace().last().unwrap_or(k).trim().to_string();
                    let val = v.trim().to_string();
                    params.push(ModuleParameter {
                        name,
                        default_value: val,
                    });
                }
            }
        }
        params
    }

    fn extract_ports(s: &str) -> Vec<PortDeclaration> {
        let mut ports = Vec::new();

        for line in s.lines() {
            let trimmed = line.trim().trim_end_matches(',').trim_end_matches(';');
            let dir = if trimmed.starts_with("input ") || trimmed.contains(" input ") {
                Some(PortDirection::Input)
            } else if trimmed.starts_with("output ") || trimmed.contains(" output ") {
                Some(PortDirection::Output)
            } else if trimmed.starts_with("inout ") || trimmed.contains(" inout ") {
                Some(PortDirection::Inout)
            } else {
                None
            };

            if let Some(direction) = dir {
                // Parse bit-width [MSB:LSB] if present
                let (width, rest) = if let Some(bracket_start) = trimmed.find('[') {
                    if let Some(bracket_end) = trimmed[bracket_start..].find(']') {
                        let range_str = &trimmed[bracket_start + 1..bracket_start + bracket_end];
                        let w = Self::parse_range_width(range_str);
                        (w, trimmed[bracket_start + bracket_end + 1..].trim())
                    } else {
                        (1, trimmed)
                    }
                } else {
                    let tokens: Vec<&str> = trimmed.split_whitespace().collect();
                    (1, tokens.last().copied().unwrap_or(""))
                };

                // Extract identifier
                let name = rest
                    .split_whitespace()
                    .last()
                    .unwrap_or("")
                    .trim_end_matches(';')
                    .trim_end_matches(',')
                    .to_string();

                if !name.is_empty() && name != "logic" && name != "wire" && name != "reg" {
                    let is_clock = name.contains("clk") || name.contains("clock");
                    let is_reset = name.contains("rst") || name.contains("reset");
                    ports.push(PortDeclaration {
                        name,
                        direction,
                        bit_width: width,
                        is_clock,
                        is_reset,
                    });
                }
            }
        }
        ports
    }

    fn parse_range_width(range_str: &str) -> u32 {
        if let Some((msb_str, lsb_str)) = range_str.split_once(':') {
            if let (Ok(msb), Ok(lsb)) = (msb_str.trim().parse::<u32>(), lsb_str.trim().parse::<u32>()) {
                if msb >= lsb {
                    return (msb - lsb) + 1;
                }
            }
        }
        1
    }

    fn find_matching_paren(s: &str) -> Option<usize> {
        let mut depth = 0;
        for (i, c) in s.char_indices() {
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
        }
        None
    }

    fn extract_registers(s: &str) -> Vec<RegisterDefinition> {
        let mut registers = Vec::new();
        for line in s.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("`define ") {
                let rest = trimmed["`define ".len()..].trim();
                let parts: Vec<&str> = rest.split_whitespace().collect();
                if parts.len() >= 2 && (parts[0].contains("REG_") || parts[0].contains("ADDR_")) {
                    registers.push(RegisterDefinition {
                        name: parts[0].to_string(),
                        offset_hex: parts[1].to_string(),
                        bit_width: None,
                        reset_value_hex: None,
                        kind: "DEFINE".to_string(),
                    });
                }
            } else if trimmed.starts_with("apb_") {
                if let Some(hash_pos) = trimmed.find('#') {
                    let kind = trimmed[..hash_pos].trim().to_string();
                    let rem = trimmed[hash_pos + 1..].trim();
                    if let Some(open_paren) = rem.find('(') {
                        if let Some(close_paren) = Self::find_matching_paren(&rem[open_paren..]) {
                            let params_str = &rem[open_paren + 1..open_paren + close_paren];
                            let after_params = rem[open_paren + close_paren + 1..].trim();
                            let inst_name = after_params
                                .split(|c: char| c == '(' || c.is_whitespace())
                                .filter(|p| !p.is_empty())
                                .next()
                                .unwrap_or("")
                                .to_string();

                            let mut offset = String::new();
                            let mut dw = None;

                            for token in params_str.split(',') {
                                let t = token.trim();
                                if let Some(a_start) = t.find(".A") {
                                    if let Some(p1) = t[a_start..].find('(') {
                                        if let Some(p2) = t[a_start + p1..].find(')') {
                                            let raw = t[a_start + p1 + 1..a_start + p1 + p2].trim();
                                            let hex_val = raw.trim_start_matches("'h").trim_start_matches("0x");
                                            offset = format!("0x{:02x}", u32::from_str_radix(hex_val, 16).unwrap_or(0));
                                        }
                                    }
                                }
                                if let Some(dw_start) = t.find(".DW") {
                                    if let Some(p1) = t[dw_start..].find('(') {
                                        if let Some(p2) = t[dw_start + p1..].find(')') {
                                            let raw = t[dw_start + p1 + 1..dw_start + p1 + p2].trim();
                                            if let Ok(w) = raw.parse::<u32>() {
                                                dw = Some(w);
                                            }
                                        }
                                    }
                                }
                            }

                            if !inst_name.is_empty() && !offset.is_empty() {
                                registers.push(RegisterDefinition {
                                    name: inst_name,
                                    offset_hex: offset,
                                    bit_width: dw,
                                    reset_value_hex: None,
                                    kind,
                                });
                            }
                        }
                    }
                }
            }
        }
        registers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_verilog_module() {
        let verilog = r#"
        // Simple UART controller module
        module uart_ctrl #(
            parameter CLK_FREQ = 350000000,
            parameter BAUD_RATE = 115200
        ) (
            input wire clk,
            input wire rst_n,
            input wire [7:0] tx_data,
            output reg tx_busy,
            output wire [15:0] status_reg
        );
        endmodule
        "#;

        let res = RtlParser::parse_str(verilog, "uart.v").unwrap();
        assert_eq!(res.module_name, "uart_ctrl");
        assert_eq!(res.parameters.len(), 2);
        assert_eq!(res.ports.len(), 5);

        let clk_port = res.ports.iter().find(|p| p.name == "clk").unwrap();
        assert!(clk_port.is_clock);
        assert_eq!(clk_port.bit_width, 1);

        let tx_data = res.ports.iter().find(|p| p.name == "tx_data").unwrap();
        assert_eq!(tx_data.bit_width, 8);
        assert_eq!(tx_data.direction, PortDirection::Input);

        let status = res.ports.iter().find(|p| p.name == "status_reg").unwrap();
        assert_eq!(status.bit_width, 16);
        assert_eq!(status.direction, PortDirection::Output);
    }

    #[test]
    fn test_parse_sfr_registers() {
        let verilog = r#"
        module bio_sfr_block (
            input wire pclk
        );
            apb_ac2r #(.A('h00), .DW(12)) sfr_ctrl (.cr({clkdiv_restart, restart, en}), .prdata32(), .*);
            apb_sr   #(.A('h04), .DW(32)) sfr_cfginfo (.sr({16'd4096, 8'd4, 8'd8}), .prdata32(), .*);
            apb_cr   #(.A('h50), .DW(32)) sfr_qdiv0 (.cr(qdiv0), .prdata32(), .*);
        endmodule
        "#;

        let res = RtlParser::parse_str(verilog, "bio.sv").unwrap();
        assert_eq!(res.registers.len(), 3);
        assert_eq!(res.registers[0].name, "sfr_ctrl");
        assert_eq!(res.registers[0].offset_hex, "0x00");
        assert_eq!(res.registers[0].bit_width, Some(12));
        assert_eq!(res.registers[1].name, "sfr_cfginfo");
        assert_eq!(res.registers[1].offset_hex, "0x04");
        assert_eq!(res.registers[2].name, "sfr_qdiv0");
        assert_eq!(res.registers[2].offset_hex, "0x50");
    }
}

