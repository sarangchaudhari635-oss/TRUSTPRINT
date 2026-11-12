# 22 — Progressive Learning Roadmap for Solo Developer

**Document Reference:** LEARN-ROADMAP-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Developer Mentorship Philosophy

This document provides a progressive, concept-by-concept learning curriculum designed to transition a beginner to a proficient embedded hardware-software systems engineer across the 30-day timeline.

---

## 2. Progressive Knowledge Modules

### Module 1: The RISC-V Instruction Set & Embedded Profiles
- **Concepts:** Base integer ISA (`RV32I`), the embedded register-reduced subset (`RV32E` with 16 registers), compressed instructions (`C`), multiplication (`M`), atomic (`A`).
- **Why Needed:** The Dabao main CPU is RV32IMAC (VexRiscv), while the BIO coprocessor cores are RV32EMC (PicoRV32).
- **Practical Milestone:** Understand why BIO assembly code must restrict general registers to `x0`–`x15`.

### Module 2: The Baochip BIO Coprocessor & Deterministic I/O
- **Concepts:** The limitation of CISC state machines (RP2040 PIO area explosion), hardware blocking FIFOs (`x16`–`x31`), the quantum snap timer, inter-core pipeline synchronization.
- **Why Needed:** Core implementation mechanism for generating sub-nanosecond jitter-free pin stimulus at 700 MHz.
- **Practical Milestone:** Write a 20-line C program that toggles a pin synchronously on exact quantum boundaries.

### Module 3: Verilator & Cycle-Accurate Hardware Simulation
- **Concepts:** How Verilator translates synthesizable Verilog into optimized C++ simulation models, clock cycle stepping, VCD vs FST trace formats.
- **Why Needed:** Safe software verification before touching physical hardware.
- **Practical Milestone:** Run `make build` and `./simulate configs/hello-world.jsonc` in `baochip/bio-sim` and inspect waveforms in GTKWave.

### Module 4: Xous Operating System & Sv39 MMU Isolation
- **Concepts:** Microkernel vs monolithic architecture, capability-based security, RISC-V Sv39 virtual memory, asynchronous IPC message passing, PDDB.
- **Why Needed:** High-assurance process isolation protecting cryptographic keys from untrusted test code.
- **Practical Milestone:** Implement a minimal Xous userland server that registers with the name server and handles incoming scalar messages.

### Module 5: Hardware Cryptography & Attestation
- **Concepts:** Entropy sources (TRNG), cryptographic hashing (SHA-512), digital signatures (Ed25519), canonical JSON serialization (RFC 8785).
- **Why Needed:** Creating verifiable, tamper-evident evidence packages.
- **Practical Milestone:** Generate a self-signed JSON evidence bundle and verify its cryptographic integrity on an external host.

### Module 6: Non-Destructive Optical Die Inspection (IRIS)
- **Concepts:** Silicon bandgap physics (1.12 eV), optical transparency in the Short-Wave Infrared (~1050 nm) spectrum, backside packaging, 2D affine image registration.
- **Why Needed:** Correlating physical silicon die micrographs with CAD floorplans.
- **Practical Milestone:** Project published Baochip-1x CAD bounding boxes onto an infrared die photograph.

