# Test — Hackathon Acceptance & Demonstration Criteria

**Document Reference:** TEST-ACCEPT-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Acceptance Criteria for Final Hackathon Demonstration

| Category | Acceptance Criterion | Verification Method | Evaluation Standard |
| :--- | :--- | :--- | :--- |
| **Pillar 1: Works** | System executes live on physical Dabao board without simulated fallbacks. | Live video demonstration showing physical board connected, boot sequence, and live UART telemetry. | Uninterrupted 5-minute video walkthrough showing real silicon execution. |
| **Pillar 2: New** | Combines RTL, high-speed BIO probing, IRIS optical alignment, and crypto evidence. | Code review of repository showing all four subsystems active and integrated. | Novelty matrix verified against prior art. |
| **Pillar 3: Open** | 100% open-source under permissive license with zero proprietary toolchain requirements. | Verify public GitHub repository, MIT License file, and build reproduction instructions. | Any external evaluator can build using open-source Rust, Verilator, and Zig. |
| **Pillar 4: Uses the Silicon** | Actively drives 4x BIO cores @ 700 MHz, uses Sv39 MMU, hardware TRNG, and IRIS packaging. | Telemetry logs and memory dumps confirming 700 MHz clock activation, quantum snap registers, and TRNG entropy queries. | Evaluators can inspect hardware registers confirming silicon utilization. |

---

## 2. Automated Master Acceptance Test Script

```bash
# Run full automated acceptance pipeline
python scripts/master_acceptance_suite.py
```
This script runs the complete sequence:
1. Ingests RTL and checks AST generation.
2. Compiles BIO test firmware and runs `bio-sim`.
3. Verifies FST waveform conformance.
4. Simulates/probes physical serial telemetry.
5. Ingests IRIS micrograph and checks alignment.
6. Packages and verifies canonical `.tpe` certificate.
7. Prints conclusive `TRUSTPRINT HACKATHON ACCEPTANCE: PASS`.
