# Contributing to TRUSTPRINT

Thank you for your interest in the **TRUSTPRINT** project.

## 1. Project Context & Solo Development Model

TRUSTPRINT is currently engineered as an individual submission for the **CrossBar Dabao Open Hardware Hackathon 2026** by a solo researcher. While external contributions, discussions, and peer reviews are welcome, all architectural decisions, code integrations, and physical hardware experiments adhere to a strictly documented, single-developer roadmap.

## 2. Code of Conduct & Scientific Integrity

Contributors and reviewers must abide by these non-negotiable principles:
- **Never fabricate physical measurements:** Simulation results must never be disguised or described as physical hardware measurements.
- **Epistemic humility:** Clearly distinguish between design-level verification, simulation, pin-level I/O experimentation, optical macro inspection, and cryptographic attestation. Never claim software proves transistor-level authenticity.
- **Hardware safety first:** Never perform or propose commands that permanently blow fuses, erase non-volatile security roots, or risk hardware damage without explicit warnings and manual confirmation steps.

## 3. Development Standards

### Rust Code Guidelines
- Format code using `cargo fmt --all`.
- Enforce clean clippy checks: `cargo clippy --all-targets -- -D warnings`.
- Document all public items (`#[warn(missing_docs)]`).
- Memory safety: Minimize `unsafe` blocks. Any `unsafe` block must be accompanied by a `// SAFETY:` rationale explaining why invariants hold.

### Commit Conventions
Follow the Conventional Commits specification:
- `feat(bio): add quantum snap timing verification test`
- `fix(xous): handle IPC scalar message overflow in evidence server`
- `docs(prd): add acceptance criteria for IRIS image alignment`
- `test(sim): add bio-sim Verilator regression for core 1 FIFO`

### Documentation Updates
Every PR or change that modifies an architectural boundary, data format, or error condition must simultaneously update:
- `CHANGELOG.md`
- `planning/progress-log.md`
- Relevant `docs/` specifications

