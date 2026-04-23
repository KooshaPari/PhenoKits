Feature: Domain Behavior

  @FR-001 @smoke @critical
  Scenario: Entity creation succeeds
    Given a valid entity configuration
    When I create a new entity
    Then the entity should be persisted

  @FR-002 @validation @negative
  Scenario: Entity creation fails with invalid data
    Given an invalid entity configuration
    When I attempt to create the entity
    Then the operation should fail with validation error

  @FR-003 @integration
  Scenario: Entity workflow transitions correctly
    Given an entity in initial state
    When the entity receives a valid event
    Then the entity should transition to the expected state

  @FR-004 @security @critical
  Scenario: Unauthorized access is rejected
    Given an unauthenticated request
    When accessing protected resources
    Then the request should be denied with authentication error

  @FR-005 @performance
  Scenario Outline: System handles load efficiently
    Given <concurrent> concurrent requests
    When all requests are processed
    Then the p99 latency should be under <threshold>ms

    Examples:
      | concurrent | threshold |
      | 100        | 100       |
      | 1000       | 500       |
