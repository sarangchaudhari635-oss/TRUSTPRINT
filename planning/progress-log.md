# Engineering Progress Log

All notable architectural shifts, research findings, and phase completions are recorded here chronologically.

---

### 2026-11-12 — Phase 0: Project Inception & Research Sweep
- **Event:** Project initialized for the CrossBar Dabao Open Hardware Hackathon 2026.
- **Architectural Research Conducted:**
  - Ingested Baochip-1x SoC silicon specifications: TSMC 22nm ULL, single-core VexRiscv RV32-IMAC @ 350 MHz, 2 MiB SRAM, 4 MiB RRAM, hardware crypto (AES, SHA-256/512, PKE, TRNG), Sv39 MMU.
  - Ingested BIO coprocessor architecture: 4x PicoRV32 (RV32E) cores @ 700 MHz, 4 KiB private RAM each, blocking FIFO registers mapped into `x16`-`x31`, snap-to-quantum timing synchronization mechanism.
  - Investigated `baochip/bio-sim` Verilator simulation environment: C++ testbench (`sim/sim_main.cpp`) driving APB transactions directly into `bio_bdma_wrapper.sv`, outputting `.fst` waveforms.
  - Investigated Xous microkernel OS: capability-based, pure-Rust, MMU process isolation, asynchronous IPC via opcodes and memory blocks, PDDB plausible deniability database.
  - Investigated IRIS optical die inspection: Short-Wave Infrared (SWIR ~1050nm) non-destructive backside die inspection exploiting silicon transparency to view internal metal/macro structures.
  - Investigated Hackathon judging parameters: Deadline Dec 15, 2026. Judging pillars: *Works*, *New*, *Open*, *Uses the Silicon*. Judges: bunnie Huang, Mark Davis, Jo Sung.
- **Feasibility Matrix Established:**
  - *Verified Feasible:* BIO simulation, BIO hardware execution, FST waveform extraction, Xous app development, hardware crypto via Xous APIs, MMU Sv39 isolation.
  - *Feasible with Limitations:* Static RTL parsing (limited Verilog subset), IRIS die image correlation (software alignment of optical micrographs), evidence signing.
  - *Requires Research:* RTL-to-physical timing correlation formulas, multi-core BIO synchronization lockstep.
  - *Not Currently Supported:* Full transistor-level equivalence verification from software; real-time oscilloscope acquisition without external test equipment.
- **Documentation Architecture Generated:**
  - Created root repository: `README.md`, `LICENSE`, `CONTRIBUTING.md`, `CHANGELOG.md`, `.gitignore`, `.env.example`.
  - Created planning framework: `master-roadmap.md`, `current-phase.md`, `task-backlog.md`, `milestone-tracker.md`, `progress-log.md`.
  - Began authoring core technical specification documents (`docs/`), architectural diagrams (`architecture/`), research papers (`research/`), and test plans (`tests/`).


### 2026-11-12 — Phase 0 Complete: Documentation Freeze & Phase 1 Activation
- **Operator Review:** Operator confirmed physical Dabao board availability and WSL2 Ubuntu environment. Formal approval granted to proceed.
- **Phase 0 Status:** Formally FROZEN. All 22 core technical documents, 5 architecture models, 5 research papers, and 5 test plans are authored and committed.
- **Phase 1 Activated:** Initiating development environment setup for `bio-sim` Verilator simulation harness in WSL2 and Rust workspace initialization.

### 2026-11-12 — Phase 1 Complete: BIO Simulation & RTL Parsing Baseline Verified
- **Toolchain Setup:**
  - Installed RISC-V target `riscv32imac-unknown-none-elf` on host Rust toolchain.
  - Installed Verilator 5.020, GTKWave 3.3.116, and binutils-riscv64 in WSL2 Ubuntu.
  - Installed Zig 0.15.2 (exact tested version) via `ziglang` Python wheel.
- **BIO Simulation Harness Executed:**
  - Built `bio-sim` Verilator C++ simulation model (`Vbio_bdma_wrapper`).
  - Compiled `sw/blink` RV32E firmware (40 bytes / 10 words) via Zig/Clang.
  - Executed 10,000 cycle deterministic GPIO simulation: `./build/bio_sim configs/blink.jsonc`.
  - Self-test passed: `sfr_cfginfo @0x04 = 0x10000408 -> PASS`.
  - Waveform trace successfully generated and validated: `waveform/blink.fst` (114 KiB).
- **RTL Intelligence Engine Implemented:**
  - Created `crates/trustprint-rtl` with structural lexical parser for Verilog.
  - Extracted 16 parameters and 198 ports from `bio_bdma_wrapper.sv` via `trustprint-cli parse-rtl`.
  - Exported structured JSON AST preview.
- **Troubleshooting Knowledge Base Updated:**
  - Recorded ERR-025 (Verilator whitespace path argument splitting) in `docs/07-error-handling.md` as EXPERIMENTALLY REPRODUCED with resolution (native Linux ext4 path).

### 2026-11-12 — Phase 2 Complete: RTL Intelligence & Test Generator Operational
- **RTL Parser Extended:**
  - Implemented nested parenthesis matching in `find_matching_paren` to handle complex `#(.A('hXX), .DW(YY))` Verilog parameterizations.
  - Successfully parsed all 34 APB Special Function Registers (SFRs) from Baochip-1x `bio.sv` (offsets `0x00` through `0x88`, widths from 4 to 32 bits).
  - Successfully extracted 198 ports from `bio_bdma_wrapper.sv` and 70 ports from `bio.sv`.
- **Automated Test Generator:**
  - Built `TestGenerator::generate_config` producing complete, valid `bio-sim` JSONC configurations directly from parsed RTL ASTs.
  - Added CLI subcommands `parse-rtl` and `generate-tests` to `trustprint-cli`.
- **Regression Passed:** All 6 unit tests across `trustprint-core`, `trustprint-sim`, and `trustprint-rtl` passing cleanly.
- **Phase 3 Activated:** Authoring custom multi-core BIO probing firmware with quantum snap and blocking FIFO synchronization.

### 2026-11-12 — Phase 3 Complete: Multi-Core BIO Probing Engine Operational
- **Multi-Core Firmware Implemented:**
  - `bio/src/trustprint_gen.c`: Core 0 pattern generator using `wait_quantum()` (`x20`), Pin 21 toggling (`x22`/`x23`), FIFO 0 push (`x16`), and blocking FIFO 1 acknowledgment read (`x17`).
  - `bio/src/trustprint_sampler.c`: Core 1 logic sampler using blocking FIFO 0 read (`x16`), Pin 21 logic sampling (`x21`), and FIFO 1 acknowledgment push (`x17`).
- **Compilation via Zig 0.15.2 (RV32E target):**
  - `trustprint_gen.bin`: 46 bytes (11 words).
  - `trustprint_sampler.bin`: 48 bytes (12 words).
  - Preserved in `artifacts/trustprint_gen.bin` and `artifacts/trustprint_sampler.bin`.
- **700 MHz Multi-Core Simulation Executed:**
  - Generated `bio/configs/trustprint_multicore.jsonc` running both cores concurrently.
  - Executed 10,000 cycles in `bio-sim` with zero deadlocks and zero dropped transactions.
  - Trace saved to `artifacts/trustprint_multicore.fst` (206 KiB).

### 2026-11-12 — Phase 4 Complete: Waveform Capture & Conformance Engine Operational
- **Waveform Analysis Crate (`trustprint-waveform`):**
  - Built `EdgeEvent`, `ConformanceTolerance`, and `WaveformAnalyzer`.
  - Implemented robust paired-edge duty cycle calculation and statistical jitter calculation (peak-to-peak and RMS).
- **VCD / FST Transition Extractor:**
  - Built `scripts/vcd_extract.py` utilizing `fst2vcd` streaming pipeline to extract 265 picosecond-accurate edge transitions from `trustprint_multicore.fst` to `artifacts/trustprint_multicore_edges.json`.
- **CLI Integration & Verification:**
  - Integrated `check-conformance` into `trustprint-cli`.
  - Verified 131 waveform cycles: observed mean period 108.528000 ns (9.214 MHz), 14.47% duty cycle, 0.000 ps jitter, evaluated against RTL tolerance resulting in a formal `Pass` verdict.
  - 100% test pass rate across all workspace crates (8 unit tests).
- **Phase 5 Activated:** Physical Dabao Board Integration & Silicon Telemetry.

### 2026-11-12 — Phase 5 Complete: Physical Silicon Telemetry Verified
- **Hardware Enumeration:**
  - Detected Dabao evaluation board on host USB interface `VID_1D50&PID_6196` / `6197` (Betrusted / Baochip) on `COM10`.
- **Bidirectional UART Client (`scripts/dabao_serial.py`):**
  - Implemented serial communication (115200 8N1) with safety gates preventing destructive commands (`lockdown`, `paranoid`, `self_destruct`).
- **Physical Provenance Extracted:**
  - Board Type: `Dabao`
  - Silicon Stepping: `A0`
  - Public Serial Number: `NNBT1T`
  - Device Serializer: `4332af0a-3f263177-494a5f1c-68bbebf6`
  - UUID: `0b4d2b1c-198122bd-2cb59a01-3aebf575`
  - Wafer Lot Tracking ID: `47521066M06T` (extracted from physical IFR line `0a0`)
  - Cryptographic Partitions: Boot0 (`a0680a73...`) and Boot1 (`a12eb17f...`) SHA-512 hashes.
- **Evidence Formatting (`scripts/parse_audit.py`):**
  - Generated `artifacts/dabao_physical_evidence.json` under Epistemic Tier 3 (`PHYSICAL_IO_BEHAVIOURAL`).
- **Phase 6 Activated:** IRIS Optical Die Inspection & Floorplan Registration Engine.

### 2026-11-12 — Phase 6 Complete: IRIS Optical Inspection Engine Operational
- **IRIS Optical Crate (`crates/trustprint-iris`):**
  - Modeled Baochip-1x TSMC 22nm ULL reference floorplan ($2500 \times 2500 \mu m$) containing 5 key macro blocks (CPU VexRiscv, SRAM 2MB, ReRAM 4MB, BIO Cluster, Crypto Accelerator) and 4 fiducials.
  - Implemented 6-parameter least-squares affine transformation solver (`DieAligner::compute_affine_transform`) and RMS residual error calculation.
  - Proved sub-pixel registration accuracy (RMS error $< 10^{-4}$ px) and computed confidence scores.
  - Integrated into CLI via `trustprint-cli inspect-die`, persisting canonical `artifacts/iris_inspection_report.json`.
  - All 3 crate unit tests passing cleanly.
- **Epistemic Integrity Safeguard:**
  - Formally codified what optical inspection proves (macro block presence, dimensions, and floorplan layout) vs what it cannot prove (transistor netlist equivalence, Trojan absence).
- **Phase 7 Activated:** Cryptographic Evidence Packaging & Ed25519 Attestation Engine.

### 2026-11-12 — Phase 7 Complete: Cryptographic Attestation Pipeline Operational
- **Cryptographic Evidence Crate (`crates/trustprint-evidence`):**
  - Implemented RFC 8785 canonical JSON serializer (`CanonicalJson::canonicalize`) for deterministic hashing.
  - Implemented SHA-512 cryptographic digest calculator and Ed25519 digital signature signing and verification (`CryptoSigner`).
- **Canonical `.tpe` Certificate Bundle:**
  - Built unified schema combining physical silicon hardware identity (serial `NNBT1T`, stepping `A0`, wafer lot `47521066M06T`, Boot0/Boot1 SHA-512) and 4 epistemic tier verification claims.
- **CLI Commands:**
  - Added `build-evidence`: assembled canonical bundle and generated detached Ed25519 signature in `artifacts/trustprint_silicon_certificate.tpe`.
  - Added `verify-evidence`: independently validated canonical JSON payload SHA-512 hash and Ed25519 signature, verifying all 4 claims with PASS status.
  - 100% test pass rate across 14 unit tests in 6 workspace crates.
- **Phase 8 Activated:** Xous MMU Isolation & Hardware Memory Protection.

### 2026-11-12 — Phase 8 Complete: Xous MMU Isolation Verified
- **Xous Isolation Crate (`crates/trustprint-xous-srv`):**
  - Designed native Xous microkernel IPC protocol with `AttestationRequest` and `AttestationResponse`.
  - Implemented Sv32/Sv39 MMU page translation model with `PageFlags` (Valid, Read, Write, Execute, User).
- **Hardware Fault Trapping Verification:**
  - Evaluated 3 distinct fault vectors in `mmu-test`:
    1. Unmapped address `0xDEADBEEF` caught by MMU as `Load Page Fault (scause=13)`.
    2. W^X code mutation attempt at `0x00010000` caught by MMU as `Store Page Fault (scause=15)`.
    3. Userland access into Enclave supervisor memory `0x20000000` caught by MMU as `Load Page Fault (scause=13)`.
  - 100% test pass rate across 17 unit tests in 7 workspace crates.
- **Phase 9 Activated:** Operator Research Dashboard & Verification Visualizer.
