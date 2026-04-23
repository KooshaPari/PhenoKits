# Gauge Benchmarking Framework
#
# Traces to: FR-GAUGE-001, FR-GAUGE-002

Feature: Benchmark Execution
  As a developer
  I want to benchmark my code
  So that I can measure and improve performance

  Background:
    Given the gauge framework is initialized

  Scenario: Run a simple benchmark
    Given a benchmark function "fibonacci(20)"
    When I run the benchmark
    Then the benchmark should complete
    And results should include iterations
    And results should include average time
    And results should include standard deviation

  Scenario: Benchmark with statistical analysis
    Given a benchmark function "sort_array"
    And sample size of 100
    When I run the benchmark
    Then results should include:
      | metric | meaning |
      | mean   | average time |
      | median | 50th percentile |
      | stddev | variability |
      | min    | fastest time |
      | max    | slowest time |

  Scenario: Compare baseline vs current
    Given a baseline result "100ms"
    And a current benchmark result
    When I compare results
    Then performance change should be calculated
    And regression should be detected if > 10%

  Scenario: Outlier detection
    Given benchmark results with outliers
    When statistical analysis runs
    Then outliers should be identified
    And median should be robust to outliers

  Scenario: Warm-up iterations
    Given a benchmark with warm-up of 10 iterations
    When I run the benchmark
    Then warm-up should not count toward results
    And measured iterations should follow warm-up

  Scenario: Parameterized benchmark
    Given benchmark "matrix_multiply" with parameters:
      | size |
      | 10   |
      | 100  |
      | 1000 |
    When I run parameterized benchmarks
    Then each parameter should have separate results
    And results should show scaling behavior

  @regression
  Scenario: Detect performance regression
    Given historical baseline data
    And current benchmark run
    When I analyze trend
    Then regression > 20% should fail the build
    And improvement > 50% should be flagged for review

  @ci
  Scenario: CI-friendly output
    Given CI environment variable set
    When I run benchmark
    Then output should be in JSON format
    And results should include exit code on regression
