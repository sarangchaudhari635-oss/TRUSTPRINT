# 21 — Project Demonstration & Submission Dossier

**Document Reference:** DEMO-SUBMIT-001  
**Project:** TRUSTPRINT  
**Version:** 0.2.0  
**GitHub:** https://github.com/sarangchaudhari635-oss/TRUSTPRINT  

---

## 1. Project Overview

TRUSTPRINT is an open-silicon verification platform for the **Dabao development board** (Baochip-1x, TSMC 22nm ULL).  
It provides a reproducible, cryptographically verifiable verification chain spanning five epistemic tiers:

| Tier | Method | Epistemic Status |
|------|--------|-----------------|
| 1 | RTL intelligence (SystemVerilog parsing) | SIMULATED |
| 2 | BIO simulation (Verilator cycle-accurate) | SIMULATED |
| 3 | Physical Dabao telemetry (UART / COM10) | PHYSICAL |
| 4 | IRIS optical die inspection | DEMONSTRATION_ONLY |
| 5 | Xous Sv32 MMU isolation model | SIMULATED |

---

## 2. Five-Minute Live Demonstration Script

| Time | Scene | Visual | Action |
|:-----|:------|:-------|:-------|
| **0:00–0:30** | **Cold open: Silicon trust dilemma** | Dashboard Overview tab | Introduce TRUSTPRINT. Explain why open-source RTL alone is insufficient — verification requires physical evidence. Show 5-tier epistemic framework. |
| **0:30–1:15** | **RTL parsing** | Terminal + Dashboard RTL tab | Run `cargo run -p trustprint-cli -- parse-rtl rtl/bio.sv`. Show: 70 ports, 34 SFRs (0x00–0x88). Click RTL tab to show SFR table. |
| **1:15–2:00** | **BIO simulation + conformance** | Terminal + Dashboard BIO tab | Show `trustprint_multicore.fst` already generated. Run `check-conformance artifacts/trustprint_multicore_edges.json 108.528`. Show 108.528 ns period, 0 ps jitter, PASS. Click waveform canvas to show gpio_out[21] edge trace. |
| **2:00–2:45** | **Physical silicon** | Physical Dabao board (camera) + terminal | Show physical board connected on COM10. Show `dabao_physical_evidence.json`: serial NNBT1T, UUID 0b4d2b1c..., Wafer Lot 47521066M06T, Boot0 SHA-512. Click Physical Silicon tab. |
| **2:45–3:30** | **IRIS die inspection** | Dashboard IRIS tab | Show SVG die floorplan with 5 macros overlaid (CPU, SRAM, ReRAM, BIO, Crypto). Explain affine transform. Run `inspect-die --mode demo`. Show DEMONSTRATION_ONLY badge and RMS residual. |
| **3:30–4:20** | **Cryptographic attestation** | Dashboard Attestation tab | Run `build-evidence` then `verify-evidence`. Show Ed25519 signature verification, 4-claim PASS, bundle ID, `key_provenance: DevelopmentHostKey`. |
| **4:20–4:50** | **MMU isolation** | Dashboard MMU tab | Run `mmu-test`. Show 5 scenarios, all traps caught by HOST_SV32_MODEL. Explain path to physical verification. |
| **4:50–5:00** | **Closing** | Dashboard Overview | Summary: open-silicon verification vision. Physical board always running. All code MIT licensed. |

---

## 3. Physical Hardware Details

| Property | Value |
|----------|-------|
| Board | Dabao (Baochip-1x) |
| Die process | TSMC 22nm ULL |
| CPU | VexRiscv RV32-IMAC @ 350 MHz |
| BIO cores | 4× PicoRV32 @ 700 MHz |
| Serial port | COM10 (Windows), USB VID 0x1D50 |
| Board serial | NNBT1T |
| Stepping | A0 |
| Wafer lot | 47521066M06T |
| UUID | 0b4d2b1c-... |
| OS | Xous microkernel |

---

## 4. CLI Reference (Current Version)

```bash
# Parse RTL — extract ports, SFRs, parameters
cargo run -p trustprint-cli -- parse-rtl rtl/bio.sv

# Check waveform conformance against expected period (ns)
cargo run -p trustprint-cli -- check-conformance artifacts/trustprint_multicore_edges.json 108.528

# IRIS die inspection (demonstration mode — no real micrograph)
cargo run -p trustprint-cli -- inspect-die --mode demo

# IRIS die inspection (physical mode — requires real SWIR micrograph)
cargo run -p trustprint-cli -- inspect-die --mode physical --image path/to/micrograph.png

# Build evidence bundle (uses key at ~/.trustprint/dev_signing_key.sec)
cargo run -p trustprint-cli -- build-evidence

# Verify evidence bundle (signature + artifact hashes)
cargo run -p trustprint-cli -- verify-evidence artifacts/trustprint_silicon_certificate.tpe

# Verify evidence with trusted public key anchor
cargo run -p trustprint-cli -- verify-evidence artifacts/trustprint_silicon_certificate.tpe \
    --trusted-key a81b17441380b2c38d314259d6f15d92ad72c054db82c510f34b0172f49b3c91

# MMU isolation test (host Sv32 model, 5 scenarios)
cargo run -p trustprint-cli -- mmu-test

# Help
cargo run -p trustprint-cli -- help
```

---

## 5. GitHub Repository & Git Workflow

```bash
# Repository is already initialised with remote:
# https://github.com/sarangchaudhari635-oss/TRUSTPRINT.git

# Push all committed changes:
git push -u origin master

# Tag for release:
git tag -a v1.0.0 -m "TRUSTPRINT v1.0.0 — production release"
git push origin v1.0.0
```

---

## 6. Demo Video Recording Checklist

**Screen capture setup:**
- Use OBS Studio, Windows Game Bar (Win+G), or ShareX
- Resolution: 1920×1080 minimum; 60fps preferred
- Audio: narrate live (English)

**Recording sequence:**
1. Open `dashboard/index.html` in browser — walk all 7 tabs (2 min)
2. Open terminal: run full pipeline (`bash scripts/run_full_pipeline.sh` in WSL, or individual CLI commands on Windows)
3. Show physical Dabao board on camera (10 sec)
4. Show `git log --oneline` and `git tag` to prove open-source (20 sec)
5. Close with dashboard Overview tab visible

**Upload:**
- YouTube (Unlisted is fine, link shared in submission)
- Title: `TRUSTPRINT — Open-Silicon Verification Lab | Dabao 2026`
- Description: paste project tagline + GitHub URL

---

## 7. Submission Package Checklist

- [ ] Public GitHub repository: https://github.com/sarangchaudhari635-oss/TRUSTPRINT
- [ ] All 27 unit tests passing (`cargo test --workspace --all-targets`)
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` → clean
- [ ] `cargo fmt --all -- --check` → clean
- [ ] `artifacts/validation_report.txt` present in repository
- [ ] `artifacts/trustprint_silicon_certificate.tpe` present (freshly signed)
- [ ] `artifacts/iris_inspection_report.json` present
- [ ] `artifacts/dabao_physical_evidence.json` present (redacted IFR, no key bytes)
- [ ] `artifacts/trustprint_multicore_edges.json` present
- [ ] MIT `LICENSE` file present
- [ ] Dashboard `index.html` renders correctly with all artifact types
- [ ] HD demo video uploaded (YouTube/Vimeo link recorded here)
- [ ] `README.md` has actual GitHub URL

---

## 8. Research Limitations Disclosure

| Claim | What It Proves | What It Does NOT Prove |
|-------|---------------|----------------------|
| RTL parsing | Published design interface (SFR offsets, port widths) | Fabricated gate equivalence, internal logic paths |
| Verilator simulation | Cycle-accurate RTL model behaviour | Physical transistor behaviour, timing margins |
| UART telemetry (COM10) | Board boots, firmware IFR fields, firmware-reported identity | Die authenticity, anti-tamper integrity |
| IRIS overlay (DEMO) | Affine transform math is correct on synthetic data | Any physical die property |
| Ed25519 attestation | Evidence bundle integrity and authorship | Physical chip origin or factory certification |
| HOST_SV32_MODEL traps | Host software model traps correctly per Sv32 spec | On-chip MMU hardware fires these traps |

This is documented in full in `docs/20-research-limitations.md` and in every CLI output tier label.
