# 07 — Comprehensive Error Handling & Troubleshooting Knowledge Base

**Document Reference:** ERR-CATALOG-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  
**Status:** ALL ENTRIES CURRENTLY LABELED AS "ANTICIPATED" (Phase 0 Baseline)

This document is the mandatory troubleshooting manual for the TRUSTPRINT platform. Every error is documented with symptoms, root causes, diagnostic commands, step-by-step resolutions, recovery procedures, and explicit hardware damage risk assessments.

---

## Error Catalog Matrix

| Category | Error ID | Error Title | Status | Hardware Risk? |
| :--- | :--- | :--- | :--- | :--- |
| **Toolchain** | ERR-001 | Missing RISC-V Cross-Compilation Target | ANTICIPATED | NO |
| **Toolchain** | ERR-002 | Zig Compiler RV32E Target Rejection | ANTICIPATED | NO |
| **Toolchain** | ERR-003 | Verilator C++ Linker Failure | ANTICIPATED | NO |
| **Toolchain** | ERR-004 | Python `pyserial` Baud Rate Discrepancy | ANTICIPATED | NO |
| **Xous Build**| ERR-005 | `cargo xtask` Missing Version Tags Failure | ANTICIPATED | NO |
| **Xous Build**| ERR-006 | Memory Section Overflow in Xous Linker Script | ANTICIPATED | NO |
| **BIO Sim**   | ERR-007 | `bio-sim` Blocking FIFO Deadlock | ANTICIPATED | NO |
| **BIO Sim**   | ERR-008 | Verilator APB Bus Timeout During Firmware Load | ANTICIPATED | NO |
| **RTL Parse** | ERR-009 | Unsupported SystemVerilog Preprocessor Directive | ANTICIPATED | NO |
| **RTL Parse** | ERR-010 | Missing Port Declaration in Top-Level Wrapper | ANTICIPATED | NO |
| **Hardware**  | ERR-011 | Dabao Mass Storage Drive Not Enumerating | ANTICIPATED | NO |
| **Hardware**  | ERR-012 | Serial COM Port Access Denied / Locked | ANTICIPATED | NO |
| **Hardware**  | ERR-013 | Partial UF2 Flash Write (Corrupted Firmware) | ANTICIPATED | NO (Recoverable via PROG button) |
| **Hardware**  | ERR-014 | Dabao USB Brownout Under Heavy GPIO Load | ANTICIPATED | LOW (Use powered USB-C hub) |
| **Hardware**  | ERR-015 | Irreversible Developer Mode Transition Hazard | ANTICIPATED | **YES — IRREVERSIBLE HARDWARE STATE** |
| **BIO Silicon**| ERR-016 | BIO FIFO Underflow in Asynchronous Probing | ANTICIPATED | NO |
| **BIO Silicon**| ERR-017 | Quantum Snap Timer Drift Across Thermal Corner | ANTICIPATED | NO |
| **Waveform**  | ERR-018 | Truncated FST Trace File During Abnormal Sim Exit | ANTICIPATED | NO |
| **Waveform**  | ERR-019 | False Negative Conformance Due to Jitter Window | ANTICIPATED | NO |
| **OS / MMU**  | ERR-020 | Sv39 Page Fault (Illegal Physical Memory Read) | ANTICIPATED | NO (Kernel isolates panic) |
| **Crypto**    | ERR-021 | Hardware TRNG Depleted / Seed Exhaustion | ANTICIPATED | NO |
| **Crypto**    | ERR-022 | Ed25519 Signature Verification Mismatch | ANTICIPATED | NO |
| **IRIS**      | ERR-023 | Optical Contrast Loss in SWIR Micrograph | ANTICIPATED | NO |
| **Dashboard** | ERR-024 | WebSerial Disconnection During Live Stream | ANTICIPATED | NO |
| **BIO Sim**   | ERR-025 | Verilator Whitespace Path Argument Splitting | EXPERIMENTALLY REPRODUCED | NO |

---

## Detailed Troubleshooting Specifications

### Category 1: Rust & Embedded Toolchain Errors

#### ERR-001: Missing RISC-V Cross-Compilation Target
- **Status:** ANTICIPATED
- **Hardware Risk:** NO
- **Symptoms:** `error[E0463]: can't find crate for 'core'` when compiling for `riscv32imac-unknown-none-elf` or `riscv32imac-unknown-xous-elf`.
- **Likely Causes:** Rustup target has not been installed for the active toolchain.
- **Diagnostic Commands:**
  ```bash
  rustup target list --installed
  ```
- **Step-by-Step Resolution:**
  1. Add the bare-metal target:
     ```bash
     rustup target add riscv32imac-unknown-none-elf
     ```
  2. For Xous compilation, ensure the target spec JSON is in the toolchain search path:
     ```bash
     cargo xtask dabao --check-toolchain
     ```
- **Recovery:** Re-run build command.
- **Prevention:** Run toolchain check script during Phase 1 setup.

---

#### ERR-002: Zig Compiler RV32E Target Rejection
- **Status:** ANTICIPATED
- **Hardware Risk:** NO
- **Symptoms:** `error: unknown target cpu 'pico_rv32'` or compilation fails with illegal register operand `x16`.
- **Likely Causes:** Zig compiler flags passed standard RV32I instead of RV32E embedded profile (which restricts general registers to 16).
- **Diagnostic Commands:**
  ```bash
  zig targets | grep -i rv32
  ```
- **Step-by-Step Resolution:**
  1. Inspect `bio/build.zig`.
  2. Ensure target is explicitly configured with CPU features:
     ```zig
     const target = b.resolveTargetQuery(.{
         .cpu_arch = .riscv32,
         .cpu_model = .{ .explicit = &std.Target.riscv.cpu.generic_rv32 },
         .cpu_features_add = std.Target.riscv.Feature.e.toFeatureSet(),
     });
     ```
  3. Re-run `zig build`.

---

#### ERR-003: Verilator C++ Linker Failure
- **Status:** ANTICIPATED
- **Hardware Risk:** NO
- **Symptoms:** `undefined reference to 'Vbio_bdma_wrapper::eval()'` during `bio-sim` make step.
- **Likely Causes:** Mismatch between Verilator generated header files and C++ testbench (`sim_main.cpp`); out-of-date build cache.
- **Diagnostic Commands:**
  ```bash
  verilator --version
  make clean && make build
  ```
- **Step-by-Step Resolution:**
  1. Delete `obj_dir/` completely:
     ```bash
     rm -rf obj_dir/
     ```
  2. Rebuild cleanly:
     ```bash
     make -j$(nproc) build
     ```

---

### Category 2: Xous Microkernel Build & Link Errors

#### ERR-005: `cargo xtask` Missing Version Tags Failure
- **Status:** ANTICIPATED
- **Hardware Risk:** NO
- **Symptoms:** Build fails with `git describe failed: fatal: No names found, cannot describe anything`.
- **Likely Causes:** Xous build scripts query Git tags to generate internal build metadata and OS version strings. If the repository was shallow-cloned without tags, xtask panics.
- **Diagnostic Commands:**
  ```bash
  git describe --tags
  ```
- **Step-by-Step Resolution:**
  1. Fetch all upstream tags:
     ```bash
     git fetch --tags
     ```
  2. Verify tag existence:
     ```bash
     git tag -l
     ```
  3. Re-execute `cargo xtask dabao dabao-console`.

---

### Category 3: BIO Simulator & FIFO Deadlocks

#### ERR-007: `bio-sim` Blocking FIFO Deadlock
- **Status:** ANTICIPATED
- **Hardware Risk:** NO
- **Symptoms:** `bio-sim` hangs indefinitely without advancing simulation cycles or generating output; CPU usage spikes to 100%.
- **Likely Causes:** Core 0 is waiting to write to a full FIFO register, while Core 1 is waiting on Core 0 before reading. The upper register bank (`x16`–`x31`) hardware blocking semantics cause both PicoRV32 cores to halt execution.
- **Diagnostic Commands:**
  1. Terminate hung simulation with `Ctrl+C`.
  2. Inspect trace log for last executed instructions per core:
     ```bash
     ./simulate configs/test.jsonc --trace-cycles
     ```
- **Step-by-Step Resolution:**
  1. Open BIO firmware C/assembly source code.
  2. Verify producer-consumer queue ordering: ensure consuming core reads FIFO before downstream processing.
  3. Add timeout sentinel checks or coordinate through the shared quantum snap register.
  4. Recompile and simulate.

---

### Category 4: Hardware Interconnect & Flashing Errors

#### ERR-011: Dabao Mass Storage Drive Not Enumerating
- **Status:** ANTICIPATED
- **Hardware Risk:** NO
- **Symptoms:** Plugging Dabao into USB while holding `PROG` does not show a `BAOCHIP` drive in File Explorer or `lsblk`.
- **Likely Causes:**
  1. `PROG` button was not held down *before and during* USB-C insertion.
  2. Charge-only USB-C cable used (no data lines).
  3. Operating system USB port power suspension.
- **Diagnostic Commands:**
  - *Windows (PowerShell):*
    ```powershell
    Get-PnpDevice -Class "DiskDrive" | Where-Object { $_.FriendlyName -like "*BAOCHIP*" }
    ```
  - *Linux:*
    ```bash
    dmesg | tail -n 20
    lsusb
    ```
- **Step-by-Step Resolution:**
  1. Unplug USB-C cable.
  2. Confirm USB cable supports High-Speed data (test with another USB device).
  3. Firmly press and hold the physical **PROG** button on the Dabao board.
  4. While holding **PROG**, insert the USB-C cable into a direct host port (avoid unpowered hubs).
  5. Wait 2 seconds, then release the **PROG** button.
  6. The `BAOCHIP` drive will appear mounted.

---

#### ERR-012: Serial COM Port Access Denied / Locked
- **Status:** ANTICIPATED
- **Hardware Risk:** NO
- **Symptoms:** `PermissionError: [Errno 13] could not open port 'COM3'` or `device or resource busy` on Linux.
- **Likely Causes:** Another terminal program (PuTTY, Arduino IDE, miniterm, VS Code Serial Monitor) is currently holding the COM port open.
- **Diagnostic Commands:**
  - *Windows:*
    ```powershell
    Get-Process | Where-Object { $_.MainWindowTitle -match "COM" }
    ```
- **Step-by-Step Resolution:**
  1. Identify and terminate any running serial monitor processes.
  2. On Linux, ensure user is in the `dialout` group:
     ```bash
     sudo usermod -aG dialout $USER
     ```
  3. Re-run telemetry script.

---

#### ERR-013: Partial UF2 Flash Write (Corrupted Firmware)
- **Status:** ANTICIPATED
- **Hardware Risk:** NO (Board remains recoverable via bootloader ROM)
- **Symptoms:** Board fails to boot after copying UF2 files; RGB status LED is dark; no serial output on power up.
- **Likely Causes:** USB cable was unplugged before write cache flushed, or UF2 files (`loader.uf2`, `xous.uf2`, `apps.uf2`) were copied in incomplete order.
- **Recovery Procedure:**
  1. Do not panic: The Baochip-1x contains a hard-wired ROM bootloader that **cannot be overwritten by software**.
  2. Unplug the board.
  3. Hold down the **PROG** button and reconnect USB to re-enter bootloader mode.
  4. Re-copy fresh, validated `.uf2` binaries.
  5. Wait for the drive to automatically dismount before touching the board.
  6. Press `PROG` once to trigger boot.

---

#### ERR-015: Irreversible Developer Mode Transition Hazard
- **Status:** ANTICIPATED
- **Hardware Risk:** **YES — PERMANENT SILICON STATE CHANGE**
- **Symptoms:** Accidental invocation of bootloader commands or security fuses that transition the Baochip-1x from factory secure state to developer mode, erasing factory provisioning secrets.
- **Likely Causes:** Running third-party flash scripts that write to OTP fuses or issue low-level security domain commands.
- **Prevention (Mandatory Safety Rule):**
  - **NEVER** pass `--developer-mode` or `--erase-factory-secrets` flags to any flashing tool.
  - TRUSTPRINT firmware operates strictly within standard Xous userland and never issues OTP write commands.
  - Flashing is restricted strictly to dragging standard `.uf2` files to the USB mass storage volume.

---

### Category 5: Timing, Conformance & Waveform Errors

#### ERR-017: Quantum Snap Timer Drift Across Thermal Corner
- **Status:** ANTICIPATED
- **Hardware Risk:** NO
- **Symptoms:** Waveform conformance checker reports failure with edge jitter delta exceeding the 2.0% specification window during extended physical hardware testing.
- **Likely Causes:** Ambient temperature rise causing slight frequency drift in the silicon PLL or clock tree.
- **Diagnostic Commands:**
  ```bash
  cargo run -p trustprint-cli -- analyze-jitter --waveform waveform/capture.fst --window-size 5000
  ```
- **Step-by-Step Resolution:**
  1. Verify whether failure is due to high-frequency jitter or low-frequency thermal wander.
  2. Ensure the test configuration specifies an appropriate statistical metric (e.g., peak-to-peak jitter over 1,000 cycles versus sliding window mean).
  3. Calibrate thermal corner baseline in `trustprint-waveform` configuration.

---

### Category 6: Cryptography & Attestation Errors

#### ERR-022: Ed25519 Signature Verification Mismatch
- **Status:** ANTICIPATED
- **Hardware Risk:** NO
- **Symptoms:** `trustprint-cli verify-evidence` returns `STATUS: REJECTED (Signature invalid)`.
- **Likely Causes:**
  1. Payload JSON keys were reordered during serialization (canonical JSON invariant violated).
  2. Public key endianness was inverted between hardware crypto engine (little-endian) and software verifier.
- **Diagnostic Commands:**
  ```bash
  cargo run -p trustprint-cli -- dump-canonical-hash --evidence evidence/EXP_001.tpe
  ```
- **Step-by-Step Resolution:**
  1. Ensure evidence serialization uses strict deterministic key sorting (`serde_json` with canonical ordering).
  2. Verify byte order of public key and signature buffer (64 bytes).
  3. Re-run verification.

