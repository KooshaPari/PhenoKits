# EyeTracker — Claude Instructions

## Project Overview

- **Name:** eyetracker
- **Type:** Real-time, on-device gaze tracking (Rust core + native iOS/Android/macOS apps)
- **Stack:** Rust (core), Swift (iOS/macOS), Kotlin (Android)
- **Language:** Rust (edition 2021), Swift 5.9+, Kotlin 1.9+

## Architecture

Follows FocalPoint pattern:

- **Rust workspace** (`crates/`): 6 independent, domain-agnostic crates
  - `eyetracker-domain`: Type definitions (Point2D, GazePoint, FaceLandmarks, etc.)
  - `eyetracker-math`: Kalman filter, camera intrinsics, coordinate transforms
  - `eyetracker-calibration`: 9-point polynomial calibration
  - `eyetracker-estimator`: Main gaze pipeline + fixation detector
  - `eyetracker-storage`: Session ring buffer
  - `eyetracker-ffi`: UniFFI bindings + FFI-safe wrappers

- **Native apps** (`apps/`):
  - `ios/`: SwiftUI + ARKit face tracking
  - `android/`: Jetpack Compose + CameraX + MLKit face detection
  - `macos/`: SwiftUI + AVFoundation + Vision face detection

## Work Requirements

1. **AgilePlus Spec:** Create spec before implementation
   ```bash
   cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
   agileplus specify --title "Feature Name" --description "..."
   ```

2. **Branch Discipline:**
   - Feature branches: `repos/eyetracker-wtrees/<category>/<branch>`
   - Return to `main` for integration checkpoints
   - Use canonical repo only for final merges

3. **Quality Gates:**
   - `cargo test --workspace`
   - `cargo clippy --workspace -- -D warnings`
   - `cargo fmt --check`
   - All tests trace to `FUNCTIONAL_REQUIREMENTS.md` FRs

## Test Traceability (REQUIRED)

Every test MUST reference an FR:

```rust
// Traces to: FR-ESTIMATOR-001
#[test]
fn test_gaze_estimator_accuracy() {
    // Test body
}
```

Verify coverage:
```bash
# All FRs in FUNCTIONAL_REQUIREMENTS.md must have >=1 test
# All tests must have >=1 FR reference
grep "Traces to:" crates/*/src/lib.rs | wc -l
```

## Cross-Project Reuse

Shared infrastructure candidates for extraction to `phenotype-shared`:

- `eyetracker-domain`: Generic 2D geometry types (Point2D, shared with other projects)
- `eyetracker-math`: Kalman filter (reusable in motion capture, sensor fusion)
- `eyetracker-calibration`: Polynomial fitting (general optimization utility)

When extracting, update both this repo and consuming repos in a single PR.

## CI/CD

- No GitHub Actions CI running (billing constraint).
- Run locally before push:
  ```bash
  cargo test --workspace
  cargo clippy --workspace
  cargo fmt
  ```
- FFI bindings validated via unit tests (no runtime native app test in CI).

## Dependency Strategy

Use workspace-level `Cargo.toml` for all crate versions:
- `serde`, `serde_json`: serialization
- `nalgebra`: linear algebra (Kalman, matrices)
- `thiserror`: error handling
- `uniffi`: FFI bindings
- No inter-crate cyclic dependencies; each crate stands alone

## FFI & Native Bindings

### Swift (iOS/macOS)
- UniFFI generates `.framework` with Swift extensions
- Wrapped in `Package.swift` for SPM distribution
- Example:
  ```swift
  let estimator = GazeEstimatorFFI(camera: cam, screen: screen)
  let gaze = estimator.estimate_gaze(landmarks: faceLandmarks, timestamp_ms: now)
  ```

### Kotlin (Android)
- UniFFI generates JNI bindings
- Wrapped in Android library module
- Example:
  ```kotlin
  val estimator = GazeEstimatorFFI(camera, screen)
  val gaze = estimator.estimateGaze(landmarks, System.currentTimeMillis())
  ```

## Documentation

- **README.md:** Architecture, use cases, quick start, performance targets
- **FUNCTIONAL_REQUIREMENTS.md:** 15+ FRs with acceptance criteria + test mappings
- **CLAUDE.md:** This file
- Update as new FRs are added or architecture changes

## Disk Budget & Multi-Session

See `Phenotype/repos/docs/governance/disk_budget_policy.md` and `multi_session_coordination.md`.

- Rust `target/` builds can consume 10+ GB per architecture (iOS/Android/macOS)
- Use `target-pruner` if disk pressure detected
- Coordinate with other agents if parallel builds running

## Debugging Tips

- **Kalman filter tuning:** Adjust `q_process_noise` and `r_meas_noise` in `KalmanFilter2D::new()`
- **Calibration accuracy:** Increase sample count per point (>50) for >9 points
- **FFI marshalling:** Check `eyetracker-ffi/src/lib.rs` for type conversions
- **iOS ARKit:** Ensure `NSCameraUsageDescription` in Info.plist
- **Android MLKit:** Need ML Kit models in project; auto-downloaded on first run

## Scripting Language Hierarchy

All scripts/tools follow Phenotype org policy:

1. **Rust** (default) — use for build helpers, CLI tools
2. **Zig/Go/Mojo** — acceptable with one-line justification
3. **Python/TS** — only if embedded in Python/TS runtime
4. **Bash/sh** — only as ≤5-line glue with inline justification

See `repos/docs/governance/scripting_policy.md` for full policy.

## Worklogs

Document research, architecture decisions, and findings:
- Location: `/Users/kooshapari/CodeProjects/Phenotype/repos/worklogs/`
- Index: `worklogs/README.md`
- Categories: ARCHITECTURE, DUPLICATION, DEPENDENCIES, RESEARCH, GOVERNANCE

Example:
```
- [eyetracker] Kalman filter parameterization: q=0.001, r=2.0 chosen via grid search on 100-point synthetic trace; 95% of error <5px after 20 samples
```
