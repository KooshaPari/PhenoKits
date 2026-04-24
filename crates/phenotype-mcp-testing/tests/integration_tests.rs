//! Integration tests for phenotype-mcp-testing

use phenotype_mcp_testing::*;
use phenotype_mcp_framework::McpServer;

#[tokio::test]
async fn test_end_to_end_test_execution() {
    let mut handler = TestingHandler::new();
    
    // Create a test suite with multiple tests
    let suite = TestSuite::new("integration-suite", "Integration Tests")
        .add_test(TestCase::new("test-1", "First Test", TestType::Unit))
        .add_test(TestCase::new("test-2", "Second Test", TestType::Integration))
        .add_test(
            TestCase::new("test-3", "Third Test", TestType::E2E)
                .with_config(TestConfig::default().with_timeout(120))
        );
    
    // Run the suite
    let result = handler.run_suite(&suite).await;
    
    // Verify results
    assert_eq!(result.total_count(), 3);
    assert_eq!(result.passed_count(), 3);
    assert!(result.all_passed());
    assert!(result.pass_rate() > 99.0);
}

#[tokio::test]
async fn test_process_management_integration() {
    let mut manager = GameProcessManager::new();
    
    // Spawn multiple processes
    let proc1 = manager.spawn_mock(
        "test-process-1",
        vec!["--verbose".to_string()],
        std::collections::HashMap::new(),
        0,
    ).await.unwrap();
    
    let proc2 = manager.spawn_mock(
        "test-process-2",
        vec!["--quick".to_string()],
        std::collections::HashMap::new(),
        0,
    ).await.unwrap();
    
    // List all processes
    let processes = manager.list_processes();
    assert_eq!(processes.len(), 2);
    
    // Verify we can retrieve individual processes
    assert!(manager.get_process(proc1.pid).is_some());
    assert!(manager.get_process(proc2.pid).is_some());
    
    // Cleanup
    manager.cleanup();
    assert!(manager.list_processes().is_empty());
}

#[tokio::test]
async fn test_report_generation_workflow() {
    let mut handler = TestingHandler::new();
    let gen = ReportGenerator::new();
    
    // Run multiple suites
    let suite1 = TestSuite::new("suite-1", "Feature A Tests")
        .add_test(TestCase::new("a1", "Test A1", TestType::Unit).with_tag("fast"))
        .add_test(TestCase::new("a2", "Test A2", TestType::Unit).with_tag("slow"));
    
    let suite2 = TestSuite::new("suite-2", "Feature B Tests")
        .add_test(TestCase::new("b1", "Test B1", TestType::Integration))
        .add_test(TestCase::new("b2", "Test B2", TestType::Integration));
    
    let result1 = handler.run_suite(&suite1).await;
    let result2 = handler.run_suite(&suite2).await;
    
    // Generate report in different formats
    let report = gen.generate("Integration Test Report", vec![result1, result2]);
    
    // JSON format
    let json = gen.to_json(&report).unwrap();
    assert!(json.contains("Integration Test Report"));
    assert!(json.contains("Feature A Tests"));
    assert!(json.contains("Feature B Tests"));
    
    // HTML format
    let html = gen.to_html(&report);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Integration Test Report"));
    
    // Markdown format
    let md = gen.to_markdown(&report);
    assert!(md.contains("# Integration Test Report"));
    
    // JUnit format
    let junit = gen.to_junit(&report);
    assert!(junit.contains("<testsuites"));
    assert!(junit.contains("<testsuite"));
}

#[tokio::test]
async fn test_mcp_server_integration() {
    let server = TestingMcpServer::new();
    
    // Verify server info
    assert_eq!(server.name(), "phenotype-testing");
    assert!(!server.version().is_empty());
    
    // Verify tools
    let tools = server.tools();
    assert!(!tools.is_empty());
    
    // Test tool handling
    let run_test_result = server.handle_tool(
        "run_test".to_string(),
        serde_json::json!({
            "id": "integration-test",
            "name": "Integration Test",
            "type": "integration"
        }),
    );
    assert!(run_test_result.is_ok());
}

#[tokio::test]
async fn test_performance_test_execution() {
    let mut runner = TestRunner::new();
    
    let test = TestCase::new("perf-test", "Performance Test", TestType::Performance)
        .with_config(TestConfig::default()
            .with_timeout(30)
            .with_command("echo", vec!["test".to_string()])
        );
    
    let result = runner.run_test(&test).await;
    
    assert!(result.is_passed());
    
    // Check performance metrics
    assert!(result.metrics.contains_key("avg_duration_ms"));
    assert!(result.metrics.contains_key("min_duration_ms"));
    assert!(result.metrics.contains_key("max_duration_ms"));
    
    let avg = result.metrics.get("avg_duration_ms").unwrap();
    assert!(*avg >= 0.0);
}

#[tokio::test]
async fn test_load_test_execution() {
    let mut runner = TestRunner::new();
    
    let test = TestCase::new("load-test", "Load Test", TestType::Load)
        .with_config(TestConfig::default().with_timeout(3)); // 3 seconds
    
    let result = runner.run_test(&test).await;
    
    assert!(result.is_passed());
    
    // Check load metrics
    assert!(result.metrics.contains_key("total_requests"));
    assert!(result.metrics.contains_key("requests_per_second"));
    
    let rps = result.metrics.get("requests_per_second").unwrap();
    assert!(*rps > 0.0);
}

#[tokio::test]
async fn test_suite_with_setup_teardown() {
    let mut runner = TestRunner::new();
    
    let suite = TestSuite::new("setup-suite", "Suite with Setup")
        .with_setup(
            TestConfig::default()
                .with_command("echo", vec!["setup".to_string()])
                .with_timeout(5)
        )
        .with_teardown(
            TestConfig::default()
                .with_command("echo", vec!["teardown".to_string()])
                .with_timeout(5)
        )
        .add_test(TestCase::new("t1", "Test 1", TestType::Unit))
        .add_test(TestCase::new("t2", "Test 2", TestType::Unit));
    
    let result = runner.run_suite(&suite).await;
    
    assert!(result.all_passed());
    assert!(result.setup_result.is_some());
    assert!(result.teardown_result.is_some());
    
    // Setup and teardown should have passed
    if let Some(setup) = &result.setup_result {
        assert!(setup.is_passed());
    }
    if let Some(teardown) = &result.teardown_result {
        assert!(teardown.is_passed());
    }
}

#[tokio::test]
async fn test_test_result_states() {
    // Test various result states
    let passed = TestResult::new("t1", "Test 1").passed();
    assert!(passed.is_passed());
    assert!(!passed.is_failed());
    
    let failed = TestResult::new("t2", "Test 2").failed("Something broke");
    assert!(!failed.is_passed());
    assert!(failed.is_failed());
    assert_eq!(failed.error_message, Some("Something broke".to_string()));
    
    let error = TestResult::new("t3", "Test 3").error("Unexpected error");
    assert!(!error.is_passed());
    assert!(error.is_failed()); // Error counts as failed
    
    let skipped = TestResult::new("t4", "Test 4").skipped();
    assert_eq!(skipped.status, TestStatus::Skipped);
    
    let timeout = TestResult::new("t5", "Test 5");
    let timeout = TestResult {
        status: TestStatus::Timeout,
        ..timeout
    };
    assert_eq!(timeout.status, TestStatus::Timeout);
}

#[tokio::test]
async fn test_suite_result_statistics() {
    let mut suite = SuiteResult::new("stat-suite", "Stats Suite");
    
    suite.add_result(TestResult::new("t1", "Test 1").passed());
    suite.add_result(TestResult::new("t2", "Test 2").passed());
    suite.add_result(TestResult::new("t3", "Test 3").failed("error"));
    suite.add_result(TestResult::new("t4", "Test 4").skipped());
    
    let suite = suite.complete();
    
    assert_eq!(suite.total_count(), 4);
    assert_eq!(suite.passed_count(), 2);
    assert_eq!(suite.failed_count(), 1);
    assert_eq!(suite.skipped_count(), 1);
    assert!(!suite.all_passed());
    assert_eq!(suite.pass_rate(), 50.0);
}

#[tokio::test]
async fn test_concurrent_test_execution() {
    let mut runner = TestRunner::new();
    
    // Create many tests
    let suite = TestSuite::new("concurrent-suite", "Concurrent Tests");
    let suite = (0..50).fold(suite, |suite, i| {
        suite.add_test(TestCase::new(
            &format!("test-{}", i),
            &format!("Test {}", i),
            TestType::Unit
        ))
    });
    
    let result = runner.run_suite(&suite).await;
    
    assert_eq!(result.total_count(), 50);
    assert_eq!(result.passed_count(), 50);
    assert!(result.all_passed());
}
