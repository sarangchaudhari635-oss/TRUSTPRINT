# Architecture — Data Flow & Transformation Pipeline

**Document Reference:** ARCH-DATA-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. End-to-End Verification Data Pipeline

The data flow connects static source code, simulation artifacts, physical telemetry, optical imagery, and cryptographic certificates:

```mermaid
flowchart TD
    %% Stage 1: Design
    subgraph S1 ["Stage 1: Design Ingestion"]
        RTL_SRC["Baochip-1x Verilog Sources (*.v)"] --> RTL_PARSER["RTL Structural Parser"]
        RTL_PARSER --> RTL_AST["Module Interface AST (JSON)"]
        RTL_AST --> TEST_SPEC["Expected Timing Spec & Port Rules"]
    end

    %% Stage 2: Simulation
    subgraph S2 ["Stage 2: Simulation & Emulation"]
        TEST_SPEC --> SIM_RUNNER["BIO-sim Verilator Runner"]
        BIO_SRC["BIO Firmware (C / ASM)"] --> ZIG_COMPILER["Zig / Clang Compiler (RV32E)"]
        ZIG_COMPILER --> BIO_BIN["Compiled BIO Binary (*.bin)"]
        BIO_BIN --> SIM_RUNNER
        SIM_RUNNER --> FST_TRACE["Simulation Waveform (*.fst)"]
    end

    %% Stage 3: Conformance
    subgraph S3 ["Stage 3: Conformance Analysis"]
        FST_TRACE --> WAV_PARSER["Fast FST Edge Extractor"]
        TEST_SPEC --> WAV_COMPARATOR["Parametric Timing Comparator"]
        WAV_PARSER --> WAV_COMPARATOR
        WAV_COMPARATOR --> VERDICT["Conformance Verdict (PASS/FAIL + Jitter)"]
    end

    %% Stage 4: Physical Probing
    subgraph S4 ["Stage 4: Physical Silicon Execution"]
        BIO_BIN --> DABAO_DEPLOY["Dabao Board UF2 Deployer"]
        DABAO_DEPLOY --> SILICON["Physical Baochip-1x BIO Cores (700 MHz)"]
        SILICON --> UART_STREAM["UART Telemetry Stream (JSON)"]
    end

    %% Stage 5: Optical Inspection
    subgraph S5 ["Stage 5: Optical Die Inspection"]
        SWIR_IMG["SWIR Die Micrograph (1050nm PNG)"] --> IRIS_ALIGN["IRIS Alignment Engine"]
        CAD_FLOORPLAN["Baochip-1x Floorplan Geometry"] --> IRIS_ALIGN
        IRIS_ALIGN --> OVERLAY_IMG["Calibrated Visual Overlay"]
        IRIS_ALIGN --> CORR_METRIC["Structural Correlation Index"]
    end

    %% Stage 6: Attestation
    subgraph S6 ["Stage 6: Evidence Attestation"]
        VERDICT --> EVID_BUILDER["Evidence Aggregator"]
        UART_STREAM --> EVID_BUILDER
        OVERLAY_IMG --> EVID_BUILDER
        CORR_METRIC --> EVID_BUILDER
        EVID_BUILDER --> HASH_TREE["Artifact SHA-512 Hash Manifest"]
        HASH_TREE --> CANONICAL_PAYLOAD["RFC 8785 Canonical JSON"]
        CANONICAL_PAYLOAD --> HW_SIGNER["Hardware TRNG / Ed25519 Engine"]
        HW_SIGNER --> SIGNED_TPE["Signed Evidence Package (*.tpe)"]
    end
```

---

## 2. Data Transformation Formats

1. **RTL Source $\to$ Interface AST:** Unstructured Verilog text is transformed into strongly typed JSON structures defining ports, widths, and clock domains.
2. **C Source $\to$ RV32E Machine Code:** High-level C code is compiled into 32-bit RISC-V embedded instructions with memory-mapped register loads.
3. **Binary FST $\to$ Parametric Edge Events:** Continuous binary waveform signals are parsed into discrete transition timestamps ($t_{rise}, t_{fall}$).
4. **Physical Telemetry $\to$ Statistical Metrics:** Raw cycle counts are transformed into mean period, duty cycle, and peak-to-peak jitter ($\Delta t$).
5. **Raw Micrograph $\to$ Registered Coordinate Canvas:** Pixel coordinates ($u, v$) are converted to chip physical coordinates ($x, y$ in $\mu\text{m}$).
6. **Heterogeneous Evidence $\to$ Canonical Digest:** All verification artifacts are hashed via SHA-512 and signed into a portable `.tpe` certificate.

