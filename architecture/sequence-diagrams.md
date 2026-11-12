# Architecture — Sequence Diagrams

**Document Reference:** ARCH-SEQ-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. End-to-End Verification Flow

```mermaid
sequenceDiagram
    autonumber
    actor Auditor as Auditor / Operator
    participant CLI as trustprint-cli
    participant RTL as trustprint-rtl
    participant Sim as bio-sim (Verilator)
    participant Dabao as Dabao Board (Silicon)
    participant IRIS as trustprint-iris
    participant Evid as trustprint-evidence

    Auditor->>CLI: trustprint run-experiment --exp-id EXP_001
    CLI->>RTL: Parse RTL Module (bio_bdma_wrapper.sv)
    RTL-->>CLI: Module AST & Port Rules
    CLI->>Sim: Run Simulation (test_probe.jsonc)
    Sim-->>CLI: Output Waveform (bio.fst)
    CLI->>CLI: Compute Conformance & Jitter
    
    CLI->>Dabao: Connect Serial (COM3) & Trigger Silicon Run
    Dabao->>Dabao: Execute Core 0 Pattern Gen @ 700 MHz
    Dabao->>Dabao: Hardware TRNG Seed Generation
    Dabao-->>CLI: Telemetry Stream (Jitter: 112 ps, Status: PASS)
    
    CLI->>IRIS: Align SWIR Micrograph with CAD Floorplan
    IRIS-->>CLI: Alignment Overlay & Correlation (0.94)
    
    CLI->>Evid: Package Artifacts (FST, Telemetry, IRIS Overlay)
    Evid->>Evid: Compute SHA-512 Hashes
    CLI->>Dabao: Request Hardware Signature(Canonical Hash)
    Dabao->>Dabao: Sign with Ed25519 Hardware Key
    Dabao-->>CLI: 64-byte Digital Signature
    CLI->>Evid: Assemble Final .tpe Package
    Evid-->>Auditor: Verified Certificate (EXP_001.tpe)
```

---

## 2. Multi-Core BIO Synchronization Flow

```mermaid
sequenceDiagram
    autonumber
    participant Host as Host APB Bus
    participant Core0 as BIO Core 0 (Generator)
    participant FIFO as Hardware Blocking FIFO (x16)
    participant Core1 as BIO Core 1 (Sampler)
    participant Pin as Physical GPIO Pin 0

    Host->>Core0: Load Firmware into 4 KiB Private RAM
    Host->>Core1: Load Firmware into 4 KiB Private RAM
    Host->>Host: Assert Core Start via APB Control Register
    
    par Core 0 Execution
        loop Every Quantum Interval
            Core0->>Pin: Toggle State (HIGH)
            Core0->>FIFO: Write Timestamp to x16 (Blocking)
            Core0->>Core0: Quantum Snap (Sleep 350 ticks)
            Core0->>Pin: Toggle State (LOW)
            Core0->>FIFO: Write Timestamp to x16 (Blocking)
            Core0->>Core0: Quantum Snap (Sleep 350 ticks)
        end
    and Core 1 Execution
        loop Continuous Sampling
            Core1->>FIFO: Read Timestamp from x16 (Blocks until data present)
            Core1->>Core1: Sample GPIO State & Measure Delta
            Core1->>Host: Stream Record to Shared DMA Buffer
        end
    end
```

