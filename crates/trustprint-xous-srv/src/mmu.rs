//! RISC-V Sv32 Hardware MMU Page Translation and Fault Emulator
//!
//! Models the Baochip-1x VexRiscv RV32-IMAC Sv32 virtual memory system:
//! - 32-bit Virtual Address: VPN[1] (10 bits) | VPN[0] (10 bits) | Page Offset (12 bits)
//! - 34-bit Physical Address: PPN[1] (12 bits) | PPN[0] (10 bits) | Page Offset (12 bits)
//! - 4 KiB Base Pages with hardware permission bits (Valid, Read, Write, Execute, User, Global, Accessed, Dirty)
//! - Explicitly labeled as HOST_SV32_MODEL for epistemic clarity.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageFlags(pub u8);

impl PageFlags {
    pub const VALID: Self = Self(1 << 0);
    pub const READ: Self = Self(1 << 1);
    pub const WRITE: Self = Self(1 << 2);
    pub const EXECUTE: Self = Self(1 << 3);
    pub const USER: Self = Self(1 << 4);
    pub const GLOBAL: Self = Self(1 << 5);
    pub const ACCESSED: Self = Self(1 << 6);
    pub const DIRTY: Self = Self(1 << 7);

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl std::ops::BitOr for PageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivilegeMode {
    User,
    Supervisor,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MmuTrap {
    #[error("Instruction Page Fault (scause=12) at vaddr 0x{vaddr:08x}")]
    InstructionPageFault { vaddr: usize },

    #[error("Load Page Fault (scause=13) at vaddr 0x{vaddr:08x}")]
    LoadPageFault { vaddr: usize },

    #[error("Store Page Fault (scause=15) at vaddr 0x{vaddr:08x}")]
    StorePageFault { vaddr: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageTableEntry {
    pub ppn: usize, // Physical Page Number
    pub flags: PageFlags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessAddressSpace {
    pub pid: u32,
    pub name: String,
    // Key: Virtual Page Number (VPN = vaddr / 4096)
    pub pages: std::collections::BTreeMap<usize, PageTableEntry>,
}

impl ProcessAddressSpace {
    pub fn new(pid: u32, name: &str) -> Self {
        Self {
            pid,
            name: name.to_string(),
            pages: std::collections::BTreeMap::new(),
        }
    }

    pub fn map_page(&mut self, vaddr: usize, paddr: usize, flags: PageFlags) {
        let vpn = vaddr / PAGE_SIZE;
        let ppn = paddr / PAGE_SIZE;
        self.pages.insert(vpn, PageTableEntry { ppn, flags });
    }

    /// Translates a virtual address into physical address while checking MMU privileges.
    pub fn translate(
        &self,
        vaddr: usize,
        is_write: bool,
        is_exec: bool,
        privilege: PrivilegeMode,
    ) -> Result<usize, MmuTrap> {
        let vpn = vaddr / PAGE_SIZE;
        let offset = vaddr % PAGE_SIZE;

        let pte = match self.pages.get(&vpn) {
            Some(entry) if entry.flags.contains(PageFlags::VALID) => entry,
            _ => {
                return if is_exec {
                    Err(MmuTrap::InstructionPageFault { vaddr })
                } else if is_write {
                    Err(MmuTrap::StorePageFault { vaddr })
                } else {
                    Err(MmuTrap::LoadPageFault { vaddr })
                };
            }
        };

        // Enforce user mode restrictions: user cannot access non-USER pages
        if privilege == PrivilegeMode::User && !pte.flags.contains(PageFlags::USER) {
            return if is_exec {
                Err(MmuTrap::InstructionPageFault { vaddr })
            } else if is_write {
                Err(MmuTrap::StorePageFault { vaddr })
            } else {
                Err(MmuTrap::LoadPageFault { vaddr })
            };
        }

        // Check write permission
        if is_write && !pte.flags.contains(PageFlags::WRITE) {
            return Err(MmuTrap::StorePageFault { vaddr });
        }

        // Check execute permission
        if is_exec && !pte.flags.contains(PageFlags::EXECUTE) {
            return Err(MmuTrap::InstructionPageFault { vaddr });
        }

        // Check read permission
        if !is_write && !is_exec && !pte.flags.contains(PageFlags::READ) {
            return Err(MmuTrap::LoadPageFault { vaddr });
        }

        Ok((pte.ppn * PAGE_SIZE) + offset)
    }
}
