use eyetracker_calibration::NinePointCalibration;
use eyetracker_domain::{FaceLandmarks, GazePoint, Point2D, ScreenInfo};
use eyetracker_estimator::{FixationDetector, GazeEstimator};
use eyetracker_math::CameraIntrinsics;

// Re-export types for UDL
pub use eyetracker_domain::{Fixation, GazePoint as GazePointType};

uniffi::setup_scaffolding!();

#[derive(Clone)]
pub struct GazeEstimatorFFI {
    estimator: std::sync::Arc<std::sync::Mutex<GazeEstimator>>,
}

impl GazeEstimatorFFI {
    pub fn new(camera: CameraIntrinsicsFFI, screen: ScreenInfoFFI) -> Self {
        let cam = CameraIntrinsics::new(
            camera.focal_length_x,
            camera.focal_length_y,
            camera.principal_point_x,
            camera.principal_point_y,
            camera.width,
            camera.height,
        );
        let scr = ScreenInfo {
            width_px: screen.width_px,
            height_px: screen.height_px,
            dpi: screen.dpi,
        };

        let estimator = GazeEstimator::new(cam, scr);
        Self {
            estimator: std::sync::Arc::new(std::sync::Mutex::new(estimator)),
        }
    }

    pub fn estimate_gaze(&self, landmarks: FaceLandmarksFFI, timestamp_ms: i64) -> GazePointFFI {
        let landmarks_domain = FaceLandmarks {
            left_eye: Point2D::new(landmarks.left_eye.x, landmarks.left_eye.y),
            right_eye: Point2D::new(landmarks.right_eye.x, landmarks.right_eye.y),
            nose: Point2D::new(landmarks.nose.x, landmarks.nose.y),
            left_cheek: Point2D::new(landmarks.left_cheek.x, landmarks.left_cheek.y),
            right_cheek: Point2D::new(landmarks.right_cheek.x, landmarks.right_cheek.y),
        };

        let mut estimator = self.estimator.lock().unwrap();
        match estimator.estimate_gaze(&landmarks_domain, timestamp_ms) {
            Ok(gaze) => GazePointFFI {
                x: gaze.x,
                y: gaze.y,
                confidence: gaze.confidence,
                timestamp_ms: gaze.timestamp_ms,
            },
            Err(_) => GazePointFFI {
                x: 0.0,
                y: 0.0,
                confidence: 0.0,
                timestamp_ms,
            },
        }
    }
}

#[derive(Clone)]
pub struct FixationDetectorFFI {
    detector: std::sync::Arc<std::sync::Mutex<FixationDetector>>,
}

impl FixationDetectorFFI {
    pub fn new(velocity_threshold: f32, dwell_time_ms: i64) -> Self {
        let detector = FixationDetector::new(velocity_threshold, dwell_time_ms);
        Self {
            detector: std::sync::Arc::new(std::sync::Mutex::new(detector)),
        }
    }

    pub fn process(&self, gaze: GazePointFFI) -> Option<FixationFFI> {
        let gaze_domain = GazePoint::new(gaze.x, gaze.y, gaze.confidence, gaze.timestamp_ms);
        let mut detector = self.detector.lock().unwrap();

        detector.process(&gaze_domain).map(|fixation| FixationFFI {
            center: Point2DFFI {
                x: fixation.center.x,
                y: fixation.center.y,
            },
            duration_ms: fixation.duration_ms,
            start_time_ms: fixation.start_time_ms,
        })
    }
}

// FFI-safe wrapper structs
#[derive(Clone, Debug)]
pub struct Point2DFFI {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct GazePointFFI {
    pub x: f32,
    pub y: f32,
    pub confidence: f32,
    pub timestamp_ms: i64,
}

#[derive(Clone, Debug)]
pub struct FixationFFI {
    pub center: Point2DFFI,
    pub duration_ms: i64,
    pub start_time_ms: i64,
}

#[derive(Clone, Debug)]
pub struct FaceLandmarksFFI {
    pub left_eye: Point2DFFI,
    pub right_eye: Point2DFFI,
    pub nose: Point2DFFI,
    pub left_cheek: Point2DFFI,
    pub right_cheek: Point2DFFI,
}

#[derive(Clone, Debug)]
pub struct ScreenInfoFFI {
    pub width_px: u32,
    pub height_px: u32,
    pub dpi: f32,
}

#[derive(Clone, Debug)]
pub struct CameraIntrinsicsFFI {
    pub focal_length_x: f32,
    pub focal_length_y: f32,
    pub principal_point_x: f32,
    pub principal_point_y: f32,
    pub width: u32,
    pub height: u32,
}
