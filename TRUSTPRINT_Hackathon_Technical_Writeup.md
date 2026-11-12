# TRUSTPRINT
## An Open-Silicon Verification and Evidence-Provenance Laboratory for the Dabao / Baochip-1x Platform

**CrossBar Dabao Open Hardware Hackathon 2026**  
**Technical submission write-up | Repository reviewed: 12 November 2026**  
**Author:** Sarang Chaudhari  
**Repository:** https://github.com/sarangchaudhari635-oss/TRUSTPRINT  
**Project status:** v0.1.0-alpha

---

## Abstract

Open-source hardware makes design files inspectable, but source-code transparency alone does not establish that a manufactured device behaves like the published design or that test results have not been altered. TRUSTPRINT is an open-source research prototype that explores a practical evidence chain for the Dabao development board based on the Baochip-1x system-on-chip (SoC). It combines lightweight RTL interface extraction, Verilator-based BIO simulation, waveform timing analysis, read-only physical-board telemetry, a demonstration-mode optical die-registration workflow, a host-side model of RISC-V memory protection, and cryptographic packaging of evidence.

The implementation is deliberately divided into evidence classes. The repository contains a live-board serial/IFR telemetry record, but its BIO waveform analysis is based on a pre-generated simulation artifact. The optical inspection output is synthetic demonstration data, the MMU scenarios execute in a host-side software model, and the evidence bundle uses a development key generated on the host rather than a hardware-rooted key. These distinctions are central to the result. TRUSTPRINT demonstrates how different evidence sources can be organized, measured, labelled and checked without claiming that software models prove unobserved properties of physical silicon.

The current prototype reports 27 passing Rust tests, clean Clippy and formatting checks, extraction of 70 BIO RTL ports and 34 special-function-register definitions, waveform conformance against a 108.528 ns reference period, a synthetic affine die-registration result, four cryptographically verified evidence claims, five host-side MMU protection scenarios, and physical Dabao board telemetry over USB serial. The work is a foundation for a stronger hardware-in-the-loop research system, not a certification that the Baochip-1x is transistor-for-transistor equivalent to its RTL.

## 1. Problem and motivation

### 1.1 The open-silicon trust gap

Open hardware projects publish RTL, architecture documents and sometimes physical-design information. A user can inspect these files, but there remains a gap between a published design and a particular manufactured device.

Three practical questions motivate TRUSTPRINT:

1. **Behaviour:** Does the physical board expose observable I/O behaviour consistent with the published design under a defined test?
2. **Structure:** Can physical die observations be correlated with a published floorplan, and what level of detail can that observation actually support?
3. **Evidence integrity:** Can another person detect whether the test report or its referenced artifacts changed after the report was created?

These questions are related but not interchangeable. RTL parsing describes source structure. Simulation exercises a model of that RTL. Physical telemetry shows what a connected board reports. Optical inspection requires actual optical measurements. A digital signature protects the integrity of a signed record, but does not independently prove that the record's claims are true.

### 1.2 Why this matters

For open-source silicon, a reproducible software build is not enough. A source repository can be public while the fabricated device remains difficult for an independent user to inspect. Conventional physical failure-analysis methods, including destructive de-packaging and high-end microscopy, can be costly or inaccessible. At the same time, a board's externally observable behaviour only samples the signals and operating conditions that the test can reach.

TRUSTPRINT approaches this as an evidence-engineering problem. Instead of presenting one PASS/FAIL flag as a complete trust decision, it separates the evidence into tiers and records the limitations of each method.

### 1.3 Project objective

Build an open-source laboratory workflow for the Dabao / Baochip-1x platform that:

- extracts a useful structural interface from BIO RTL;
- supports a simulation-first workflow for BIO experiments;
- measures digital transition timing from edge data;
- records read-only physical board identity and telemetry;
- demonstrates how optical landmarks can be registered to a CAD floorplan;
- models memory-isolation fault scenarios;
- creates a tamper-evident, independently verifiable evidence package; and
- exposes the results through a lightweight operator dashboard and CLI.

## 2. Proposed approach and evidence model

TRUSTPRINT uses five evidence tiers. The tiers are a classification scheme, not five claims that have all been physically completed.

| Tier | Evidence class | Current implementation and status | What it supports |
|---|---|---|---|
| 1 | RTL structural analysis | Implemented and tested; lightweight parser | Extracted module name, parameters, ports and SFR-like definitions from the selected source file |
| 2 | BIO simulation and waveform analysis | Simulation artifacts and analysis code present; waveform result is labelled simulated | Timing measurements and comparison against configured expectations for the supplied edge data |
| 3 | Physical board telemetry | Physically captured over USB serial on a Dabao board | Board-reported identity, firmware/IFR fields and read-only diagnostic output |
| 4 | IRIS optical registration | Implemented in synthetic demonstration mode only | Mathematical operation of affine landmark registration on synthetic inputs |
| 5 | Cryptographic evidence package | Implemented and tested using a host-generated development key | Integrity of the signed payload and verification of the signer key, not hardware origin |

A separate host-side Sv32 memory-protection model is included. It is a security experiment associated with the intended Xous deployment, not evidence that the physical Baochip MMU has been exercised by this build.

### 2.1 Evidence labels

The write-up uses the following terms precisely:

- **Implemented:** Code and interfaces exist in the repository.
- **Unit-tested:** A test exercises a function or model in the Rust workspace.
- **Simulated:** A result comes from a software model or simulation artifact, not a measurement of physical silicon.
- **Demonstrated:** A workflow is shown with synthetic or controlled demonstration inputs.
- **Physically captured:** Data was collected from the physical Dabao board through its USB serial interface.
- **Physically verified:** Reserved for a test that directly measures the relevant physical property and has adequate provenance. A board identity string reported by firmware is physical-board telemetry, but is not by itself independent proof of die authenticity.

## 3. System architecture

The repository is a Rust workspace with eight crates:

| Crate | Responsibility |
|---|---|
| `trustprint-core` | Shared data models, target definitions, evidence tiers and timing result types |
| `trustprint-sim` | Runs an external BIO-sim executable, checks required inputs and validates waveform existence, size and freshness |
| `trustprint-rtl` | Lightweight extraction of module parameters, ports and register definitions |
| `trustprint-waveform` | Parses edge data, validates ordering, calculates timing metrics and evaluates configured tolerances |
| `trustprint-iris` | CAD floorplan model, landmark alignment and projected macro overlays |
| `trustprint-evidence` | Canonical payload construction, SHA-512 hashing, Ed25519 signing and verification |
| `trustprint-xous-srv` | Host-side memory-protection evaluation model for selected Sv32-style access scenarios |
| `trustprint-cli` | Operator-facing command-line interface joining the workflows |

The repository also includes:

- `bio/` for BIO firmware source and configuration;
- `rtl/` for the selected SystemVerilog and Verilog source inputs;
- `scripts/` for serial telemetry, waveform extraction, audit parsing and pipeline orchestration;
- `dashboard/` for a zero-dependency HTML operator dashboard;
- `docs/` for the product requirements, architecture, test strategy, security model, limitations and demo guide; and
- `artifacts/` for sample inputs and generated evidence.

### 3.1 Host-side workflow

The host workstation performs most analysis and orchestration:

1. Read selected RTL source.
2. Extract interface information.
3. Run or reuse BIO simulation output.
4. convert waveform data into transition edges where required.
5. Calculate period, duty cycle and jitter metrics.
6. Process optical-registration inputs.
7. run the memory-protection model.
8. assemble and verify an evidence bundle.
9. display outputs through the CLI and dashboard.

The physical board is a separate evidence source. The serial script opens the USB CDC interface, sends a diagnostic query and records returned lines. The repository documents COM10 for the captured session.

### 3.2 Reproducibility boundary

The full pipeline script can run Cargo tests and the individual CLI stages. However, the simulation and waveform stages can reuse checked-in artifacts when the external `bio-sim` directory or conversion utility is unavailable. Therefore, a successful pipeline run does not necessarily mean every stage was freshly regenerated in that run. A reproducible submission should record whether each artifact was freshly generated or reused.

## 4. Implementation details

### 4.1 RTL intelligence engine

The `trustprint-rtl` crate reads a selected RTL file, strips comments, and extracts a module name, parameters, ports and register-like definitions. For the repository's `rtl/bio.sv` input, the validation report records:

- module: `bio_apb`;
- 70 extracted ports; and
- 34 extracted SFR definitions, spanning offsets reported from `0x00` through `0x88`.

This provides a compact interface inventory that can help an experimenter identify control, status, FIFO and GPIO-related signals before writing a test.

**Important implementation boundary:** this is a lightweight, regex-oriented extractor, not a complete SystemVerilog parser or standards-compliant AST. It does not claim to resolve arbitrary elaboration, generate blocks, packages, interfaces, macros or all language constructs. The extracted interface should be checked against the upstream source and treated as a convenience for the selected design subset, not as a complete semantic interpretation of the HDL.

### 4.2 BIO simulation and waveform analysis

The project includes BIO firmware sources and a Verilator/BIO-sim workflow. The simulation harness launches an external simulator, passes a JSONC configuration and optional cycle limit, and checks that the expected waveform exists, is non-empty and is not stale. It also attempts to extract a cycle count from simulator output.

The waveform analyzer accepts transition events with timestamps and logic states. It checks timestamp monotonicity, removes redundant consecutive states, separates rising and falling transitions, and calculates:

- mean period from consecutive rising edges;
- period deviation against a configured expected period;
- duty cycle from rising-to-following-falling intervals;
- peak-to-peak period variation; and
- RMS period variation.

For the checked-in reference edge artifact, the validation report records a mean period of **108.528 ns** against an expected **108.528 ns**, with **0 ps** reported peak-to-peak and RMS jitter and a conformant result. The README describes 131 analyzed cycles and a 14.47% duty cycle.

This result must be described as **simulated artifact analysis**. The validation report explicitly says that the edge JSON is a pre-generated reference artifact and that the FST was generated in WSL. It is not a physical pin measurement. A zero-jitter result in a deterministic digital simulation is not evidence of zero analog jitter on the physical board.

### 4.3 Physical Dabao telemetry

The serial client uses USB CDC serial communication, with a documented baud rate of 115200 and 8-N-1 framing. It supports read-only diagnostic collection and redacts records associated with IFR offsets `0x1A0` and `0x1C0`, which the project identifies as device private key material.

The validation report records a live Dabao session on COM10, board type identification, an audit query and an IFR dump. It reports a Baochip-1x stepping A0 and records device/firmware provenance fields and boot-partition hashes in the evidence artifact.

This is the strongest physical interaction currently evidenced in the repository. It supports the statement that a physical Dabao board was connected and that its serial diagnostic interface returned the recorded telemetry. It does **not** independently establish that the chip's internal logic matches RTL, that all telemetry is unforgeable, or that the reported identifiers are a factory-certified chain of custody.

For a public hackathon submission, device serials, UUIDs, wafer-lot identifiers and detailed partition fingerprints should be omitted or masked unless the organizers explicitly require them. Private IFR key/seed material must never be published.

### 4.4 IRIS optical inspection and CAD registration

The `trustprint-iris` crate contains a reference floorplan model and an affine alignment engine. An affine transform maps CAD coordinates to image coordinates using scale, rotation/shear and translation parameters. Once landmarks are paired, the engine can estimate a transform, calculate residual error and project macro bounding boxes into image space.

The checked-in `iris_inspection_report.json` is explicit:

- `inspection_mode` is `DemonstrationSynthetic`;
- `synthetic_source` is `cad_benchmark_fiducials`;
- `is_synthetic` is `true`; and
- `verification_status` is `DEMONSTRATION_ONLY`.

The report uses four synthetic fiducials and projects five macro regions. Its extremely small RMS residual is the expected numerical result of fitting synthetic coordinates generated to match the transform. It is not a measurement of a physical die image.

The next real research step is to acquire a genuine backside SWIR micrograph using an appropriate optical setup, document calibration and image provenance, identify reliable landmarks, estimate the transform, report residuals and uncertainty, and compare observed structures with an authoritative floorplan. Until that work is completed, no claim of physical macro presence or physical silicon authenticity should be made.

### 4.5 Memory isolation model

The `trustprint-xous-srv` crate evaluates selected memory access rules in a host-side Rust model. The validation report identifies the evaluation model as `HOST_SV32_MODEL`, not the physical MMU.

The latest validation report lists five scenarios:

1. legitimate access to an allowed data page succeeds;
2. an unmapped load produces a modeled load page fault (`scause=13`);
3. a write to a code page produces a modeled store page fault (`scause=15`);
4. a user-mode access to a supervisor-only region produces a modeled load page fault (`scause=13`); and
5. an instruction fetch from a non-executable data page produces a modeled instruction page fault (`scause=12`).

The four prohibited accesses are caught in the model. This validates the model's selected policy cases. It does not prove that the on-chip Baochip MMU, Xous kernel integration, trap vector or physical privilege transitions have been tested. The next stage requires cross-compiling a real Xous service, deploying it to Dabao, and collecting on-device fault evidence.

### 4.6 Cryptographic evidence package

The `trustprint-evidence` crate builds a `.tpe` evidence bundle. The documented flow serializes the payload canonically, computes a SHA-512 digest and signs the digest with Ed25519. Verification reconstructs the canonical payload, recomputes the digest and checks the signature. An optional trusted-public-key argument allows a verifier to reject a signer key that is not on an expected allowlist. Artifact hashes can also be recomputed and compared with the values recorded in the bundle.

The validation report records:

- successful Ed25519 verification;
- a matching canonical payload SHA-512 digest;
- four claims in the bundle;
- four matching artifact hashes;
- successful verification with the expected trusted public key; and
- rejection of an intentionally incorrect trusted public key.

**Key provenance is critical:** the validation report states that the signing key was generated using the host operating system's CSPRNG and stored as a development key under the user's home directory. The bundle uses `DevelopmentHostKey`. It is not generated by the Baochip-1x TRNG and is not protected by an on-chip hardware root of trust. The signature proves that the bundle has not changed since it was signed by the holder of that development key. It does not prove that the evidence originated from genuine silicon.

A further implementation review should verify the canonicalization implementation against the full RFC 8785 test vectors, ensure that every signed claim has an explicit evidence source and status, and ensure artifact paths are resolved safely. A public certificate should not be described as hardware-backed.

## 5. Experiments and results

### 5.1 Repository validation

The latest checked-in `artifacts/validation_report.txt` records the following:

| Check | Reported result | Interpretation |
|---|---:|---|
| Rust workspace tests | 27/27 pass | Unit tests pass for the listed workspace crates |
| Clippy | Zero warnings | Static lint check passed in the recorded environment |
| Formatting | No violations | `cargo fmt --check` passed |
| RTL extraction | 70 ports, 34 SFRs | Successful extraction for the selected BIO source |
| Waveform analysis | 108.528 ns mean period | Conformant result for the checked-in simulated edge artifact |
| IRIS registration | Synthetic demo pass | Affine registration works on synthetic landmarks |
| Evidence verification | 4/4 claims and 4/4 artifact hashes reported passing | Bundle integrity checks pass for the recorded bundle |
| Trusted-key test | Correct key accepted, wrong key rejected | Trust-anchor comparison behaves as expected |
| MMU model | 4/4 prohibited accesses trapped | Host-side model scenarios pass |
| Physical telemetry | COM10 session recorded | Physical board serial telemetry captured |

These are repository-recorded results, not a claim that this report independently reran the tests against the physical board.

### 5.2 Results that can be presented confidently

- The Rust workspace has a recorded validation run with 27 passing tests, clean formatting and clean Clippy output.
- The selected BIO RTL file can be processed to extract a useful interface summary.
- The waveform analyzer can evaluate a supplied transition dataset and compare it to explicit timing tolerances.
- The serial capture tooling has been used with a physical Dabao board and includes redaction for sensitive IFR offsets.
- The affine registration code can fit and project synthetic fiducial data.
- The evidence bundle can be signed and verified using a host development key, including artifact-hash and trusted-key checks.
- The host-side MMU model catches the listed invalid-access scenarios.

### 5.3 Results that are not established

- Physical GPIO timing conformance at 700 MHz.
- Analog jitter, voltage/temperature corner behaviour or timing margin on the physical board.
- Physical presence or location of the five macro regions from a real die micrograph.
- On-chip execution of the MMU test under Xous.
- Hardware-TRNG-backed signing or a device-rooted attestation chain.
- Complete transistor-level equivalence between the published RTL and manufactured silicon.
- Absence of dormant hardware Trojans or hidden logic that is not observable through the tested interfaces.

## 6. Security and safety considerations

TRUSTPRINT is designed to use read-only diagnostic interaction for the current physical-board evidence capture. The serial script blocks a set of dangerous command strings associated with lockdown, fuse operations, zeroization and destructive actions. The project documentation also states that the workflow must not transition the device into an irreversible security state or blow OTP anti-fuses.

These controls are useful safeguards, but a string blocklist is not a complete hardware safety mechanism. Before broader use, the command interface should be constrained by an explicit allowlist of known-safe commands, the target firmware command parser should be reviewed, and destructive commands should be impossible to reach through the host tool's public API. Hardware operations should be tested first on disposable or explicitly approved devices where appropriate.

Sensitive device data requires careful handling. The repository's serial script redacts IFR offsets `0x1A0` and `0x1C0`; the validation report says these contain private key material. Public artifacts should continue to exclude these values. Board identifiers and wafer-lot data should also be treated as potentially sensitive provenance metadata and shared only when needed.

## 7. Limitations and research honesty

The prototype is a verification workflow, not a silicon certification system. Its most important limitations are:

1. **Limited RTL language coverage.** The parser is a lightweight extractor rather than a full SystemVerilog frontend.
2. **Simulation artifact dependency.** The validated waveform edge file is pre-generated. The full simulator must be rebuilt and rerun to establish fresh reproducibility.
3. **No physical waveform capture demonstrated in the validation report.** The current 108.528 ns result is simulated.
4. **Synthetic optical input.** The current IRIS report is synthetic and explicitly demonstration-only.
5. **Host-only memory model.** MMU scenarios have not been demonstrated on the physical Baochip-1x under Xous.
6. **Host development signing key.** The Ed25519 key is not hardware-rooted.
7. **Attestation is not truth.** Cryptography can protect the integrity of a claim, but it cannot make an unsupported claim true.
8. **Limited observability.** External I/O tests cannot reveal every internal node, dormant logic path or transistor-level modification.
9. **Physical telemetry trust boundary.** Board-reported identity and IFR fields are useful observations, but the current system does not independently establish their unforgeability.
10. **Pipeline freshness.** The orchestration script can reuse existing artifacts when external tools are unavailable. A PASS summary should be accompanied by a per-stage fresh/reused marker.

The project's central scientific statement is therefore:

> TRUSTPRINT organizes and checks evidence about source structure, simulated behaviour, physical-board telemetry, demonstration optical registration and host-model memory protection. It does not currently certify transistor-level equivalence, physical macro presence from a real micrograph, or hardware-rooted attestation.

## 8. Development journey

The implementation follows a layered path from source inspection toward a more complete hardware evidence workflow:

**Stage 1: Define the trust problem.** The project began by identifying the gap between publicly inspectable RTL and the physical board that a developer receives. The five-tier evidence model was introduced to prevent simulation, telemetry, optical inspection and cryptographic integrity from being conflated.

**Stage 2: Build the host analysis foundation.** A Rust workspace was created with shared models, a CLI, an RTL extractor and a waveform-analysis crate. This gave the project a reproducible host-side foundation before attempting deeper hardware integration.

**Stage 3: Add BIO simulation support.** The project added BIO firmware/configuration inputs and a wrapper around the external Verilator simulator. The waveform analyzer then evaluated transition data against explicit timing limits.

**Stage 4: Establish physical-board communication.** A USB serial client was added for read-only diagnostics and IFR provenance collection. Sensitive IFR regions were redacted in the capture output.

**Stage 5: Prototype optical registration.** The IRIS crate introduced a CAD floorplan model, fiducial landmarks and affine projection. Synthetic inputs were used to validate the mathematics while clearly labelling the result as demonstration-only.

**Stage 6: Add isolation and evidence integrity.** A host-side Sv32 access model exercised permitted and prohibited memory operations. The evidence crate added canonical payload hashing, Ed25519 signing, artifact hash verification and trusted-key checks.

**Stage 7: Integrate operator workflows.** A CLI, dashboard, scripts, validation report and technical documentation were assembled to make the research workflow easier to reproduce and inspect.

This sequence reflects an incremental prototype strategy: build and test the analysis logic first, then replace synthetic and host-only components with measurements and execution on the target board.

## 9. Future research and engineering roadmap

### Priority 1: Fresh, reproducible simulation

- Pin the exact upstream `bio-sim` revision and Verilator version.
- Make the pipeline fail clearly when a requested fresh simulation cannot run, rather than silently reusing old artifacts.
- Record simulator binary hash, source revision, configuration hash, firmware hashes, cycle count and generation timestamp.
- Regenerate the FST and edge JSON from the same run and verify their provenance.
- Add negative tests for malformed, incomplete, stale and mismatched waveforms.

### Priority 2: Physical BIO hardware-in-the-loop measurement

- Deploy a known BIO stimulus/sampler firmware build to the physical Dabao board.
- Capture actual GPIO transitions with a calibrated external logic analyzer or oscilloscope.
- Record instrument model, sample rate, timebase accuracy, probe loading, pin mapping, voltage, temperature and test configuration.
- Compare physical edge measurements with simulation predictions while reporting uncertainty and measurement limits.
- Repeat across multiple runs and, if possible, multiple boards.

### Priority 3: Real IRIS micrograph processing

- Obtain a genuine Baochip-1x backside SWIR micrograph using a documented acquisition setup.
- Preserve original image bytes and acquisition metadata.
- Calibrate pixel scale and optical distortion.
- Use independently identified landmarks rather than coordinates generated from the target transform.
- Report fit residuals, uncertainty, image quality and ambiguous regions.
- Compare macro-scale observations with a verified floorplan source and clearly state the optical resolution limit.

### Priority 4: On-device Xous isolation test

- Build a minimal Xous application or service for the target revision.
- Cross-compile with the correct RISC-V target and linker configuration.
- Load it onto Dabao without changing OTP or irreversible security state.
- Exercise allowed and denied accesses in separate processes.
- Capture actual trap cause, faulting address, privilege level and process identity from the device.
- Compare on-device behaviour with the host model.

### Priority 5: Hardware-rooted attestation

- Replace the development key with a device-held key generated or protected by the Baochip security facilities, where the documented hardware interfaces permit.
- Define a provisioning and public-key enrollment process.
- Bind device identity, firmware measurement, test configuration, raw measurement hashes and tool versions into the signed payload.
- Add verifier-side trust policy, revocation and key-rotation procedures.
- Keep host-development signatures explicitly labelled as development provenance.

### Priority 6: Stronger parser and evidence formats

- Replace regex-oriented RTL extraction with a maintained SystemVerilog parser or a deliberately constrained grammar with explicit unsupported-syntax errors.
- Validate canonical JSON against RFC 8785 test vectors and edge cases.
- Version the `.tpe` schema and define stable claim identifiers.
- Require each claim to carry an evidence status, source artifact, tool version, timestamp and limitations.
- Separate cryptographic validity from measurement validity in the verifier output.


## 10. Conclusion

TRUSTPRINT is a research prototype for making open-silicon verification more structured, inspectable and reproducible. Its contribution is not a claim that every verification tier has already been completed on physical hardware. Its contribution is the architecture and working software that separate source analysis, simulation, physical telemetry, optical-registration experiments, memory-protection modelling and cryptographic evidence integrity.

The current repository demonstrates a functioning Rust toolchain, useful BIO RTL extraction, timing analysis of a simulation artifact, physical Dabao serial telemetry capture, synthetic optical registration, host-side memory-protection tests and signed evidence verification using a development key. It also makes the important limitations explicit.

The next decisive milestone is to replace the remaining synthetic and host-only evidence with fresh, instrumented measurements on the physical board: BIO pin capture, real SWIR imaging, on-device Xous isolation tests and hardware-rooted signing. That would move TRUSTPRINT from an integrated verification prototype toward a stronger independent silicon-assurance research platform.

**TRUSTPRINT: Verify the observable behaviour. Preserve the evidence. State the limits.**

---

## Appendix A. Repository map

```text
TRUSTPRINT/
├── bio/                         # BIO firmware sources and configuration
├── crates/
│   ├── trustprint-core/
│   ├── trustprint-sim/
│   ├── trustprint-rtl/
│   ├── trustprint-waveform/
│   ├── trustprint-iris/
│   ├── trustprint-evidence/
│   ├── trustprint-xous-srv/
│   └── trustprint-cli/
├── dashboard/                   # Static operator dashboard
├── docs/                        # PRD, architecture, security, testing, limits
├── artifacts/                   # Sample inputs and recorded outputs
├── rtl/                         # Selected BIO RTL and supporting sources
└── scripts/                     # Serial, extraction and pipeline scripts
```

## Appendix B. Key artifacts reviewed

- `artifacts/validation_report.txt`
- `artifacts/trustprint_multicore_edges.json`
- `artifacts/dabao_physical_evidence.json`
- `artifacts/dabao_telemetry.log`
- `artifacts/dabao_ifr.log`
- `artifacts/iris_inspection_report.json`
- `artifacts/trustprint_silicon_certificate.tpe`
- `scripts/dabao_serial.py`
- `scripts/run_full_pipeline.sh`
- `docs/14-cryptographic-evidence.md`
- `docs/15-MMU-isolation.md`
- `docs/20-research-limitations.md`
- `docs/21-demo-and-submission.md`

## Appendix C. Important consistency note

The repository README contains older references to 17 tests and earlier descriptions of some results. This write-up uses the later checked-in validation report, dated 12 November 2026, which records 27 tests and explicitly labels the waveform, IRIS and MMU results as simulated or demonstration-only. Before submission, synchronize the README, dashboard, certificate, validation report and demo narration so that all show the same current status and do not overstate the evidence.
