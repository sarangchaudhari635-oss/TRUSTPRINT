//! TRUSTPRINT Xous Microkernel Enclave & MMU Isolation Service
//!
//! Models Xous OS capability-based IPC messaging and hardware MMU (Sv32/Sv39)
//! memory page table translations and fault isolation.

pub mod api;
pub mod mmu;

pub use api::*;
pub use mmu::*;

pub struct TrustprintXousServer {
    pub enclave_space: ProcessAddressSpace,
    pub client_spaces: std::collections::BTreeMap<u32, ProcessAddressSpace>,
}

impl TrustprintXousServer {
    /// Initializes the Xous TRUSTPRINT enclave service with MMU address spaces.
    pub fn new() -> Self {
        let mut enclave = ProcessAddressSpace::new(2, "trustprint-enclave");
        // Map Enclave Private Memory: 0x2000_0000 - 0x2000_3000 (User=false, Read=true, Write=true)
        enclave.map_page(
            0x2000_0000,
            0x4000_0000,
            PageFlags::VALID | PageFlags::READ | PageFlags::WRITE,
        );
        enclave.map_page(
            0x2000_1000,
            0x4000_1000,
            PageFlags::VALID | PageFlags::READ | PageFlags::WRITE,
        );

        Self {
            enclave_space: enclave,
            client_spaces: std::collections::BTreeMap::new(),
        }
    }

    /// Registers an untrusted client process with isolated userland memory.
    pub fn register_client(&mut self, pid: u32, name: &str) {
        let mut client = ProcessAddressSpace::new(pid, name);
        // Map Client User Memory: 0x1000_0000 (User=true, Read=true, Write=true)
        client.map_page(
            0x1000_0000,
            0x5000_0000,
            PageFlags::VALID | PageFlags::READ | PageFlags::WRITE | PageFlags::USER,
        );
        // Map Client Code Page: 0x0001_0000 (User=true, Read=true, Exec=true)
        client.map_page(
            0x0001_0000,
            0x6000_0000,
            PageFlags::VALID | PageFlags::READ | PageFlags::EXECUTE | PageFlags::USER,
        );
        self.client_spaces.insert(pid, client);
    }

    /// Handles an IPC memory attestation request from a client.
    pub fn handle_attestation_request(
        &self,
        req: &AttestationRequest,
    ) -> Result<AttestationResponse, MmuTrap> {
        let client_space = match self.client_spaces.get(&req.client_pid) {
            Some(space) => space,
            None => {
                return Err(MmuTrap::LoadPageFault {
                    vaddr: req.virtual_addr,
                })
            }
        };

        // MMU hardware checks translation for the requested memory range
        let mut addr = req.virtual_addr;
        let end_addr = addr + req.length_bytes;
        let mut checksum: u64 = 0;

        while addr < end_addr {
            let paddr = client_space.translate(addr, false, false, PrivilegeMode::User)?;
            checksum = checksum.wrapping_add(paddr as u64);
            addr += PAGE_SIZE;
        }

        Ok(AttestationResponse {
            client_pid: req.client_pid,
            virtual_addr: req.virtual_addr,
            length_bytes: req.length_bytes,
            memory_checksum: checksum,
            mmu_isolation_verified: true,
            evaluation_model: "HOST_SV32_MODEL".to_string(),
        })
    }

    /// Simulates a rogue client attempting to directly access another address space or kernel page.
    pub fn simulate_rogue_access(
        &self,
        client_pid: u32,
        target_vaddr: usize,
        is_write: bool,
    ) -> Result<usize, MmuTrap> {
        let client_space = match self.client_spaces.get(&client_pid) {
            Some(space) => space,
            None => {
                return Err(MmuTrap::LoadPageFault {
                    vaddr: target_vaddr,
                })
            }
        };

        client_space.translate(target_vaddr, is_write, false, PrivilegeMode::User)
    }

    /// Simulates attempting to execute instructions from an unmapped or non-executable page.
    pub fn simulate_execute_access(
        &self,
        client_pid: u32,
        target_vaddr: usize,
    ) -> Result<usize, MmuTrap> {
        let client_space = match self.client_spaces.get(&client_pid) {
            Some(space) => space,
            None => {
                return Err(MmuTrap::InstructionPageFault {
                    vaddr: target_vaddr,
                })
            }
        };

        client_space.translate(target_vaddr, false, true, PrivilegeMode::User)
    }
}

impl Default for TrustprintXousServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legitimate_client_memory_attestation() {
        let mut server = TrustprintXousServer::new();
        server.register_client(3, "legitimate_app");

        let req = AttestationRequest {
            client_pid: 3,
            virtual_addr: 0x1000_0000,
            length_bytes: 4096,
        };

        let res = server.handle_attestation_request(&req).unwrap();
        assert_eq!(res.client_pid, 3);
        assert!(res.mmu_isolation_verified);
        assert_eq!(res.memory_checksum, 0x5000_0000);
    }

    #[test]
    fn test_unmapped_access_triggers_load_page_fault() {
        let mut server = TrustprintXousServer::new();
        server.register_client(3, "rogue_app");

        // Attempt to read unmapped virtual address 0xDEAD_BEEF
        let err = server
            .simulate_rogue_access(3, 0xDEAD_BEEF, false)
            .unwrap_err();
        assert_eq!(err, MmuTrap::LoadPageFault { vaddr: 0xDEAD_BEEF });
    }

    #[test]
    fn test_write_to_code_page_triggers_store_page_fault() {
        let mut server = TrustprintXousServer::new();
        server.register_client(3, "rogue_app");

        // Code page at 0x0001_0000 has READ | EXECUTE, but NOT WRITE
        let err = server
            .simulate_rogue_access(3, 0x0001_0000, true)
            .unwrap_err();
        assert_eq!(err, MmuTrap::StorePageFault { vaddr: 0x0001_0000 });
    }

    #[test]
    fn test_execute_from_data_page_triggers_instruction_page_fault() {
        let mut server = TrustprintXousServer::new();
        server.register_client(3, "rogue_app");

        // Data page at 0x1000_0000 has READ | WRITE | USER, but NOT EXECUTE
        let err = server.simulate_execute_access(3, 0x1000_0000).unwrap_err();
        assert_eq!(err, MmuTrap::InstructionPageFault { vaddr: 0x1000_0000 });
    }
}
