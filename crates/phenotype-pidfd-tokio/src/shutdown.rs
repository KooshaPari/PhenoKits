/*
Vendored from orbstack/pidfd-rs @ ffbeaa6b4693d28a1274911fde4cba7d40b3f672
Original file: shutdown.rs

Adapted to compile as a standalone module:
  - Imports filled in (upstream snippet lacked a `use` preamble).
  - `is_process_kthread` helper inlined (reads `/proc/<pid>/status` and checks
    whether the task has no user address space — `VmSize:` absent ⇒ kthread).
  - Upstream `InitError::PollPidFd(err)` replaced with a dedicated
    [`BroadcastError`] enum local to this crate.

Copyright (c) 2023 Orbital Labs, LLC <license@orbstack.dev>
(MIT — see LICENSE for full text)
*/

use std::error::Error;
use std::fs::{self, DirEntry};
use std::io;
use std::time::Duration;

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

use crate::pidfd::PidFd;

/// Errors produced by [`wait_for_pidfds_exit`].
#[derive(Debug, thiserror::Error)]
pub enum BroadcastError {
    #[error("timed out waiting for pidfds to exit")]
    Timeout(#[from] tokio::time::error::Elapsed),
    #[error("failed to poll pidfd: {0}")]
    PollPidFd(#[from] tokio::io::Error),
}

/// Check whether `pid` refers to a kernel thread.
///
/// Kernel threads have no user address space, which is visible in
/// `/proc/<pid>/status` as the absence of the `VmSize:` field.
fn is_process_kthread(pid: i32) -> io::Result<bool> {
    let path = format!("/proc/{}/status", pid);
    let status = fs::read_to_string(path)?;
    Ok(!status.lines().any(|l| l.starts_with("VmSize:")))
}

fn kill_one_entry(
    entry: Result<DirEntry, io::Error>,
    signal: Signal,
) -> Result<Option<PidFd>, Box<dyn Error>> {
    let filename = entry?.file_name();
    if let Ok(pid) = filename.to_str().unwrap().parse::<i32>() {
        // skip pid 1
        if pid == 1 {
            return Ok(None);
        }

        // skip kthreads (they won't exit)
        if is_process_kthread(pid)? {
            return Ok(None);
        }

        // open a pidfd before killing, then kill via pidfd for safety
        let pidfd = PidFd::open(pid)?;
        pidfd.kill(signal)?;
        Ok(Some(pidfd))
    } else {
        Ok(None)
    }
}

/// Broadcast `signal` to every process on the system (except pid 1 and kernel
/// threads), returning the collected pidfds so the caller can await exit.
///
/// Uses SIGSTOP/SIGCONT around the `/proc` scan to get a consistent snapshot.
/// Intended for init-style shutdown paths.
pub fn broadcast_signal(signal: Signal) -> nix::Result<Vec<PidFd>> {
    // freeze to get consistent snapshot and avoid thrashing
    kill(Pid::from_raw(-1), Signal::SIGSTOP)?;

    // can't use kill(-1) because we need to know which PIDs to wait for exit
    // otherwise unmount returns EBUSY
    let mut pids = Vec::new();
    match fs::read_dir("/proc") {
        Ok(entries) => {
            for entry in entries {
                match kill_one_entry(entry, signal) {
                    Ok(Some(pid)) => {
                        pids.push(pid);
                    }
                    Err(e) => {
                        eprintln!(" !!! Failed to read /proc entry: {}", e);
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            eprintln!(" !!! Failed to read /proc: {}", e);
        }
    }

    // always make sure to unfreeze
    kill(Pid::from_raw(-1), Signal::SIGCONT)?;
    Ok(pids)
}

/// Wait for every pidfd in `pidfds` to exit, or return
/// [`BroadcastError::Timeout`] if `timeout` elapses first.
pub async fn wait_for_pidfds_exit(
    pidfds: Vec<PidFd>,
    timeout: Duration,
) -> Result<(), BroadcastError> {
    let futures_ = pidfds
        .into_iter()
        .map(|pidfd| async move {
            let _guard = pidfd.wait().await?;
            Ok::<(), tokio::io::Error>(())
        })
        .collect::<Vec<_>>();

    let results = tokio::time::timeout(timeout, futures::future::join_all(futures_)).await?;
    for result in results {
        if let Err(err) = result {
            return Err(BroadcastError::PollPidFd(err));
        }
    }

    Ok(())
}
