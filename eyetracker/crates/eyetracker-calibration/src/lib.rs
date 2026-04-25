use eyetracker_domain::{CalibrationPoint, EyetrackerError, Point2D, Result};
use nalgebra::{Matrix3, Vector3};

/// 9-point calibration model using least-squares polynomial fitting
pub struct NinePointCalibration {
    coefficients: Option<CalibrationModel>,
}

#[derive(Clone, Debug)]
struct CalibrationModel {
    // 2D affine + quadratic coefficients for gaze->screen mapping
    a0: f32, a1: f32, a2: f32, a3: f32,  // x = a0 + a1*gx + a2*gy + a3*gx^2
    b0: f32, b1: f32, b2: f32, b3: f32,  // y = b0 + b1*gx + b2*gy + b3*gy^2
}

impl NinePointCalibration {
    pub fn new() -> Self {
        Self { coefficients: None }
    }

    /// Calibrate from 9 points with their gaze samples
    pub fn calibrate(&mut self, points: &[CalibrationPoint]) -> Result<()> {
        if points.len() < 9 {
            return Err(EyetrackerError::CalibrationFailed(
                format!("need 9 points, got {}", points.len()),
            ));
        }

        // Compute centroid of gaze samples for each point
        let mut samples_x = vec![];
        let mut samples_y = vec![];

        for point in &points[..9] {
            if point.gaze_samples.is_empty() {
                return Err(EyetrackerError::CalibrationFailed(
                    "calibration point has no gaze samples".to_string(),
                ));
            }

            let mean_gaze = Point2D::new(
                point.gaze_samples.iter().map(|p| p.x).sum::<f32>() / point.gaze_samples.len() as f32,
                point.gaze_samples.iter().map(|p| p.y).sum::<f32>() / point.gaze_samples.len() as f32,
            );

            samples_x.push((mean_gaze.x, mean_gaze.y, point.screen_position.x));
            samples_y.push((mean_gaze.x, mean_gaze.y, point.screen_position.y));
        }

        // Solve least-squares for x and y coefficients
        let (a0, a1, a2, a3) = self.fit_polynomial(&samples_x)?;
        let (b0, b1, b2, b3) = self.fit_polynomial(&samples_y)?;

        self.coefficients = Some(CalibrationModel { a0, a1, a2, a3, b0, b1, b2, b3 });
        Ok(())
    }

    fn fit_polynomial(&self, samples: &[(f32, f32, f32)]) -> Result<(f32, f32, f32, f32)> {
        // Simple 4-parameter fit using normal equations (linear least squares)
        let mut sum_gx = 0.0;
        let mut sum_gy = 0.0;
        let mut sum_gx2 = 0.0;
        let mut sum_gy2 = 0.0;
        let mut sum_screen = 0.0;
        let mut sum_gx_screen = 0.0;
        let mut sum_gy_screen = 0.0;
        let mut sum_gx2_screen = 0.0;

        for (gx, gy, screen) in samples {
            sum_gx += gx;
            sum_gy += gy;
            sum_gx2 += gx * gx;
            sum_gy2 += gy * gy;
            sum_screen += screen;
            sum_gx_screen += gx * screen;
            sum_gy_screen += gy * screen;
            sum_gx2_screen += gx * gx * screen;
        }

        let n = samples.len() as f32;
        // Linear fit: screen = a0 + a1*gx + a2*gy + a3*gx^2
        // Using simplified normal equations
        let a0 = (sum_screen - sum_gx * sum_gx_screen / sum_gx2.max(0.001) - sum_gy * sum_gy_screen / sum_gy2.max(0.001)) / n;
        let a1 = sum_gx_screen / sum_gx2.max(0.001);
        let a2 = sum_gy_screen / sum_gy2.max(0.001);
        let a3 = sum_gx2_screen / (sum_gx2 * sum_gx2).max(0.001);

        Ok((a0, a1, a2, a3))
    }

    /// Map gaze coordinates to screen coordinates
    pub fn map_to_screen(&self, gaze: &Point2D) -> Result<Point2D> {
        let coeff = self.coefficients.as_ref()
            .ok_or_else(|| EyetrackerError::CalibrationFailed("not calibrated".to_string()))?;

        let screen_x = coeff.a0 + coeff.a1 * gaze.x + coeff.a2 * gaze.y + coeff.a3 * gaze.x * gaze.x;
        let screen_y = coeff.b0 + coeff.b1 * gaze.x + coeff.b2 * gaze.y + coeff.b3 * gaze.y * gaze.y;

        Ok(Point2D::new(screen_x, screen_y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_insufficient_points() {
        let mut cal = NinePointCalibration::new();
        let err = cal.calibrate(&[]);
        assert!(err.is_err());
    }

    #[test]
    fn test_calibration_with_9_points() {
        let mut cal = NinePointCalibration::new();
        let mut points = vec![];
        for i in 0..9 {
            let screen_x = ((i % 3) as f32) * 320.0;
            let screen_y = ((i / 3) as f32) * 240.0;
            points.push(CalibrationPoint {
                screen_position: Point2D::new(screen_x, screen_y),
                gaze_samples: vec![
                    Point2D::new(screen_x / 1000.0, screen_y / 1000.0),
                    Point2D::new(screen_x / 1000.0 + 0.01, screen_y / 1000.0),
                ],
            });
        }
        assert!(cal.calibrate(&points).is_ok());
    }
}
