//! Game process management for test execution

use std::collections::HashMap;
use std::time::Duration;
use tokio::process::{Child, Command};
use tokio::time::timeout;
use tracing::{debug, error, info, warn};
use chrono::Utc;

use crate::types::{ProcessInfo, ProcessStatus};

/// Manages game/test processes
pub struct GameProcessManager {
    /// Active processes
    processes: HashMap<u32, ManagedProcess>,
    /// Next process ID
    next_pid: u32,
}

struct ManagedProcess {
    info: ProcessInfo,
    #[allow(dead_code)]
    child: Option<Child>,
}

impl GameProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
            next_pid: 1000,
        }
    }

    /// Spawn a new process
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute
    /// * `args` - Command arguments
    /// * `env_vars` - Environment variables
    /// * `working_dir` - Working directory
    pub async fn spawn(
        &mut self,
        command: impl AsRef<str>,
        args: Vec<String>,
        env_vars: HashMap<String, String>,
        working_dir: Option<impl AsRef<std::path::Path>>,
    ) -> Result<ProcessInfo, ProcessError> {
        let cmd_str = command.as_ref();
        let cmd_display = format!("{} {}", cmd_str, args.join(" "));
        
        info!("Spawning process: {}", cmd_display);

        let mut cmd = Command::new(cmd_str);
        cmd.args(&args);
        
        // Set environment variables
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
        
        // Set working directory
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }
        
        // Configure stdio
        cmd.stdin(std::process::Stdio::null());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
        
        // Spawn the process
        let child = cmd.spawn().map_err(|e| ProcessError::SpawnFailed {
            command: cmd_str.to_string(),
            source: e,
        })?;
        
        let pid = self.next_pid;
        self.next_pid += 1;
        
        let info = ProcessInfo {
            pid,
            name: cmd_str.to_string(),
            command: cmd_display,
            started_at: Utc::now(),
            status: ProcessStatus::Running,
            exit_code: None,
        };
        
        debug!("Process spawned with PID: {}", pid);
        
        self.processes.insert(pid, ManagedProcess {
            info: info.clone(),
            child: Some(child),
        });
        
        Ok(info)
    }

    /// Spawn a mock process for testing (returns immediately with success)
    pub async fn spawn_mock(
        &mut self,
        command: impl AsRef<str>,
        args: Vec<String>,
        _env_vars: HashMap<String, String>,
        exit_code: i32,
    ) -> Result<ProcessInfo, ProcessError> {
        let cmd_str = command.as_ref();
        let cmd_display = format!("{} {}", cmd_str, args.join(" "));
        
        debug!("Spawning mock process: {}", cmd_display);
        
        let pid = self.next_pid;
        self.next_pid += 1;
        
        let status = if exit_code == 0 {
            ProcessStatus::Completed
        } else {
            ProcessStatus::Failed
        };
        
        let info = ProcessInfo {
            pid,
            name: cmd_str.to_string(),
            command: cmd_display,
            started_at: Utc::now(),
            status,
            exit_code: Some(exit_code),
        };
        
        self.processes.insert(pid, ManagedProcess {
            info: info.clone(),
            child: None,
        });
        
        Ok(info)
    }

    /// Get process information
    pub fn get_process(&self, pid: u32) -> Option<&ProcessInfo> {
        self.processes.get(&pid).map(|p| &p.info)
    }

    /// Get all processes
    pub fn list_processes(&self) -> Vec<&ProcessInfo> {
        self.processes.values().map(|p| &p.info).collect()
    }

    /// Get active (running) processes
    pub fn active_processes(&self) -> Vec<&ProcessInfo> {
        self.processes
            .values()
            .map(|p| &p.info)
            .filter(|p| p.status.is_active())
            .collect()
    }

    /// Kill a process
    pub async fn kill(&mut self, pid: u32) -> Result<(), ProcessError> {
        info!("Killing process: {}", pid);
        
        let process = self.processes.get_mut(&pid)
            .ok_or(ProcessError::ProcessNotFound(pid))?;
        
        if let Some(ref mut child) = process.child {
            // Try graceful shutdown first
            #[cfg(all(unix, feature = "unix"))]
            {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid;
                
                if let Some(id) = child.id() {
                    let _ = kill(Pid::from_raw(id as i32), Signal::SIGTERM);
                }
            }
            
            // Force kill if needed
            let _ = child.kill().await;
        }
        
        process.info.status = ProcessStatus::Killed;
        
        Ok(())
    }

    /// Wait for a process to complete with timeout
    pub async fn wait(
        &mut self,
        pid: u32,
        timeout_duration: Duration,
    ) -> Result<ProcessInfo, ProcessError> {
        let process = self.processes.get_mut(&pid)
            .ok_or(ProcessError::ProcessNotFound(pid))?;
        
        if process.info.status.is_complete() {
            return Ok(process.info.clone());
        }
        
        if let Some(ref mut child) = process.child {
            let wait_result = timeout(timeout_duration, child.wait()).await;
            
            match wait_result {
                Ok(Ok(status)) => {
                    process.info.status = if status.success() {
                        ProcessStatus::Completed
                    } else {
                        ProcessStatus::Failed
                    };
                    process.info.exit_code = status.code();
                }
                Ok(Err(e)) => {
                    error!("Process wait error: {}", e);
                    process.info.status = ProcessStatus::Failed;
                }
                Err(_) => {
                    warn!("Process {} timed out", pid);
                    process.info.status = ProcessStatus::Timeout;
                    // Kill the process
                    let _ = child.kill().await;
                }
            }
        }
        
        Ok(process.info.clone())
    }

    /// Clean up completed processes
    pub fn cleanup(&mut self) {
        let completed: Vec<u32> = self.processes
            .iter()
            .filter(|(_, p)| p.info.status.is_complete())
            .map(|(pid, _)| *pid)
            .collect();
        
        for pid in completed {
            debug!("Cleaning up process: {}", pid);
            self.processes.remove(&pid);
        }
    }

    /// Kill all active processes
    pub async fn kill_all(&mut self) {
        let active_pids: Vec<u32> = self.active_processes()
            .iter()
            .map(|p| p.pid)
            .collect();
        
        for pid in active_pids {
            let _ = self.kill(pid).await;
        }
    }
}

impl Default for GameProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Process error types
#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("Failed to spawn process '{command}': {source}")]
    SpawnFailed { command: String, source: std::io::Error },
    
    #[error("Process not found: {0}")]
    ProcessNotFound(u32),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Process timed out")]
    Timeout,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_manager_new() {
        let manager = GameProcessManager::new();
        assert!(manager.list_processes().is_empty());
    }

    #[tokio::test]
    async fn test_spawn_mock_success() {
        let mut manager = GameProcessManager::new();
        
        let info = manager.spawn_mock(
            "test",
            vec!["--arg".to_string()],
            HashMap::new(),
            0,
        ).await.unwrap();
        
        assert_eq!(info.name, "test");
        assert_eq!(info.status, ProcessStatus::Completed);
        assert_eq!(info.exit_code, Some(0));
    }

    #[tokio::test]
    async fn test_spawn_mock_failure() {
        let mut manager = GameProcessManager::new();
        
        let info = manager.spawn_mock(
            "test",
            vec![],
            HashMap::new(),
            1,
        ).await.unwrap();
        
        assert_eq!(info.status, ProcessStatus::Failed);
        assert_eq!(info.exit_code, Some(1));
    }

    #[tokio::test]
    async fn test_get_process() {
        let mut manager = GameProcessManager::new();
        
        let info = manager.spawn_mock("test", vec![], HashMap::new(), 0).await.unwrap();
        let pid = info.pid;
        
        let retrieved = manager.get_process(pid);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().pid, pid);
    }

    #[tokio::test]
    async fn test_get_process_not_found() {
        let manager = GameProcessManager::new();
        
        let retrieved = manager.get_process(99999);
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_list_processes() {
        let mut manager = GameProcessManager::new();
        
        manager.spawn_mock("test1", vec![], HashMap::new(), 0).await.unwrap();
        manager.spawn_mock("test2", vec![], HashMap::new(), 0).await.unwrap();
        
        let processes = manager.list_processes();
        assert_eq!(processes.len(), 2);
    }

    #[tokio::test]
    async fn test_active_processes() {
        let mut manager = GameProcessManager::new();
        
        // Completed process
        manager.spawn_mock("completed", vec![], HashMap::new(), 0).await.unwrap();
        
        // Active processes don't stay active in mock mode, but let's verify the method works
        let active = manager.active_processes();
        // Mock processes are immediately completed
        assert!(active.is_empty());
    }

    #[tokio::test]
    async fn test_wait_completed_process() {
        let mut manager = GameProcessManager::new();
        
        let info = manager.spawn_mock("test", vec![], HashMap::new(), 0).await.unwrap();
        let pid = info.pid;
        
        let result = manager.wait(pid, Duration::from_secs(1)).await.unwrap();
        assert_eq!(result.status, ProcessStatus::Completed);
    }

    #[tokio::test]
    async fn test_wait_not_found() {
        let mut manager = GameProcessManager::new();
        
        let result = manager.wait(99999, Duration::from_secs(1)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cleanup() {
        let mut manager = GameProcessManager::new();
        
        manager.spawn_mock("test", vec![], HashMap::new(), 0).await.unwrap();
        assert_eq!(manager.list_processes().len(), 1);
        
        manager.cleanup();
        assert!(manager.list_processes().is_empty());
    }

    #[tokio::test]
    async fn test_spawn_with_env_vars() {
        let mut manager = GameProcessManager::new();
        
        let mut env = HashMap::new();
        env.insert("TEST_VAR".to_string(), "test_value".to_string());
        
        let info = manager.spawn_mock("test", vec![], env, 0).await.unwrap();
        assert_eq!(info.name, "test");
    }

    #[tokio::test]
    async fn test_kill_not_found() {
        let mut manager = GameProcessManager::new();
        
        let result = manager.kill(99999).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ProcessError::ProcessNotFound(_)));
    }

    #[tokio::test]
    async fn test_kill_all() {
        let mut manager = GameProcessManager::new();
        
        // Note: mock processes are already completed, so kill_all has no effect
        // But we can verify it doesn't error
        manager.spawn_mock("test1", vec![], HashMap::new(), 0).await.unwrap();
        manager.spawn_mock("test2", vec![], HashMap::new(), 0).await.unwrap();
        
        manager.kill_all().await;
        
        // Processes should still exist in the map (cleanup not called)
        assert_eq!(manager.list_processes().len(), 2);
    }

    #[test]
    fn test_process_status_is_active() {
        assert!(ProcessStatus::Starting.is_active());
        assert!(ProcessStatus::Running.is_active());
        assert!(!ProcessStatus::Completed.is_active());
        assert!(!ProcessStatus::Failed.is_active());
        assert!(!ProcessStatus::Killed.is_active());
    }

    #[test]
    fn test_process_error_display() {
        let err = ProcessError::ProcessNotFound(123);
        assert!(err.to_string().contains("Process not found"));
    }
}
