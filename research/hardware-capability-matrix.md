# Research — Hardware Capability Matrix (Baochip-1x / Dabao)

**Document Reference:** RES-HW-MAT-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Baochip-1x Subsystem Audit

| Silicon Feature | Specification on Baochip-1x | Relevance to TRUSTPRINT | Verification Status in TRUSTPRINT |
| :--- | :--- | :--- | :--- |
| **Main CPU** | VexRiscv RV32-IMAC @ 350 MHz | Runs the Xous operating system microkernel and verification coordinator. | **VERIFIED FEASIBLE** via `betrusted-io/xous-core`. |
| **Virtual Memory (MMU)** | RISC-V Sv39 Virtual Memory (3-level page tables) | Enforces process isolation between test runners and cryptographic key storage. | **VERIFIED FEASIBLE** in hardware and Xous architecture. |
| **Volatile RAM** | 2 MiB integrated high-speed SRAM | Holds microkernel page tables, user heaps, and test telemetry buffers. | **VERIFIED FEASIBLE** on Baochip-1x. |
| **Non-Volatile RAM** | 4 MiB integrated ReRAM (Resistive RAM) | Fast XIP code execution; persistent encrypted PDDB evidence database. | **VERIFIED FEASIBLE** on Baochip-1x. |
| **BIO Coprocessor** | 4x PicoRV32 (RV32E) cores @ 700 MHz | Jitter-free, cycle-accurate I/O stimulus generation and real-time logic sampling. | **VERIFIED FEASIBLE** via `baochip/bio-sim` and silicon. |
| **BIO Blocking FIFOs** | Registers `x16`–`x31` mapped to hardware queues | Zero-overhead, inter-core synchronization without software spinlocks. | **VERIFIED FEASIBLE** via `bio-sim` and architecture docs. |
| **Quantum Snap** | Dedicated clock quantum alignment register | Eliminates cycle-counting instruction jitter; hard phase-alignment. | **VERIFIED FEASIBLE** via architecture specification. |
| **Cryptographic Subsystem** | Hardware TRNG, AES-256, SHA-512, PKE (Ed25519) | Anchors evidence bundles in true physical entropy and digital signatures. | **VERIFIED FEASIBLE** via Xous security services. |
| **Physical Tamper Mesh** | Active top-metal security mesh over active die | Detects physical intrusion and microprobing attempts. | **VERIFIED FEASIBLE** on silicon. |
| **Die Inspectability** | Exposed backside silicon (WLCSP / bare die) | Allows non-destructive Short-Wave Infrared (~1050nm) die microscopy. | **VERIFIED FEASIBLE** via IRIS methodology. |
| **USB Interconnect** | USB 2.0 High-Speed (480 Mbps) with PHY | High-speed telemetry streaming and UF2 mass storage firmware flashing. | **VERIFIED FEASIBLE** via Dabao board design. |

