# Architecture — Component Decomposition

**Document Reference:** ARCH-COMP-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. High-Level Component Structure

```mermaid
graph TD
    subgraph HostWorkstation ["Host Workstation (Rust & Web)"]
        CLI["trustprint-cli<br/>(Master CLI Harness)"]
        UI["trustprint-dashboard<br/>(React / TypeScript GUI)"]
        
        subgraph HostEngines ["Core Analysis Engines"]
            RTL_ENG["trustprint-rtl<br/>(Structural Verilog Parser)"]
            SIM_ENG["trustprint-sim<br/>(bio-sim Verilator Orchestrator)"]
            WAV_ENG["trustprint-waveform<br/>(FST Conformance & Jitter)"]
            IRIS_ENG["trustprint-iris<br/>(SWIR Image Alignment)"]
            EVID_ENG["trustprint-evidence<br/>(Canonical Hasher & Ed25519)"]
        end
    end

    subgraph DabaoHardware ["Dabao Silicon (Baochip-1x)"]
        subgraph XousUserland ["Xous Sv39 Userland"]
            SRV["trustprint-srv<br/>(Verification Coordinator)"]
            CRYPTO["crypto-srv<br/>(TRNG & Key Storage)"]
            PDDB["pddb-srv<br/>(Evidence Storage)"]
        end

        subgraph BIOCoprocessor ["BIO 700 MHz Subsystem"]
            CORE0["Core 0: Pattern Gen"]
            CORE1["Core 1: Logic Sampler"]
            FIFOS["Blocking FIFOs (x16-x31)"]
            SNAP["Quantum Snap Timer"]
        end
    end

    UI <--> CLI
    CLI --> RTL_ENG
    CLI --> SIM_ENG
    CLI --> WAV_ENG
    CLI --> IRIS_ENG
    CLI --> EVID_ENG

    CLI <-- "USB-C Serial / Mass Storage" --> SRV
    SRV <--> CRYPTO
    SRV <--> PDDB
    SRV <-- "MMIO APB Bus" --> BIOCoprocessor
```

---

## 2. Component Interface Contracts

| Component | Inbound Interface | Outbound Interface | Storage / State |
| :--- | :--- | :--- | :--- |
| **`trustprint-rtl`** | Path to Verilog source files | Structured JSON AST (ports, clocks, registers) | Stateless |
| **`trustprint-sim`** | Simulation config (`.jsonc`), BIO firmware binary | Raw `.fst` waveform trace, cycle log | Temporary disk cache |
| **`trustprint-waveform`** | `.fst` waveform file, expected timing rules | Conformance verdict (PASS/FAIL), jitter statistics | Stateless |
| **`trustprint-iris`** | SWIR PNG micrograph, CAD floorplan JSON | Aligned visual overlay, correlation index | In-memory canvas |
| **`trustprint-evidence`** | Experiment manifest, artifact hashes | Signed `.tpe` canonical JSON bundle | Persistent filesystem / PDDB |
| **`trustprint-srv`** | USB CDC serial commands | BIO MMIO writes, Xous IPC messages | Xous userland heap |

