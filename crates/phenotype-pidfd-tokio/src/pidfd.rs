/*
Vendored from orbstack/pidfd-rs @ ffbeaa6b4693d28a1274911fde4cba7d40b3f672
Original file: pidfd.rs

Copyright (c) 2023 Orbital Labs, LLC <license@orbstack.dev>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};

use nix::{
    libc::{siginfo_t, syscall, SYS_pidfd_open, SYS_pidfd_send_signal, PIDFD_NONBLOCK},
    sys::signal::Signal,
};
use tokio::io::unix::{AsyncFd, AsyncFdReadyGuard};

/// A non-blocking, Tokio-integrated pidfd.
///
/// Created via [`PidFd::open`]. Dropping the `PidFd` closes the underlying
/// file descriptor.
pub struct PidFd(AsyncFd<OwnedFd>);

impl PidFd {
    /// Open a pidfd for the given PID (via `SYS_pidfd_open` with `PIDFD_NONBLOCK`).
    pub fn open(pid: i32) -> std::io::Result<Self> {
        let fd = unsafe { syscall(SYS_pidfd_open, pid, PIDFD_NONBLOCK) };
        if fd < 0 {
            return Err(std::io::Error::last_os_error());
        }
        let fd = unsafe { OwnedFd::from_raw_fd(fd as _) };
        let fd = AsyncFd::new(fd)?;
        Ok(Self(fd))
    }

    /// Send a signal to the process via `SYS_pidfd_send_signal`.
    pub fn kill(&self, signal: Signal) -> nix::Result<()> {
        let res = unsafe {
            syscall(
                SYS_pidfd_send_signal,
                self.as_raw_fd(),
                signal,
                std::ptr::null::<*const siginfo_t>(),
                0,
            )
        };
        if res < 0 {
            return Err(nix::Error::last());
        }

        Ok(())
    }

    /// Await process exit — returns when the pidfd becomes readable, which
    /// happens exactly when the tracked process terminates.
    pub async fn wait(&self) -> tokio::io::Result<AsyncFdReadyGuard<'_, OwnedFd>> {
        self.0.readable().await
    }
}

impl AsRawFd for PidFd {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}
