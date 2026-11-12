//! TRUSTPRINT RTL Intelligence & Interface Parser
//!
//! Statically parses synthesizable Verilog and SystemVerilog files from the
//! Baochip-1x repository to extract module definitions, port lists,
//! parameters, and memory-mapped register address boundaries.

pub mod parser;
pub mod test_generator;

pub use parser::{ModuleInterface, PortDeclaration, PortDirection, RegisterDefinition, RtlParser};
pub use test_generator::{BioSimTestConfig, TestGenerator};

