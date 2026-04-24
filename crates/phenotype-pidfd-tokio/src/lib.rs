//! `phenotype-pidfd-tokio` — Tokio-integrated PidFd process reaper.
//!
//! Vendored from `orbstack/pidfd-rs` (MIT, © 2023 Orbital Labs, LLC).
//! Upstream commit: `ffbeaa6b4693d28a1274911fde4cba7d40b3f672`.

#[cfg(target_os = "linux")]
pub mod pidfd;

#[cfg(target_os = "linux")]
pub mod shutdown;

#[cfg(target_os = "linux")]
pub use pidfd::PidFd;

/// `true` when the crate has active Linux pidfd support; `false` on other targets.
pub const SUPPORTED: bool = cfg!(target_os = "linux");

#[cfg(test)]
mod tests {
    use super::SUPPORTED;

    #[test]
    fn supported_matches_target() {
        #[cfg(target_os = "linux")]
        assert!(SUPPORTED);
        #[cfg(not(target_os = "linux"))]
        assert!(!SUPPORTED);
    }
}
