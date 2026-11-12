# 02 — System Architecture

**Document Reference:** ARCH-SYS-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Architectural Philosophy

TRUSTPRINT is designed around the principle of **co-verification across heterogeneous domains**: bridging high-level software abstractions, microkernel OS process isolation, cycle-accurate simulation, physical silicon coprocessor execution, and optical physics.

The architecture strictly decouples:
- **Host Tools (Workstation Domain):** Responsible for parsing RTL, managing simulation jobs, analyzing waveforms, processing IRIS optical images, and rendering the operator interface.
- **Embedded System (Dabao Silicon Domain):** Responsible for executing cycle-accurate pin stimulus across 4x 700 MHz BIO cores, maintaining high-assurance memory isolation via Xous and Sv39 MMU, and anchoring evidence in on-chip hardware cryptography (TRNG, SHA, PKE).

---

## 2. High-Level System Architecture

```mermaid
flowchart TB
    subgraph HostDomain ["Host Workstation (Auditor / Engineer)"]
        direction TB
        subgraph UI_Layer ["Presentation & Control Layer"]
            DASH["TRUSTPRINT Web Dashboard<br/>(React / TypeScript / Canvas)"]
            CLI["trustprint-cli<br/>(Rust Host Runner)"]
        end

        subgraph Analysis_Layer ["Host Intelligence Engines"]
            RTL_ENG["RTL Intelligence Engine<br/>(Structural Verilog Parser)"]
            WAV_ENG["Waveform Conformance Engine<br/>(FST/VCD Fast Edge Analyzer)"]
            IRIS_ENG["IRIS Optical Inspection Engine<br/>(SWIR Feature Alignment)"]
            VERIFY_ENG["Evidence Verification Engine<br/>(Ed25519 / SHA-512 Validator)"]
        end

        subgraph Sim_Layer ["EDA Simulation Sandbox"]
            BIOSIM["baochip/bio-sim<br/>(Verilated bio_bdma Subsystem)"]
            FST_STORAGE["Waveform Traces (.fst)"]
        end
    end

    subgraph HardwareDomain ["Dabao Silicon: Baochip-1x SoC (22nm ULL)"]
        direction TB
        subgraph MainCPU ["Main Processor: VexRiscv RV32-IMAC @ 350 MHz"]
            subgraph XousMicrokernel ["Xous Microkernel (Pure Rust)"]
                KERNEL["Xous Kernel Core<br/>(Memory, Scheduler, Interrupts)"]
            end
            
            subgraph UserlandServices ["MMU Isolated Userland Services (Sv39)"]
                TP_SRV["trustprint-srv<br/>(Experiment Coordinator)"]
                CRYPTO_SRV["Crypto & Key Store Server<br/>(Hardware TRNG / Ed25519)"]
                PDDB_SRV["PDDB Storage Server<br/>(Encrypted Database)"]
                UART_SRV["Serial Console Server<br/>(Telemetry Streamer)"]
            end
        end

        subgraph BIOCoprocessor ["BIO Coprocessor Subsystem @ 700 MHz"]
            direction LR
            BIO0["Core 0<br/>Stimulus Gen<br/>(RV32E)"]
            BIO1["Core 1<br/>Logic Sampler<br/>(RV32E)"]
            BIO2["Core 2<br/>Protocol FSM<br/>(RV32E)"]
            BIO3["Core 3<br/>DMA Coord<br/>(RV32E)"]
            FIFOS["Hardware Blocking FIFOs<br/>(Upper Registers x16-x31)"]
            QUANTUM["Quantum Snap Timer"]
            BIO0 --- FIFOS --- BIO1
            BIO2 --- FIFOS --- BIO3
        end

        subgraph PhysicalAssets ["Physical On-Chip & Board Features"]
            RRAM_MEM["4 MiB Embedded ReRAM"]
            SRAM_MEM["2 MiB High-Speed SRAM"]
            SEC_MESH["Top-Metal Active Security Mesh"]
            PINS["Dabao Exposed GPIO Headers"]
            DIE_BACKSIDE["Exposed Die Backside (SWIR 1050nm Window)"]
        end
    end

    %% Host Interconnections
    DASH <--> CLI
    CLI --> RTL_ENG
    CLI --> WAV_ENG
    CLI --> IRIS_ENG
    CLI --> VERIFY_ENG
    CLI <--> BIOSIM
    BIOSIM --> FST_STORAGE
    FST_STORAGE --> WAV_ENG

    %% Host to Hardware Interconnections
    CLI <-- "USB-C (High-Speed CDC / Mass Storage UF2)" --> MainCPU
    IRIS_ENG -. "SWIR Optical Microscopy" .-> DIE_BACKSIDE

    %% Silicon Internal Interconnections
    KERNEL --- TP_SRV
    TP_SRV <--> CRYPTO_SRV
    TP_SRV <--> PDDB_SRV
    TP_SRV <--> UART_SRV
    TP_SRV <-- "MMIO APB Control / Shared RAM" --> BIOCoprocessor
    BIOCoprocessor <--> PINS
    MainCPU --- RRAM_MEM
    MainCPU --- SRAM_MEM
    MainCPU --- SEC_MESH
```

---

## 3. Subsystem Decomposition

### 1. RTL Intelligence Engine (`trustprint-rtl`)
- **Role:** Parses published open-source Baochip-1x Verilog source files.
- **Inputs:** Verilog source trees (`bao1x/rtl/*.v`), module parameter headers.
- **Outputs:** Module interface JSON AST, signal name maps, clock frequency definitions, expected register address maps.
- **Execution Domain:** Host workstation (Rust / Python).

### 2. BIO Simulation Engine (`trustprint-sim`)
- **Role:** Provides a software-safe test harness for cycle-accurate stimulus and response verification prior to silicon deployment.
- **Inputs:** BIO firmware binaries (`.bin`), simulation configuration scripts (`.jsonc`).
- **Outputs:** Cycle-accurate `.fst` waveforms, execution trace logs, core register dumps.
- **Execution Domain:** Host workstation (Verilator C++ executable).

### 3. BIO Silicon Experiment Engine (`trustprint-bio`)
- **Role:** Drives physical pin stimulus and real-time logic analysis directly on the Baochip-1x silicon.
- **Architecture:** Quad-core PicoRV32 (RV32E) cluster clocked at 700 MHz (2x CPU clock).
  - Uses upper registers (`x16`–`x31`) as hardware blocking queues for zero-overhead inter-core synchronization.
  - Uses the "snap to quantum" register to align edge transitions precisely to clock quanta, eliminating instruction-timing jitter.
- **Execution Domain:** Dabao BIO coprocessor cores.

### 4. Waveform & Conformance Engine (`trustprint-waveform`)
- **Role:** Ingests simulation or physical logic capture traces, measures signal properties, and checks conformance against RTL specifications.
- **Metrics:** Clock period, duty cycle, transition rise/fall intervals, edge-to-edge jitter.
- **Verdict Logic:** PASS (within $\pm$ 2% tolerance), FAIL (out of envelope), INCONCLUSIVE (missing edge events).
- **Execution Domain:** Host workstation.

### 5. IRIS Optical Inspection Engine (`trustprint-iris`)
- **Role:** Bridges macro-scale physical die layout with CAD floorplan blueprints.
- **Mechanism:** Ingests Short-Wave Infrared (~1050nm) micrographs of the Baochip-1x exposed backside die. Segments boundaries of major physical features (SRAM banks, ReRAM blocks, VexRiscv core, BIO cluster, security mesh perimeter).
- **Outputs:** Calibrated visual overlay and structural correlation index.
- **Execution Domain:** Host workstation.

### 6. Cryptographic Evidence & Attestation Engine (`trustprint-evidence`)
- **Role:** Creates tamper-evident, verifiable proof bundles of all verification activities.
- **Mechanism:** Aggregates experiment parameters, hashes external artifacts (RTL, firmware, waveforms, IRIS images) with SHA-512, and cryptographically signs the canonical bundle with Ed25519.
- **Execution Domain:** Dabao hardware crypto domain (production) or host development key (testing fallback).

### 7. Xous Microkernel & Process Isolation Layer (`trustprint-srv`)
- **Role:** Manages the lifecycle of verification routines on the Dabao board while enforcing strict memory protection.
- **Architecture:** Pure-Rust capability-based microkernel with Sv39 virtual memory. Isolates test coordination logic from cryptographic key stores, preventing arbitrary test vectors from compromising system root keys.
- **Execution Domain:** Dabao VexRiscv 350 MHz CPU under Xous OS.

### 8. Operator Research Dashboard (`trustprint-dashboard`)
- **Role:** Provides an intuitive graphical control center for researchers, auditors, and hackathon evaluators.
- **Features:** Real-time hardware telemetry, live serial log viewer, interactive waveform trace display, optical IRIS image comparison viewer, and cryptographic certificate verifier.
- **Execution Domain:** Host workstation browser (React / TypeScript).

---

## 4. Hardware-Software Boundaries

```mermaid
flowchart LR
    subgraph Host ["Host Workstation"]
        H_RTL["RTL Sources"]
        H_SIM["Verilator (bio-sim)"]
        H_IMG["IRIS Images"]
        H_UI["Dashboard"]
    end

    subgraph Boundary ["Communication Boundary"]
        USB_MASS["USB Mass Storage (UF2 Flashing)"]
        USB_CDC["USB CDC ACM (UART Telemetry)"]
        OPTICAL["SWIR Infrared Optical Path"]
    end

    subgraph Silicon ["Baochip-1x Silicon"]
        S_BOOT["ROM Bootloader"]
        S_XOUS["Xous Userland (Sv39)"]
        S_BIO["BIO Cores (700 MHz)"]
        S_PINS["GPIO Pins"]
        S_DIE["Silicon Backside"]
    end

    H_SIM -.->|"Firmware Binaries"| USB_MASS
    USB_MASS --> S_BOOT
    S_BOOT --> S_XOUS
    S_XOUS <-->|"Control Registers"| S_BIO
    S_BIO <--> S_PINS
    S_XOUS <-->|"Telemetry Stream"| USB_CDC
    USB_CDC <--> H_UI
    H_IMG -.->|"SWIR 1050nm"| OPTICAL
    OPTICAL -.-> S_DIE
```

---

## 5. Security & Isolation Architecture

The system enforces three concentric rings of trust:
1. **Ring 0: Silicon Hardware Security Subsystem**
   - Active top-metal security mesh (monitors physical tampering/microprobing).
   - Glitch sensors (monitors voltage and clock anomalies).
   - Hardware True Random Number Generator (TRNG) and secure key storage.
2. **Ring 1: Xous Microkernel & Sv39 MMU**
   - Strictly isolated virtual address spaces per userland server.
   - Capability-based message routing.
   - Memory-mapped I/O is restricted exclusively to authorized driver processes.
3. **Ring 2: Experimental Userland (`trustprint-srv`)**
   - Coordinates experiment execution, triggers BIO routines, and streams telemetry.
   - Completely isolated from master cryptographic key storage.

