# 15 — Xous Microkernel & MMU Process Isolation

**Document Reference:** SPEC-MMU-001  
**Project:** TRUSTPRINT  
**Version:** 0.2.0  

---

## 1. High-Assurance Embedded Architecture

Most commercial microcontrollers operate with flat, unprotected physical memory address spaces or rudimentary MPUs limited to 8–16 fixed regions. Under these architectures, any buffer overflow can overwrite root-of-trust secrets, bootloader code, or peripheral registers.

The **Baochip-1x** incorporates a full **RISC-V Sv32 Virtual Memory Management Unit (MMU)** alongside its VexRiscv core, enabling **Xous** — a pure-Rust, capability-based microkernel OS developed by the Betrusted team.

> [!NOTE]
> The TRUSTPRINT MMU test (`trustprint-xous-srv`) implements a **host-side software model** of the Sv32
> memory protection semantics. All five test scenarios run in Rust on the development workstation.
> They verify that the *model* traps correctly — not that physical on-chip MMU hardware fires these traps.
> Every output line is labelled `evaluation_model: HOST_SV32_MODEL`.

```mermaid
flowchart TD
    subgraph PhysicalSilicon ["Physical Memory Hardware (Sv32 MMU)"]
        SRAM_PHYS["2 MiB SRAM (Physical Address Space)"]
        RRAM_PHYS["4 MiB ReRAM (Non-Volatile Physical Space)"]
        MMIO_PHYS["Hardware Security & Crypto Registers"]
    end

    subgraph XousKernelDomain ["Kernel Mode (Supervisor Mode)"]
        KERNEL["Xous Microkernel Core<br/>Page Table Manager & Scheduler"]
    end

    subgraph UserlandDomain ["User Mode (Isolated Virtual Address Spaces)"]
        subgraph ProcessA ["Process 1: trustprint-srv (Test Runner)"]
            VA_A["Virtual Address Space 0x0000_0000 – 0x7FFF_FFFF"]
            TEST_LOGIC["Experiment Orchestration"]
        end

        subgraph ProcessB ["Process 2: crypto-srv (Security Root)"]
            VA_B["Virtual Address Space 0x0000_0000 – 0x7FFF_FFFF"]
            KEYS["Hardware Key Store Access"]
        end

        subgraph ProcessC ["Process 3: pddb-srv (Encrypted Database)"]
            VA_C["Virtual Address Space 0x0000_0000 – 0x7FFF_FFFF"]
            VAULT["Evidence Record Storage"]
        end
    end

    KERNEL -->|"Sv32 2-Level Page Tables"| PhysicalSilicon
    VA_A -.->|"Isolated Mapping"| SRAM_PHYS
    VA_B -.->|"Isolated Mapping (Exclusive Crypto MMIO)"| MMIO_PHYS
    VA_C -.->|"Isolated Mapping (Encrypted Storage)"| RRAM_PHYS
    ProcessA <-->|"Capability-Gated IPC Messages"| ProcessB
    ProcessA <-->|"Capability-Gated IPC Messages"| ProcessC
```

---

## 2. Sv32 Virtual Memory (Baochip-1x)

The Baochip-1x VexRiscv uses **Sv32** (RV32 paging):
- Virtual addresses are **32 bits** wide → 4 GiB virtual address space per process.
- Physical addresses are **34 bits** wide (supports up to 16 GiB physical).
- Page size: **4 KiB**, two-level hierarchical page table tree.
- Each userland process occupies an independent root page table (`satp` register swapped on context switch).

> [!NOTE]
> The documentation for doc version 0.1 incorrectly stated Sv39 (39-bit virtual addresses / 3-level page tables).
> Baochip-1x with VexRiscv RV32-IMAC uses **Sv32** (32-bit virtual addresses, 2-level page tables).

### Memory Separation Guarantees

1. **Zero Direct Access to Cryptographic Keys:** `trustprint-srv` has zero page mappings to the hardware security subsystem physical addresses.
2. **Capability-Gated IPC:** To request a signature or query the TRNG, `trustprint-srv` dispatches an asynchronous IPC message across the microkernel boundary to `crypto-srv`.
3. **Hardware Trap on Illegal Memory Access:** Any pointer dereference outside allocated virtual pages triggers an immediate RISC-V `PageFault` exception.

---

## 3. Five MMU Isolation Test Scenarios (`TEST-MMU-001` through `TEST-MMU-005`)

Run with:

```bash
cargo run -p trustprint-cli -- mmu-test
```

Output includes `evaluation_model: HOST_SV32_MODEL` on every line.

| Scenario | Label | Access Type | Expected Trap | RISC-V mcause |
|----------|-------|-------------|---------------|---------------|
| 1 | Unmapped Physical Address | Load | `LoadPageFault` | 13 (`0x0D`) |
| 2 | Write to Code Page | Store | `StorePageFault` | 15 (`0x0F`) |
| 3 | Execute from Data Page | Execute | `InstructionPageFault` | 12 (`0x0C`) |
| 4 | User→Kernel Privilege Escalation | Store | `StorePageFault` | 15 (`0x0F`) |
| 5 | NX Enforcement on Data Page | Execute | `InstructionPageFault` | 12 (`0x0C`) |

> [!IMPORTANT]
> Scenarios 1–5 all report traps **caught** (verdict `PASS`) in the host Sv32 software model.
> This demonstrates the *model* is correct. It does **not** verify that physical on-chip MMU
> hardware enforces these traps. That would require running `trustprint-xous-srv` on the actual
> Baochip-1x with Xous and observing UART trap output.

### AttestationResponse Fields

The IPC call result includes:

| Field | Type | Description |
|-------|------|-------------|
| `is_isolated` | bool | Whether process isolation is modelled |
| `memory_regions` | Vec<MemoryRegion> | Mapped regions with permissions |
| `mmio_accessible` | bool | Whether MMIO is reachable from this process |
| `evaluation_model` | String | Always `"HOST_SV32_MODEL"` in current implementation |

---

## 4. Verification States per Scenario

| State | Meaning |
|-------|---------|
| `SIMULATED` | Test ran in host software model; not on physical hardware |
| `VERIFIED` | Reserved for future on-chip execution with UART trap confirmation |
| `NOT_RUN` | Test was not executed |

All five scenarios currently report `SIMULATED` with `evaluation_model: HOST_SV32_MODEL`.

---

## 5. Path to Physical Verification

To elevate from `SIMULATED` to `VERIFIED`:

1. Cross-compile `trustprint-xous-srv` for RISC-V (`riscv32imac-unknown-none-elf`).
2. Load the ELF into Xous running on physical Baochip-1x (via USB JTAG or Xous app loader).
3. Instrument each test scenario to emit a known UART token when the trap fires.
4. Capture the UART output via `scripts/dabao_serial.py` on COM10.
5. Record the UART token in `dabao_physical_evidence.json` and re-run `build-evidence`.

This upgrade path is tracked in `docs/16-physical-integration-roadmap.md`.
