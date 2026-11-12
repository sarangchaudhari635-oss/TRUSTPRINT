//! Xous IPC Message and Opcode Protocol Specifications

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(usize)]
pub enum TrustprintOpcode {
    GetVersion = 1,
    AttestMemoryRange = 2,
    TestFaultIsolation = 3,
    QueryIsolationStatus = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttestationRequest {
    pub client_pid: u32,
    pub virtual_addr: usize,
    pub length_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttestationResponse {
    pub client_pid: u32,
    pub virtual_addr: usize,
    pub length_bytes: usize,
    pub memory_checksum: u64,
    pub mmu_isolation_verified: bool,
}

