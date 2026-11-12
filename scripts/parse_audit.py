#!/usr/bin/env python3
"""
TRUSTPRINT Audit & IFR Telemetry Parser
Converts raw serial telemetry from physical Dabao board into structured
Tier 3 & Tier 5 evidence models.
"""

import re
import json
import time
from pathlib import Path

def parse_audit_log(audit_log_path: str, ifr_log_path: str = None) -> dict:
    data = {
        "timestamp_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "epistemic_tier": "PHYSICAL_IO_BEHAVIOURAL",
        "board_type": None,
        "silicon_stepping": None,
        "semver": None,
        "description": None,
        "device_serializer": None,
        "public_serial": None,
        "uuid": None,
        "boot0_partition_hash": None,
        "boot0_code_hash": None,
        "boot0_toolchain_hash": None,
        "boot1_partition_hash": None,
        "boot1_code_hash": None,
        "boot1_toolchain_hash": None,
        "developer_mode": False,
        "debug_fused_off": False,
        "wafer_lot_tracking_id": None,
        "raw_ifr_records": {}
    }

    # 1. Parse audit log
    if Path(audit_log_path).exists():
        with open(audit_log_path, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if "Board type reads as:" in line:
                    data["board_type"] = line.split(":")[-1].strip()
                elif "Stepping is:" in line:
                    data["silicon_stepping"] = line.split(":")[-1].strip()
                elif "Semver is:" in line:
                    data["semver"] = line.split(":")[-1].strip()
                elif "Description is:" in line:
                    data["description"] = line.split(":")[-1].strip()
                elif "Device serializer:" in line:
                    data["device_serializer"] = line.split(":")[-1].strip()
                elif "Public serial number:" in line:
                    data["public_serial"] = line.split(":")[-1].strip()
                elif "UUID:" in line:
                    data["uuid"] = line.split(":")[-1].strip()
                elif "boot0 partition:" in line:
                    data["boot0_partition_hash"] = line.split(":")[-1].strip()
                elif "boot0 code only:" in line:
                    data["boot0_code_hash"] = line.split(":")[-1].strip()
                elif "boot0 baobit toolchain:" in line:
                    data["boot0_toolchain_hash"] = line.split(":")[-1].strip()
                elif "boot1 partition:" in line:
                    data["boot1_partition_hash"] = line.split(":")[-1].strip()
                elif "boot1 code only:" in line:
                    data["boot1_code_hash"] = line.split(":")[-1].strip()
                elif "boot1 baobit toolchain:" in line:
                    data["boot1_toolchain_hash"] = line.split(":")[-1].strip()
                elif "== IN DEVELOPER MODE ==" in line:
                    data["developer_mode"] = True
                elif "CM7 & debug confirmed fused off" in line:
                    data["debug_fused_off"] = True

    # 2. Parse IFR log for wafer tracking lot ID
    if ifr_log_path and Path(ifr_log_path).exists():
        with open(ifr_log_path, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if "] 0a0: [" in line:
                    raw_bytes_str = line.split("[")[-1].replace("]", "")
                    try:
                        hex_bytes = [int(x.strip(), 16) for x in raw_bytes_str.split(",") if x.strip()]
                        # First 12 bytes are ASCII tracking string
                        ascii_chars = [chr(b) for b in hex_bytes[:12] if 32 <= b <= 126]
                        data["wafer_lot_tracking_id"] = "".join(ascii_chars)
                    except Exception as e:
                        pass
                # Extract any hex row
                match = re.search(r'\]\s+([0-9a-fA-F]{3}):\s+\[(.*)\]', line)
                if match:
                    addr_hex = match.group(1)
                    byte_vals = [x.strip() for x in match.group(2).split(",") if x.strip()]
                    data["raw_ifr_records"][addr_hex] = byte_vals

    return data

if __name__ == "__main__":
    import sys
    audit_file = sys.argv[1] if len(sys.argv) > 1 else "artifacts/dabao_telemetry.log"
    ifr_file = sys.argv[2] if len(sys.argv) > 2 else "artifacts/dabao_ifr.log"
    out_file = sys.argv[3] if len(sys.argv) > 3 else "artifacts/dabao_physical_evidence.json"

    evidence = parse_audit_log(audit_file, ifr_file)
    with open(out_file, "w", encoding="utf-8") as f:
        json.dump(evidence, f, indent=2)

    print(f"[PARSER] Extracted Physical Silicon Evidence to {out_file}:")
    print(f"  Board Type:          {evidence['board_type']}")
    print(f"  Silicon Stepping:    {evidence['silicon_stepping']}")
    print(f"  Public Serial:       {evidence['public_serial']}")
    print(f"  Device Serializer:   {evidence['device_serializer']}")
    print(f"  Wafer Lot ID:        {evidence['wafer_lot_tracking_id']}")
    print(f"  Boot0 Hash:          {evidence['boot0_partition_hash'][:16]}...")
    print(f"  Boot1 Hash:          {evidence['boot1_partition_hash'][:16]}...")
    print(f"  Developer Mode:      {evidence['developer_mode']}")
    print(f"  Debug Fused Off:     {evidence['debug_fused_off']}")

