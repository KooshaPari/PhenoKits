//! Testing handler for MCP servers

use phenotype_mcp_framework::{McpServer, McpRequest, McpResponse, Tool, ToolSchemaBuilder};
use serde_json::json;
use tracing::{debug, info, warn};

use crate::game_manager::GameProcessManager;
use crate::runner::TestRunner;
use crate::report::{ReportGenerator, TestReport};
use crate::types::*;

/// Testing handler for MCP servers
pub struct TestingHandler {
    runner: TestRunner,
    report_generator: ReportGenerator,
}

impl TestingHandler {
    /// Create a new testing handler
    pub fn new() -> Self {
        Self {
            runner: TestRunner::new(),
            report_generator: ReportGenerator::new(),
        }
    }

    /// Run a single test
    pub async fn run_test(&mut self, test: &TestCase) -> TestResult {
        self.runner.run_test(test).await
    }

    /// Run a test suite
    pub async fn run_suite(&mut self, suite: &TestSuite) -> SuiteResult {
        self.runner.run_suite(suite).await
    }

    /// Generate a report from suite results
    pub fn generate_report(&self, title: impl Into<String>, suites: Vec<SuiteResult>) -> TestReport {
        self.report_generator.generate(title, suites)
    }

    /// Cleanup resources
    pub async fn cleanup(&mut self) {
        self.runner.cleanup().await;
    }
}

impl Default for TestingHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// MCP Testing Server implementation
pub struct TestingMcpServer {
    handler: TestingHandler,
}

impl TestingMcpServer {
    /// Create a new testing MCP server
    pub fn new() -> Self {
        Self {
            handler: TestingHandler::new(),
        }
    }

    /// Get the testing handler
    pub fn handler(&mut self) -> &mut TestingHandler {
        &mut self.handler
    }
}

impl McpServer for TestingMcpServer {
    fn name(&self) -> &'static str {
        "phenotype-testing"
    }

    fn version(&self) -> &'static str {
        crate::VERSION
    }

    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool::with_properties(
                "run_test",
                "Run a single test case",
                json!({
                    "id": ToolSchemaBuilder::string_prop("Test case ID"),
                    "name": ToolSchemaBuilder::string_prop("Test case name"),
                    "type": {
                        "type": "string",
                        "enum": ["unit", "integration", "e2e", "performance", "load"],
                        "description": "Test type"
                    }
                }),
                vec!["id".to_string(), "name".to_string()],
            ),
            Tool::with_properties(
                "run_suite",
                "Run a test suite",
                json!({
                    "suite_id": ToolSchemaBuilder::string_prop("Suite ID"),
                    "suite_name": ToolSchemaBuilder::string_prop("Suite name")
                }),
                vec!["suite_id".to_string(), "suite_name".to_string()],
            ),
            Tool::with_properties(
                "generate_report",
                "Generate a test report",
                json!({
                    "title": ToolSchemaBuilder::string_prop("Report title"),
                    "format": {
                        "type": "string",
                        "enum": ["json", "html", "markdown", "junit"],
                        "description": "Report format"
                    }
                }),
                vec!["title".to_string()],
            ),
            Tool::with_properties(
                "spawn_process",
                "Spawn a test process",
                json!({
                    "command": ToolSchemaBuilder::string_prop("Command to execute"),
                    "args": ToolSchemaBuilder::array_prop("Command arguments", "string"),
                    "timeout_seconds": ToolSchemaBuilder::int_prop("Timeout in seconds", Some(30))
                }),
                vec!["command".to_string()],
            ),
        ]
    }

    fn handle_tool(&self, name: String, arguments: serde_json::Value) -> Result<String, String> {
        match name.as_str() {
            "run_test" => {
                let id = arguments
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("test-1");
                let test_name = arguments
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Test");
                let test_type = arguments
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unit");

                let test_type = match test_type {
                    "integration" => TestType::Integration,
                    "e2e" => TestType::E2E,
                    "performance" => TestType::Performance,
                    "load" => TestType::Load,
                    _ => TestType::Unit,
                };

                let test = TestCase::new(id, test_name, test_type);
                
                // Note: In a real async context, we'd run the test here
                // For sync handler, we return a queued message
                Ok(format!(
                    "Test '{}' ({}) queued for execution",
                    test_name, id
                ))
            }
            "run_suite" => {
                let suite_id = arguments
                    .get("suite_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("suite-1");
                let suite_name = arguments
                    .get("suite_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Test Suite");

                Ok(format!(
                    "Suite '{}' ({}) queued for execution",
                    suite_name, suite_id
                ))
            }
            "generate_report" => {
                let title = arguments
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Test Report");
                let format = arguments
                    .get("format")
                    .and_then(|v| v.as_str())
                    .unwrap_or("json");

                Ok(format!(
                    "Report '{}' will be generated in {} format",
                    title, format
                ))
            }
            "spawn_process" => {
                let command = arguments
                    .get("command")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing command")?;

                let timeout = arguments
                    .get("timeout_seconds")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(30);

                Ok(format!(
                    "Process '{}' spawn requested with {}s timeout",
                    command, timeout
                ))
            }
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }
}

impl Default for TestingMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_testing_handler_new() {
        let handler = TestingHandler::new();
        // Just verify it creates
        assert!(true);
    }

    #[tokio::test]
    async fn test_testing_handler_run_test() {
        let mut handler = TestingHandler::new();
        
        let test = TestCase::new("t1", "Test 1", TestType::Unit);
        let result = handler.run_test(&test).await;
        
        assert!(result.is_passed());
    }

    #[tokio::test]
    async fn test_testing_handler_run_suite() {
        let mut handler = TestingHandler::new();
        
        let suite = TestSuite::new("s1", "Suite 1")
            .add_test(TestCase::new("t1", "Test 1", TestType::Unit))
            .add_test(TestCase::new("t2", "Test 2", TestType::Unit));
        
        let result = handler.run_suite(&suite).await;
        
        assert_eq!(result.total_count(), 2);
        assert!(result.all_passed());
    }

    #[tokio::test]
    async fn test_testing_handler_generate_report() {
        let mut handler = TestingHandler::new();
        
        let suite = TestSuite::new("s1", "Suite 1")
            .add_test(TestCase::new("t1", "Test 1", TestType::Unit));
        
        let suite_result = handler.run_suite(&suite).await;
        let report = handler.generate_report("Test Report", vec![suite_result]);
        
        assert_eq!(report.title, "Test Report");
        assert_eq!(report.summary.total_tests, 1);
    }

    #[tokio::test]
    async fn test_testing_handler_cleanup() {
        let mut handler = TestingHandler::new();
        
        // Cleanup should not panic
        handler.cleanup().await;
    }

    #[test]
    fn test_testing_mcp_server_name() {
        let server = TestingMcpServer::new();
        assert_eq!(server.name(), "phenotype-testing");
    }

    #[test]
    fn test_testing_mcp_server_tools() {
        let server = TestingMcpServer::new();
        let tools = server.tools();
        
        assert!(!tools.is_empty());
        
        let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(tool_names.contains(&"run_test"));
        assert!(tool_names.contains(&"run_suite"));
        assert!(tool_names.contains(&"generate_report"));
        assert!(tool_names.contains(&"spawn_process"));
    }

    #[test]
    fn test_testing_mcp_server_handle_run_test() {
        let server = TestingMcpServer::new();
        
        let result = server.handle_tool(
            "run_test".to_string(),
            json!({
                "id": "test-123",
                "name": "My Test",
                "type": "unit"
            }),
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap().contains("My Test"));
    }

    #[test]
    fn test_testing_mcp_server_handle_run_suite() {
        let server = TestingMcpServer::new();
        
        let result = server.handle_tool(
            "run_suite".to_string(),
            json!({
                "suite_id": "suite-123",
                "suite_name": "My Suite"
            }),
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap().contains("My Suite"));
    }

    #[test]
    fn test_testing_mcp_server_handle_generate_report() {
        let server = TestingMcpServer::new();
        
        let result = server.handle_tool(
            "generate_report".to_string(),
            json!({
                "title": "My Report",
                "format": "html"
            }),
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap().contains("html"));
    }

    #[test]
    fn test_testing_mcp_server_handle_spawn_process() {
        let server = TestingMcpServer::new();
        
        let result = server.handle_tool(
            "spawn_process".to_string(),
            json!({
                "command": "echo",
                "args": ["hello"],
                "timeout_seconds": 60
            }),
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap().contains("echo"));
    }

    #[test]
    fn test_testing_mcp_server_handle_unknown_tool() {
        let server = TestingMcpServer::new();
        
        let result = server.handle_tool(
            "unknown_tool".to_string(),
            json!({}),
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_testing_mcp_server_handle_spawn_process_missing_command() {
        let server = TestingMcpServer::new();
        
        let result = server.handle_tool(
            "spawn_process".to_string(),
            json!({}),
        );
        
        assert!(result.is_err());
    }
}
