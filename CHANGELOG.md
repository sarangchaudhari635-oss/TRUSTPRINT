# Changelog

All notable changes to the TRUSTPRINT project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Demo video recording and YouTube/Vimeo upload.
- GitHub repository creation and tag v1.0.0.
- Hackathon portal submission.

## [0.9.0-alpha] - 2026-11-12

### Added
- **Phase 9 — Operator Research Dashboard (`dashboard/index.html`):**
  - 990-line zero-dependency single-file HTML dashboard.
  - 7 tabs: Overview, RTL Analysis, BIO Simulation, Physical Silicon, IRIS Inspection, Attestation, MMU Isolation.
  - Real `gpio_out[21]` waveform edge data (50 transitions) rendered on HTML5 Canvas.
  - SVG die floorplan with all 5 Baochip-1x macros (CPU, SRAM, ReRAM, BIO, Crypto) drawn from CAD coordinates.
  - Ed25519 certificate fields with 4 verified claim rows and 5-tier colour legend.
  - Responsive CSS grid (desktop + mobile 900px breakpoint).
- **Phase 10 — End-to-End Validation:**
  - `artifacts/validation_report.txt`: 11/11 CLI acceptance criteria documented and confirmed PASS.
  - `scripts/run_full_pipeline.sh`: Full WSL bash pipeline from cargo test → sim → extract → parse-rtl → conformance → iris → evidence → mmu.
  - 17/17 cargo unit tests confirmed passing across all 7 workspace crates.
  - All 5 epistemic boundary disclosures verified present in every output tier.

### Fixed
- Dashboard `drawWaveform()` now triggers on resize only when BIO Simulation tab is active (prevents canvas dimension error on hidden tab).
- Pipeline script uses `set -euo pipefail` for reliable error propagation.

### Changed
- `planning/current-phase.md` updated to Phase 11 (Final Submission).
- `planning/milestone-tracker.md` M9 and M10 marked DONE with completion evidence.

## [0.1.0-alpha] - 2026-11-12

### Added
- **Formal Project Documentation System:**
  - Complete Product Requirements Document (`docs/01-PRD.md`) with 35 Functional Requirements and 20 Non-Functional Requirements.
  - Comprehensive System and Technical Architecture blueprints (`docs/02`, `docs/03`).
  - Full Technology Stack analysis and evaluation matrix (`docs/04-techstack.md`).
  - 12-phase development roadmap and solo developer execution schedule (`docs/05`, `planning/`).
  - Detailed error-handling knowledge base covering hardware, simulation, toolchain, and OS errors (`docs/07-error-handling.md`).
  - Security model and physical trust boundaries (`docs/10-security-model.md`).
  - Hardware integration specs for Baochip-1x SoC and Dabao dev board (`docs/09`).
  - BIO coprocessor architecture analysis and execution specification (`docs/11`).
  - IRIS physical optical inspection integration model (`docs/13`).
  - Cryptographic evidence format and verification schema (`docs/14`, `docs/16`).
  - Xous MMU process isolation design (`docs/15`).
  - Research limitations and epistemic boundaries manifesto (`docs/20`).
- **Research Foundations:**
  - Literature review of existing silicon verification platforms (`research/existing-solutions.md`).
  - Hardware capability matrix for Baochip-1x (`research/hardware-capability-matrix.md`).
  - Feasibility study and novelty analysis (`research/feasibility-study.md`, `research/novelty-analysis.md`).
- **Testing Architecture:**
  - Master verification and validation plan (`tests/test-plan.md`).

