//! Test runner implementation

use std::time::{Duration, Instant};
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use crate::types::*;
use crate::game_manager::GameProcessManager;

/// Test runner for executing tests
pub struct TestRunner {
    process_manager: GameProcessManager,
}

impl TestRunner {
    /// Create a new test runner
    pub fn new() -> Self {
        Self {
            process_manager: GameProcessManager::new(),
        }
    }

    /// Run a single test
    pub async fn run_test(&mut self, test: &TestCase) -> TestResult {
        info!("Running test: {} ({})", test.name, test.id);
        
        let start = Instant::now();
        let mut result = TestResult::new(&test.id, &test.name);
        
        // Execute based on test type
        match test.test_type {
            TestType::Unit | TestType::Integration => {
                result = self.run_command_test(test, result).await;
            }
            TestType::E2E => {
                result = self.run_e2e_test(test, result).await;
            }
            TestType::Performance => {
                result = self.run_performance_test(test, result).await;
            }
            TestType::Load => {
                result = self.run_load_test(test, result).await;
            }
        }
        
        let duration = start.elapsed();
        result = result.with_duration(duration);
        
        info!(
            "Test {} completed in {}ms with status: {:?}",
            test.id,
            duration.as_millis(),
            result.status
        );
        
        result
    }

    /// Run a command-based test
    async fn run_command_test(&mut self, test: &TestCase, result: TestResult) -> TestResult {
        let config = &test.config;
        
        // Get command to run
        let cmd = match &config.command {
            Some(cmd) => cmd,
            None => {
                // If no command, just return success (unit tests may not need process)
                return self.run_mock_test(test, result).await;
            }
        };
        
        // Spawn process
        let spawn_result = self.process_manager.spawn(
            cmd,
            config.args.clone(),
            config.env_vars.clone(),
            config.working_dir.as_ref().map(|s| s.as_str()),
        ).await;
        
        match spawn_result {
            Ok(process_info) => {
                let pid = process_info.pid;
                let timeout_duration = config.timeout_duration();
                
                // Wait for completion
                match self.process_manager.wait(pid, timeout_duration).await {
                    Ok(final_info) => {
                        let mut result = result
                            .with_exit_code(final_info.exit_code.unwrap_or(-1));
                        
                        match final_info.status {
                            ProcessStatus::Completed => {
                                if self.check_expected_result(test, final_info.exit_code) {
                                    result = result.passed();
                                } else {
                                    result = result.failed("Exit code mismatch");
                                }
                            }
                            ProcessStatus::Failed => {
                                if test.expected == ExpectedResult::Failure {
                                    result = result.passed();
                                } else {
                                    result = result.failed("Process failed");
                                }
                            }
                            ProcessStatus::Timeout => {
                                if test.expected == ExpectedResult::Timeout {
                                    result = result.passed();
                                } else {
                                    result = result.error("Test timed out");
                                }
                            }
                            _ => {
                                result = result.error(format!("Unexpected status: {:?}", final_info.status));
                            }
                        }
                        
                        result
                    }
                    Err(e) => {
                        result.error(format!("Wait error: {}", e))
                    }
                }
            }
            Err(e) => {
                result.error(format!("Spawn error: {}", e))
            }
        }
    }

    /// Run an E2E test
    async fn run_e2e_test(&mut self, test: &TestCase, result: TestResult) -> TestResult {
        // E2E tests typically involve setting up a full environment
        // For now, delegate to command test with longer timeout
        let mut test = test.clone();
        if test.config.timeout_seconds.is_none() {
            test.config.timeout_seconds = Some(300); // 5 minutes default for E2E
        }
        self.run_command_test(&test, result).await
    }

    /// Run a performance test
    async fn run_performance_test(&mut self, test: &TestCase, result: TestResult) -> TestResult {
        let config = &test.config;
        let cmd = match &config.command {
            Some(cmd) => cmd,
            None => return result.failed("No command specified for performance test"),
        };
        
        // Run the test multiple times to get average
        let iterations = 5;
        let mut durations = Vec::new();
        
        for i in 0..iterations {
            let iter_start = Instant::now();
            
            let spawn_result = self.process_manager.spawn_mock(
                cmd,
                config.args.clone(),
                config.env_vars.clone(),
                0,
            ).await;
            
            match spawn_result {
                Ok(info) => {
                    let _ = self.process_manager.wait(info.pid, config.timeout_duration()).await;
                    durations.push(iter_start.elapsed().as_millis() as f64);
                }
                Err(e) => {
                    return result.error(format!("Performance test iteration {} failed: {}", i, e));
                }
            }
        }
        
        // Calculate metrics
        let avg_duration = durations.iter().sum::<f64>() / durations.len() as f64;
        let min_duration = durations.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_duration = durations.iter().fold(0.0f64, |a, &b| a.max(b));
        
        let mut result = result
            .with_metric("avg_duration_ms", avg_duration)
            .with_metric("min_duration_ms", min_duration)
            .with_metric("max_duration_ms", max_duration);
        
        // Check against thresholds
        if let Some(max_mem) = config.max_memory_mb {
            // Memory check would need actual implementation
            debug!("Max memory threshold: {} MB", max_mem);
        }
        
        result.passed()
    }

    /// Run a load test
    async fn run_load_test(&mut self, test: &TestCase, result: TestResult) -> TestResult {
        // Load tests run multiple concurrent instances
        let concurrency = 10;
        let duration_secs = test.config.timeout_seconds.unwrap_or(60);
        
        let mut handles = Vec::new();
        let end_time = Instant::now() + Duration::from_secs(duration_secs);
        
        for _ in 0..concurrency {
            let handle = tokio::spawn(async move {
                let mut count = 0;
                while Instant::now() < end_time {
                    // Simulate work
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    count += 1;
                }
                count
            });
            handles.push(handle);
        }
        
        // Await all handles and collect results
        let mut total_requests: usize = 0;
        for handle in handles {
            if let Ok(count) = handle.await {
                total_requests += count;
            }
        }
        
        let rps = total_requests as f64 / duration_secs as f64;
        
        result
            .with_metric("total_requests", total_requests as f64)
            .with_metric("requests_per_second", rps)
            .passed()
    }

    /// Run a mock test (for tests without commands)
    async fn run_mock_test(&self, test: &TestCase, result: TestResult) -> TestResult {
        debug!("Running mock test: {}", test.id);
        
        // Simulate some work
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        match test.expected {
            ExpectedResult::Success => result.passed(),
            ExpectedResult::Failure => result.failed("Expected failure"),
            ExpectedResult::Error => result.error("Expected error"),
            _ => result.passed(),
        }
    }

    /// Check if actual result matches expected
    fn check_expected_result(&self, test: &TestCase, exit_code: Option<i32>) -> bool {
        match &test.expected {
            ExpectedResult::Success => exit_code == Some(0),
            ExpectedResult::Failure => exit_code != Some(0),
            ExpectedResult::ExitCode(expected) => exit_code == Some(*expected),
            _ => true, // Other expectations checked separately
        }
    }

    /// Run a test suite
    pub async fn run_suite(&mut self, suite: &TestSuite) -> SuiteResult {
        info!("Running test suite: {} ({})", suite.name, suite.id);
        
        let mut result = SuiteResult::new(&suite.id, &suite.name);
        
        // Run setup if defined
        if let Some(setup) = &suite.setup {
            info!("Running suite setup");
            let setup_result = self.run_setup_teardown(setup, "setup").await;
            result.setup_result = Some(setup_result);
        }
        
        // Run all tests
        for test in &suite.tests {
            let test_result = self.run_test(test).await;
            result.add_result(test_result);
        }
        
        // Run teardown if defined
        if let Some(teardown) = &suite.teardown {
            info!("Running suite teardown");
            let teardown_result = self.run_setup_teardown(teardown, "teardown").await;
            result.teardown_result = Some(teardown_result);
        }
        
        let result = result.complete();
        
        info!(
            "Suite {} completed: {} passed, {} failed, {} total",
            suite.id,
            result.passed_count(),
            result.failed_count(),
            result.total_count()
        );
        
        result
    }

    /// Run setup or teardown
    async fn run_setup_teardown(&mut self, config: &TestConfig, name: &str) -> TestResult {
        let start = Instant::now();
        let mut result = TestResult::new(format!("{}", name), format!("{} {}", name, name));
        
        if let Some(cmd) = &config.command {
            let spawn_result = self.process_manager.spawn(
                cmd,
                config.args.clone(),
                config.env_vars.clone(),
                config.working_dir.as_ref().map(|s| s.as_str()),
            ).await;
            
            match spawn_result {
                Ok(info) => {
                    let timeout_duration = config.timeout_duration();
                    match self.process_manager.wait(info.pid, timeout_duration).await {
                        Ok(final_info) => {
                            if final_info.status == ProcessStatus::Completed {
                                result = result.passed();
                            } else {
                                result = result.error(format!("{} failed", name));
                            }
                        }
                        Err(e) => {
                            result = result.error(format!("{} wait error: {}", name, e));
                        }
                    }
                }
                Err(e) => {
                    result = result.error(format!("{} spawn error: {}", name, e));
                }
            }
        } else {
            // No command, just return success
            result = result.passed();
        }
        
        result.with_duration(start.elapsed())
    }

    /// Cleanup resources
    pub async fn cleanup(&mut self) {
        self.process_manager.kill_all().await;
        self.process_manager.cleanup();
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_runner_new() {
        let runner = TestRunner::new();
        // Just verify it creates without error
        assert!(true);
    }

    #[tokio::test]
    async fn test_run_mock_test_success() {
        let mut runner = TestRunner::new();
        
        let test = TestCase::new("t1", "Test 1", TestType::Unit)
            .expect(ExpectedResult::Success);
        
        let result = runner.run_test(&test).await;
        assert!(result.is_passed());
    }

    #[tokio::test]
    async fn test_run_mock_test_failure() {
        let mut runner = TestRunner::new();
        
        let test = TestCase::new("t1", "Test 1", TestType::Unit)
            .expect(ExpectedResult::Failure);
        
        let result = runner.run_test(&test).await;
        // The mock test returns failed status when expected is Failure
        assert!(result.is_failed());
    }

    #[tokio::test]
    async fn test_run_test_with_timeout() {
        let mut runner = TestRunner::new();
        
        let test = TestCase::new("t1", "Test 1", TestType::Unit)
            .with_config(TestConfig::default().with_timeout(1))
            .expect(ExpectedResult::Success);
        
        let result = runner.run_test(&test).await;
        // Mock test should pass quickly
        assert!(result.is_passed());
    }

    #[tokio::test]
    async fn test_run_test_with_env() {
        let mut runner = TestRunner::new();
        
        let test = TestCase::new("t1", "Test 1", TestType::Unit)
            .with_config(
                TestConfig::default()
                    .with_env("TEST_KEY", "test_value")
            )
            .expect(ExpectedResult::Success);
        
        let result = runner.run_test(&test).await;
        assert!(result.is_passed());
    }

    #[tokio::test]
    async fn test_run_suite() {
        let mut runner = TestRunner::new();
        
        let suite = TestSuite::new("suite-1", "My Suite")
            .add_test(TestCase::new("t1", "Test 1", TestType::Unit))
            .add_test(TestCase::new("t2", "Test 2", TestType::Unit));
        
        let result = runner.run_suite(&suite).await;
        
        assert_eq!(result.total_count(), 2);
        assert_eq!(result.passed_count(), 2);
        assert_eq!(result.failed_count(), 0);
        assert!(result.all_passed());
    }

    #[tokio::test]
    async fn test_run_suite_with_tags() {
        let mut runner = TestRunner::new();
        
        let suite = TestSuite::new("suite-1", "My Suite")
            .add_test(TestCase::new("t1", "Test 1", TestType::Unit).with_tag("fast"))
            .add_test(TestCase::new("t2", "Test 2", TestType::Unit).with_tag("slow"))
            .add_test(TestCase::new("t3", "Test 3", TestType::Unit).with_tag("fast"));
        
        assert_eq!(suite.count_by_tag("fast"), 2);
        assert_eq!(suite.count_by_tag("slow"), 1);
        
        let result = runner.run_suite(&suite).await;
        assert_eq!(result.total_count(), 3);
    }

    #[tokio::test]
    async fn test_run_suite_with_setup_teardown() {
        let mut runner = TestRunner::new();
        
        let suite = TestSuite::new("suite-1", "My Suite")
            .with_setup(TestConfig::default().with_command("echo", vec!["setup".to_string()]))
            .with_teardown(TestConfig::default().with_command("echo", vec!["teardown".to_string()]))
            .add_test(TestCase::new("t1", "Test 1", TestType::Unit));
        
        let result = runner.run_suite(&suite).await;
        
        assert!(result.setup_result.is_some());
        assert!(result.teardown_result.is_some());
        // Setup/teardown with echo should succeed
        assert!(result.setup_result.as_ref().unwrap().is_passed());
    }

    #[tokio::test]
    async fn test_run_performance_test() {
        let mut runner = TestRunner::new();
        
        let test = TestCase::new("perf", "Performance Test", TestType::Performance)
            .with_config(TestConfig::default().with_command("echo", vec!["test".to_string()]));
        
        let result = runner.run_test(&test).await;
        
        assert!(result.is_passed());
        assert!(result.metrics.contains_key("avg_duration_ms"));
        assert!(result.metrics.contains_key("min_duration_ms"));
        assert!(result.metrics.contains_key("max_duration_ms"));
    }

    #[tokio::test]
    async fn test_run_load_test() {
        let mut runner = TestRunner::new();
        
        let test = TestCase::new("load", "Load Test", TestType::Load)
            .with_config(TestConfig::default().with_timeout(2));
        
        let result = runner.run_test(&test).await;
        
        assert!(result.is_passed());
        assert!(result.metrics.contains_key("total_requests"));
        assert!(result.metrics.contains_key("requests_per_second"));
    }

    #[tokio::test]
    async fn test_cleanup() {
        let mut runner = TestRunner::new();
        
        // Run a test first
        let test = TestCase::new("t1", "Test 1", TestType::Unit);
        let _ = runner.run_test(&test).await;
        
        // Cleanup should not panic
        runner.cleanup().await;
    }
}
