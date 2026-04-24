# Attribution

This crate is a vendored copy of [`orbstack/pidfd-rs`](https://github.com/orbstack/pidfd-rs).

- **Upstream repository**: https://github.com/orbstack/pidfd-rs
- **Upstream commit**: `ffbeaa6b4693d28a1274911fde4cba7d40b3f672`
- **Upstream license**: MIT — full text in `LICENSE`
- **Original authors**: Orbital Labs, LLC (`license@orbstack.dev`)

## Changes from upstream

The upstream repository consists of two loose `.rs` files (`pidfd.rs`, `shutdown.rs`) with no `Cargo.toml`. To vendor them as a self-contained crate we:

1. Added a `Cargo.toml` manifest targeting the Phenotype workspace.
2. Moved files into `src/pidfd.rs` and `src/shutdown.rs` with a top-level `src/lib.rs` that re-exports them behind `#[cfg(target_os = "linux")]`.
3. Filled in missing `use` imports in `shutdown.rs` (the upstream snippet referenced `DirEntry`, `io`, `Error`, `kill`, `Pid`, `Duration`, etc. without importing them).
4. Inlined an `is_process_kthread` helper that the upstream file referenced but did not define.
5. Replaced the upstream `InitError::PollPidFd(err)` reference (defined in a different upstream binary) with a local `BroadcastError` enum.
6. Replaced `println!` error logging with `eprintln!` (errors belong on stderr).

All behavior is preserved. The MIT notice is reproduced verbatim in every source file and in `LICENSE`.

## Why vendor instead of fork?

Per organization policy, this is a vendor-absorb — not a maintained fork. The upstream is small, stable, and unlikely to evolve. We track the SHA we pulled from; future syncs are manual.
