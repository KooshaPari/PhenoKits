use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum EyetrackerError {
    #[error("calibration failed: {0}")]
    CalibrationFailed(String),
    #[error("invalid gaze estimate: {0}")]
    InvalidGazeEstimate(String),
    #[error("storage error: {0}")]
    StorageError(String),
}

pub type Result<T> = std::result::Result<T, EyetrackerError>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point2D) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceLandmarks {
    pub left_eye: Point2D,
    pub right_eye: Point2D,
    pub nose: Point2D,
    pub left_cheek: Point2D,
    pub right_cheek: Point2D,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GazePoint {
    pub x: f32,
    pub y: f32,
    pub confidence: f32,
    pub timestamp_ms: i64,
}

impl GazePoint {
    pub fn new(x: f32, y: f32, confidence: f32, timestamp_ms: i64) -> Self {
        Self { x, y, confidence, timestamp_ms }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Fixation {
    pub center: Point2D,
    pub duration_ms: i64,
    pub start_time_ms: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Saccade {
    pub start: Point2D,
    pub end: Point2D,
    pub duration_ms: i64,
    pub velocity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationPoint {
    pub screen_position: Point2D,
    pub gaze_samples: Vec<Point2D>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenInfo {
    pub width_px: u32,
    pub height_px: u32,
    pub dpi: f32,
}

impl ScreenInfo {
    pub fn dpr(&self) -> f32 {
        self.dpi / 96.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_distance() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_gaze_point_creation() {
        let gaze = GazePoint::new(100.0, 200.0, 0.95, 1000);
        assert_eq!(gaze.x, 100.0);
        assert_eq!(gaze.confidence, 0.95);
    }

    #[test]
    fn test_screen_info_dpr() {
        let screen = ScreenInfo { width_px: 1080, height_px: 1920, dpi: 326.0 };
        assert!((screen.dpr() - 3.395833).abs() < 0.01);
    }
}
