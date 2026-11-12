# Test — Hardware-in-the-Loop (HIL) Test Suite

**Document Reference:** TEST-HW-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. Hardware Test Cases (Dabao Board)

| Test ID | Objective | Preconditions | Execution Procedure | Expected Result | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **HW-001** | USB Bootloader Enumeration | Dabao board unpowered. | Hold PROG button, insert USB-C, release after 2s. | Mass storage drive `BAOCHIP` mounts on host. | **PLANNED** |
| **HW-002** | Xous OS Boot & Console | Board flashed with `apps.uf2`. | Press PROG button to boot; open serial terminal at 115,200 baud. | Xous boot banner and console prompt emitted. | **PLANNED** |
| **HW-003** | BIO Core Clock Activation | Xous running. | Send `START_BIO_CLOCK` command over serial. | Serial console logs `BIO cluster running @ 700 MHz`. | **PLANNED** |
| **HW-004** | Physical GPIO Pin Probing | Logic analyzer / probe on Pin 0. | Execute `bio_gpio_deterministic_pulse` routine. | Logic analyzer measures 1.000 MHz square wave with sub-nanosecond jitter. | **PLANNED** |
| **HW-005** | Hardware TRNG Entropy Check | Xous crypto server active. | Query 1,024 bytes of TRNG random data. | Entropy test passes; non-repeating byte stream. | **PLANNED** |
| **HW-006** | Hardware Ed25519 Signing | Xous crypto server active. | Send canonical test digest for hardware signing. | Valid 64-byte Ed25519 signature returned via serial. | **PLANNED** |

---

## 2. Hardware Test Logging

Every hardware test logs directly to `evidence/hardware_logs/` with timestamp, board serial/DNA hash, and raw serial capture stream.
