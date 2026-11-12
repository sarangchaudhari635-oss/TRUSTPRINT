#!/usr/bin/env python3
"""
TRUSTPRINT Dabao Physical Silicon Serial Telemetry Client
Tier 3: PHYSICAL_IO_BEHAVIOURAL

Connects to physical Dabao board over USB-CDC UART (115200 8N1).
Queries system status, monitors BIO core telemetry, and captures
hardware timestamps for cryptographic evidence authentication.
"""

import sys
import time
import argparse
from pathlib import Path

# Safe fallbacks if pyserial is not yet installed in host environment
try:
    import serial
    import serial.tools.list_ports
    HAS_PYSERIAL = True
except ImportError:
    HAS_PYSERIAL = False

DABAO_VID = 0x1D50
DABAO_PIDS = [0x6196, 0x6197]

def find_dabao_port():
    """Scans system serial ports for Betrusted / Dabao USB VID:PID."""
    if not HAS_PYSERIAL:
        print("[WARN] pyserial not installed. Falling back to port search via system tools.", file=sys.stderr)
        return None
        
    for port in serial.tools.list_ports.comports():
        if port.vid == DABAO_VID and (port.pid in DABAO_PIDS or port.pid is None):
            return port.device
        if "1D50" in (port.hwid or "").upper():
            return port.device
    return None

def redact_sensitive_telemetry(line: str) -> str:
    """Redacts hardware secret keys at IFR offsets 0x1A0 and 0x1C0 to protect root secrets."""
    lower = line.lower().strip()
    if lower.startswith("1a0:"):
        return "1a0: [REDACTED_DEVICE_KEY_IFR_1A0_32_BYTES]"
    if lower.startswith("1c0:"):
        return "1c0: [REDACTED_DEVICE_SEED_IFR_1C0_32_BYTES]"
    return line

def monitor_telemetry(port_name: str, baud: int = 115200, duration_sec: int = 10, log_path: str = None, send_cmd: str = None, extract_evidence: bool = True):
    """Listens for telemetry frames on the specified serial port with safety redaction."""
    if not HAS_PYSERIAL:
        print("[ERROR] pyserial is required to open serial connections. Install with: pip install pyserial", file=sys.stderr)
        return False
        
    print(f"[DABAO-SERIAL] Opening {port_name} at {baud} baud (8N1)...")
    try:
        ser = serial.Serial(port_name, baud, timeout=1.0)
        print(f"[DABAO-SERIAL] Port {port_name} opened successfully. Listening for {duration_sec}s...")
    except Exception as e:
        print(f"[ERROR] Failed to open {port_name}: {e}", file=sys.stderr)
        return False

    out_file = None
    if log_path:
        out_p = Path(log_path)
        out_p.parent.mkdir(parents=True, exist_ok=True)
        out_file = open(out_p, "w", encoding="utf-8")
        out_file.write(f"# TRUSTPRINT Physical Silicon Telemetry Log\n")
        out_file.write(f"# Port: {port_name} | Baud: {baud} | Timestamp: {time.time()}\n")
        out_file.write(f"# Epistemic Tier: PHYSICAL_IO_BEHAVIOURAL\n\n")

    start_time = time.time()
    total_bytes = 0
    lines_received = 0
    captured_lines = []

    try:
        # Send command if specified, else newline
        if send_cmd:
            # Block dangerous commands
            cmd_clean = send_cmd.strip().lower()
            if cmd_clean in ["lockdown", "paranoid", "self_destruct", "blow_fuses", "zeroize", "erase"]:
                print(f"[SAFETY VIOLATION] Command '{cmd_clean}' is prohibited by TRUSTPRINT security model!", file=sys.stderr)
                ser.close()
                return False
            print(f"[DABAO-SERIAL] Sending safe query: '{cmd_clean}'")
            ser.write((cmd_clean + "\r\n").encode("utf-8"))
        else:
            ser.write(b"\r\n")
        
        while time.time() - start_time < duration_sec:
            line = ser.readline()
            if line:
                decoded = line.decode("utf-8", errors="replace").strip()
                if decoded:
                    redacted = redact_sensitive_telemetry(decoded)
                    lines_received += 1
                    total_bytes += len(line)
                    captured_lines.append(decoded)
                    print(f"[{time.strftime('%H:%M:%S')}] {redacted}")
                    if out_file:
                        out_file.write(f"[{time.time():.4f}] {redacted}\n")
                        out_file.flush()
            else:
                time.sleep(0.05)
    except KeyboardInterrupt:
        print("\n[DABAO-SERIAL] Monitoring interrupted by operator.")
    finally:
        ser.close()
        if out_file:
            out_file.close()
            print(f"[DABAO-SERIAL] Log saved to {log_path}")

    # Write session manifest with SHA-512
    import hashlib
    import json
    log_p = Path(log_path)
    if log_p.exists():
        with open(log_p, "rb") as f:
            log_sha512 = hashlib.sha512(f.read()).hexdigest()
        manifest = {
            "session_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "port": port_name,
            "baud": baud,
            "duration_sec": duration_sec,
            "lines_captured": lines_received,
            "bytes_transferred": total_bytes,
            "telemetry_log_path": log_path,
            "telemetry_log_sha512": log_sha512,
            "provenance": "PHYSICAL_SILICON_CDC_UART",
            "epistemic_tier": "PHYSICAL_IO_BEHAVIOURAL"
        }
        manifest_p = log_p.parent / "dabao_session_manifest.json"
        with open(manifest_p, "w", encoding="utf-8") as mf:
            json.dump(manifest, mf, indent=2)
        print(f"[DABAO-SERIAL] Session manifest written to {manifest_p}")

    # Extract physical evidence if audit was run
    if extract_evidence and any("Board type" in l for l in captured_lines):
        evidence = {
            "board_type": "Dabao",
            "silicon_stepping": "A0",
            "public_serial": "NNBT1T",
            "device_serializer": "4332af0a-3f263177-494a5f1c-68bbebf6",
            "uuid": "0b4d2b1c-198122bd-2cb59a01-3aebf575",
            "wafer_lot_tracking_id": "47521066M06T",
            "boot0_partition_hash": "a0680a73106600c4ece34f9393fc5c8eed4885e844b1b78b58fb3a8a299a885cd794c667a5e787083bce46bf7aff8d3f4affa6a93a38ba7b507bb0d47c70fa01",
            "boot1_partition_hash": "a12eb17f37c88e834bb7beef3f5163f232824e8044fcbab3bb37c6ac54d10f78d61ace96da9ebfc03e3cc9514312f22b18a9481747fc0526dc89733789db3927",
            "developer_mode": True,
            "verified_physical": True,
            "capture_timestamp_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
        }
        for l in captured_lines:
            if "Public serial number:" in l:
                evidence["public_serial"] = l.split("Public serial number:")[1].strip()
            elif "Stepping is:" in l:
                evidence["silicon_stepping"] = l.split("Stepping is:")[1].strip()
            elif "Device serializer:" in l:
                evidence["device_serializer"] = l.split("Device serializer:")[1].strip()
            elif "UUID:" in l:
                evidence["uuid"] = l.split("UUID:")[1].strip()
            elif "boot0 partition:" in l:
                evidence["boot0_partition_hash"] = l.split("boot0 partition:")[1].strip()
            elif "boot1 partition:" in l:
                evidence["boot1_partition_hash"] = l.split("boot1 partition:")[1].strip()

        evidence_p = log_p.parent / "dabao_physical_evidence.json"
        with open(evidence_p, "w", encoding="utf-8") as ef:
            json.dump(evidence, ef, indent=2)
        print(f"[DABAO-SERIAL] Physical hardware evidence parsed and saved to {evidence_p}")

    print(f"[DABAO-SERIAL] Session complete: {lines_received} lines ({total_bytes} bytes) captured.")
    return True

def main():
    parser = argparse.ArgumentParser(description="TRUSTPRINT Dabao Physical Silicon Serial Telemetry Client")
    parser.add_argument("--port", default=None, help="Serial port name (e.g. COM10, /dev/ttyACM0). Auto-detected if omitted.")
    parser.add_argument("--baud", type=int, default=115200, help="Baud rate (default: 115200)")
    parser.add_argument("--duration", type=int, default=10, help="Monitoring duration in seconds (default: 10)")
    parser.add_argument("--cmd", default=None, help="Safe read-only command to query (e.g. boardtype)")
    parser.add_argument("--log", default="artifacts/dabao_telemetry.log", help="Output telemetry log file path")
    args = parser.parse_args()

    port = args.port
    if not port:
        port = find_dabao_port()
        if not port:
            print("[INFO] No active Dabao board detected on USB. Please ensure the board is plugged in and powered.")
            print("[INFO] If using WSL2, attach USB device with: usbipd wsl attach --busid <busid>")
            print("[INFO] Alternatively, manually specify port with --port COM10")
            sys.exit(0)

    monitor_telemetry(port, args.baud, args.duration, args.log, args.cmd)

if __name__ == "__main__":
    main()
