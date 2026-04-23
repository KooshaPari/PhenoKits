//! Test types and structures

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Test case definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    /// Unique test ID
    pub id: String,
    /// Test name
    pub name: String,
    /// Test description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Test type
    pub test_type: TestType,
    /// Test configuration
    pub config: TestConfig,
    /// Expected result
    pub expected: ExpectedResult,
    /// Tags for categorization
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
}

impl TestCase {
    /// Create a new test case
    pub fn new(id: impl Into<String>, name: impl Into<String>, test_type: TestType) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            test_type,
            config: TestConfig::default(),
            expected: ExpectedResult::Success,
            tags: Vec::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set configuration
    pub fn with_config(mut self, config: TestConfig) -> Self {
        self.config = config;
        self
    }

    /// Set expected result
    pub fn expect(mut self, expected: ExpectedResult) -> Self {
        self.expected = expected;
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}

/// Test type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestType {
    /// Unit test
    Unit,
    /// Integration test
    Integration,
    /// End-to-end test
    E2E,
    /// Performance test
    Performance,
    /// Load test
    Load,
}

/// Test configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestConfig {
    /// Timeout for test execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u64>,
    /// Environment variables
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub env_vars: HashMap<String, String>,
    /// Working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// Command to execute (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// Command arguments
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub args: Vec<String>,
    /// Maximum memory usage (MB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_mb: Option<u64>,
    /// Retry count on failure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u32>,
}

impl TestConfig {
    /// Set timeout
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    /// Add environment variable
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars.insert(key.into(), value.into());
        self
    }

    /// Set working directory
    pub fn with_working_dir(mut self, dir: impl Into<String>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// Set command
    pub fn with_command(mut self, cmd: impl Into<String>, args: Vec<String>) -> Self {
        self.command = Some(cmd.into());
        self.args = args;
        self
    }

    /// Get timeout as Duration
    pub fn timeout_duration(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds.unwrap_or(30))
    }
}

/// Expected test result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedResult {
    /// Test should succeed
    Success,
    /// Test should fail
    Failure,
    /// Test should error
    Error,
    /// Test should timeout
    Timeout,
    /// Expected exit code
    ExitCode(i32),
    /// Expected output contains
    OutputContains(String),
    /// Expected output matches
    OutputMatches(String),
}

/// Test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Test ID
    pub test_id: String,
    /// Test name
    pub test_name: String,
    /// Test status
    pub status: TestStatus,
    /// Duration of test execution
    pub duration_ms: u64,
    /// Start time
    pub started_at: DateTime<Utc>,
    /// End time
    pub completed_at: DateTime<Utc>,
    /// Exit code (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Standard output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    /// Standard error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Additional metrics
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub metrics: HashMap<String, f64>,
}

impl TestResult {
    /// Create a new test result
    pub fn new(test_id: impl Into<String>, test_name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            test_id: test_id.into(),
            test_name: test_name.into(),
            status: TestStatus::Pending,
            duration_ms: 0,
            started_at: now,
            completed_at: now,
            exit_code: None,
            stdout: None,
            stderr: None,
            error_message: None,
            metrics: HashMap::new(),
        }
    }

    /// Mark as passed
    pub fn passed(mut self) -> Self {
        self.status = TestStatus::Passed;
        self.completed_at = Utc::now();
        self
    }

    /// Mark as failed
    pub fn failed(mut self, error: impl Into<String>) -> Self {
        self.status = TestStatus::Failed;
        self.error_message = Some(error.into());
        self.completed_at = Utc::now();
        self
    }

    /// Mark as error
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.status = TestStatus::Error;
        self.error_message = Some(error.into());
        self.completed_at = Utc::now();
        self
    }

    /// Mark as skipped
    pub fn skipped(mut self) -> Self {
        self.status = TestStatus::Skipped;
        self.completed_at = Utc::now();
        self
    }

    /// Set duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration_ms = duration.as_millis() as u64;
        self
    }

    /// Set output
    pub fn with_output(mut self, stdout: impl Into<String>, stderr: impl Into<String>) -> Self {
        self.stdout = Some(stdout.into());
        self.stderr = Some(stderr.into());
        self
    }

    /// Set exit code
    pub fn with_exit_code(mut self, code: i32) -> Self {
        self.exit_code = Some(code);
        self
    }

    /// Add metric
    pub fn with_metric(mut self, name: impl Into<String>, value: f64) -> Self {
        self.metrics.insert(name.into(), value);
        self
    }

    /// Check if test passed
    pub fn is_passed(&self) -> bool {
        self.status == TestStatus::Passed
    }

    /// Check if test failed (includes errors)
    pub fn is_failed(&self) -> bool {
        matches!(self.status, TestStatus::Failed | TestStatus::Error)
    }
}

/// Test status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestStatus {
    /// Test is pending
    Pending,
    /// Test is running
    Running,
    /// Test passed
    Passed,
    /// Test failed
    Failed,
    /// Test had an error
    Error,
    /// Test was skipped
    Skipped,
    /// Test timed out
    Timeout,
}

impl TestStatus {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            TestStatus::Pending => "Pending",
            TestStatus::Running => "Running",
            TestStatus::Passed => "Passed",
            TestStatus::Failed => "Failed",
            TestStatus::Error => "Error",
            TestStatus::Skipped => "Skipped",
            TestStatus::Timeout => "Timeout",
        }
    }

    /// Check if test is complete
    pub fn is_complete(&self) -> bool {
        !matches!(self, TestStatus::Pending | TestStatus::Running)
    }
}

/// Test suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    /// Suite ID
    pub id: String,
    /// Suite name
    pub name: String,
    /// Suite description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Test cases in the suite
    pub tests: Vec<TestCase>,
    /// Setup configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup: Option<TestConfig>,
    /// Teardown configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teardown: Option<TestConfig>,
    /// Suite-level tags
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
}

impl TestSuite {
    /// Create a new test suite
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            tests: Vec::new(),
            setup: None,
            teardown: None,
            tags: Vec::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Add a test
    pub fn add_test(mut self, test: TestCase) -> Self {
        self.tests.push(test);
        self
    }

    /// Set setup
    pub fn with_setup(mut self, setup: TestConfig) -> Self {
        self.setup = Some(setup);
        self
    }

    /// Set teardown
    pub fn with_teardown(mut self, teardown: TestConfig) -> Self {
        self.teardown = Some(teardown);
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Count tests by tag
    pub fn count_by_tag(&self, tag: &str) -> usize {
        self.tests
            .iter()
            .filter(|t| t.tags.contains(&tag.to_string()))
            .count()
    }
}

/// Test suite result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteResult {
    /// Suite ID
    pub suite_id: String,
    /// Suite name
    pub suite_name: String,
    /// Test results
    pub results: Vec<TestResult>,
    /// Start time
    pub started_at: DateTime<Utc>,
    /// End time
    pub completed_at: DateTime<Utc>,
    /// Setup result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_result: Option<TestResult>,
    /// Teardown result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teardown_result: Option<TestResult>,
}

impl SuiteResult {
    /// Create a new suite result
    pub fn new(suite_id: impl Into<String>, suite_name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            suite_id: suite_id.into(),
            suite_name: suite_name.into(),
            results: Vec::new(),
            started_at: now,
            completed_at: now,
            setup_result: None,
            teardown_result: None,
        }
    }

    /// Add a test result
    pub fn add_result(&mut self, result: TestResult) {
        self.results.push(result);
    }

    /// Complete the suite
    pub fn complete(mut self) -> Self {
        self.completed_at = Utc::now();
        self
    }

    /// Count passed tests
    pub fn passed_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == TestStatus::Passed)
            .count()
    }

    /// Count failed tests
    pub fn failed_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_failed()).count()
    }

    /// Count skipped tests
    pub fn skipped_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.status == TestStatus::Skipped)
            .count()
    }

    /// Get total test count
    pub fn total_count(&self) -> usize {
        self.results.len()
    }

    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.failed_count() == 0 && self.passed_count() > 0
    }

    /// Get pass rate as percentage
    pub fn pass_rate(&self) -> f64 {
        let total = self.total_count();
        if total == 0 {
            return 0.0;
        }
        let passed = self.passed_count();
        (passed as f64 / total as f64) * 100.0
    }
}

/// Process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub name: String,
    /// Command that was executed
    pub command: String,
    /// Start time
    pub started_at: DateTime<Utc>,
    /// Status
    pub status: ProcessStatus,
    /// Exit code (if completed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
}

/// Process status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessStatus {
    /// Process is starting
    Starting,
    /// Process is running
    Running,
    /// Process completed successfully
    Completed,
    /// Process failed
    Failed,
    /// Process was killed
    Killed,
    /// Process timed out
    Timeout,
}

impl ProcessStatus {
    /// Check if process is active
    pub fn is_active(&self) -> bool {
        matches!(self, ProcessStatus::Starting | ProcessStatus::Running)
    }

    /// Check if process has completed
    pub fn is_complete(&self) -> bool {
        !self.is_active()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_case_creation() {
        let test = TestCase::new("test-1", "My Test", TestType::Unit)
            .with_description("A test")
            .with_tag("fast")
            .expect(ExpectedResult::Success);

        assert_eq!(test.id, "test-1");
        assert_eq!(test.name, "My Test");
        assert_eq!(test.test_type, TestType::Unit);
        assert_eq!(test.tags, vec!["fast"]);
    }

    #[test]
    fn test_test_config() {
        let config = TestConfig::default()
            .with_timeout(60)
            .with_env("KEY", "VALUE")
            .with_command("test", vec!["--verbose".to_string()]);

        assert_eq!(config.timeout_seconds, Some(60));
        assert_eq!(config.env_vars.get("KEY"), Some(&"VALUE".to_string()));
        assert_eq!(config.command, Some("test".to_string()));
    }

    #[test]
    fn test_test_config_timeout_duration() {
        let config = TestConfig::default().with_timeout(30);
        assert_eq!(config.timeout_duration(), Duration::from_secs(30));

        // Default timeout
        let config = TestConfig::default();
        assert_eq!(config.timeout_duration(), Duration::from_secs(30));
    }

    #[test]
    fn test_test_result_passed() {
        let result = TestResult::new("t1", "Test 1").passed();
        assert!(result.is_passed());
        assert!(!result.is_failed());
    }

    #[test]
    fn test_test_result_failed() {
        let result = TestResult::new("t1", "Test 1").failed("error");
        assert!(!result.is_passed());
        assert!(result.is_failed());
    }

    #[test]
    fn test_test_result_with_metric() {
        let result = TestResult::new("t1", "Test 1")
            .with_metric("duration", 1.5)
            .with_metric("memory", 100.0);

        assert_eq!(result.metrics.get("duration"), Some(&1.5));
    }

    #[test]
    fn test_test_status_display() {
        assert_eq!(TestStatus::Passed.display_name(), "Passed");
        assert_eq!(TestStatus::Failed.display_name(), "Failed");
    }

    #[test]
    fn test_test_suite() {
        let suite = TestSuite::new("suite-1", "My Suite")
            .with_description("A suite")
            .add_test(TestCase::new("t1", "Test 1", TestType::Unit))
            .add_test(TestCase::new("t2", "Test 2", TestType::Integration));

        assert_eq!(suite.tests.len(), 2);
    }

    #[test]
    fn test_suite_result() {
        let mut result = SuiteResult::new("s1", "Suite 1");

        result.add_result(TestResult::new("t1", "Test 1").passed());
        result.add_result(TestResult::new("t2", "Test 2").failed("error"));
        result.add_result(TestResult::new("t3", "Test 3").passed());

        let result = result.complete();

        assert_eq!(result.passed_count(), 2);
        assert_eq!(result.failed_count(), 1);
        assert_eq!(result.total_count(), 3);
        assert!(!result.all_passed());
        assert!((result.pass_rate() - 66.66666666666667).abs() < 0.0001);
    }

    #[test]
    fn test_suite_result_all_passed() {
        let mut result = SuiteResult::new("s1", "Suite 1");
        result.add_result(TestResult::new("t1", "Test 1").passed());

        assert!(result.all_passed());
    }

    #[test]
    fn test_process_status() {
        assert!(ProcessStatus::Running.is_active());
        assert!(!ProcessStatus::Completed.is_active());
        assert!(ProcessStatus::Completed.is_complete());
    }
}
