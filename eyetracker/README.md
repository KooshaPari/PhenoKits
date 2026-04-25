# EyeTracker MVP

Real-time, on-device gaze tracking for iOS, Android, and macOS. Cross-platform Rust core with native app integration via UniFFI.

## Architecture

```
┌─ Rust Core (FFI-hardened)
│  ├─ eyetracker-domain: Types (Point2D, GazePoint, Fixation, FaceLandmarks)
│  ├─ eyetracker-math: Kalman filtering, camera intrinsics, coordinate transforms
│  ├─ eyetracker-calibration: 9-point polynomial calibration (≤1.5° accuracy)
│  ├─ eyetracker-estimator: GazeEstimator pipeline, FixationDetector
│  ├─ eyetracker-storage: Session ring buffer (RAM, auto-evict)
│  └─ eyetracker-ffi: UniFFI bindings + JNI stubs
│
├─ iOS App (ARKit + Vision)
│  ├─ EyetrackerView: SwiftUI AR view controller
│  ├─ ARFaceAnchor → landmarks feed to Rust via FFI
│  └─ Display gaze cursor + fixation feedback
│
├─ Android App (CameraX + ML Kit)
│  ├─ EyetrackerScreen: Compose UI with camera preview
│  ├─ MLKit FaceMesh → landmarks via JNI to Rust
│  └─ Accessibility GazeService scaffold
│
└─ macOS App (AVFoundation + Vision)
   ├─ CameraPreview with Vision face detection
   ├─ Feed landmarks to Rust via FFI
   └─ Display gaze dot overlay
```

## Use Cases

1. **Accessibility:** Hands-free navigation for motor impairment (iOS/Android/macOS)
2. **Focus Measurement:** Integrate with FocalPoint to measure attention on UI elements
3. **Attention Analytics:** Track fixations/saccades for user research
4. **Eye-Gaze Gaming:** Dwell-to-select, gaze-scroll in games/apps

## Quick Start

### Build Rust Core

```bash
cd /repos/eyetracker
cargo test --workspace
cargo build --release
```

### Build iOS (SPM)

```bash
cd apps/ios
# Build the FFI dylib first
cargo build --target aarch64-apple-ios --release
# Or use Xcode with SwiftPM Package.swift
xcodebuild -scheme EyetrackerSwift -destination 'generic/platform=iOS'
```

### Build Android

```bash
cd apps/android
gradle assembleDebug
```

### Build macOS

```bash
cd apps/macos
swiftc -parse-as-library EyetrackerApp.swift -o EyetrackerApp
```

## Key Algorithms

### 9-Point Calibration (FR-CALIBRATION-001)
Least-squares polynomial fit to map gaze coordinates → screen coordinates:
```
screen_x = a0 + a1*gaze_x + a2*gaze_y + a3*gaze_x²
screen_y = b0 + b1*gaze_x + b2*gaze_y + b3*gaze_y²
```
Target: ≤1.5° accuracy (~26 px @ 1920×1080, 24" away).

### 2D Kalman Smoothing (FR-MATH-001)
Continuous state (position + velocity) with:
- Process noise σ²: 0.001
- Measurement noise σ²: 2.0
- Produces smooth 60 Hz gaze stream with <5ms latency.

### Fixation Detection (FR-ESTIMATOR-002)
- Cluster gaze points with velocity <50 px/s
- Emit Fixation event after ≥100ms dwell
- Enable dwell-to-select gestures.

## Requirements

| Component | iOS | Android | macOS |
|-----------|-----|---------|-------|
| **Min OS** | 16.0 | 24 | 12.0 |
| **Runtime** | ARKit + Vision | CameraX + MLKit | AVFoundation + Vision |
| **Accuracy** | ≤1.5° (calibrated) | ≤1.5° (calibrated) | ≤2.0° (webcam) |
| **Latency** | <30ms | <30ms | <50ms (CPU-bound) |
| **Privacy** | On-device | On-device | On-device |

## Testing

Run unit tests:
```bash
cargo test --workspace --lib
```

Test coverage by FR (see `FUNCTIONAL_REQUIREMENTS.md` for test mappings):
```bash
# Domain & math tests (deterministic)
cargo test -p eyetracker-domain
cargo test -p eyetracker-math

# Calibration tests
cargo test -p eyetracker-calibration

# FFI tests (requires bindings)
cargo test -p eyetracker-ffi
```

## Integration with FocalPoint

EyeTracker can feed gaze data into FocalPoint's attention measurement system:

1. **Gaze Stream API:** Expose `GazePoint` stream via local socket or shared buffer
2. **UI Element Mapping:** Annotate screen coords with DOM node IDs (web) or view hierarchies (mobile)
3. **Attention Metric:** Compute dwell time per element → attention score

Example: measure gaze attention on FocalPoint dashboard UI.

## Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Calibration Accuracy | ≤1.5° | 26 px error @ 1920×1080, 24" distance |
| Per-Frame Latency | <15ms | Estimate only; capture+process ≤30ms e2e |
| Fixation Detection | ≥95% recall | Dwell-to-select responsiveness |
| Memory (10k buffer) | <50 MB | Ring buffer auto-eviction |
| CPU (per-frame) | <10% | Single core, mobile device baseline |

## Privacy & Security

- **On-Device:** All processing local; no cloud transmission
- **No Persistence:** Gaze history only in RAM (ring buffer auto-clears)
- **Optional Telemetry:** Hash-based counts (not coordinates)

## Roadmap

- [ ] Phase 1: Rust core + basic calibration (current)
- [ ] Phase 2: iOS ARKit app, Android CameraX app, macOS Vision app
- [ ] Phase 3: Fixation detector, dwell-to-select gestures
- [ ] Phase 4: Accuracy tuning, accessibility integration, FocalPoint bridge
- [ ] Phase 5: Cloud-optional analytics, multi-user calibration sharing

## License

MIT OR Apache-2.0
