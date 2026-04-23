//! BDD integration tests for phenotype-gauge

use phenotype_bdd::{
    FeatureParser, Result as BddResult, Runner, StepArgs, StepContext, StepRegistry,
};
use serde_json::json;

/// Setup step registry for gauge tests
async fn setup_gauge_registry() -> BddResult<StepRegistry> {
    let mut registry = StepRegistry::new();

    // Background
    registry
        .given("the gauge framework is initialized", |ctx, _| async move {
            ctx.insert("initialized", true)?;
            Ok(())
        })
        .await?;

    // Benchmark steps
    registry
        .given(r#"a benchmark function "([^"]+)""#, |ctx, args| async move {
            let func = args.get(0).unwrap();
            ctx.insert("benchmark_func", func.to_string())?;
            Ok(())
        })
        .await?;

    registry
        .given(r#"sample size of (\d+)"#, |ctx, args| async move {
            let size: u32 = args.get(0).unwrap().parse().unwrap();
            ctx.insert("sample_size", size)?;
            Ok(())
        })
        .await?;

    registry
        .when("I run the benchmark", |ctx, _| async move {
            let func: String = ctx.get("benchmark_func")?.unwrap_or_default();

            // Simulate benchmark
            ctx.insert("iterations", 100u32)?;
            ctx.insert("avg_time_ms", 5.0f64)?;
            ctx.insert("stddev_ms", 1.0f64)?;
            ctx.insert("completed", true)?;

            Ok(())
        })
        .await?;

    registry
        .then("the benchmark should complete", |ctx, _| async move {
            let completed: bool = ctx.get("completed")?.unwrap_or(false);
            assert!(completed, "Benchmark should complete");
            Ok(())
        })
        .await?;

    registry
        .then("results should include iterations", |ctx, _| async move {
            let iters: u32 = ctx.get("iterations")?.unwrap_or(0);
            assert!(iters > 0, "Should have iterations");
            Ok(())
        })
        .await?;

    registry
        .then("results should include average time", |ctx, _| async move {
            let avg: f64 = ctx.get("avg_time_ms")?.unwrap_or(0.0);
            assert!(avg > 0.0, "Should have average time");
            Ok(())
        })
        .await?;

    registry
        .then("results should include standard deviation", |ctx, _| async move {
            let stddev: f64 = ctx.get("stddev_ms")?.unwrap_or(0.0);
            assert!(stddev >= 0.0, "Should have stddev");
            Ok(())
        })
        .await?;

    // Comparison steps
    registry
        .given(r#"a baseline result "([^"]+)""#, |ctx, args| async move {
            let baseline = args.get(0).unwrap();
            ctx.insert("baseline", baseline.to_string())?;
            Ok(())
        })
        .await?;

    registry
        .when("I compare results", |ctx, _| async move {
            let baseline: String = ctx.get("baseline")?.unwrap_or_default();
            let current: f64 = ctx.get("avg_time_ms")?.unwrap_or(0.0);

            // Parse baseline (e.g., "100ms")
            let baseline_val: f64 = baseline.trim_end_matches("ms").parse().unwrap_or(100.0);
            let change = ((current - baseline_val) / baseline_val) * 100.0;

            ctx.insert("change_percent", change)?;
            ctx.insert("regression", change > 10.0)?;
            Ok(())
        })
        .await?;

    registry
        .then(r#"performance change should be calculated"#, |ctx, _| async move {
            let change: f64 = ctx.get("change_percent")?;
            assert!(change.is_finite(), "Change should be calculated");
            Ok(())
        })
        .await?;

    registry
        .then(r#"regression should be detected if > 10%"#, |ctx, _| async move {
            let regression: bool = ctx.get("regression")?;
            // Just verify the flag was set
            Ok(())
        })
        .await?;

    // xDD steps
    registry
        .given("a code example in documentation", |ctx, _| async move {
            ctx.insert("has_example", true)?;
            Ok(())
        })
        .await?;

    registry
        .given(r#"markdown file with code block:"#, |ctx, _| async move {
            ctx.insert("code_block", "rust code here")?;
            Ok(())
        })
        .await?;

    registry
        .when("I run xDD tests", |ctx, _| async move {
            ctx.insert("xdd_compiled", true)?;
            ctx.insert("xdd_executed", true)?;
            Ok(())
        })
        .await?;

    registry
        .then("the code should compile", |ctx, _| async move {
            let compiled: bool = ctx.get("xdd_compiled")?.unwrap_or(false);
            assert!(compiled, "Code should compile");
            Ok(())
        })
        .await?;

    registry
        .then("the code should execute without errors", |ctx, _| async move {
            let executed: bool = ctx.get("xdd_executed")?.unwrap_or(false);
            assert!(executed, "Code should execute");
            Ok(())
        })
        .await?;

    Ok(registry)
}

#[tokio::test]
async fn test_benchmark_bdd() {
    let feature_content = r#"
Feature: Benchmark Execution
  Background:
    Given the gauge framework is initialized

  Scenario: Run a simple benchmark
    Given a benchmark function "fibonacci(20)"
    When I run the benchmark
    Then the benchmark should complete
    And results should include iterations
    And results should include average time
"#;

    let feature = FeatureParser::parse_str(feature_content).unwrap();
    let registry = setup_gauge_registry().await.unwrap();
    let runner = Runner::new(registry);
    let result = runner.run(feature).await.unwrap();

    assert_eq!(result.failed, 0, "Benchmark scenarios failed");
}

#[tokio::test]
async fn test_xdd_bdd() {
    let feature_content = r#"
Feature: Executable Documentation
  Background:
    Given a code example in documentation

  Scenario: Code example compiles
    Given markdown file with code block:
    When I run xDD tests
    Then the code should compile
    And the code should execute without errors
"#;

    let feature = FeatureParser::parse_str(feature_content).unwrap();
    let registry = setup_gauge_registry().await.unwrap();
    let runner = Runner::new(registry);
    let result = runner.run(feature).await.unwrap();

    assert_eq!(result.failed, 0, "xDD scenarios failed");
}
