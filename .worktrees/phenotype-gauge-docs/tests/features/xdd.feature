# Executable Documentation (xDD)
#
# Traces to: FR-GAUGE-003

Feature: Executable Documentation
  As a developer
  I want documentation that runs as tests
  So that examples never go out of date

  Background:
    Given a code example in documentation

  Scenario: Code example compiles
    Given markdown file with code block:
      """rust
      fn main() {
          println!("Hello");
      }
      """
    When I run xDD tests
    Then the code should compile
    And the code should execute without errors

  Scenario: Asserted code example
    Given markdown with asserted code:
      """rust
      assert_eq!(2 + 2, 4);
      """
    When I run xDD tests
    Then the assertion should pass

  Scenario: Hidden setup code
    Given markdown with hidden setup:
      <!-- # use my crate::prelude; -->
      ```rust
      let result = function_under_test();
      ```
    When I run xDD tests
    Then hidden code should be included in compilation
    But hidden code should not appear in documentation

  Scenario: Multiple code blocks
    Given a tutorial with sequential code blocks:
      | step | code |
      | 1    | init code |
      | 2    | use init result |
      | 3    | final result |
    When I run xDD tests
    Then all blocks should compile together
    And execution should flow sequentially

  @ignore
  Scenario: Ignored example
    Given markdown with ignored code block:
      ```rust,ignore
      // This won't compile but that's ok
      ```
    When I run xDD tests
    Then the block should be skipped
    And no error should be reported

  @should_panic
  Scenario: Expected panic
    Given markdown with should_panic:
      """rust,should_panic
      panic!("expected");
      """
    When I run xDD tests
    Then the panic should be expected
    And test should pass if panic occurs
