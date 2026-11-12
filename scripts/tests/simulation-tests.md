# Test — Simulation Test Suite (`bio-sim`)

**Document Reference:** TEST-SIM-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Simulation Test Cases

| Test ID | Target Component | Description | Input Configuration | Expected Output | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **SIM-001** | `bio_bdma_wrapper` | Baseline Verilator smoke self-test. | `configs/smoke.jsonc` | Exit code 0; APB registers return expected values. | **PLANNED** |
| **SIM-002** | Core 0 GPIO Toggle | Verify deterministic square wave generation on Pin 0. | `configs/hello-world.jsonc` | Waveform shows periodic toggle on `gpio_out[0]`. | **PLANNED** |
| **SIM-003** | Core 0/1 Blocking FIFO | Core 0 writes timestamps to `x16`; Core 1 reads from `x16`. | `configs/fifo_sync.jsonc` | Zero FIFO deadlocks; Core 1 receives 10,000 ordered samples. | **PLANNED** |
| **SIM-004** | Quantum Snap Timing | Validate that `BIO_QUANTUM_SNAP` aligns edge transitions to exact clock boundaries. | `configs/quantum_snap.jsonc` | Waveform confirms transition edges match exact clock multiples. | **PLANNED** |
| **SIM-005** | Multi-Core Crossbar | All 4 cores run concurrently driving independent GPIO channels. | `configs/multicore_probe.jsonc` | All 4 output nets show independent, non-interfering signals. | **PLANNED** |

---

## 2. Automated Simulation Runner

Simulation test suites are invoked via the automated host runner:
```bash
cargo test -p trustprint-sim
python scripts/run_simulation_regression.py --all
```
