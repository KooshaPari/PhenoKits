# FFI Hardening Checklist

Cross-cutting robustness policy for native code surfaces exposed via foreign-function interfaces (FFI) in the Phenotype organization.

## Purpose

FFI boundaries are high-risk: panics in FFI-exported code crash native hosts (Swift/Kotlin/Python/Go), and memory safety errors propagate across language boundaries. This checklist ensures consistent hardening across UniFFI, JNI, PyO3, and cgo surfaces.

## Surfaces in Scope

1. **focus-ffi (UniFFI)** — Swift/Kotlin bindings from Rust core
2. **Android JNI** — Direct Kotlin JNI calls from FocalPoint Android app
3. **heliosCLI/harness_pyo3** — Python bindings via PyO3 (arm64 dynamic_lookup)
4. **KDesktopVirt automation_engine FFI** — OS automation (Quartz/Win32/X11)
5. **agentapi-plusplus (cgo)** — Go ↔ Rust interop if present
6. **eye-tracker-sdk FFI** — Platform-specific eye-tracking APIs

## Hardening Checklist (Per Surface)

### 1. Panic Guard Coverage

**Rule:** No panic in FFI-exported functions. All fallible operations must convert to explicit errors.

**Audit:**
- Count `panic!` statements and `unreachable!` in FFI entry points
- Count `.unwrap()` and `.expect(...)` on real operations (exclude test-only code)
- Identify mutex/rwlock poison panics (`lock().expect("...")`)

**Fixes:**
- Wrap top-level FFI functions in `std::panic::catch_unwind(AssertUnwindSafe(|| {...}))`
- Replace `.unwrap()` → `.map_err(|e| FfiError::...)?`
- Replace `.expect("msg")` → `?` with proper Result propagation
- Use `Result<T, FfiError>` for all FFI-exported functions
- Add `UnwindSafe` bound on generics crossing FFI boundary if present

### 2. Drop Semantics for FFI-Owned Types

**Rule:** Explicit `Drop` impl on opaque pointers and shared references.

**Audit:**
- Search for opaque `struct Opaque<T>` used in FFI bindings
- Check for Arc/Rc wrapping without explicit cleanup
- Verify `Drop` trait is present on any handle/resource type

**Fixes:**
- Add `impl Drop for OpaqueHandle { fn drop(&mut self) { /* cleanup */ } }`
- Document reference-count semantics in docstrings
- Mark pointers `#[repr(C)]` if crossing language boundary
- Add validation in Drop to ensure resource is in safe state

### 3. Input Validation

**Rule:** All FFI inputs validated before use.

**Audit:**
- Check for length checks on slice inputs
- Verify null/empty collection handling
- Look for unchecked index access on FFI-provided data

**Fixes:**
- Add range checks before raw slice access
- Validate non-empty preconditions upfront
- Add explicit error type for validation failures
- Document expected input constraints in docstrings

### 4. Async Runtime Ownership

**Rule:** Tokio runtime owned by Rust; no runtime creation in FFI if avoidable.

**Audit:**
- Count `Runtime::new()` calls in FFI code
- Check for multi-threaded vs single-threaded runtimes
- Verify no runtime duplication across FFI calls

**Fixes:**
- Create one persistent runtime per language bridge (via lazy_static or once_cell)
- Convert `Runtime::new().expect(...)` → single-instance Arc<Runtime>
- Document runtime lifecycle in FFI comments
- Test that runtime is not created multiple times

### 5. Error Type Coverage

**Rule:** Custom `FfiError` type with explicit variant for each failure mode.

**Audit:**
- Check for generic `anyhow::Result` in FFI signatures
- Count distinct error cases (poison, timeout, validation, io, etc.)
- Verify `#[from]` derives cover all relevant error types

**Fixes:**
- Create custom `#[repr(C)] enum FfiError { Poisoned, Timeout, Validation, Io, Unknown }`
- Implement `std::fmt::Display` for FFI errors
- Add `#[from]` impls for conversion from std/thiserror types
- Map all `.expect(msg)` failures to explicit error variants

### 6. GIL (PyO3 Only)

**Rule:** GIL acquired and released correctly for async operations.

**Audit:**
- Check for `.allow_threads()` wrapping on blocking calls
- Verify no GIL held across `await` points
- Look for `send` violations in async PyO3 code

**Fixes:**
- Wrap blocking FFI calls in `py.allow_threads(|| {...})`
- Extract async-safe portions outside PyO3 boundary
- Add `#[pyo3(signature = (...))]` with explicit argument types
- Test GIL reacquisition after async boundary

## Recurring Execution

**Frequency:** Per FFI surface addition or quarterly on main surfaces (focus-ffi, pyo3, jni).

**Automation:** Integrate into CI via:
- Clippy lint for `unwrap_used` + `expect_used` in crates marked `ffi = true`
- `cargo clippy --all-targets -- -D clippy::unwrap_used` in pre-push hook
- Audit script: `find . -name "*.rs" -path "*/ffi/*" -exec grep -l "unsafe\|unwrap\|expect\|panic" {} \;`

## Exemptions

Exemptions require ADR:
- `.unwrap()` in test-only code (`#[cfg(test)]` modules)
- `.expect(...)` on infallible operations (with explicit comment: `// infallible: <reason>`)
- Mutex poison recovery (document panic guarantee in type's Drop)

## References

- [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) — safety patterns
- [UniFFI Safety](https://mozilla.github.io/uniffi-rs/) — bindings generation
- [PyO3 Concurrency](https://pyo3.rs/latest/concurrency.html) — GIL semantics
- [Android JNI Best Practices](https://developer.android.com/training/articles/perf-jni) — exception handling
