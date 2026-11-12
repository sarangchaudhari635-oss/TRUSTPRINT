# 19 — Comprehensive Risk Register & Safety Protocols

**Document Reference:** RISK-REG-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Risk Matrix Overview

| Risk ID | Risk Description | Probability | Impact | Severity | Primary Mitigation | Contingency Plan |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **RSK-01** | **Accidental Developer Mode Silicon Fuse Blow** | Low | Catastrophic | **CRITICAL** | Zero OTP fuse commands in software; strict manual UF2 drag-and-drop workflow. | If blown, board is locked in dev mode (hardware secrets wiped); project continues with software keys. |
| **RSK-02** | **BIO Coprocessor Hardware Deadlock** | Medium | Medium | **MEDIUM** | Comprehensive testing in `bio-sim` before any hardware deployment. | Press RESET button on Dabao board to restore CPU state. |
| **RSK-03** | **USB Brownout During High-Speed Probing** | Low | Medium | **LOW** | Limit concurrent switching I/O pins; use a powered USB-C host hub. | Reset board and power down external circuits. |
| **RSK-04** | **Toolchain Target Version Desynchronization** | Medium | High | **HIGH** | Pin Rust toolchain channel (`rust-toolchain.toml`) and Zig version (0.12). | Document exact working versions in `docs/04-techstack.md`. |
| **RSK-05** | **Hackathon Time Exhaustion on Advanced UI** | High | Medium | **MEDIUM** | Follow strict MoSCoW prioritization (simulation & hardware first, dashboard last). | CLI tools generate standalone HTML/JSON reports if dashboard is unready. |
| **RSK-06** | **Optical Infrared Imagery Inaccessibility** | Low | Medium | **LOW** | Bundle verified open-source SWIR micrographs of Baochip-1x die in repository. | Use bundled reference micrographs for floorplan alignment validation. |

---

## 2. Irreversible Silicon Operations: Zero-Tolerance Policy

> [!CAUTION]
> **IRREVERSIBLE SILICON ACTION ALERT**
>
> The Baochip-1x SoC incorporates one-time programmable (OTP) anti-fuse registers and factory security locks. Once developer mode transition fuses are blown or factory keys are zeroed, **this action CANNOT be undone by software, re-flashing, or physical intervention**.
>
> **TRUSTPRINT Engineering Mandate:**
> - TRUSTPRINT will **NEVER** include automated commands, scripts, or build steps that touch security fuses, developer mode locks, or factory attestation roots.
> - Flashing is strictly constrained to standard userland memory regions via the mass storage UF2 bootloader interface.

