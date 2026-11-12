//! Optical Die Inspection & CAD Registration Data Models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl BoundingBox {
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }

    pub fn center(&self) -> Point2D {
        Point2D::new(
            (self.min_x + self.max_x) / 2.0,
            (self.min_y + self.max_y) / 2.0,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacroKind {
    CpuCore,
    SramBlock,
    ReramBlock,
    BioCluster,
    CryptoEngine,
    IoRing,
    FiducialMark,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacroBlock {
    pub id: String,
    pub name: String,
    pub kind: MacroKind,
    pub bbox_um: BoundingBox,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadFloorplan {
    pub chip_name: String,
    pub process_node: String,
    pub die_width_um: f64,
    pub die_height_um: f64,
    pub macros: Vec<MacroBlock>,
    pub fiducials: Vec<(String, Point2D)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffineTransform {
    pub a: f64, // scale_x * cos(theta)
    pub b: f64, // -scale_y * sin(theta)
    pub c: f64, // translation_x
    pub d: f64, // scale_x * sin(theta)
    pub e: f64, // scale_y * cos(theta)
    pub f: f64, // translation_y
}

impl AffineTransform {
    pub fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            e: 1.0,
            f: 0.0,
        }
    }

    pub fn apply(&self, p: &Point2D) -> Point2D {
        Point2D {
            x: self.a * p.x + self.b * p.y + self.c,
            y: self.d * p.x + self.e * p.y + self.f,
        }
    }

    pub fn apply_bbox(&self, bbox: &BoundingBox) -> BoundingBox {
        let p1 = self.apply(&Point2D::new(bbox.min_x, bbox.min_y));
        let p2 = self.apply(&Point2D::new(bbox.max_x, bbox.min_y));
        let p3 = self.apply(&Point2D::new(bbox.min_x, bbox.max_y));
        let p4 = self.apply(&Point2D::new(bbox.max_x, bbox.max_y));

        let min_x = p1.x.min(p2.x).min(p3.x).min(p4.x);
        let max_x = p1.x.max(p2.x).max(p3.x).max(p4.x);
        let min_y = p1.y.min(p2.y).min(p3.y).min(p4.y);
        let max_y = p1.y.max(p2.y).max(p3.y).max(p4.y);

        BoundingBox::new(min_x, min_y, max_x, max_y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandmarkPair {
    pub name: String,
    pub cad_um: Point2D,
    pub observed_px: Point2D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroOverlayResult {
    pub id: String,
    pub name: String,
    pub kind: MacroKind,
    pub cad_bbox_um: BoundingBox,
    pub projected_bbox_px: BoundingBox,
    pub feature_confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticalInspectionReport {
    pub timestamp_utc: String,
    pub epistemic_tier: String,
    pub chip_name: String,
    pub transform: AffineTransform,
    pub rms_error_px: f64,
    pub overlays: Vec<MacroOverlayResult>,
    pub scientific_caveats: Vec<String>,
}

