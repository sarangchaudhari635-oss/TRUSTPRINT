# Architecture — Hardware-Software Boundaries

**Document Reference:** ARCH-BOUND-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Domain Separation Matrix

A critical design choice in TRUSTPRINT is where each capability is hosted:

```mermaid
flowchart TB
    subgraph HostDomain ["Host Domain (Workstation PC)"]
        direction TB
        H1["RTL AST Structural Parser (Rust)"]
        H2["Verilator C++ Simulation Engine (bio-sim)"]
        H3["FST Waveform Trace Analysis (Rust)"]
        H4["SWIR Infrared Die Image Registration (OpenCV / Rust)"]
        H5["Interactive Research Dashboard (React / TS)"]
        H6["Independent Evidence Verifier (Rust)"]
    end

    subgraph InterconnectDomain ["Bridge Boundary (USB-C & Optics)"]
        B1["USB CDC ACM Serial (115,200 baud JSON Telemetry)"]
        B2["USB Mass Storage (UF2 Firmware Flashing)"]
        B3["SWIR 1050nm Optical Path (Physical Die Window)"]
    end

    subgraph SiliconDomain ["Physical Silicon Domain (Baochip-1x)"]
        direction TB
        S1["VexRiscv RV32-IMAC CPU @ 350 MHz (Xous OS)"]
        S2["Sv39 MMU Virtual Memory Protection"]
        S3["4x PicoRV32 BIO Cores @ 700 MHz (Hardware FIFOs)"]
        S4["On-Chip True Random Number Generator (TRNG)"]
        S5["Hardware Public Key Engine (Ed25519 Signing)"]
        S6["Active Top-Metal Security Mesh & Glitch Sensors"]
        S7["4 MiB ReRAM & 2 MiB SRAM"]
    end

    HostDomain <==> InterconnectDomain
    InterconnectDomain <==> SiliconDomain
```

---

## 2. Rationale for Boundary Placement

| Functionality | Assigned Domain | Why Not Other Domain? |
| :--- | :--- | :--- |
| **RTL Parsing** | Host | Parsing large SystemVerilog files requires high RAM (>100 MB) and disk I/O, exceeding Dabao's 2 MiB SRAM. |
| **Waveform Analysis** | Host | FST traces from 100,000 cycles can reach 20–50 MB. Processing is best performed in host memory with 64-bit SIMD math. |
| **High-Speed Probing** | Dabao Silicon | Host USB introduces millisecond-level jitter and latency. Only the on-chip 700 MHz BIO coprocessor can achieve deterministic sub-nanosecond timing. |
| **Cryptographic Root Signing** | Dabao Silicon | Signing must be anchored in the physical chip's hardware TRNG and unextractable private key to guarantee physical provenance. |
| **IRIS Image Registration** | Host | Matrix image transformations and edge filtering are computationally heavy and require floating-point hardware. |
| **Process Memory Isolation** | Dabao Silicon | Hardware Sv39 MMU enforces true memory isolation directly at the CPU bus level. |

