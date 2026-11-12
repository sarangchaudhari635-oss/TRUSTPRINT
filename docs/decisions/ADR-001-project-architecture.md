# ADR-001: System Architecture and Hardware-Software Boundary Decisions

**Status:** ACCEPTED  
**Date:** 2026-11-12  
**Deciders:** Sarang Chaudhari (Solo Architect & Engineer)  

---

## 1. Context & Problem Statement

Building an open-silicon physical identity and behavioral verification platform on the **Baochip-1x / Dabao** board for the CrossBar hackathon requires balancing high technical ambition with the practical execution constraints of a single developer on a 30-day timeline.

We must decide:
1. What software components run on the host workstation versus the Dabao board.
2. How to achieve hard deterministic I/O without introducing RTOS scheduling jitter.
3. How to protect cryptographic attestation keys from buggy or untrusted test code.
4. How to represent and verify experimental evidence across independent external systems.

---

## 2. Decision Outcomes

### Decision 1: Hybrid Host-Embedded Architecture
- **Decision:** Heavyweight parsing (RTL AST), waveform processing (FST analysis), and optical die micrograph registration run on the host workstation in Rust/TypeScript. The Dabao board is dedicated to high-speed physical execution (BIO coprocessor at 700 MHz), memory-isolated verification coordination (Xous userland), and hardware-backed cryptographic signing (TRNG/Ed25519).
- **Rationale:** The Baochip-1x has 2 MiB SRAM and 4 MiB ReRAM. Ingesting large 50 MB waveform files or high-resolution infrared micrographs directly on the embedded chip would exhaust memory. Distributing responsibilities aligns with real-world failure analysis workflows.

### Decision 2: Pure PicoRV32 Freestanding C for BIO Firmware
- **Decision:** Write BIO firmware routines in freestanding C and RV32E assembly compiled via Zig/Clang, rather than attempting to run a high-level language runtime inside the 4 KiB private core RAMs.
- **Rationale:** With only 4 KiB instruction/data RAM per BIO core, zero-overhead execution and direct register mapping (`x16`–`x31`) are essential.

### Decision 3: Sv39 MMU Isolation via Xous OS
- **Decision:** Deploy the verification coordinator as a separate userland service (`trustprint-srv`) under the Xous microkernel, communicating via capability-gated IPC to the hardware crypto server.
- **Rationale:** Ensures that even if an experimental stimulus vector causes a memory violation or stack overflow, the master cryptographic signing keys and kernel remain completely protected.

### Decision 4: Canonical JSON (RFC 8785) with SHA-512 and Ed25519 for Evidence
- **Decision:** Encode all evidence records into canonical JSON (`.tpe`), hash referenced artifacts with SHA-512, and sign with Ed25519.
- **Rationale:** JSON is human-readable, machine-parseable, and universally supported. Canonicalization eliminates whitespace/endianness verification failures across platforms.

---

## 3. Consequences & Trade-offs
- **Positive:** Maximum performance where needed (700 MHz BIO), high-assurance security (Sv39 MMU), independent verifiable attestation, clean solo-developer workflow.
- **Negative:** Requires maintaining cross-compilation toolchains for both host (x86_64/ARM64) and embedded targets (RV32IMAC for VexRiscv, RV32EMC for BIO).

