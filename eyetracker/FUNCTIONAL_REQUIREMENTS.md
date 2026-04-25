# Eye Tracker — Functional Requirements

## Overview
Real-time, on-device gaze tracking for iOS, Android, and macOS with 9-point calibration, sub-1.5° accuracy, and <30ms latency. Privacy-first architecture with no server transmission.

## Core Requirements

### FR-ESTIMATOR-001: Gaze Point Estimation
- **Requirement:** Estimate (x, y) screen coordinates from face landmarks with ≥0.8 confidence
- **Acceptance Criteria:**
  - Input: FaceLandmarks (left_eye, right_eye, nose, cheeks)
  - Output: GazePoint with (x, y, confidence, timestamp)
  - Latency: ≤15ms per estimation
  - Coordinate space: screen pixels (0..width, 0..height)
- **Test:** `test_gaze_estimator_accuracy_within_threshold`

### FR-ESTIMATOR-002: Fixation Detection
- **Requirement:** Detect fixations (sustained gaze ≥100ms) and saccades (rapid eye movements)
- **Acceptance Criteria:**
  - Velocity threshold: >50 px/s = saccade
  - Dwell threshold: ≥100ms without saccade = fixation
  - Output: Fixation { center, duration_ms, start_time_ms }
- **Test:** `test_fixation_detector_clustering`

### FR-CALIBRATION-001: 9-Point Calibration
- **Requirement:** Calibrate gaze→screen mapping using 9 screen points
- **Acceptance Criteria:**
  - Collect ≥10 gaze samples per calibration point
  - Fit polynomial model (a0 + a1*gx + a2*gy + a3*gx²)
  - Achieve ≤1.5° error (≤26 px @ 24" @ 1920x1080)
- **Test:** `test_nine_point_calibration_polynomial_fit`

### FR-CALIBRATION-002: Dynamic Recalibration
- **Requirement:** Re-calibrate if accuracy drift detected (>2° per minute)
- **Acceptance Criteria:**
  - Monitor fixation accuracy against known targets
  - Trigger auto-recalibration if drift > threshold
  - Preserve calibration across app restart (JSON serialization)
- **Test:** `test_recalibration_trigger_on_drift`

### FR-MATH-001: 2D Kalman Smoothing
- **Requirement:** Smooth gaze samples using Kalman filter to reduce jitter
- **Acceptance Criteria:**
  - State: position + velocity
  - Process noise: 0.001, measurement noise: 2.0
  - Output: smoothed Point2D with ≤5ms latency
- **Test:** `test_kalman_filter_convergence`

### FR-MATH-002: Camera Intrinsics
- **Requirement:** Convert camera pixel coords → normalized device coords using focal length + principal point
- **Acceptance Criteria:**
  - Input: (camera_x, camera_y), focal_length_x/y, principal_point_x/y
  - Output: normalized Point2D
  - Support variable camera resolutions (320-1920px)
- **Test:** `test_camera_intrinsics_normalization`

### FR-STORAGE-001: Session Buffer
- **Requirement:** Store gaze history in ring buffer (RAM, no persistence)
- **Acceptance Criteria:**
  - Max 10,000 gaze points; auto-evict oldest
  - Separate queues: gaze_points, fixations, saccades
  - JSON serialization for temp export
- **Test:** `test_buffer_ring_eviction`

### FR-FFI-001: UniFFI Bindings
- **Requirement:** Expose Rust APIs to Swift (iOS/macOS) and JNI (Android)
- **Acceptance Criteria:**
  - GazeEstimator interface: new(), estimate_gaze()
  - FixationDetector interface: new(), process()
  - All domain types cross-language (Point2D, GazePoint, FaceLandmarks)
  - Zero-copy or minimal marshalling
- **Test:** `test_ffi_swift_binding_instantiation`

### FR-iOS-001: ARKit Integration
- **Requirement:** Capture face landmarks via ARKit ARFaceAnchor
- **Acceptance Criteria:**
  - Real-time face tracking at 60 Hz
  - Extract left_eye, right_eye, lookAtPoint transforms
  - Feed to Rust estimator via FFI
  - Display gaze cursor overlay
- **Test:** Manual AR session test (e.g., front-facing camera availability)

### FR-iOS-002: Gesture Support
- **Requirement:** Enable dwell-to-select and gaze-scroll gestures
- **Acceptance Criteria:**
  - Detect 500ms+ fixation → trigger action
  - Vertical velocity >100 px/s → scroll direction
  - Integration with native UIGestureRecognizer or Combine
- **Test:** `test_gaze_gesture_dwell_timing`

### FR-ANDROID-001: CameraX + MLKit FaceMesh
- **Requirement:** Capture video frames and detect face landmarks via ML Kit
- **Acceptance Criteria:**
  - Real-time face detection at 30 Hz
  - Extract face mesh points and eye centers
  - JNI call to Rust estimator
  - Render gaze cursor on Compose Canvas
- **Test:** Integration test (device with ML Kit support)

### FR-ANDROID-002: Accessibility Integration
- **Requirement:** Hook into AccessibilityService to enable gaze-based navigation
- **Acceptance Criteria:**
  - Expose gaze point as accessibility event (custom action)
  - Compatible with system focus traversal
  - Optional: scroll-by-gaze in accessible scrollable views
- **Test:** Manual accessibility testing on device

### FR-MACOS-001: AVFoundation Webcam
- **Requirement:** Capture webcam frames and detect face landmarks via Vision
- **Acceptance Criteria:**
  - Real-time face detection at 30 Hz (CPU-bound)
  - Vision.framework VNDetectFaceLandmarksRequest
  - Display gaze dot on screen
- **Test:** `test_macos_vision_face_detection`

### FR-PRIVACY-001: On-Device Processing
- **Requirement:** All gaze processing occurs locally; no network transmission
- **Acceptance Criteria:**
  - No cloud APIs called
  - Face landmarks never leave device
  - Optional: hash-based telemetry (gaze-point counts, not coords)
- **Test:** Network traffic inspection (manual or via mitmproxy)

### FR-ACCURACY-001: Accuracy Target ≤1.5°
- **Requirement:** Achieve calibration accuracy within 1.5° visual angle
- **Acceptance Criteria:**
  - Test on 3+ subjects, 20 gaze targets
  - Measure: arc-tan(pixel_error / distance_to_screen)
  - Goal: 26 px error @ 24" @ 1920x1080
- **Test:** Calibration accuracy benchmark (manual)

### FR-LATENCY-001: Latency <30ms
- **Requirement:** End-to-end gaze latency (frame capture → estimate) < 30ms
- **Acceptance Criteria:**
  - Measure: timestamp(capture) → timestamp(estimate available)
  - P95 latency < 40ms (allows some jitter)
  - Profile on iOS/Android/macOS devices
- **Test:** `benchmark_e2e_latency`

## Testing Strategy

- **Unit Tests:** Domain types, math, calibration (no mocks needed; deterministic)
- **Integration Tests:** FFI bindings, AR/CameraX capture, Vision/MLKit detection
- **System Tests:** Full pipeline on device (iOS/Android/macOS)
- **Benchmarks:** Latency, accuracy (calibration), memory (buffer ring)

## Priority Order

1. **Phase 1 (MVP):** FR-ESTIMATOR-001, FR-CALIBRATION-001, FR-MATH-001/002, FR-FFI-001
2. **Phase 2 (Native):** FR-iOS-001, FR-ANDROID-001, FR-MACOS-001
3. **Phase 3 (UX):** FR-ESTIMATOR-002, FR-iOS-002, FR-ANDROID-002
4. **Phase 4 (Polish):** FR-CALIBRATION-002, FR-PRIVACY-001, FR-ACCURACY-001, FR-LATENCY-001

## Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Accuracy | ≤1.5° | Calibration test (manual) |
| Latency | <30ms | Timestamp instrumentation |
| Fixation Recall | ≥95% | Synthetic gaze traces |
| Memory (buffer) | <50 MB | Profiler at max capacity |
| CPU (per-frame) | <10% | Activity Monitor / CPU trace |
| Privacy | 100% | Network inspection |
