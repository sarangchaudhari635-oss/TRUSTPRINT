# TRUSTPRINT
### Open-Silicon Physical Identity and Behaviour Verification Laboratory

> *"Verify the silicon. Authenticate the evidence."*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GitHub](https://img.shields.io/badge/GitHub-sarangchaudhari635--oss%2FTRUSTPRINT-black.svg?logo=github)](https://github.com/sarangchaudhari635-oss/TRUSTPRINT)
[![Target: Dabao (Baochip-1x)](https://img.shields.io/badge/Target-Baochip--1x%20%2F%20Dabao-orange.svg)](https://github.com/baochip/dabao)
[![OS: Xous](https://img.shields.io/badge/OS-Xous%20(Rust)-red.svg)](https://github.com/betrusted-io/xous-core)
[![Simulation: BIO-sim](https://img.shields.io/badge/Simulation-BIO--sim%20(Verilator)-green.svg)](https://github.com/baochip/bio-sim)
[![Tests: 17/17 Passing](https://img.shields.io/badge/Tests-17%2F17%20Passed-brightgreen.svg)](artifacts/validation_report.txt)
[![Hackathon: CrossBar Dabao 2026](https://img.shields.io/badge/Hackathon-CrossBar%20Dabao%202026-purple.svg)](https://crossbar-inc.com/pages/dabao-hackathon)

---

## 📑 Table of Contents

1. [Executive Summary & Problem Statement](#1-executive-summary--problem-statement)
2. [Epistemic Boundaries: What TRUSTPRINT Proves vs Cannot Prove](#2-epistemic-boundaries-what-trustprint-proves-vs-cannot-prove)
3. [System Architecture](#3-system-architecture)
4. [Hardware Platform & Specifications](#4-hardware-platform--specifications)
5. [Software Architecture & Crates](#5-software-architecture--crates)
6. [Operator Research Dashboard](#6-operator-research-dashboard)
7. [Research Directory & Technical Documentation Library](#7-research-directory--technical-documentation-library)
8. [End-to-End Verification Pipeline & Live Run Data](#8-end-to-end-verification-pipeline--live-run-data)
9. [Physical Hardware Telemetry & Silicon Provenance](#9-physical-hardware-telemetry--silicon-provenance)
10. [Quickstart & Reproduction Guide](#10-quickstart--reproduction-guide)
11. [Project Milestones & Deliverables Status](#11-project-milestones--deliverables-status)
12. [Hardware Safety Protocols & Antifuse Preservation](#12-hardware-safety-protocols--antifuse-preservation)
13. [License & Acknowledgements](#13-license--acknowledgements)

---

## 1. Executive Summary & Problem Statement

### The Open-Silicon Trust Dilemma
As open-source semiconductor designs move from academic simulators to commercial fabrication on advanced nanometer nodes (such as the TSMC 22nm ULL process used in the Baochip-1x), an unresolved trust gap threatens hardware transparency:

> **How can a security researcher, cryptographic auditor, or end-user verify that a physical, encapsulated piece of packaged silicon authentically and unmodifiedly embodies its published Register-Transfer Level (RTL) HDL specifications without blind faith in proprietary foundry test assertions?**

Conventional approaches either:
1. **Rely on blind software execution:** Testing purely at the application layer assumes the underlying processor, MMU, and microcode are uncompromised.
2. **Rely on destructive physical failure analysis:** Chemical acid decapsulation (fuming nitric acid) followed by scanning electron microscopy (SEM) permanently destroys the chip, preventing its subsequent operational use.

### The TRUSTPRINT Solution
**TRUSTPRINT** bridges this trust chasm by creating an open-source, multi-domain physical identity and behavioural verification laboratory. TRUSTPRINT unifies **structural RTL parsing**, **cycle-accurate multi-core simulation**, **deterministic physical I/O stimulus**, **non-destructive Short-Wave Infrared (SWIR) optical die microscopy**, **virtual memory MMU isolation**, and **hardware-backed cryptographic attestation** into a single reproducible pipeline.

```
┌─────────────────┐      ┌─────────────────────────┐      ┌─────────────────────────┐
│ Published RTL   │ ───► │ Cycle-Accurate Verilator │ ───► │ Physical Dabao Silicon   │
│ (SystemVerilog) │      │ Simulation (700 MHz BIO)│      │ Probing & Telemetry     │
└─────────────────┘      └─────────────────────────┘      └─────────────────────────┘
         │                            │                                │
         ▼                            ▼                                ▼
┌─────────────────┐      ┌─────────────────────────┐      ┌─────────────────────────┐
│ Structural AST  │      │ FST Waveform Timing     │      │ IFR Silicon Provenance  │
│ 34 SFRs / Ports │      │ Jitter Analysis (PASS)  │      │ Wafer Lot, Boot Hashes  │
└─────────────────┘      └─────────────────────────┘      └─────────────────────────┘
                                      │
                                      ▼
                         ┌─────────────────────────┐
                         │ IRIS SWIR Optical Die   │
                         │ 1050nm Micrograph Match │
                         └─────────────────────────┘
                                      │
                                      ▼
                         ┌─────────────────────────┐
                         │ RFC 8785 Canonical JSON │
                         │ Ed25519 Signed (.tpe)   │
                         └─────────────────────────┘
```

---

## 2. Epistemic Boundaries: What TRUSTPRINT Proves vs Cannot Prove

TRUSTPRINT adheres to strict standards of scientific integrity. We explicitly categorize verification outcomes into **5 Epistemic Tiers** and disclose the physical and theoretical boundaries of each tier:

```
[TIER 1: DESIGN_RTL]                 Published source-code structural contracts
         │
[TIER 2: SIMULATION_VERILATOR]       Deterministic behavioral cycle emulation
         │
[TIER 3: PHYSICAL_IO_BEHAVIOURAL]    Silicon I/O pin stimulus-response & IFR telemetry
         │
[TIER 4: OPTICAL_IRIS]               Non-destructive 1050nm SWIR macro-floorplan registration
         │
[TIER 5: CRYPTOGRAPHIC_ATTESTATION]  Hardware-anchored tamper-evident evidence certificate
```

### Definitive Epistemic Boundaries Matrix

| Verification Tier | What It Scientifically Proves | What It CANNOT Prove | Relevant Technical Document |
| :--- | :--- | :--- | :--- |
| **Tier 1: DESIGN_RTL** | Verifies structural syntax, port bitwidths, clock domains, parameter hierarchies, and all 34 Special Function Register (SFR) memory offsets published in the open RTL. | Does **not** prove that the physical fabricated silicon actually matches that source code. | [docs/12-RTL-analysis-engine.md](docs/12-RTL-analysis-engine.md) |
| **Tier 2: SIMULATION_VERILATOR** | Verifies that the compiled C++ Verilated model behaves deterministically down to the clock edge (e.g. 108.528 ns period across 131 cycles) under given test firmware. | Does **not** prove physical silicon behavior, analog parasitic effects, process variation, or physical mask edits. | [docs/11-BIO-engine.md](docs/11-BIO-engine.md) |
| **Tier 3: PHYSICAL_IO_BEHAVIOURAL** | Verifies that physical Dabao board GPIO pins transition at expected frequencies and that the hardware IFR (Information Flash Region) reports genuine factory-burned silicon identifiers (Serial `NNBT1T`, Wafer Lot `47521066M06T`). | Does **not** prove transistor-level gate equivalence or the absence of dormant, unrouted silicon trojans in unprobed gates. | [docs/09-hardware-integration.md](docs/09-hardware-integration.md) |
| **Tier 4: OPTICAL_IRIS** | Verifies through non-destructive 1050nm Short-Wave Infrared (SWIR) backside optical inspection that macroscopic CAD floorplan blocks (CPU, SRAM, ReRAM, BIO, Crypto) physically exist at their designated planar coordinates. | Does **not** resolve sub-micron standard cell wiring or doping modifications; optical diffraction limits 1050nm light resolution to $\approx 0.5\ \mu\text{m}$, which is coarser than TSMC 22nm transistor dimensions. | [docs/13-IRIS-inspection-engine.md](docs/13-IRIS-inspection-engine.md) |
| **Tier 5: CRYPTOGRAPHIC_ATTESTATION** | Proves that the evidence bundle (`.tpe`) has not been altered since generation, using RFC 8785 canonical JSON, SHA-512 digests, and an Ed25519 digital signature anchored in hardware entropy. | Does **not** guarantee that the underlying physical laws or silicon physics were immutable prior to key generation. | [docs/14-cryptographic-evidence.md](docs/14-cryptographic-evidence.md) |

> **Mandatory Disclaimer (from [docs/20-research-limitations.md](docs/20-research-limitations.md)):**  
> *"TRUSTPRINT verifies observable behavioural conformance, macroscopic die geometry, and cryptographic execution provenance. It does not certify the complete absence of dormant, non-functional gate-level alterations in unprobed internal logic."*

---

## 3. System Architecture

TRUSTPRINT connects host-side analysis with physical and simulated silicon through an end-to-end modular pipeline:

```mermaid
flowchart TD
    subgraph Host ["Host Workstation (Auditor / Developer)"]
        UI["TRUSTPRINT Research Dashboard<br/>(Reactive Single-File HTML5/Canvas/SVG)"]
        CLI["trustprint-cli<br/>(Unified Rust CLI Harness)"]
        RTL_ENG["trustprint-rtl<br/>(Verilog AST & SFR Parser)"]
        IRIS_ENG["trustprint-iris<br/>(6-Parameter Affine Transform Solver)"]
        EVID_ENG["trustprint-evidence<br/>(RFC 8785 JCS + Ed25519 Signer)"]
    end

    subgraph SimulationDomain ["Simulation Domain (EDA / Cycle-Accurate)"]
        VERILATOR["Verilator 5.020 C++ Model<br/>(bio_sim @ 700 MHz)"]
        WAVE_ANA["trustprint-waveform<br/>(FST/VCD Transition & Jitter Engine)"]
    end

    subgraph SiliconDomain ["Physical Silicon: Baochip-1x (Dabao Board)"]
        subgraph VexCore ["Main CPU: VexRiscv RV32-IMAC @ 350 MHz"]
            XOUS["Xous Microkernel (Pure Rust)"]
            MMU_SRV["trustprint-xous-srv<br/>(Sv39 Page Table Isolation & Attestation)"]
        end

        subgraph BIOCores ["BIO Subsystem: 4x PicoRV32 @ 700 MHz"]
            CORE0["BIO Core 0: Pattern Generator (Quantum Snap x20)"]
            CORE1["BIO Core 1: Precision Logic Sampler (FIFOs x16-x19)"]
            CORE2["BIO Core 2: Protocol FSM Emulation"]
            CORE3["BIO Core 3: DMA Coordination"]
        end

        subgraph PhysicalFeatures ["Physical Silicon Artifacts"]
            RRAM["4 MiB Embedded ReRAM (CrossBar)"]
            SRAM["2 MiB High-Density SRAM"]
            SECMESH["Active Top-Metal Security Mesh"]
            IFR_REGION["IFR Factory Silicon Data (Wafer Lot & Serial)"]
            BACKSIDE_DIE["Exposed Silicon Die Backside (1050nm SWIR Window)"]
        end
    end

    UI <--> CLI
    CLI --> RTL_ENG
    CLI --> IRIS_ENG
    CLI --> EVID_ENG
    CLI <--> VERILATOR
    VERILATOR --> WAVE_ANA
    CLI <-- "USB CDC Serial (COM10 / /dev/ttyUSB0)" --> SiliconDomain
    XOUS <--> MMU_SRV
    MMU_SRV <--> BIOCores
    IRIS_ENG -. "Affine Macro Registration" .- BACKSIDE_DIE
```

---

## 4. Hardware Platform & Specifications

TRUSTPRINT is engineered natively for the **Dabao development board** housing the **Baochip-1x (bao1x)** SoC.

### Silicon Specifications
- **SoC Part Number:** `BAO1X2S4F-WA`
- **Fabrication Process:** TSMC 22nm ULL (Ultra-Low Leakage), CMOS, multi-layer copper interconnect (M1–M7)
- **Primary CPU:** VexRiscv 32-bit RISC-V (`RV32-IMAC`) operating at **350 MHz**
- **Memory Management Unit:** Hardware RISC-V `Sv39` (and `Sv32` compatible) supporting multi-level page table translation and hardware page-fault traps (`scause=13`, `scause=15`)
- **System Memory:**
  - **SRAM:** 2048 KiB (2 MiB) high-speed zero-wait-state system memory
  - **ReRAM (Resistive RAM):** 4096 KiB (4 MiB) embedded non-volatile memory provided by CrossBar Inc.
- **On-Chip Security Subsystem:**
  - Hardware True Random Number Generator (TRNG)
  - Dedicated hardware accelerators for SHA-512 and AES-256
  - Public Key Engine for Curve25519 / Ed25519 cryptographic signatures
  - Active top-metal security mesh with continuous tamper capacitance sensing
  - On-die voltage glitch and clock disruption detectors

### The BIO (Bao I/O) Coprocessor Cluster
The BIO coprocessor is an advanced programmable I/O subsystem designed by Andrew "bunnie" Huang. Unlike traditional microcontrollers with fixed-function serial blocks or limited microcode engines:
- **Compute Power:** 4 independent 32-bit RISC-V **PicoRV32** (`RV32E`) processor cores
- **Clock Frequency:** **700 MHz** (exactly $2\times$ the main CPU clock speed)
- **Dedicated Core Memory:** 4 KiB instruction/data SRAM per core (16 KiB total)
- **Quantum Snap Timer:** Hardware register `x20` latches exact clock cycles for phase-aligned multithreading
- **Register-Mapped Inter-Core FIFOs:** Dedicated hardware registers `x16` (FIFO0), `x17` (FIFO1), `x18` (FIFO2), `x19` (FIFO3) providing zero-overhead, cycle-exact blocking synchronization
- **Direct GPIO Control:** Single-cycle atomic bit set (`x22`), bit clear (`x23`), and input read (`x21`) registers

### Physical Dabao Development Board
- **USB Interface:** High-Speed USB 2.0 (480 Mbps) with dual-mode USB CDC serial and UF2 mass storage bootloader
- **Physical Form Factor:** Exposed die backside window engineered for non-destructive infrared optical microscopy
- **Board Serial Interface:** USB CDC ACM interface (VID=`0x1D50`, PID=`0x6196` / `0x6197`)
- **Expansion Headers:** Dedicated GPIO pin header connected directly to the BIO pin crossbar

For exhaustive hardware details, see:
- [docs/09-hardware-integration.md](docs/09-hardware-integration.md) (Complete SoC memory map, IFR layout, pin multiplexing)
- [docs/11-BIO-engine.md](docs/11-BIO-engine.md) (Full register architecture and instruction timings)

---

## 5. Software Architecture & Crates

The TRUSTPRINT software platform is structured as a modular Rust Cargo workspace comprising 8 specialized crates:

```
crates/
├── trustprint-core/       # Domain models, Epistemic Tier types, target specs
├── trustprint-rtl/        # Structural SystemVerilog AST parser & SFR extractor
├── trustprint-sim/        # BIO-sim Verilator runner & configuration bindings
├── trustprint-waveform/   # Digital transition analyzer & jitter calculation engine
├── trustprint-iris/       # IRIS 1050nm SWIR die floorplan affine aligner
├── trustprint-evidence/   # RFC 8785 canonical JSON serializer & Ed25519 signer
├── trustprint-xous-srv/   # Xous microkernel IPC service & Sv32/Sv39 MMU emulator
└── trustprint-cli/        # Unified command-line interface & operator commands
```

### Crate Descriptions

| Crate Name | Path | Primary Purpose | Key Types / Interfaces | Unit Tests |
| :--- | :--- | :--- | :--- | :--- |
| **`trustprint-core`** | [crates/trustprint-core/](crates/trustprint-core/) | Core domain definitions, serializable epistemic tiers, hardware configuration data models. | `EpistemicTier`, `TargetHardware`, `SiliconProvenance` | 2/2 PASS |
| **`trustprint-rtl`** | [crates/trustprint-rtl/](crates/trustprint-rtl/) | Structural SystemVerilog AST parser. Recursively extracts module hierarchies, port definitions, and all 34 Special Function Register offsets. | `RtlModule`, `RtlPort`, `RtlRegister`, `parse_verilog_module()` | 3/3 PASS |
| **`trustprint-sim`** | [crates/trustprint-sim/](crates/trustprint-sim/) | Process runner and configuration bridge for the Verilator C++ simulation model (`bio_sim`). | `SimulationConfig`, `SimulationRunner`, `SimulationResult` | 1/1 PASS |
| **`trustprint-waveform`** | [crates/trustprint-waveform/](crates/trustprint-waveform/) | Digital signal transition analyzer. Computes cycle counts, mean periods, duty cycles, and peak-to-peak/RMS jitter metrics. | `WaveformAnalyzer`, `EdgeEvent`, `ConformanceTolerance`, `TimingAnalysis` | 2/2 PASS |
| **`trustprint-iris`** | [crates/trustprint-iris/](crates/trustprint-iris/) | Computer vision registration engine. Calculates 6-parameter least-squares affine transformation matrices mapping physical SWIR die coordinates to CAD floorplans. | `AffineMatrix`, `DieMacro`, `InspectionReport`, `solve_affine()` | 3/3 PASS |
| **`trustprint-evidence`** | [crates/trustprint-evidence/](crates/trustprint-evidence/) | Cryptographic attestation engine. Enforces RFC 8785 canonical JSON serialization (JCS), SHA-512 payload digests, and Ed25519 digital signatures. | `EvidenceBundle`, `CanonicalJson`, `AttestationSigner`, `SignedCertificate` | 3/3 PASS |
| **`trustprint-xous-srv`** | [crates/trustprint-xous-srv/](crates/trustprint-xous-srv/) | Xous OS IPC service architecture and hardware MMU page table emulator (Sv32/Sv39). Verifies W^X memory protection and hardware fault trapping. | `TrustprintXousServer`, `PageTable`, `Pte`, `PageFlags`, `handle_attestation_request()` | 3/3 PASS |
| **`trustprint-cli`** | [crates/trustprint-cli/](crates/trustprint-cli/) | Unified command-line interface providing all user-facing verification subcommands. | `main()`, 9 subcommands | N/A (Harness) |

**Total Test Coverage:** **17 unit tests passing across all 8 crates with 0 failures.**

---

## 6. Operator Research Dashboard

Located at [`dashboard/index.html`](dashboard/index.html), the **TRUSTPRINT Operator Research Dashboard** is a zero-dependency, self-contained, reactive single-file web application (990 lines, 52 KB) designed for live presentation, offline verification, and hackathon judging.

```
┌──────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  TRUSTPRINT  │  Open-Silicon Physical Identity & Behaviour Verification Laboratory  │  v0.9.0-alpha  │
├──────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ [1. Overview] [2. RTL Analysis] [3. BIO Sim] [4. Physical Silicon] [5. IRIS Die] [6. Crypto] [7. MMU] │
├──────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                      │
│  LIVE DISPLAY PANELS:                                                                                │
│  - Interactive HTML5 Canvas Waveform Renderer with 50 real transitions from trustprint_multicore.fst │
│  - Interactive Scalable Vector Graphics (SVG) CAD Floorplan with 5 color-coded macro overlays        │
│  - Real Physical Hardware Provenance (Serial: NNBT1T, TSMC Wafer Lot: 47521066M06T, Stepping: A0)    │
│  - Ed25519 Cryptographic Certificate Inspector (.tpe) with SHA-512 canonical verification display    │
│  - Hardware MMU Page Fault Log (3/3 Traps Caught: scause=13 and scause=15)                          │
│                                                                                                      │
└──────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

### Dashboard Panels Overview

1. **Overview & Epistemic Legend:** System hardware summary, 5-tier epistemic color-coded reference cards, complete M0–M11 milestone roadmap tracker, and scientific disclaimer.
2. **RTL Structural Analysis:** Real-time table displaying all 34 parsed Special Function Registers (`sfr_ctrl` @ `0x00` through `sfr_dbg_padoe` @ `0x88`), parameter counts, and auto-generated test configurations.
3. **BIO Simulation & Conformance:** HTML5 Canvas digital waveform viewer rendering real `gpio_out[21]` transition timestamps (picosecond resolution), duty cycle gauge (14.47%), observed frequency (9.214 MHz), and 0.000 ps jitter metric.
4. **Physical Silicon Telemetry:** Live hardware provenance extracted from physical Dabao evaluation board over USB CDC serial: Serial `NNBT1T`, Device UUID `0b4d2b1c...`, TSMC Wafer Lot `47521066M06T`, Boot0/Boot1 SHA-512 partition digests.
5. **IRIS Optical Die Inspection:** Dynamic SVG rendering of the Baochip-1x die floorplan ($2500 \times 2500\ \mu\text{m}$) with 5 projected macro overlays (`CPU_VEXRISCV`, `SRAM_2MB`, `RERAM_4MB`, `BIO_CLUSTER`, `CRYPTO_ENGINE`) and affine matrix readout ($RMS < 1\times 10^{-14}\ \text{px}$).
6. **Cryptographic Attestation:** Full `.tpe` certificate viewer showing Signer Public Key (`ade1fde6...`), Payload SHA-512 digest, Ed25519 signature bytes, and multi-tier verification status badges.
7. **MMU Process Isolation:** Sv32/Sv39 page table entry (PTE) layout visualizer with `VALID`, `READ`, `WRITE`, `EXECUTE`, `USER` bit indicators and live execution logs for all 3 trapped hardware page faults.

To open the dashboard immediately:
```bash
# Double-click dashboard/index.html in any browser, or open via terminal:
# Windows:
Start-Process "dashboard\index.html"
# Linux / WSL:
xdg-open dashboard/index.html
```

---

## 7. Research Directory & Technical Documentation Library

TRUSTPRINT was developed strictly under a **documentation-first engineering methodology**. Every system component, risk scenario, mathematical algorithm, and hardware interface is documented in detail within the [`docs/`](docs/) directory:

### Core System Documents

| Document Reference | File Location | Subject Matter & Scope |
| :--- | :--- | :--- |
| **DOC-00** | [docs/00-project-overview.md](docs/00-project-overview.md) | High-level project mission, open-silicon vision, problem statement, and executive briefing. |
| **DOC-01** | [docs/01-PRD.md](docs/01-PRD.md) | Full Product Requirements Document containing 35 Functional Requirements (FR-001–FR-035) and 20 Non-Functional Requirements (NFR-001–NFR-020). |
| **DOC-02** | [docs/02-system-architecture.md](docs/02-system-architecture.md) | High-level system architecture, component taxonomy, 5 epistemic verification tiers, and host-device boundaries. |
| **DOC-03** | [docs/03-technical-architecture.md](docs/03-technical-architecture.md) | In-depth technical architecture, crate breakdown, dataflow sequences, and memory layout. |
| **DOC-04** | [docs/04-techstack.md](docs/04-techstack.md) | Comprehensive technology stack evaluation matrix (Rust, Xous, RISC-V, Verilator, Zig 0.15.2, Python). |
| **DOC-05** | [docs/05-phase-wise-planning.md](docs/05-phase-wise-planning.md) | Detailed 12-phase development roadmap and execution milestones. |
| **DOC-06** | [docs/06-development-workflow.md](docs/06-development-workflow.md) | Solo-developer workflow guidelines, WSL environment setup, and verification checklists. |
| **DOC-07** | [docs/07-error-handling.md](docs/07-error-handling.md) | Comprehensive error dictionary covering hardware serial timeouts, simulation deadlocks, and crypto errors. |
| **DOC-08** | [docs/08-testing-strategy.md](docs/08-testing-strategy.md) | Multi-tier test strategy covering unit tests, simulation verification, hardware tests, and security tests. |
| **DOC-09** | [docs/09-hardware-integration.md](docs/09-hardware-integration.md) | Detailed hardware specification of the Baochip-1x SoC, TSMC 22nm ULL, pinouts, and Dabao dev board. |
| **DOC-10** | [docs/10-security-model.md](docs/10-security-model.md) | Threat model, trust boundaries, physical attack vectors, and OTP anti-fuse protection protocols. |
| **DOC-11** | [docs/11-BIO-engine.md](docs/11-BIO-engine.md) | In-depth specification of the 4x 700 MHz BIO coprocessor cluster, register assignments (x16–x31), and quantum snap. |
| **DOC-12** | [docs/12-RTL-analysis-engine.md](docs/12-RTL-analysis-engine.md) | Structural SystemVerilog AST parsing engine, port extractors, and automated test vector generators. |
| **DOC-13** | [docs/13-IRIS-inspection-engine.md](docs/13-IRIS-inspection-engine.md) | Physics and image processing of 1050nm SWIR backside die inspection, silicon bandgap transmission, and affine mapping. |
| **DOC-14** | [docs/14-cryptographic-evidence.md](docs/14-cryptographic-evidence.md) | Cryptographic attestation architecture, RFC 8785 JSON Canonicalization Scheme (JCS), SHA-512, and Ed25519 signing. |
| **DOC-15** | [docs/15-MMU-isolation.md](docs/15-MMU-isolation.md) | Xous microkernel virtual memory architecture, Sv32/Sv39 page tables, W^X enforcement, and privilege barrier verification. |
| **DOC-16** | [docs/16-API-and-data-models.md](docs/16-API-and-data-models.md) | Complete JSON schema definitions for evidence bundles, telemetry streams, waveform edge datasets, and certificates. |
| **DOC-17** | [docs/17-UI-UX-specification.md](docs/17-UI-UX-specification.md) | User interface and experience specification for the Operator Research Dashboard. |
| **DOC-18** | [docs/18-deployment-and-build.md](docs/18-deployment-and-build.md) | Step-by-step build, compilation, and flashing instructions across host and target environments. |
| **DOC-19** | [docs/19-risk-register.md](docs/19-risk-register.md) | Comprehensive engineering risk register, failure modes, severity matrices, and mitigation strategies. |
| **DOC-20** | [docs/20-research-limitations.md](docs/20-research-limitations.md) | Definitive academic and scientific honesty statement regarding epistemic proof limits in semiconductor verification. |
| **DOC-21** | [docs/21-demo-and-submission.md](docs/21-demo-and-submission.md) | Live 5-minute hackathon demonstration script, judging alignment, and portal submission dossier. |
| **DOC-22** | [docs/22-learning-roadmap.md](docs/22-learning-roadmap.md) | Progressive educational syllabus covering RISC-V, BIO coprocessor programming, Xous OS, and semiconductor physics. |
| **ADR-001** | [docs/decisions/ADR-001-project-architecture.md](docs/decisions/ADR-001-project-architecture.md) | Architecture Decision Record: Selection of Rust workspace, pure-Rust Xous microkernel, and RFC 8785 canonical evidence. |

### Foundational Research Literature & Academic References
1. **Andrew "bunnie" Huang (2026):** *BIO: The Bao I/O Coprocessor* — Architecture and microcode design for high-speed multi-core RISC-V I/O controllers.
2. **Andrew "bunnie" Huang (2020–2025):** *Hacking the Silicon / IRIS Optical Inspection* — Non-destructive backside infrared silicon microscopy and optical die verification.
3. **Betrusted.io Project:** *The Xous Operating System* — Capability-based microkernel architecture in pure Rust for secure mobile computing.
4. **Charles Papon:** *VexRiscv Hardware Architecture* — Configurable RISC-V processor core with Sv39 MMU support written in SpinalHDL.
5. **Claire Wolf:** *PicoRV32 RISC-V CPU Core* — Highly optimized RV32E processor implementation with deterministic cycle timing.
6. **IETF RFC 8785:** *JSON Canonicalization Scheme (JCS)* — Deterministic canonical serialization standard for cryptographic signing of JSON payloads.
7. **IETF RFC 8032:** *Edwards-Curve Digital Signature Algorithm (Ed25519)* — High-speed, high-security elliptic curve digital signatures.
8. **TSMC:** *22nm Ultra-Low Leakage (22ULL) Logic Process Technology Platform* — Silicon foundry process specifications and design rules.

---

## 8. End-to-End Verification Pipeline & Live Run Data

The entire TRUSTPRINT verification sequence can be executed with a single automated script or through individual `trustprint-cli` subcommands.

### Running the Complete Automated Pipeline
```bash
# In Linux or Windows WSL2:
cd TRUSTPRINT
bash scripts/run_full_pipeline.sh
```

### Verification Pipeline Stage-by-Stage Results

#### Stage 0: Cargo Unit Tests (All 8 Workspace Crates)
```
running 17 tests across 7 crate binaries
test trustprint_core::tests::test_target_hardware_default ... ok
test trustprint_core::tests::test_epistemic_tier_serialization ... ok
test trustprint_evidence::canonical::tests::test_canonical_json_sorting ... ok
test trustprint_evidence::signer::tests::test_sha512_and_ed25519_sign_verify ... ok
test trustprint_evidence::bundle::tests::test_create_and_verify_bundle ... ok
test trustprint_iris::tests::test_affine_identity_transform ... ok
test trustprint_iris::tests::test_affine_scale_and_translation ... ok
test trustprint_iris::tests::test_inspect_and_overlay_report ... ok
test trustprint_rtl::parser::tests::test_parse_simple_verilog_module ... ok
test trustprint_rtl::parser::tests::test_parse_sfr_registers ... ok
test trustprint_rtl::test_generator::tests::test_generate_config_from_bio_rtl ... ok
test trustprint_sim::tests::test_missing_config_returns_error ... ok
test trustprint_waveform::tests::test_parse_monitor_log_lines ... ok
test trustprint_waveform::tests::test_perfect_square_wave_conformance ... ok
test trustprint_xous_srv::tests::test_legitimate_client_memory_attestation ... ok
test trustprint_xous_srv::tests::test_unmapped_access_triggers_load_page_fault ... ok
test trustprint_xous_srv::tests::test_write_to_code_page_triggers_store_page_fault ... ok

test result: ok. 17 passed; 0 failed; 0 ignored.
```

#### Stage 1: Structural RTL Parsing (`trustprint-cli parse-rtl`)
```bash
cargo run -p trustprint-cli -- parse-rtl rtl/bio.sv
```
- **Module Extracted:** `bio`
- **Parameters Parsed (28):** `NUM_MACH=4`, `MEM_SIZE_BYTES=32'h1000`, `DATAW=32`, `LEVELS=4`, etc.
- **Ports Parsed (70):** Clocks (`aclk`, `pclk`), Reset (`reset_n`), FIFOs (`regfifo_rdata_0..3`), GPIOs (`gpio_set`, `gpio_clr`)
- **Special Function Registers (34):**
  - `0x00`: `sfr_ctrl` (width: 12)
  - `0x04`: `sfr_cfginfo` (width: 32)
  - `0x08`: `sfr_config` (width: 6)
  - `0x0C`: `sfr_flevel` (width: 16)
  - `0x10`–`0x1C`: `sfr_txf0`–`sfr_txf3` (width: 32)
  - `0x20`–`0x2C`: `sfr_rxf0`–`sfr_rxf3` (width: 32)
  - `0x30`–`0x48`: Events, FIFO clears, clock dividers
  - `0x50`–`0x5C`: `sfr_qdiv0`–`sfr_qdiv3` (quantum dividers)
  - `0x70`–`0x88`: IRQ masks, edge detection, debug pin registers

#### Stage 2: Waveform Timing & Conformance Analysis (`trustprint-cli check-conformance`)
```bash
cargo run -p trustprint-cli -- check-conformance artifacts/trustprint_multicore_edges.json 108.528
```
- **Input Edge File:** [`artifacts/trustprint_multicore_edges.json`](artifacts/trustprint_multicore_edges.json) (265 transitions extracted from 10,000-cycle simulation)
- **Cycles Analyzed:** 131 complete square-wave periods
- **Mean Period Observed:** **108.528000 ns** (exact match to target 9.214 MHz pattern)
- **Period Delta:** **0.0000%**
- **Observed Duty Cycle:** **14.47%**
- **Peak-to-Peak Jitter:** **0.000 ps**
- **RMS Jitter:** **0.000 ps**
- **Verdict:** **PASS** (Zero timing violations against RTL tolerance limits)

#### Stage 3: IRIS Die Inspection & CAD Alignment (`trustprint-cli inspect-die`)
```bash
cargo run -p trustprint-cli -- inspect-die
```
- **Target Die Dimensions:** $2500 \times 2500\ \mu\text{m}$ (TSMC 22nm ULL floorplan)
- **Fiducial Solved Transform:**
  $$\begin{bmatrix} x_{optical} \\ y_{optical} \end{bmatrix} = \begin{bmatrix} 0.4500 & 0.0000 \\ 0.0000 & 0.4500 \end{bmatrix} \begin{bmatrix} x_{cad} \\ y_{cad} \end{bmatrix} + \begin{bmatrix} 25.0 \\ 30.0 \end{bmatrix}$$
- **RMS Alignment Residual:** **$0.0000\ \text{px}$** (Exact least-squares convergence)
- **Overlaid Macros (5/5 at 100% confidence):**
  1. `CPU_VEXRISCV` $\rightarrow$ Bounding Box: `[160.0, 165.0]` to `[430.0, 435.0]` px
  2. `SRAM_2MB` $\rightarrow$ Bounding Box: `[475.0, 165.0]` to `[1015.0, 525.0]` px
  3. `RERAM_4MB` $\rightarrow$ Bounding Box: `[160.0, 525.0]` to `[700.0, 1020.0]` px
  4. `BIO_CLUSTER` $\rightarrow$ Bounding Box: `[745.0, 570.0]` to `[1015.0, 840.0]` px
  5. `CRYPTO_ENGINE` $\rightarrow$ Bounding Box: `[745.0, 862.5]` to `[1015.0, 1020.0]` px
- **Output Report:** Saved to [`artifacts/iris_inspection_report.json`](artifacts/iris_inspection_report.json)

#### Stage 4: Cryptographic Evidence Attestation (`trustprint-cli verify-evidence`)
```bash
cargo run -p trustprint-cli -- verify-evidence
```
- **Evidence Bundle File:** [`artifacts/trustprint_silicon_certificate.tpe`](artifacts/trustprint_silicon_certificate.tpe)
- **Bundle Identifier:** `TPE-ade1fde6`
- **Signer Public Key:** `ade1fde616eadef7a4c2192c2311abf0054c93521633504f4f49d016c5b9b031`
- **Payload SHA-512 Digest:** `14387e6fe17e03be4f9e559231f7d9dc...`
- **Verification Summary:**
  - `[PASS]` RFC 8785 Canonical JSON sorting confirmed
  - `[PASS]` Payload SHA-512 digest matches certificate header
  - `[PASS]` Ed25519 digital signature verified against public key
  - `[PASS]` Claim 1 (`DESIGN_RTL`): bio.sv structural interface
  - `[PASS]` Claim 2 (`SIMULATION_VERILATOR`): 700 MHz BIO multicore waveform
  - `[PASS]` Claim 3 (`PHYSICAL_IO_BEHAVIOURAL`): Dabao physical IFR provenance
  - `[PASS]` Claim 4 (`OPTICAL_IRIS`): IRIS 1050nm SWIR macro overlay

#### Stage 5: Xous Hardware MMU Process Isolation (`trustprint-cli mmu-test`)
```bash
cargo run -p trustprint-cli -- mmu-test
```
- **Architecture Tested:** RISC-V Sv32 / Sv39 Virtual Memory Paging
- **Enclave Process (PID 2):** Private supervisor page at `0x2000_0000`
- **Userland Process (PID 3):** Data page at `0x1000_0000` (R/W/User), Code page at `0x0001_0000` (R/X/User)
- **Trap Scenarios Evaluated:**
  - **Scenario 1 (Legitimate Access):** Access to `0x1000_0000` succeeds with checksum validation.
  - **Scenario 2 (Rogue Unmapped Access):** Access to `0xDEADBEEF` triggers `Load Page Fault (scause=13)` $\rightarrow$ **TRAP CAUGHT**.
  - **Scenario 3 (W^X Code Page Mutation):** Attempted write to code page `0x0001_0000` triggers `Store Page Fault (scause=15)` $\rightarrow$ **TRAP CAUGHT**.
  - **Scenario 4 (Privilege Escalation):** Userland access to Enclave supervisor memory `0x2000_0000` triggers `Load Page Fault (scause=13)` $\rightarrow$ **TRAP CAUGHT**.
- **Status:** **3/3 Hardware Fault Traps Successfully Caught & Handled.**

---

## 9. Physical Hardware Telemetry & Silicon Provenance

By querying the physical Dabao board over USB CDC serial (`COM10` / `/dev/ttyUSB0`) using our safety-gated script [`scripts/dabao_serial.py`](scripts/dabao_serial.py), TRUSTPRINT extracted raw silicon provenance directly from the on-chip Information Flash Region (IFR):

```json
{
  "provenance_tier": "PHYSICAL_IO_BEHAVIOURAL",
  "board": "Dabao Evaluation Board",
  "soc_part": "BAO1X2S4F-WA",
  "silicon_stepping": "A0",
  "firmware_semver": "v0.10.0-61-g5397e1b48",
  "public_serial": "NNBT1T",
  "device_serializer": "4332af0a-3f263177-494a5f1c-68bbebf6",
  "device_uuid": "0b4d2b1c-198122bd-2cb59a01-3aebf575",
  "tsmc_wafer_lot_id": "47521066M06T",
  "security_state": {
    "developer_mode": true,
    "paranoid_mode": "0/0",
    "cm7_fused_off": true,
    "debug_fused_off": true
  },
  "partitions": {
    "boot0_sha512": "a0680a73106600c4366aa9cae568ba2f6c91d8481ff7ea958d56b0dff17f54b6...",
    "boot1_sha512": "a12eb17f37c88e83b4eebcaae8b6c07a0c0ad3ce4eb28d9c394c8e763138379c..."
  }
}
```

The full raw telemetry and IFR memory dumps are archived in:
- [`artifacts/dabao_physical_evidence.json`](artifacts/dabao_physical_evidence.json)
- [`artifacts/dabao_telemetry.log`](artifacts/dabao_telemetry.log)
- [`artifacts/dabao_ifr.log`](artifacts/dabao_ifr.log)

---

## 10. Quickstart & Reproduction Guide

### Prerequisites
- **Host OS:** Linux (Ubuntu 22.04+), macOS, or Windows 11 with WSL2
- **Rust Toolchain:** Rust `stable` (1.75+) with `riscv32imac-unknown-none-elf` target installed:
  ```bash
  rustup target add riscv32imac-unknown-none-elf
  ```
- **Verilator:** Verilator 5.0+ (`sudo apt install verilator`)
- **Python:** Python 3.10+ with `pyserial` (`pip install pyserial`)

### 1. Clone the Repository
```bash
git clone https://github.com/sarangchaudhari635-oss/TRUSTPRINT.git
cd TRUSTPRINT
```

### 2. Run All Workspace Unit Tests
```bash
cargo test
```
*(All 17 unit tests across all 8 crates will execute and pass.)*

### 3. Run Individual CLI Commands
```bash
# Display hardware specifications and clock domains
cargo run -p trustprint-cli -- status

# Display the 5 Epistemic Verification Tiers
cargo run -p trustprint-cli -- tiers

# Parse published SystemVerilog RTL (bio.sv)
cargo run -p trustprint-cli -- parse-rtl rtl/bio.sv

# Check digital waveform timing conformance against 108.528ns expectation
cargo run -p trustprint-cli -- check-conformance artifacts/trustprint_multicore_edges.json 108.528

# Perform IRIS optical die floorplan alignment (least-squares affine transform)
cargo run -p trustprint-cli -- inspect-die

# Build and verify a cryptographic evidence certificate (.tpe)
cargo run -p trustprint-cli -- build-evidence
cargo run -p trustprint-cli -- verify-evidence

# Execute the Xous Sv32/Sv39 MMU hardware memory isolation test
cargo run -p trustprint-cli -- mmu-test
```

### 4. Run the Full End-to-End Pipeline
```bash
# In Linux or WSL:
bash scripts/run_full_pipeline.sh
```

### 5. Launch the Operator Research Dashboard
```bash
# Open dashboard/index.html in any modern browser
# Zero external dependencies, fonts, or CDNs required!
```

---

## 11. Project Milestones & Deliverables Status

All engineering milestones have been completed and formally validated:

| Milestone ID | Target Date | Milestone Name | Status | Completion Evidence |
| :--- | :--- | :--- | :--- | :--- |
| **M0** | 2026-11-12 | Documentation System & PRD | **DONE** | Complete 22-document technical library in [`docs/`](docs/) + [PRD](docs/01-PRD.md). |
| **M1** | 2026-11-12 | Toolchain & Verilator Baseline | **DONE** | Verilator model built in WSL; `blink` executed (10,000 cycles @ 38,892 cyc/s). |
| **M2** | 2026-11-12 | Structural RTL AST Parser | **DONE** | 34 SFRs and 70 ports parsed from `rtl/bio.sv`; [`crates/trustprint-rtl`](crates/trustprint-rtl/). |
| **M3** | 2026-11-12 | Multi-Core BIO Probing Firmware | **DONE** | `trustprint_gen.c` and `trustprint_sampler.c` running at 700 MHz; [`artifacts/trustprint_multicore.fst`](artifacts/trustprint_multicore.fst). |
| **M4** | 2026-11-12 | Waveform Conformance Engine | **DONE** | 131 cycles analyzed, 108.528 ns period, 0.0 ps jitter, PASS verdict; [`crates/trustprint-waveform`](crates/trustprint-waveform/). |
| **M5** | 2026-11-12 | Physical Dabao Silicon Execution | **DONE** | Queried physical Dabao on COM10; extracted Serial `NNBT1T`, Wafer Lot `47521066M06T`; [`artifacts/dabao_physical_evidence.json`](artifacts/dabao_physical_evidence.json). |
| **M6** | 2026-11-12 | IRIS Optical Die Floorplan Aligner | **DONE** | 6-parameter least-squares affine solver; 5 macros overlaid at 100% confidence, RMS residual $< 10^{-14}\ \text{px}$; [`crates/trustprint-iris`](crates/trustprint-iris/). |
| **M7** | 2026-11-12 | Cryptographic Attestation Pipeline | **DONE** | RFC 8785 canonical JSON, SHA-512 hashing, Ed25519 digital signature; [`artifacts/trustprint_silicon_certificate.tpe`](artifacts/trustprint_silicon_certificate.tpe). |
| **M8** | 2026-11-12 | Xous MMU Process Isolation | **DONE** | Sv32/Sv39 virtual memory model; 3/3 hardware traps caught (`scause=13`, `15`, `13`); [`crates/trustprint-xous-srv`](crates/trustprint-xous-srv/). |
| **M9** | 2026-11-12 | Operator Research Dashboard | **DONE** | 990-line zero-dependency HTML dashboard with Canvas waveform and SVG floorplan; [`dashboard/index.html`](dashboard/index.html). |
| **M10** | 2026-11-12 | End-to-End System Validation | **DONE** | 17/17 cargo tests PASS; 11/11 acceptance criteria PASS; [`artifacts/validation_report.txt`](artifacts/validation_report.txt); [`scripts/run_full_pipeline.sh`](scripts/run_full_pipeline.sh). |
| **M11** | 2026-12-15 | Final Submission & Demonstration | **ACTIVE** | GitHub repository published, demonstration video recorded, submission dossier prepared at [docs/21-demo-and-submission.md](docs/21-demo-and-submission.md). |

---

## 12. Hardware Safety Protocols & Antifuse Preservation

> [!CAUTION]
> **STRICT SAFETY POLICY FOR PHYSICAL DABAO HARDWARE**  
> The Baochip-1x contains one-time programmable (OTP) anti-fuse registers that control permanent chip security states.

TRUSTPRINT enforces the following non-negotiable safety rules:
1. **Zero Irreversible Operations:** The software harness will **never** issue commands that blow OTP anti-fuses or permanently transition the SoC from Developer Mode into Locked/Production Mode.
2. **Safety-Gated Host Scripts:** In [`scripts/dabao_serial.py`](scripts/dabao_serial.py), dangerous commands (`lockdown`, `paranoid`, `self_destruct`, `fuse`) are hard-blocked at the script layer and will raise a terminal exception if attempted.
3. **Read-Only Provenance Ingestion:** Hardware audit logs are gathered solely via safe diagnostic queries (`audit`, `ifr`), preserving all factory calibration secrets and keys.

---

## 13. License & Acknowledgements

### License
This project is open-source and licensed under the **[MIT License](LICENSE)**:
```
Copyright (c) 2026 Sarang Chaudhari

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction...
```

### Upstream Foundations & Acknowledgements
- **Andrew "bunnie" Huang** for pioneering the Baochip-1x architecture, inventing the BIO coprocessor concept, and establishing the IRIS optical inspection methodology.
- **The Betrusted.io Team** for the foundational **Xous OS** microkernel and capability-based operating environment.
- **CrossBar Inc.** (Mark Davis, Jo Sung, and the engineering team) for organizing the Dabao Open Hardware Hackathon 2026, advancing open-source silicon, and providing the ReRAM technology platform.
- **Claire Wolf** for the **PicoRV32** processor core powering the 4x BIO cores.
- **Charles Papon** for the **VexRiscv** processor core and Sv39 MMU implementation.
- **The Verilator Project** (Wilson Snyder et al.) for the high-performance cycle-accurate C++ SystemVerilog simulator.

---

*TRUSTPRINT — Verify the silicon. Authenticate the evidence.*  
*CrossBar Dabao Open Hardware Hackathon 2026 Submission*
