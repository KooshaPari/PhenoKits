use eyetracker_domain::{FaceLandmarks, GazePoint, Point2D, Result, ScreenInfo};
use eyetracker_math::{CameraIntrinsics, KalmanFilter2D};
use eyetracker_calibration::NinePointCalibration;

/// Main gaze estimator pipeline
pub struct GazeEstimator {
    calibration: NinePointCalibration,
    kalman: KalmanFilter2D,
    camera_intrinsics: CameraIntrinsics,
    screen_info: ScreenInfo,
    last_timestamp_ms: i64,
}

impl GazeEstimator {
    pub fn new(
        camera_intrinsics: CameraIntrinsics,
        screen_info: ScreenInfo,
    ) -> Self {
        Self {
            calibration: NinePointCalibration::new(),
            kalman: KalmanFilter2D::new(0.001, 2.0),
            camera_intrinsics,
            screen_info,
            last_timestamp_ms: 0,
        }
    }

    /// Estimate gaze point from face landmarks
    /// Traces to: FR-ESTIMATOR-001
    pub fn estimate_gaze(&mut self, landmarks: &FaceLandmarks, timestamp_ms: i64) -> Result<GazePoint> {
        // Compute eye center from left and right eye landmarks
        let eye_center = Point2D::new(
            (landmarks.left_eye.x + landmarks.right_eye.x) / 2.0,
            (landmarks.left_eye.y + landmarks.right_eye.y) / 2.0,
        );

        // Normalize to device coordinates using camera intrinsics
        let normalized = self.camera_intrinsics.normalize_point(eye_center.x, eye_center.y);

        // Apply Kalman smoothing
        let dt = if self.last_timestamp_ms > 0 {
            ((timestamp_ms - self.last_timestamp_ms) as f32) / 1000.0
        } else {
            0.016  // assume 60Hz
        };

        let smoothed = self.kalman.update(normalized, dt);

        // Map to screen coordinates using calibration
        let screen_point = self.calibration.map_to_screen(&smoothed)?;

        self.last_timestamp_ms = timestamp_ms;

        // Clamp to screen bounds
        let clamped = Point2D::new(
            screen_point.x.clamp(0.0, self.screen_info.width_px as f32),
            screen_point.y.clamp(0.0, self.screen_info.height_px as f32),
        );

        Ok(GazePoint::new(clamped.x, clamped.y, 0.85, timestamp_ms))
    }

    /// Get reference to calibration for 9-point setup
    pub fn calibration_mut(&mut self) -> &mut NinePointCalibration {
        &mut self.calibration
    }

    pub fn calibration(&self) -> &NinePointCalibration {
        &self.calibration
    }
}

/// Fixation detector using velocity and dwell time
pub struct FixationDetector {
    velocity_threshold: f32,
    dwell_time_ms: i64,
    last_gaze: Option<GazePoint>,
    current_cluster: Vec<GazePoint>,
}

impl FixationDetector {
    pub fn new(velocity_threshold: f32, dwell_time_ms: i64) -> Self {
        Self {
            velocity_threshold,
            dwell_time_ms,
            last_gaze: None,
            current_cluster: vec![],
        }
    }

    /// Process gaze point and detect fixations
    /// Traces to: FR-ESTIMATOR-002
    pub fn process(&mut self, gaze: &GazePoint) -> Option<eyetracker_domain::Fixation> {
        self.current_cluster.push(*gaze);

        if let Some(last) = self.last_gaze {
            let dx = gaze.x - last.x;
            let dy = gaze.y - last.y;
            let distance = (dx * dx + dy * dy).sqrt();
            let velocity = distance / ((gaze.timestamp_ms - last.timestamp_ms).max(1) as f32 / 1000.0);

            if velocity > self.velocity_threshold {
                // Saccade detected, reset cluster
                self.current_cluster.clear();
                self.current_cluster.push(*gaze);
            } else if gaze.timestamp_ms - self.current_cluster[0].timestamp_ms >= self.dwell_time_ms {
                // Fixation detected
                let mean_x = self.current_cluster.iter().map(|g| g.x).sum::<f32>() / self.current_cluster.len() as f32;
                let mean_y = self.current_cluster.iter().map(|g| g.y).sum::<f32>() / self.current_cluster.len() as f32;

                let fixation = eyetracker_domain::Fixation {
                    center: Point2D::new(mean_x, mean_y),
                    duration_ms: gaze.timestamp_ms - self.current_cluster[0].timestamp_ms,
                    start_time_ms: self.current_cluster[0].timestamp_ms,
                };

                self.current_cluster.clear();
                self.current_cluster.push(*gaze);

                self.last_gaze = Some(*gaze);
                return Some(fixation);
            }
        }

        self.last_gaze = Some(*gaze);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaze_estimator_creation() {
        let cam = CameraIntrinsics::new(500.0, 500.0, 320.0, 240.0, 640, 480);
        let screen = ScreenInfo { width_px: 1080, height_px: 1920, dpi: 326.0 };
        let _estimator = GazeEstimator::new(cam, screen);
    }

    #[test]
    fn test_fixation_detector_init() {
        let detector = FixationDetector::new(50.0, 100);
        assert_eq!(detector.velocity_threshold, 50.0);
    }
}
