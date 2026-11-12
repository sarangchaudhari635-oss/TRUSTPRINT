# 16 — API Specifications & Data Models

**Document Reference:** SPEC-API-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Host-to-Device Telemetry Protocol (UART / CDC ACM)

Communication between the host CLI/Dashboard and the Dabao board occurs over USB CDC serial streaming line-delimited JSON messages at 115,200 baud.

### Inbound Commands (Host $\to$ Dabao)
```json
{
  "cmd": "START_EXPERIMENT",
  "experiment_id": "EXP-20261112-BIO-001",
  "target_core": 0,
  "clock_ticks_high": 350,
  "clock_ticks_low": 350,
  "iterations": 10000,
  "enable_snap": true
}
```

### Outbound Telemetry (Dabao $\to$ Host)
```json
{
  "type": "TELEMETRY",
  "timestamp_ms": 1420,
  "soc_status": {
    "cpu_freq_hz": 350000000,
    "bio_freq_hz": 700000000,
    "security_mesh": "ACTIVE",
    "glitch_counter": 0
  },
  "experiment_progress": {
    "cycles_completed": 10000,
    "dropped_fifo_events": 0,
    "observed_period_mean_ns": 499.988,
    "jitter_ps": 112
  }
}
```

---

## 2. Evidence Record Schema (`.tpe`)

```json
{
  "$schema": "https://trustprint.org/schemas/v1/evidence.json",
  "type": "object",
  "required": [
    "trustprint_version",
    "record_id",
    "timestamp_utc",
    "epistemic_tier",
    "target_hardware",
    "experiment_manifest",
    "artifacts",
    "conformance_result",
    "cryptographic_attestation"
  ],
  "properties": {
    "trustprint_version": { "type": "string" },
    "record_id": { "type": "string" },
    "timestamp_utc": { "type": "string", "format": "date-time" },
    "epistemic_tier": {
      "type": "string",
      "enum": [
        "DESIGN_RTL",
        "SIMULATION_VERILATOR",
        "PHYSICAL_IO_BEHAVIOURAL",
        "OPTICAL_IRIS",
        "CRYPTOGRAPHIC_ATTESTATION"
      ]
    },
    "target_hardware": { "type": "object" },
    "experiment_manifest": { "type": "object" },
    "artifacts": { "type": "array" },
    "conformance_result": { "type": "object" },
    "cryptographic_attestation": { "type": "object" }
  }
}
```

---

## 3. Host CLI Interface Definition (`trustprint-cli`)

```bash
# General CLI invocation syntax
trustprint-cli <SUBCOMMAND> [OPTIONS]

# Subcommands
trustprint-cli parse-rtl --input <PATH_TO_VERILOG> --output <JSON_AST_PATH>
trustprint-cli run-sim --config <JSONC_CONFIG> --waveform-out <FST_PATH>
trustprint-cli check-conformance --waveform <FST_PATH> --spec <SPEC_JSON>
trustprint-cli align-iris --image <PNG_PATH> --floorplan <FLOORPLAN_JSON>
trustprint-cli run-hardware --port <COM_OR_DEV> --firmware <UF2_PATH>
trustprint-cli bundle-evidence --manifest <MANIFEST_JSON> --output <TPE_PATH>
trustprint-cli verify-evidence --evidence <TPE_PATH>
```

