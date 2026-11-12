# 05 — Phase-Wise Planning & Implementation Guide

**Document Reference:** PLAN-PHASE-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

This document defines the actionable implementation plan across all 12 development phases. Every phase specifies exact files, commands, expected outputs, test cases, and recovery procedures for a solo developer.

---

## Phase 0: Research, Feasibility & Formal Documentation
- **Timeline:** Nov 12 – Nov 15, 2026
- **Objective:** Finalize all system requirements, epistemic boundaries, hardware capability audits, and system architecture specifications prior to code execution.
- **Prerequisites:** Access to official repositories (`baochip/dabao`, `baochip/bio-sim`, `betrusted-io/xous-core`).
- **Implementation Tasks:**
  1. Audit Baochip-1x silicon specifications (TSMC 22nm ULL, 4x 700 MHz BIO, Sv39 MMU, TRNG).
  2. Define epistemic boundaries distinguishing simulation, hardware I/O, optical inspection, and crypto evidence.
  3. Author all 22 technical documentation files (`docs/`), planning documents, and architecture models.
- **Expected Files:**
  - `README.md`, `LICENSE`, `CONTRIBUTING.md`, `CHANGELOG.md`, `.gitignore`, `.env.example`
  - `docs/00` through `docs/22`
  - `planning/*`, `architecture/*`, `research/*`, `tests/*`
- **Commands to Execute:**
  ```bash
  # Check document tree completeness
  find docs/ architecture/ research/ tests/ -name "*.md" | wc -l
  ```
- **Acceptance Criteria:** Zero empty documents; all technical claims verified against official code repositories.
- **Common Errors & Recovery:**
  - *Error:* Unverified assumptions about hardware APIs.
  - *Recovery:* Conduct explicit web/source search across `baochip` repositories before freezing design.

---

## Phase 1: Development Environment & BIO Simulator Harness
- **Timeline:** Nov 15 – Nov 19, 2026
- **Objective:** Establish the local host development toolchain, compile `baochip/bio-sim`, and execute cycle-accurate baseline simulations.
- **Required Knowledge:** Verilator build conventions, C++17 compilation, RISC-V cross-compilation.
- **Prerequisites:** C++ compiler, Verilator 5.0+, Make, Git.
- **Implementation Tasks:**
  1. Clone `baochip/bio-sim` and build the Verilated C++ model.
  2. Run `configs/smoke.jsonc` and `configs/hello-world.jsonc`.
  3. Validate `.fst` waveform generation in `waveform/bio.fst`.
  4. Write a host-side Rust runner (`trustprint-sim`) to invoke simulations programmatically.
- **Expected Files:**
  - `crates/trustprint-sim/src/lib.rs`
  - `crates/trustprint-sim/src/runner.rs`
  - `crates/trustprint-sim/Cargo.toml`
  - `scripts/run_simulation.py`
- **Commands to Execute:**
  ```bash
  cd ../bio-sim
  make build
  ./simulate configs/hello-world.jsonc
  gtkwave waveform/bio.fst
  ```
- **Expected Output:**
  ```
  Simulation completed: 10000 cycles.
  Waveform written to waveform/bio.fst.
  Exit code: 0
  ```
- **Acceptance Criteria:** `bio-sim` compiles cleanly without fatal warnings and outputs viewable waveforms.
- **Common Errors & Recovery:**
  - *Error:* `verilator: command not found`.
  - *Recovery:* Install Verilator via package manager (`sudo apt install verilator`) or build from source v5.020+.

---

## Phase 2: RTL Intelligence & Signal Parsing Engine
- **Timeline:** Nov 19 – Nov 23, 2026
- **Objective:** Build a static structural Verilog parser to extract module definitions, port lists, and register boundaries from public Baochip-1x RTL.
- **Required Knowledge:** Abstract Syntax Trees (AST), Verilog structural syntax, regular expressions/nom parser in Rust.
- **Prerequisites:** Rust stable toolchain; access to `bao1x` RTL sources.
- **Implementation Tasks:**
  1. Create `crates/trustprint-rtl`.
  2. Implement lexical analyzer for Verilog module declarations, parameter definitions, and port lists (`input`, `output`, `inout`).
  3. Ingest Baochip-1x peripheral RTL (UART, I2C, BIO wrapper) and output structured JSON ASTs.
  4. Generate expected test-vector configurations based on extracted port names.
- **Expected Files:**
  - `crates/trustprint-rtl/src/lib.rs`
  - `crates/trustprint-rtl/src/parser.rs`
  - `crates/trustprint-rtl/src/ast.rs`
  - `crates/trustprint-rtl/tests/parser_tests.rs`
- **Commands to Execute:**
  ```bash
  cargo test -p trustprint-rtl
  cargo run -p trustprint-cli -- parse-rtl --input ../dabao/rtl/bio_bdma_wrapper.sv
  ```
- **Expected Output:** JSON AST containing port names, bit widths, and signal directions.
- **Acceptance Criteria:** 100% accurate extraction of all ports on test Verilog modules.

---

## Phase 3: BIO Stimulus & Probing Engine
- **Timeline:** Nov 23 – Nov 27, 2026
- **Objective:** Author high-precision C and RV32E assembly routines for the 4x 700 MHz BIO cores to execute deterministic pin stimulus.
- **Required Knowledge:** RISC-V RV32E assembly, Zig embedded Clang compiler, BIO hardware registers (blocking FIFOs, snap-to-quantum).
- **Prerequisites:** Zig 0.12+ installed; Phase 1 simulation harness active.
- **Implementation Tasks:**
  1. Setup `bio/` directory with Zig build file targeting RV32E freestanding.
  2. Implement Core 0 pattern generator using the quantum snap register.
  3. Implement Core 1 logic sampler receiving data over hardware blocking FIFOs (`x16`–`x31`).
  4. Verify multi-core execution inside `bio-sim`.
- **Expected Files:**
  - `bio/src/main.c`
  - `bio/src/bio_regs.h`
  - `bio/src/quantum_probe.s`
  - `bio/build.zig`
  - `bio/linker.ld`
- **Commands to Execute:**
  ```bash
  cd bio
  zig build
  cd ../bio-sim
  ./simulate ../bio/configs/test_probe.jsonc
  ```
- **Acceptance Criteria:** Zero deadlocks in multi-core execution; waveform confirms square wave output on GPIO net.

---

## Phase 4: Waveform Capture & Conformance Engine
- **Timeline:** Nov 27 – Nov 30, 2026
- **Objective:** Ingest `.fst` simulation traces and verify that observed signal transitions conform to expected timing envelopes.
- **Required Knowledge:** Binary waveform formats, signal edge detection, jitter calculation.
- **Prerequisites:** Phase 1 and 3 outputs (`waveform/bio.fst`).
- **Implementation Tasks:**
  1. Build `crates/trustprint-waveform` with streaming FST transition extractor.
  2. Implement parametric measurement algorithms (frequency, period, duty cycle, edge jitter).
  3. Compare observed edge intervals against expected RTL specs from Phase 2.
  4. Generate automated PASS/FAIL verdict JSON.
- **Expected Files:**
  - `crates/trustprint-waveform/src/lib.rs`
  - `crates/trustprint-waveform/src/fst_reader.rs`
  - `crates/trustprint-waveform/src/conformance.rs`
- **Commands to Execute:**
  ```bash
  cargo test -p trustprint-waveform
  cargo run -p trustprint-cli -- check-conformance --waveform waveform/bio.fst --spec specs/bio_gpio.json
  ```
- **Acceptance Criteria:** Automated verdict output with exact jitter and period statistics.

---

## Phase 5: Physical Dabao Hardware Integration
- **Timeline:** Nov 30 – Dec 03, 2026
- **Objective:** Deploy verified firmware to physical Dabao hardware via USB mass storage and stream real-time UART telemetry.
- **Required Knowledge:** Dabao PROG bootloader flow, UF2 binary format, USB CDC serial communication.
- **Prerequisites:** Physical Dabao evaluation board, USB-C cable.
- **Implementation Tasks:**
  1. Document safe flashing checklist (never overwrite factory security keys).
  2. Build Dabao binary image via `cargo xtask dabao`.
  3. Flash board in bootloader mode (PROG button -> copy `.uf2`).
  4. Capture serial console output via Python `pyserial` streamer.
- **Expected Files:**
  - `scripts/dabao_flash.py`
  - `scripts/serial_telemetry.py`
  - `firmware/Cargo.toml`
- **Commands to Execute:**
  ```bash
  # With Dabao connected in PROG mode:
  cp target/riscv32imac-unknown-xous-elf/release/*.uf2 /media/$USER/BAOCHIP/
  # Read serial telemetry:
  python scripts/serial_telemetry.py --port COM3 --baud 115200
  ```
- **Acceptance Criteria:** Serial terminal emits verified boot banner and experiment telemetry from physical silicon.

---

## Phase 6: IRIS Optical Die Inspection Engine
- **Timeline:** Dec 03 – Dec 06, 2026
- **Objective:** Integrate Short-Wave Infrared (~1050nm) backside die micrographs and align them with published CAD floorplan geometry.
- **Required Knowledge:** Image segmentation, affine transformation, coordinate registration.
- **Prerequisites:** Calibrated Baochip-1x SWIR die micrographs.
- **Implementation Tasks:**
  1. Create `crates/trustprint-iris`.
  2. Implement coordinate transformation mapping optical pixels to physical silicon micrometers.
  3. Render bounding box overlays for 2 MiB SRAM, 4 MiB ReRAM, VexRiscv core, and security mesh perimeter.
  4. Calculate geometric correlation index.
- **Expected Files:**
  - `crates/trustprint-iris/src/lib.rs`
  - `crates/trustprint-iris/src/align.rs`
  - `crates/trustprint-iris/src/overlay.rs`
  - `inspection/sample_swir_die.png`
- **Commands to Execute:**
  ```bash
  cargo run -p trustprint-cli -- align-iris --image inspection/sample_swir_die.png --floorplan data/bao1x_floorplan.json
  ```
- **Acceptance Criteria:** Visual alignment artifact showing CAD blocks accurately projected onto physical die features.

---

## Phase 7: Cryptographic Evidence & Attestation Engine
- **Timeline:** Dec 06 – Dec 08, 2026
- **Objective:** Package all experiment outputs into canonical `.tpe` JSON evidence bundles, hash artifacts with SHA-512, and sign with Ed25519.
- **Required Knowledge:** Cryptographic hashing, Ed25519 digital signatures, canonical JSON formatting.
- **Prerequisites:** Phase 4 and 6 completed artifacts.
- **Implementation Tasks:**
  1. Create `crates/trustprint-evidence`.
  2. Implement canonical SHA-512 hasher for external files (waveforms, logs, micrographs).
  3. Implement Ed25519 signing engine (supporting both on-chip hardware key and development key fallback).
  4. Implement independent host-side verification tool.
- **Expected Files:**
  - `crates/trustprint-evidence/src/lib.rs`
  - `crates/trustprint-evidence/src/bundle.rs`
  - `crates/trustprint-evidence/src/verify.rs`
  - `crates/trustprint-evidence/src/schema.rs`
- **Commands to Execute:**
  ```bash
  cargo run -p trustprint-cli -- bundle-evidence --exp-id EXP_001 --output evidence/EXP_001.tpe
  cargo run -p trustprint-cli -- verify-evidence --evidence evidence/EXP_001.tpe
  ```
- **Acceptance Criteria:** Verification tool outputs `VERDICT: VALID SIGNATURE` and confirms 100% hash integrity.

---

## Phase 8: Xous Application & MMU Isolation
- **Timeline:** Dec 08 – Dec 10, 2026
- **Objective:** Encapsulate verification coordination inside a dedicated Xous microkernel userland service (`trustprint-srv`), verifying Sv39 memory protection.
- **Required Knowledge:** Xous IPC architecture, capability tokens, RISC-V Sv39 virtual memory.
- **Prerequisites:** `xous-core` repository clone; Phase 5 hardware workflow.
- **Implementation Tasks:**
  1. Implement `crates/trustprint-xous-srv` service loop.
  2. Define IPC opcodes and message serialization.
  3. Implement memory fault test: attempt deliberate read to unauthorized physical memory to demonstrate hardware trap.
  4. Verify service registration with `xous-names`.
- **Expected Files:**
  - `crates/trustprint-xous-srv/src/main.rs`
  - `crates/trustprint-xous-srv/src/api.rs`
- **Commands to Execute:**
  ```bash
  cargo xtask dabao trustprint-srv
  ```
- **Acceptance Criteria:** Deliberate memory violation causes clean Xous process panic without crashing microkernel.

---

## Phase 9: Operator Research Dashboard
- **Timeline:** Dec 10 – Dec 12, 2026
- **Objective:** Build a clean, responsive web-based research dashboard for real-time telemetry, waveform visualization, and evidence verification.
- **Required Knowledge:** React, TypeScript, HTML5 Canvas, WebSockets / WebSerial.
- **Prerequisites:** Node.js 18+; Phase 7 evidence artifacts.
- **Implementation Tasks:**
  1. Initialize React + Vite + TypeScript application in `dashboard/`.
  2. Implement Hardware Status panel (SoC info, clock frequency, security mesh status).
  3. Implement interactive Waveform Viewer component.
  4. Implement IRIS Die Inspection overlay component.
  5. Implement Evidence Verifier drag-and-drop tool.
- **Expected Files:**
  - `dashboard/src/App.tsx`
  - `dashboard/src/components/WaveformViewer.tsx`
  - `dashboard/src/components/IrisViewer.tsx`
  - `dashboard/src/components/CertificateVerifier.tsx`
- **Commands to Execute:**
  ```bash
  cd dashboard
  npm install
  npm run dev
  ```
- **Acceptance Criteria:** Interactive web dashboard renders locally at `localhost:5173` without runtime errors.

---

## Phase 10: Complete System Integration & Validation
- **Timeline:** Dec 12 – Dec 13, 2026
- **Objective:** Run full regression suite connecting RTL parsing, simulation, hardware execution, IRIS alignment, and cryptographic signing into a single automated pipeline.
- **Implementation Tasks:**
  1. Author master end-to-end integration test script (`tests/e2e_pipeline.py`).
  2. Run 10 consecutive verification cycles to evaluate repeatability and timing stability.
  3. Audit all log outputs to ensure zero fabricated data.
- **Acceptance Criteria:** 100% pass rate across end-to-end automated verification run.

---

## Phase 11: Demo Video, Documentation Freeze & Hackathon Submission
- **Timeline:** Dec 13 – Dec 15, 2026
- **Objective:** Finalize public GitHub repository, record high-quality 5-minute technical demonstration video, and submit project to CrossBar portal before Dec 15 deadline.
- **Implementation Tasks:**
  1. Record live demonstration script showing real hardware, waveforms, and certificate verification.
  2. Tag GitHub release `v1.0.0`.
  3. Submit official entry on CrossBar hackathon submission page.
- **Acceptance Criteria:** Submission confirmation received; public repository live and accessible.

