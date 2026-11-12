# Master Task Backlog

| Task ID | Phase | Priority | Title | Status | Dependencies | Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **TSK-001** | Phase 0 | P0 | Repository Research | DONE | None | Inspect Baochip-1x, Dabao, Xous, and BIO-sim repos. |
| **TSK-002** | Phase 0 | P0 | Epistemic Model | DONE | None | Define what experiments prove vs cannot prove. |
| **TSK-003** | Phase 0 | P0 | Root Documentation | DONE | TSK-001 | Create README, LICENSE, CONTRIBUTING, CHANGELOG, gitignore. |
| **TSK-004** | Phase 0 | P0 | Master Roadmap | DONE | TSK-002 | Outline 12-phase development timeline to Dec 15. |
| **TSK-005** | Phase 0 | P0 | System Specifications | DONE | TSK-003 | Author docs/00 through docs/22 specifications. |
| **TSK-006** | Phase 0 | P0 | Architectural Diagrams | DONE | TSK-005 | Produce Mermaid system context and component diagrams. |
| **TSK-007** | Phase 0 | P0 | Error Knowledge Base | DONE | TSK-005 | Draft docs/07 covering all hardware/toolchain errors. |
| **TSK-008** | Phase 0 | P1 | Research Papers | DONE | TSK-001 | Produce research/ analysis documents. |
| **TSK-009** | Phase 0 | P1 | Test Strategy Plan | DONE | TSK-005 | Draft tests/ verification specifications. |
| **TSK-010** | Phase 0 | P0 | Documentation Freeze | DONE | TSK-005..009 | Review all docs and obtain operator authorization. |
| **TSK-011** | Phase 1 | P0 | Rust & RISC-V Target Setup | IN PROGRESS | TSK-010 | Install toolchains for riscv32imac-unknown-none-elf. |
| **TSK-012** | Phase 1 | P0 | Verilator Build Setup | IN PROGRESS | TSK-010 | Setup Verilator 5.x build environment for BIO-sim. |
| **TSK-013** | Phase 1 | P0 | Run BIO-sim Hello-World | TODO | TSK-012 | Build and run hello-world.jsonc to verify simulation. |
| **TSK-014** | Phase 1 | P1 | Automated Sim Runner | TODO | TSK-013 | Build Python/Rust script to execute configs automatically. |
| **TSK-015** | Phase 2 | P0 | Verilog AST Parser | TODO | TSK-010 | Implement structural parser for Baochip-1x RTL. |
| **TSK-016** | Phase 2 | P1 | Signal Interface Extractor | TODO | TSK-015 | Extract module input/output ports and register maps. |
| **TSK-017** | Phase 2 | P1 | Test Vector Generator | TODO | TSK-016 | Generate expected stimulus vectors from port types. |
| **TSK-018** | Phase 3 | P0 | BIO C Firmware Scaffold | TODO | TSK-013 | Compile C code targeting PicoRV32 via Zig/clang. |
| **TSK-019** | Phase 3 | P0 | Blocking FIFO Integration | TODO | TSK-018 | Implement inter-core queueing using x16-x31 registers. |
| **TSK-020** | Phase 3 | P0 | Quantum Snap Timing Probing | TODO | TSK-019 | Implement cycle-accurate GPIO toggling with snap register. |
| **TSK-021** | Phase 4 | P0 | FST Waveform Reader | TODO | TSK-013 | Ingest Verilator FST traces and extract signal epochs. |
| **TSK-022** | Phase 4 | P0 | Timing Conformance Checker | TODO | TSK-021 | Compare simulated edges against expected clock intervals. |
| **TSK-023** | Phase 5 | P0 | Dabao Flashing Protocol | TODO | TSK-010 | Flash UF2 images via BAOCHIP mass storage mode safely. |
| **TSK-024** | Phase 5 | P0 | Hardware Serial Telemetry | TODO | TSK-023 | Ingest live UART logs from running Dabao firmware. |
| **TSK-025** | Phase 5 | P1 | Physical Pin Verification | TODO | TSK-024 | Correlate physical GPIO pin toggling against sim. |
| **TSK-026** | Phase 6 | P0 | IRIS Image Ingestion | TODO | TSK-010 | Import SWIR 1050nm infrared die micrographs. |
| **TSK-027** | Phase 6 | P1 | Feature Segmentation | TODO | TSK-026 | Identify SRAM/ReRAM boundaries and security mesh. |
| **TSK-028** | Phase 6 | P1 | Floorplan Registration | TODO | TSK-027 | Overlay physical coordinates on published CAD layouts. |
| **TSK-029** | Phase 7 | P0 | Evidence JSON Schema | TODO | TSK-010 | Finalize .tpe canonical JSON structure. |
| **TSK-030** | Phase 7 | P0 | Artifact Hashing Pipeline | TODO | TSK-029 | SHA-512 hashing of waveforms, code, and IRIS segments. |
| **TSK-031** | Phase 7 | P0 | Ed25519 Signing Engine | TODO | TSK-030 | Sign evidence using on-chip TRNG key generation. |
| **TSK-032** | Phase 8 | P0 | Xous Server Scaffolding | TODO | TSK-023 | Register trustprint-srv inside Xous microkernel. |
| **TSK-033** | Phase 8 | P0 | MMU Memory Protection Tests | TODO | TSK-032 | Validate Sv39 process isolation with deliberate page faults. |
| **TSK-034** | Phase 9 | P0 | Dashboard UI Scaffolding | TODO | TSK-010 | Build React/TS web app for operator visualization. |
| **TSK-035** | Phase 9 | P1 | Waveform Canvas Renderer | TODO | TSK-034 | Render interactive digital signal traces in browser. |
| **TSK-036** | Phase 9 | P1 | IRIS Viewer & Overlay | TODO | TSK-035 | Interactive optical die viewer with CAD bounding boxes. |
| **TSK-037** | Phase 10 | P0 | End-to-End System Audit | TODO | TSK-011..036 | Full automated verification pipeline regression. |
| **TSK-038** | Phase 11 | P0 | Live Demo Recording | TODO | TSK-037 | Record 5-minute technical demonstration for judges. |
| **TSK-039** | Phase 11 | P0 | GitHub Release Publication | TODO | TSK-038 | Tag v1.0.0, freeze repository, publish documentation. |
| **TSK-040** | Phase 11 | P0 | Hackathon Submission | TODO | TSK-039 | Submit project dossier to CrossBar hackathon portal. |

