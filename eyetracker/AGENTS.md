# EyeTracker — Agent Instructions

## Before Starting Any Work

1. **Check AgilePlus:** Is there a spec for this task?
   ```bash
   cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
   agileplus search --query "eyetracker" | head -20
   ```

2. **Use Worktree:** All feature work in `repos/eyetracker-wtrees/<topic>/`
   ```bash
   git worktree add repos/eyetracker-wtrees/feature-name
   cd repos/eyetracker-wtrees/feature-name
   ```

3. **Return to Main:** After completing work, return canonical repo to `main`

## Delegation Patterns

| Need | Delegate To | Example |
|------|-------------|---------|
| Explore crate structure | `explore` | "Map cross-crate dependencies; find circular refs" |
| Design FFI binding approach | `plan` | "Design Swift type bridge for GazePoint" |
| Implement & test crate | `general-purpose` | "Implement eyetracker-calibration with 5 tests" |
| Audit code quality | `audit` | "Check all tests for FR traceability" |
| Build/test on device | `local` | "Run `cargo test --workspace` and report failures" |

## Parallel vs Sequential

**Parallel (independent):**
- Explore 3 separate crates simultaneously
- Run unit tests for each crate in parallel (cargo handles this)

**Sequential (dependent):**
- Explore workspace → design FFI → implement FFI → build native app
- Can't test iOS app without FFI dylib

## Code Review Checklist

Before approving a PR:

- [ ] All new tests trace to FUNCTIONAL_REQUIREMENTS.md FRs
- [ ] `cargo clippy --workspace` passes
- [ ] `cargo fmt` compliant
- [ ] No inter-crate circular dependencies
- [ ] FFI types are Copy + Clone (or Serialize)
- [ ] Error handling uses `thiserror`
- [ ] No `unwrap()` in production code (use `?` or explicit error handling)
- [ ] Documentation: FRs updated if behavior changed

## Known Issues & Workarounds

### UniFFI Swift Binding Generation

UniFFI can be finicky with build paths. If bindings don't generate:

```bash
cd crates/eyetracker-ffi
rm -rf target/
cargo build --lib
cargo build --features=uniffi/cli  # Force regeneration
```

### Android JNI Linking

If JNI symbols not found at runtime:

1. Ensure Rust target is `aarch64-linux-android` (not `x86_64`)
2. Verify `$ANDROID_NDK_HOME` environment variable set
3. Link with `-llog` for Android logging

### macOS Vision Face Detection

Vision.framework requires at least macOS 10.13. If `VNDetectFaceLandmarksRequest` not available, verify deployment target.

## Performance Profiling

### iOS ARKit Profiling

```bash
# Xcode: Product > Profile > System Trace
# Look for ARKit.framework latency, face anchor update frequency
```

### Android ML Kit Profiling

```bash
# Android Studio: Profiler > CPU
# Monitor MLKit FaceMesh inference time (typically 30-50ms)
```

### macOS Vision Profiling

```bash
# Instruments: System Trace > Vision
# Check VNDetectFaceLandmarksRequest + CVPixelBuffer processing
```

## Extending the System

### Adding a New Crate

1. Create `crates/eyetracker-<name>/`
2. Add to `Cargo.toml` members
3. Use `workspace = true` for shared deps
4. Add inline tests (`#[cfg(test)] mod tests`)
5. Update README.md crate table
6. Ensure all tests trace to FUNCTIONAL_REQUIREMENTS.md

### Adding a New Native App

1. Create `apps/<platform>/` directory
2. Implement platform-specific camera + face detection
3. Create FFI wrapper (Swift/Kotlin) to call Rust estimator
4. Add integration test or manual test plan
5. Document in README.md architecture section

### Adding an Accessibility Feature

See `FUNCTIONAL_REQUIREMENTS.md` FR-ANDROID-002. Steps:

1. Implement `AccessibilityDelegate` in Android app
2. Emit `ACTION_ACCESSIBILITY_FOCUS` from gaze detector
3. Hook into system focus traversal
4. Test with TalkBack enabled

## Debugging Gaze Estimation

If gaze estimates are wildly off:

1. **Verify calibration:** Run 9-point calibration with >50 samples/point
2. **Check camera intrinsics:** Ensure focal length matches actual camera (usually 480-550 for mobile)
3. **Inspect Kalman state:** Log `KalmanFilter2D.x_state` during tracking
4. **Plot raw vs smoothed:** Export gaze history via `GazeSessionBuffer`; plot in Python/matplotlib
5. **Test on synthetic data:** Use `NinePointCalibration` with known transformations

## Commit Message Guidelines

Prefix with scope + verb:

```
feat(calibration): improve polynomial fitting with regularization
fix(ffi): correct Swift Point2D marshalling
docs(fr): update latency requirements to <30ms
refactor(math): extract KalmanFilter2D into util module
test(estimator): add accuracy benchmark for 9-point calibration
```

## Integration with FocalPoint

EyeTracker can complement FocalPoint's attention measurement:

1. Export `GazePoint` stream via local IPC (Unix socket or shared mem)
2. FocalPoint subscribes and correlates with DOM nodes / view hierarchies
3. Produce attention heatmaps on UI elements
4. Feed into focus analytics dashboard

See README.md "Integration with FocalPoint" section.
