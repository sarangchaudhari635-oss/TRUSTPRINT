# 17 — UI/UX Specification: Operator Research Dashboard

**Document Reference:** SPEC-UIUX-001  
**Project:** TRUSTPRINT  
**Version:** 0.1.0-alpha  

---

## 1. UI Design Principles

The TRUSTPRINT Operator Dashboard is an engineering research tool built for researchers, auditors, and hackathon evaluators.

- **Dark Mode Silicon Aesthetic:** Clean, high-contrast dark theme (#0d1117 background, #58a6ff accents, #3fb950 pass green, #f85149 fail red).
- **Zero Mock Concealment:** If data is simulated, an amber banner displays: `[SIMULATION / MOCK RUNTIME — NOT PHYSICAL SILICON]`.
- **High-Density Data Visualization:** Waveforms and register states render on high-performance HTML5 Canvas / WebGL.

---

## 2. Dashboard Screen Wireframes & Component Layout

```mermaid
graph TD
    subgraph Navigation ["Global Top Navigation Bar"]
        NAV_TITLE["TRUSTPRINT Lab v0.1.0"]
        NAV_STATUS["Hardware: CONNECTED (Baochip-1x @ 350MHz | BIO @ 700MHz)"]
        NAV_MESH["Security Mesh: INTACT"]
    end

    subgraph MainLayout ["Three-Column Operator Workspace"]
        subgraph Col1 ["Left: System Explorer"]
            RTL_PANEL["RTL Module Tree & Port Inspector"]
            EXP_SELECT["Experiment Suite Selector"]
        end

        subgraph Col2 ["Center: Active Telemetry & Visualizations"]
            WAV_PANEL["Interactive Waveform Viewer (FST / VCD Canvas)"]
            IRIS_PANEL["IRIS Optical Die Inspection (SWIR Micrograph + CAD Overlay)"]
            SERIAL_LOG["Live Serial Telemetry Console (115,200 baud)"]
        end

        subgraph Col3 ["Right: Attestation Vault"]
            CONF_PANEL["Conformance Verdict (PASS / Jitter Stats)"]
            CRYPTO_PANEL["Hardware Ed25519 Certificate Verifier"]
            EXPORT_BTN["Export Signed .tpe Bundle"]
        end
    end
```

---

## 3. Screen Specifications

### Screen 1: Overview & Hardware Status
- Displays live CPU frequency (350 MHz), BIO frequency (700 MHz), voltage rail health, active security mesh state, and glitch counter.
- Shows connection status indicator (USB CDC serial or Simulator backend).

### Screen 2: RTL Explorer
- Interactive tree view of parsed Verilog modules (`bio_bdma_wrapper`, `uart`, `pke_engine`).
- Clicking a module renders its port list, clock parameters, and register offset table.

### Screen 3: Waveform Viewer (Digital Trace Canvas)
- Renders high-speed digital logic traces (`gpio_out[0]`, `gpio_in[1]`, `quantum_snap_tick`).
- Features interactive pan and zoom down to picosecond resolution.
- Renders expected timing envelope boundary lines in dashed green.

### Screen 4: IRIS Die Inspection Viewer
- Split-screen comparison or translucent overlay slider comparing SWIR 1050nm infrared optical micrographs against CAD floorplan bounding boxes.
- Interactive toggle buttons highlighting SRAM arrays, ReRAM matrices, and CPU core perimeters.

### Screen 5: Evidence & Certificate Verifier
- Drag-and-drop zone for `.tpe` evidence bundles.
- Displays full cryptographic hash tree, Ed25519 signature validity status, and hardware provenance metadata.

