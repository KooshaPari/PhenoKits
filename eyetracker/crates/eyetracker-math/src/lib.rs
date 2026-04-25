use eyetracker_domain::Point2D;

/// Kalman filter for 2D gaze smoothing (simplified exponential smoothing for MVP)
pub struct KalmanFilter2D {
    last_position: Point2D,
    alpha: f32,  // exponential smoothing factor
}

impl KalmanFilter2D {
    pub fn new(q_process_noise: f32, r_meas_noise: f32) -> Self {
        // Alpha = 2 / (n + 1) where n = window size
        // Higher r_meas_noise → lower alpha (more smoothing)
        let alpha = (q_process_noise / (q_process_noise + r_meas_noise)).min(1.0).max(0.0);
        Self {
            last_position: Point2D::new(0.0, 0.0),
            alpha,
        }
    }

    pub fn update(&mut self, measurement: Point2D, _dt: f32) -> Point2D {
        // Exponential smoothing: smoothed = alpha * measurement + (1 - alpha) * last
        let smoothed = Point2D::new(
            self.alpha * measurement.x + (1.0 - self.alpha) * self.last_position.x,
            self.alpha * measurement.y + (1.0 - self.alpha) * self.last_position.y,
        );
        self.last_position = smoothed;
        smoothed
    }
}

/// Camera intrinsics for gaze-to-screen mapping
#[derive(Clone, Debug)]
pub struct CameraIntrinsics {
    pub focal_length_x: f32,
    pub focal_length_y: f32,
    pub principal_point_x: f32,
    pub principal_point_y: f32,
    pub width: u32,
    pub height: u32,
}

impl CameraIntrinsics {
    pub fn new(focal_x: f32, focal_y: f32, px: f32, py: f32, w: u32, h: u32) -> Self {
        Self {
            focal_length_x: focal_x,
            focal_length_y: focal_y,
            principal_point_x: px,
            principal_point_y: py,
            width: w,
            height: h,
        }
    }

    /// Project camera coordinates to normalized device coordinates
    pub fn normalize_point(&self, camera_x: f32, camera_y: f32) -> Point2D {
        let norm_x = (camera_x - self.principal_point_x) / self.focal_length_x;
        let norm_y = (camera_y - self.principal_point_y) / self.focal_length_y;
        Point2D::new(norm_x, norm_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kalman_filter_init() {
        let kf = KalmanFilter2D::new(0.01, 1.0);
        assert_eq!(kf.last_position.x, 0.0);
    }

    #[test]
    fn test_camera_intrinsics_normalize() {
        let cam = CameraIntrinsics::new(500.0, 500.0, 320.0, 240.0, 640, 480);
        let norm = cam.normalize_point(320.0, 240.0);
        assert!((norm.x - 0.0).abs() < 0.01);
        assert!((norm.y - 0.0).abs() < 0.01);
    }
}
