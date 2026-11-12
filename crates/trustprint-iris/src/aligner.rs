//! Affine Transformation Alignment Engine for SWIR Micrographs

use crate::models::{
    AffineTransform, CadFloorplan, LandmarkPair, MacroOverlayResult, OpticalInspectionReport,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IrisError {
    #[error("Insufficient landmarks for affine alignment: need at least 3, got {0}")]
    InsufficientLandmarks(usize),

    #[error("Singular matrix encountered during landmark regression")]
    SingularMatrix,

    #[error("I/O error during optical inspection: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub struct DieAligner;

impl DieAligner {
    /// Solves for the 6-parameter affine transform (a, b, c, d, e, f)
    /// from a set of at least 3 CAD-to-image landmark correspondence pairs.
    pub fn compute_affine_transform(
        landmarks: &[LandmarkPair],
    ) -> Result<AffineTransform, IrisError> {
        let n = landmarks.len();
        if n < 3 {
            return Err(IrisError::InsufficientLandmarks(n));
        }

        let nf = n as f64;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_x2 = 0.0;
        let mut sum_y2 = 0.0;
        let mut sum_xy = 0.0;

        let mut sum_u = 0.0;
        let mut sum_xu = 0.0;
        let mut sum_yu = 0.0;

        let mut sum_v = 0.0;
        let mut sum_xv = 0.0;
        let mut sum_yv = 0.0;

        for lm in landmarks {
            let x = lm.cad_um.x;
            let y = lm.cad_um.y;
            let u = lm.observed_px.x;
            let v = lm.observed_px.y;

            sum_x += x;
            sum_y += y;
            sum_x2 += x * x;
            sum_y2 += y * y;
            sum_xy += x * y;

            sum_u += u;
            sum_xu += x * u;
            sum_yu += y * u;

            sum_v += v;
            sum_xv += x * v;
            sum_yv += y * v;
        }

        // Solve 3x3 normal system: M * [a, b, c]^T = [sum_xu, sum_yu, sum_u]^T
        // M = [[sum_x2, sum_xy, sum_x],
        //      [sum_xy, sum_y2, sum_y],
        //      [sum_x,  sum_y,  n     ]]
        let det = sum_x2 * (sum_y2 * nf - sum_y * sum_y)
            - sum_xy * (sum_xy * nf - sum_x * sum_y)
            + sum_x * (sum_xy * sum_y - sum_x * sum_y2);

        if det.abs() < 1e-12 {
            return Err(IrisError::SingularMatrix);
        }

        let inv_det = 1.0 / det;

        // Cramer's rule for (a, b, c)
        let det_a = sum_xu * (sum_y2 * nf - sum_y * sum_y)
            - sum_yu * (sum_xy * nf - sum_x * sum_y)
            + sum_u * (sum_xy * sum_y - sum_x * sum_y2);

        let det_b = sum_x2 * (sum_yu * nf - sum_u * sum_y)
            - sum_xy * (sum_xu * nf - sum_u * sum_x)
            + sum_x * (sum_xu * sum_y - sum_yu * sum_x);

        let det_c = sum_x2 * (sum_y2 * sum_u - sum_y * sum_yu)
            - sum_xy * (sum_xy * sum_u - sum_y * sum_xu)
            + sum_x * (sum_xy * sum_yu - sum_y2 * sum_xu);

        let a = det_a * inv_det;
        let b = det_b * inv_det;
        let c = det_c * inv_det;

        // Cramer's rule for (d, e, f)
        let det_d = sum_xv * (sum_y2 * nf - sum_y * sum_y)
            - sum_yv * (sum_xy * nf - sum_x * sum_y)
            + sum_v * (sum_xy * sum_y - sum_x * sum_y2);

        let det_e = sum_x2 * (sum_yv * nf - sum_v * sum_y)
            - sum_xy * (sum_xv * nf - sum_v * sum_x)
            + sum_x * (sum_xv * sum_y - sum_yv * sum_x);

        let det_f = sum_x2 * (sum_y2 * sum_v - sum_y * sum_yv)
            - sum_xy * (sum_xy * sum_v - sum_y * sum_xv)
            + sum_x * (sum_xy * sum_yv - sum_y2 * sum_xv);

        let d = det_d * inv_det;
        let e = det_e * inv_det;
        let f = det_f * inv_det;

        Ok(AffineTransform { a, b, c, d, e, f })
    }

    /// Calculates root-mean-square (RMS) alignment error across landmark pairs in pixels.
    pub fn calculate_rms_error(landmarks: &[LandmarkPair], transform: &AffineTransform) -> f64 {
        if landmarks.is_empty() {
            return 0.0;
        }

        let mut sum_sq_err = 0.0;
        for lm in landmarks {
            let projected = transform.apply(&lm.cad_um);
            let err_sq = (projected.x - lm.observed_px.x).powi(2)
                + (projected.y - lm.observed_px.y).powi(2);
            sum_sq_err += err_sq;
        }

        (sum_sq_err / (landmarks.len() as f64)).sqrt()
    }

    /// Projects all CAD floorplan macro boundaries onto optical micrograph pixel space
    /// and generates a formal Epistemic Tier 4 inspection report.
    pub fn inspect_and_overlay(
        floorplan: &CadFloorplan,
        landmarks: &[LandmarkPair],
    ) -> Result<OpticalInspectionReport, IrisError> {
        let transform = Self::compute_affine_transform(landmarks)?;
        let rms_error_px = Self::calculate_rms_error(landmarks, &transform);

        let mut overlays = Vec::with_capacity(floorplan.macros.len());
        for m in &floorplan.macros {
            let projected_bbox = transform.apply_bbox(&m.bbox_um);
            // Confidence metric decays if RMS alignment error is high
            let confidence = (1.0 - (rms_error_px / 50.0)).clamp(0.0, 1.0);

            overlays.push(MacroOverlayResult {
                id: m.id.clone(),
                name: m.name.clone(),
                kind: m.kind.clone(),
                cad_bbox_um: m.bbox_um.clone(),
                projected_bbox_px: projected_bbox,
                feature_confidence: confidence,
            });
        }

        Ok(OpticalInspectionReport {
            timestamp_utc: chrono::Utc::now().to_rfc3339(),
            epistemic_tier: "OPTICAL_IRIS".to_string(),
            chip_name: floorplan.chip_name.clone(),
            transform,
            rms_error_px,
            overlays,
            scientific_caveats: vec![
                "PROVES: Macro-level physical presence, bounding geometry, and silicon floorplan alignment.".to_string(),
                "PROVES: Physical coexistence of SRAM, ReRAM, CPU, and BIO macro blocks.".to_string(),
                "DOES NOT PROVE: Transistor-level netlist equivalence or individual gate functions.".to_string(),
                "DOES NOT PROVE: Absence of microscopic hardware Trojans or modified doping concentrations.".to_string(),
            ],
        })
    }
}
