# phenotype-pidfd-tokio

Fast, polling-free approach to kill and wait for all processes to exit on Linux — integrated with Tokio via `AsyncFd` over a `pidfd`.

- Rust + Tokio
- `pidfd_open` + `pidfd_send_signal` + epoll readiness

## Status

Vendored from [`orbstack/pidfd-rs`](https://github.com/orbstack/pidfd-rs) @ `ffbeaa6b4693d28a1274911fde4cba7d40b3f672` under the MIT license. See `LICENSE` and `ATTRIBUTION.md`.

The crate compiles on all targets but only exposes `PidFd` / `broadcast_signal` / `wait_for_pidfds_exit` on `target_os = "linux"`. Use the `SUPPORTED` const to probe at runtime.

## Use cases

- Init-style orchestrators needing to `SIGTERM` every process and await exit before unmounting.
- Agent swarm supervisors (e.g. thegent-dispatch) that need precise, race-free per-child reaping without polling.

## API

```rust
#[cfg(target_os = "linux")]
use phenotype_pidfd_tokio::{PidFd, broadcast_signal, wait_for_pidfds_exit};
use nix::sys::signal::Signal;
use std::time::Duration;

#[cfg(target_os = "linux")]
async fn shutdown() -> Result<(), Box<dyn std::error::Error>> {
    let pidfds = broadcast_signal(Signal::SIGTERM)?;
    wait_for_pidfds_exit(pidfds, Duration::from_secs(10)).await?;
    Ok(())
}
```
