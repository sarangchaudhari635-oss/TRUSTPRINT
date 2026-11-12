# Research — Academic References & Upstream Codebases

**Document Reference:** RES-REF-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Upstream Source Code Repositories

1. **Dabao Evaluation Board Hardware Design**
   - Repository: [https://github.com/baochip/dabao](https://github.com/baochip/dabao)
   - Scope: KiCad PCB layouts, schematic, pinout documentation, BOM, and production Gerbers for Dabao board v3c.

2. **BIO Coprocessor Simulation Harness**
   - Repository: [https://github.com/baochip/bio-sim](https://github.com/baochip/bio-sim)
   - Scope: Verilator C++ testbench (`sim/sim_main.cpp`), `bio_bdma_wrapper.sv` RTL, and example configuration files (`configs/hello-world.jsonc`).

3. **Xous Operating System Core**
   - Repository: [https://github.com/betrusted-io/xous-core](https://github.com/betrusted-io/xous-core)
   - Scope: Pure-Rust capability-based microkernel, Sv39 MMU page table handling, asynchronous IPC messaging, `cargo xtask` build pipeline.

4. **Baochip Introduction & Architecture**
   - URL: [https://baochip.com/dabao-intro](https://baochip.com/dabao-intro)
   - Scope: Architecture overview of Baochip-1x SoC, memory map, pinmux specifications, and bootloader operation.

5. **Devcon 8 Workshop: Hardware Signer on Open OS & Chip**
   - Workshop Link: [https://archive.devcon.org/devcon8/getting-ethereum-signer-running-on-open-source-operating-system-and-chip](https://archive.devcon.org/devcon8/getting-ethereum-signer-running-on-open-source-operating-system-and-chip)
   - Repository: [https://github.com/btchip/devcon8](https://github.com/btchip/devcon8)
   - Scope: Hardware cryptographic signing, TRNG seed extraction, and Xous application development.

6. **CrossBar Dabao Hackathon Portal**
   - Hackathon Info: [https://crossbar-inc.com/pages/dabao-hackathon](https://crossbar-inc.com/pages/dabao-hackathon)
   - Submission: [https://crossbar-inc.com/pages/dabao-submit](https://crossbar-inc.com/pages/dabao-submit)

---

## 2. Technical Articles & Papers by Andrew "bunnie" Huang

1. **"BIO: The Bao I/O Coprocessor" (2026)**
   - Bunniestudios Blog: [https://www.bunniestudios.com/blog/2026/bio-the-bao-i-o-coprocessor/](https://www.bunniestudios.com/blog/2026/bio-the-bao-i-o-coprocessor/)
   - Key Insight: Explains the architectural trade-offs between CISC state machines (RP2040 PIO) and RISC-V RV32E cores with hardware blocking FIFOs and quantum snap registers.

2. **"Infra-Red In Situ (IRIS) Inspection of Silicon Chips"**
   - Bunniestudios / Betrusted Research: Explains Short-Wave Infrared (~1050nm) transmission through polished backside silicon to view internal metal layers without chemical destruction.

3. **"Trusting Open Silicon: Verification Beyond the Netlist"**
   - Discusses supply-chain security, physical inspectability, and the challenge of verifying fabricated silicon against open-source RTL.

---

## 3. Standards & Cryptographic Specifications

- **RFC 8785:** JSON Canonicalization Scheme (JCS) — standard for deterministic JSON serialization for cryptographic hashing.
- **RFC 8032:** Edwards-Curve Digital Signature Algorithm (Ed25519) — high-speed, constant-time public-key signatures.
- **FIPS 180-4:** Secure Hash Standard (SHA-512) — cryptographic hashing.
- **RISC-V Privileged Architecture Version 1.10:** Sv39 Virtual Memory System.

