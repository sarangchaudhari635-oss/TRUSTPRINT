//! TRUSTPRINT Optical Die Inspection & CAD Registration Engine (IRIS)
//!
//! Epistemic Tier 4: OPTICAL_IRIS
//! Provides non-destructive backside infrared silicon microscopy analysis,
//! CAD floorplan modeling, and affine landmark registration for the Baochip-1x SoC.

pub mod aligner;
pub mod models;

pub use aligner::{DieAligner, IrisError};
pub use models::*;

pub struct Baochip1xFloorplan;

impl Baochip1xFloorplan {
    /// Returns the canonical reference CAD floorplan for the Baochip-1x (TSMC 22nm ULL).
    pub fn reference() -> CadFloorplan {
        CadFloorplan {
            chip_name: "Baochip-1x".to_string(),
            process_node: "TSMC 22nm ULL".to_string(),
            die_width_um: 2500.0,
            die_height_um: 2500.0,
            macros: vec![
                MacroBlock {
                    id: "CPU_VEXRISCV".to_string(),
                    name: "VexRiscv RV32-IMAC CPU Core".to_string(),
                    kind: MacroKind::CpuCore,
                    bbox_um: BoundingBox::new(300.0, 300.0, 900.0, 900.0),
                    description: "Main 350 MHz system processor with Sv39 MMU support".to_string(),
                },
                MacroBlock {
                    id: "SRAM_2MB".to_string(),
                    name: "2 MiB High-Density System SRAM".to_string(),
                    kind: MacroKind::SramBlock,
                    bbox_um: BoundingBox::new(1000.0, 300.0, 2200.0, 1100.0),
                    description: "Primary system memory for code execution and DMA buffers".to_string(),
                },
                MacroBlock {
                    id: "RERAM_4MB".to_string(),
                    name: "4 MiB CrossBar ReRAM Non-Volatile Storage".to_string(),
                    kind: MacroKind::ReramBlock,
                    bbox_um: BoundingBox::new(300.0, 1100.0, 1500.0, 2200.0),
                    description: "Non-volatile resistive RAM for firmware and encrypted secrets".to_string(),
                },
                MacroBlock {
                    id: "BIO_CLUSTER".to_string(),
                    name: "4x PicoRV32 BIO Coprocessor Cluster".to_string(),
                    kind: MacroKind::BioCluster,
                    bbox_um: BoundingBox::new(1600.0, 1200.0, 2200.0, 1800.0),
                    description: "700 MHz multi-core I/O coprocessor cluster with private RAMs".to_string(),
                },
                MacroBlock {
                    id: "CRYPTO_ENGINE".to_string(),
                    name: "Hardware Cryptography Acceleration Subsystem".to_string(),
                    kind: MacroKind::CryptoEngine,
                    bbox_um: BoundingBox::new(1600.0, 1850.0, 2200.0, 2200.0),
                    description: "AES-256, SHA-256/512, PKE engine, and avalanche TRNG".to_string(),
                },
            ],
            fiducials: vec![
                ("FID_TL".to_string(), Point2D::new(100.0, 100.0)),
                ("FID_TR".to_string(), Point2D::new(2400.0, 100.0)),
                ("FID_BL".to_string(), Point2D::new(100.0, 2400.0)),
                ("FID_BR".to_string(), Point2D::new(2400.0, 2400.0)),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affine_identity_transform() {
        let landmarks = vec![
            LandmarkPair {
                name: "TL".to_string(),
                cad_um: Point2D::new(100.0, 100.0),
                observed_px: Point2D::new(100.0, 100.0),
            },
            LandmarkPair {
                name: "TR".to_string(),
                cad_um: Point2D::new(2400.0, 100.0),
                observed_px: Point2D::new(2400.0, 100.0),
            },
            LandmarkPair {
                name: "BL".to_string(),
                cad_um: Point2D::new(100.0, 2400.0),
                observed_px: Point2D::new(100.0, 2400.0),
            },
            LandmarkPair {
                name: "BR".to_string(),
                cad_um: Point2D::new(2400.0, 2400.0),
                observed_px: Point2D::new(2400.0, 2400.0),
            },
        ];

        let tf = DieAligner::compute_affine_transform(&landmarks).unwrap();
        assert!((tf.a - 1.0).abs() < 1e-4);
        assert!(tf.b.abs() < 1e-4);
        assert!(tf.c.abs() < 1e-4);
        assert!(tf.d.abs() < 1e-4);
        assert!((tf.e - 1.0).abs() < 1e-4);
        assert!(tf.f.abs() < 1e-4);

        let rms = DieAligner::calculate_rms_error(&landmarks, &tf);
        assert!(rms < 1e-4);
    }

    #[test]
    fn test_affine_scale_and_translation() {
        // Micrograph resolution: 1000x1000 pixels covering 2500x2500 um die
        // Scale = 0.4 px/um, translation = (50 px, 50 px)
        let scale = 0.4;
        let tx = 50.0;
        let ty = 50.0;

        let landmarks = vec![
            LandmarkPair {
                name: "TL".to_string(),
                cad_um: Point2D::new(100.0, 100.0),
                observed_px: Point2D::new(100.0 * scale + tx, 100.0 * scale + ty),
            },
            LandmarkPair {
                name: "TR".to_string(),
                cad_um: Point2D::new(2400.0, 100.0),
                observed_px: Point2D::new(2400.0 * scale + tx, 100.0 * scale + ty),
            },
            LandmarkPair {
                name: "BL".to_string(),
                cad_um: Point2D::new(100.0, 2400.0),
                observed_px: Point2D::new(100.0 * scale + tx, 2400.0 * scale + ty),
            },
        ];

        let tf = DieAligner::compute_affine_transform(&landmarks).unwrap();
        assert!((tf.a - scale).abs() < 1e-4);
        assert!(tf.b.abs() < 1e-4);
        assert!((tf.c - tx).abs() < 1e-4);
        assert!(tf.d.abs() < 1e-4);
        assert!((tf.e - scale).abs() < 1e-4);
        assert!((tf.f - ty).abs() < 1e-4);
    }

    #[test]
    fn test_inspect_and_overlay_report() {
        let fp = Baochip1xFloorplan::reference();
        let scale = 0.5;
        let landmarks = vec![
            LandmarkPair {
                name: "TL".to_string(),
                cad_um: Point2D::new(100.0, 100.0),
                observed_px: Point2D::new(50.0, 50.0),
            },
            LandmarkPair {
                name: "TR".to_string(),
                cad_um: Point2D::new(2400.0, 100.0),
                observed_px: Point2D::new(1200.0, 50.0),
            },
            LandmarkPair {
                name: "BL".to_string(),
                cad_um: Point2D::new(100.0, 2400.0),
                observed_px: Point2D::new(50.0, 1200.0),
            },
        ];

        let report = DieAligner::inspect_and_overlay(&fp, &landmarks).unwrap();
        assert_eq!(report.epistemic_tier, "OPTICAL_IRIS");
        assert_eq!(report.overlays.len(), 5);
        assert!(report.rms_error_px < 1e-4);

        // Check projected coordinates for CPU
        let cpu_overlay = report.overlays.iter().find(|o| o.id == "CPU_VEXRISCV").unwrap();
        assert!((cpu_overlay.projected_bbox_px.min_x - (300.0 * scale)).abs() < 1e-3);
        assert!((cpu_overlay.projected_bbox_px.max_x - (900.0 * scale)).abs() < 1e-3);
        assert_eq!(report.scientific_caveats.len(), 4);
    }
}

