#!/usr/bin/env python3
"""
VCD / FST Transition Extractor for TRUSTPRINT Conformance Analysis
Extracts timestamped transitions for a given net or bit index.
"""

import sys
import subprocess
from pathlib import Path

def extract_signal_transitions(fst_path: str, target_signal: str, bit_index: int = None):
    p = subprocess.Popen(["fst2vcd", fst_path], stdout=subprocess.PIPE, text=True, bufsize=65536)
    
    scopes = []
    ident_to_name = {}
    target_ident = None
    target_width = 1
    
    # 1. Parse header
    for line in p.stdout:
        line = line.strip()
        if not line:
            continue
        parts = line.split()
        if parts[0] == "$scope":
            scopes.append(parts[2])
        elif parts[0] == "$upscope":
            if scopes:
                scopes.pop()
        elif parts[0] == "$var":
            # $var wire 32 d gpio_out [31:0] $end
            v_type = parts[1]
            width = int(parts[2])
            ident = parts[3]
            name = parts[4]
            full_name = ".".join(scopes + [name])
            if name == target_signal or full_name.endswith(target_signal):
                target_ident = ident
                target_width = width
                print(f"[EXTRACTOR] Found target signal '{full_name}' -> ident '{ident}', width {width}", file=sys.stderr)
        elif parts[0] == "$enddefinitions":
            break
            
    if not target_ident:
        print(f"[EXTRACTOR] Error: signal '{target_signal}' not found in VCD definitions", file=sys.stderr)
        p.kill()
        return []
        
    # 2. Parse value changes
    current_time_ps = 0
    transitions = []
    last_val = None
    
    for line in p.stdout:
        line = line.strip()
        if not line:
            continue
        if line.startswith("#"):
            try:
                current_time_ps = int(line[1:])
            except ValueError:
                pass
        elif line.endswith(target_ident):
            val_part = line[:-len(target_ident)].strip()
            # If multi-bit vector e.g. "b00000000001000000000000000000000"
            if val_part.startswith("b"):
                bin_str = val_part[1:]
                if bit_index is not None:
                    # Pad to full width if needed
                    bin_str = bin_str.zfill(target_width)
                    # bit_index 0 is rightmost (LSB)
                    char_idx = len(bin_str) - 1 - bit_index
                    if 0 <= char_idx < len(bin_str):
                        bit_val = 1 if bin_str[char_idx] == '1' else 0
                        if bit_val != last_val:
                            transitions.append((current_time_ps, bit_val))
                            last_val = bit_val
            else:
                # 1-bit signal e.g. "1", "0"
                bit_val = 1 if val_part == "1" else 0
                if bit_val != last_val:
                    transitions.append((current_time_ps, bit_val))
                    last_val = bit_val

    p.wait()
    return transitions

if __name__ == "__main__":
    import argparse
    import json
    
    parser = argparse.ArgumentParser(description="Extract transition events from FST/VCD for TRUSTPRINT")
    import os
    default_fst = os.environ.get("BIO_SIM_WAVEFORM", "artifacts/trustprint_multicore.fst")
    parser.add_argument("fst_file", nargs="?", default=default_fst, help="Input .fst/.vcd file")
    parser.add_argument("--signal", default="gpio_out", help="Target net name (default: gpio_out)")
    parser.add_argument("--bit", type=int, default=21, help="Bit index for vector net (default: 21)")
    parser.add_argument("--json", dest="json_out", default=None, help="Optional output JSON path")
    args = parser.parse_args()
    
    events = extract_signal_transitions(args.fst_file, args.signal, args.bit)
    print(f"[EXTRACTOR] Extracted {len(events)} transitions for {args.signal}[{args.bit}]")
    
    if args.json_out:
        edge_data = [{"timestamp_ps": t, "state": bool(v)} for t, v in events]
        with open(args.json_out, "w") as f:
            json.dump(edge_data, f, indent=2)
        print(f"[EXTRACTOR] Wrote {len(edge_data)} edge events to {args.json_out}")
    else:
        for t_ps, v in events[:20]:
            print(f"  {t_ps} ps: {v}")
        if len(events) > 20:
            print(f"  ... ({len(events) - 20} more transitions)")

