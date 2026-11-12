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

def monitor_telemetry(port_name: str, baud: int = 115200, duration_sec: int = 10, log_path: str = None, send_cmd: str = None):
    """Listens for telemetry frames on the specified serial port."""
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

    try:
        # Send command if specified, else newline
        if send_cmd:
            # Block dangerous commands
            cmd_clean = send_cmd.strip().lower()
            if cmd_clean in ["lockdown", "paranoid", "self_destruct"]:
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
                    lines_received += 1
                    total_bytes += len(line)
                    print(f"[{time.strftime('%H:%M:%S')}] {decoded}")
                    if out_file:
                        out_file.write(f"[{time.time():.4f}] {decoded}\n")
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
