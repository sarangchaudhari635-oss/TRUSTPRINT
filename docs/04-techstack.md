# 04 — Technology Stack & Toolchain Matrix

**Document Reference:** TECH-STACK-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Core Architectural Technologies

| Technology | Supported Version | Purpose in TRUSTPRINT | Why Selected | Where Used | Alternatives Considered | Known Limitations & Mitigations |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Rust** | `stable` (1.75+) & `nightly` | Core programming language for host tools and Xous firmware. | Memory safety without garbage collection, zero-cost abstractions, strong typing, native RISC-V cross-compilation. | `crates/trustprint-*`, Xous userland services. | C / C++ | Long compile times for complex macro dependencies; mitigated by modular cargo workspace layout. |
| **Xous OS** | `0.9.x` (`betrusted-io/xous-core`) | Microkernel operating system for Dabao VexRiscv CPU. | Capability-based security, pure-Rust microkernel architecture, native Sv39 MMU virtual memory support, built-in PDDB. | Dabao main CPU system runtime. | FreeRTOS, Zephyr, Linux | Smaller developer community; requires adherence to Xous IPC patterns. |
| **PicoRV32** | RV32E Core Configuration | RISC-V processor core used in the BIO coprocessor. | Lightweight, highly optimized silicon area, open-source (Claire Wolf), customized with hardware FIFO registers. | Baochip-1x 4x BIO cores. | Custom FSM, RP2040 PIO | Limited to 16 integer registers (`x0`–`x15`), requiring compact register allocation. |
| **Verilator** | `5.0+` | Cycle-accurate C++ simulation model compiler for RTL. | World's fastest open-source Verilog simulator, compiles RTL into highly optimized C++ binaries, native `.fst` waveform export. | `bio-sim` simulation harness. | Icarus Verilog, GHDL, ModelSim | Cycle-accurate 2-state simulator (does not model unknown `X`/`Z` states like 4-state simulators). |
| **Zig / Clang** | `0.12+` | Embedded C compiler toolchain targeting RV32E. | Bundles a self-contained, multi-target Clang/LLVM compiler supporting RV32E embedded targets without manual GCC toolchain building. | BIO firmware compilation (`bio/build.zig`). | GNU RISC-V GCC (`riscv-none-elf-gcc`) | RV32E ABI requires strict compiler flags (`-march=rv32e -mabi=ilp32e`). |
| **Python** | `3.11+` | Automation scripting, UART telemetry ingestion, mathematical analysis. | Rapid prototyping, extensive ecosystem for serial communication (`pyserial`) and numerical analysis (`numpy`, `matplotlib`). | `scripts/`, test orchestration, host telemetry. | Bash, PowerShell | Slower runtime than compiled Rust; limited to non-time-critical host tasks. |
| **FST & GTKWave** | FST format / GTKWave 3.3+ | Waveform trace representation and visualization. | Fast Signal Trace (FST) files are ~10x more compact and faster to parse than raw ASCII VCD files. | Waveform capture, conformance analysis. | VCD, Surfer | FST reader library in Rust requires C bindings or custom binary parsing. |
| **Ed25519 & SHA-512** | `ed25519-dalek` / `sha2` crate | Cryptographic hashing and digital signatures. | High-performance, constant-time, 128-bit security level, deterministic signatures, native hardware accelerator compatibility on Baochip-1x. | Evidence generation and verification. | ECDSA (secp256k1), RSA | Hardware engine uses specific endianness; endian conversions must be unit-tested. |
| **React & TypeScript** | React 18 / TS 5.x | Operator research dashboard. | Component-driven UI, rich visualization ecosystem, type safety, responsive design for live charts and image overlays. | `dashboard/` | Desktop Qt, Electron, Tauri | Web browsers cannot access raw serial ports without WebSerial API or a local proxy daemon. |
| **pyserial** | `3.5+` | Host-to-hardware serial communication bridge. | Standard, reliable cross-platform serial library for streaming UART telemetry from Dabao. | Host CLI and telemetry forwarder. | Native Rust `serialport` | Requires Python runtime environment. |

---

## 2. Hardware Toolchain & Compilers

### A. Host Rust Toolchains
```bash
# Required targets
rustup target add riscv32imac-unknown-none-elf
rustup target add riscv32imac-unknown-xous-elf # Via Xous xtask setup
```

### B. BIO C/Assembly Compiler Flags
```bash
# RV32E specific compiler invocation via Clang
clang --target=riscv32-unknown-none-elf \
      -march=rv32emc \
      -mabi=ilp32e \
      -O2 -ffreestanding -nostdlib \
      -Wl,-T,bio/linker.ld \
      -o bio_probe.elf bio_probe.c
```

---

## 3. Technology Selection Rationale

### Why Not Raspberry Pi RP2040 PIO?
While the RP2040 Programmable I/O (PIO) architecture popularized deterministic bit-banging, it relies on a micro-coded 9-instruction CISC state machine. As bunnie Huang's research revealed, a silicon implementation of the PIO state machine actually consumes **more silicon area than an entire RISC-V CPU core** due to complex muxing and barrel shifters, and fails to achieve 700 MHz clock speeds on 22nm silicon. The Baochip-1x's BIO uses complete RV32E RISC-V cores with hardware FIFO registers, giving developers full C programmability at 700 MHz.

### Why Xous Microkernel over Linux or FreeRTOS?
- **Linux:** Requires significant memory overhead (typically >32 MiB) and lacks hard real-time determinism. The Baochip-1x integrates 2 MiB SRAM and 4 MiB ReRAM, which cannot fit a standard Linux kernel and rootfs.
- **FreeRTOS:** While lightweight, FreeRTOS runs in a single flat memory space without leveraging the Baochip-1x's hardware Sv39 MMU, meaning any memory bug or rogue test vector can corrupt cryptographic root keys.
- **Xous:** Purpose-built in pure Rust for MMU-equipped embedded RISC-V SoCs. It enforces capability-based process isolation while operating comfortably within the 2 MiB SRAM footprint.

