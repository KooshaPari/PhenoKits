use eyetracker_domain::{Fixation, GazePoint, Saccade};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// In-memory ring buffer storage for gaze session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GazeSessionBuffer {
    pub gaze_points: VecDeque<GazePoint>,
    pub fixations: VecDeque<Fixation>,
    pub saccades: VecDeque<Saccade>,
    max_size: usize,
}

impl GazeSessionBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            gaze_points: VecDeque::with_capacity(max_size),
            fixations: VecDeque::with_capacity(max_size / 10),
            saccades: VecDeque::with_capacity(max_size / 20),
            max_size,
        }
    }

    /// Add gaze point, evicting oldest if at capacity
    /// Traces to: FR-STORAGE-001
    pub fn add_gaze_point(&mut self, gaze: GazePoint) {
        if self.gaze_points.len() >= self.max_size {
            self.gaze_points.pop_front();
        }
        self.gaze_points.push_back(gaze);
    }

    pub fn add_fixation(&mut self, fixation: Fixation) {
        if self.fixations.len() >= self.max_size / 10 {
            self.fixations.pop_front();
        }
        self.fixations.push_back(fixation);
    }

    pub fn add_saccade(&mut self, saccade: Saccade) {
        if self.saccades.len() >= self.max_size / 20 {
            self.saccades.pop_front();
        }
        self.saccades.push_back(saccade);
    }

    pub fn clear(&mut self) {
        self.gaze_points.clear();
        self.fixations.clear();
        self.saccades.clear();
    }

    pub fn gaze_count(&self) -> usize {
        self.gaze_points.len()
    }

    pub fn fixation_count(&self) -> usize {
        self.fixations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buf = GazeSessionBuffer::new(1000);
        assert_eq!(buf.gaze_count(), 0);
    }

    #[test]
    fn test_buffer_add_gaze() {
        let mut buf = GazeSessionBuffer::new(100);
        let gaze = GazePoint::new(100.0, 200.0, 0.9, 1000);
        buf.add_gaze_point(gaze);
        assert_eq!(buf.gaze_count(), 1);
    }

    #[test]
    fn test_buffer_ring_eviction() {
        let mut buf = GazeSessionBuffer::new(10);
        for i in 0..15 {
            buf.add_gaze_point(GazePoint::new(i as f32, i as f32, 0.9, i as i64));
        }
        assert_eq!(buf.gaze_count(), 10);
    }
}
