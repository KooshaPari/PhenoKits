//! Cross-platform smoke test: ensures the crate compiles and the `SUPPORTED`
//! constant is consistent with the build target. Linux-specific runtime
//! behavior is covered by the `#[cfg(target_os = "linux")]` tests below, which
//! spawn a child via `tokio::process::Command`, open a pidfd for it, kill it,
//! and await the exit via epoll.

use phenotype_pidfd_tokio::SUPPORTED;

#[test]
fn supported_matches_cfg() {
    assert_eq!(SUPPORTED, cfg!(target_os = "linux"));
}

#[cfg(target_os = "linux")]
mod linux {
    use phenotype_pidfd_tokio::PidFd;
    use std::time::Duration;

    #[tokio::test]
    async fn pidfd_detects_child_exit() {
        // Spawn a child that sleeps for a while so we have time to open the pidfd.
        let mut child = tokio::process::Command::new("sleep")
            .arg("30")
            .spawn()
            .expect("failed to spawn sleep");
        let pid = child.id().expect("no pid") as i32;

        let pidfd = PidFd::open(pid).expect("pidfd_open");

        // Kill via pidfd and wait for exit notification.
        pidfd
            .kill(nix::sys::signal::Signal::SIGTERM)
            .expect("pidfd_send_signal");

        let waited = tokio::time::timeout(Duration::from_secs(5), pidfd.wait())
            .await
            .expect("timed out waiting for child to exit");
        waited.expect("readable guard");

        // Reap.
        let _ = child.wait().await;
    }
}
