# Architecture — System Context (C4 Model)

**Document Reference:** ARCH-CTX-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Context Boundary Diagram

The System Context diagram illustrates how TRUSTPRINT interfaces with external actors, open-source repositories, and the physical silicon target.

```mermaid
C4Context
    title System Context Diagram for TRUSTPRINT Platform

    Person(auditor, "Hardware Auditor / Researcher", "Analyzes silicon conformance and verifies cryptographic evidence.")
    Person(dev, "Firmware / Silicon Developer", "Develops cycle-accurate BIO routines and tests hardware interfaces.")

    System(trustprint, "TRUSTPRINT Verification Suite", "Integrates RTL parsing, BIO simulation, physical Dabao probing, IRIS inspection, and cryptographic attestation.")

    System_Ext(dabao_hw, "Dabao Board (Baochip-1x SoC)", "22nm RISC-V SoC with 4x 700MHz BIO cores, Sv39 MMU, ReRAM, and hardware crypto.")
    System_Ext(github_repos, "Official GitHub Repositories", "baochip/dabao, baochip/bio-sim, betrusted-io/xous-core, baochip/bao1x.")
    System_Ext(iris_optics, "IRIS Optical Setup (SWIR 1050nm)", "Non-destructive infrared backside die inspection camera.")

    Rel(auditor, trustprint, "Controls experiments, reviews waveforms, validates certificates", "HTTPS / Local GUI / CLI")
    Rel(dev, trustprint, "Authors BIO C probe firmware, compiles simulation vectors", "Rust / CLI")
    Rel(trustprint, github_repos, "Fetches open RTL, linker scripts, and OS sources", "Git / HTTPS")
    Rel(trustprint, dabao_hw, "Deploys UF2 firmware, streams UART telemetry, executes BIO stimulus", "USB-C (CDC ACM / Mass Storage)")
    Rel(trustprint, iris_optics, "Ingests SWIR die micrographs for CAD floorplan alignment", "File Import / Image Stream")
```

---

## 2. External System Interactions

1. **Official GitHub Repositories:** Serves as the upstream source of truth for RTL module definitions, pinouts, and kernel APIs.
2. **Dabao Board Hardware:** The execution environment for high-speed I/O stimulus, memory-isolated testing, and hardware cryptographic key operations.
3. **IRIS Optical Fixture:** Provides raw physical imagery of the exposed silicon die backside for macroscopic geometric validation.
4. **Auditor / Developer:** Interacts with the platform via the command-line harness or web research dashboard.

