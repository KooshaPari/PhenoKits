# Service Discovery BDD Tests
# Traces to: E2.1, E3.1, E4.1

Feature: Service Discovery
  As a client application
  I want to discover available service instances
  So that I can route requests to healthy services

  Background:
    Given a clean service registry with the following healthy services:
      | name         | address           | tags                    |
      | api-v1       | localhost:8001    | version=1.0,env=prod    |
      | api-v2       | localhost:8002    | version=2.0,env=prod    |
      | api-canary   | localhost:8003    | version=2.0,env=canary  |
      | worker-1     | localhost:9001    | pool=default            |
      | worker-2     | localhost:9002    | pool=default            |

  Scenario: Discover all instances of a service
    When I discover services named "api-v1"
    Then I should find 1 instance
    And the instance should be at "localhost:8001"

  Scenario: Round-robin load balancing
    Given 3 instances of "round-robin-svc" are registered
    When I discover the next instance 3 times
    Then each instance should be returned exactly once
    And the 4th discovery should return the first instance again

  Scenario: Discover by tag
    When I discover services with tag "env=prod"
    Then I should find 2 services
    And the services should be "api-v1" and "api-v2"

  Scenario: Discover by tag with specific value
    When I discover services with tag "version=2.0"
    Then I should find 2 services
    And the services should include "api-v2" and "api-canary"

  Scenario: No healthy instances available
    Given all instances of "api-v1" are marked unhealthy
    When I discover services named "api-v1"
    Then I should find 0 instances
    And round-robin discovery should return nothing

  Scenario: Discover worker pool by tag
    When I discover services with tag "pool=default"
    Then I should find 2 services
    And the addresses should be "localhost:9001" and "localhost:9002"

  Scenario: Empty discovery for non-existent tag
    When I discover services with tag "env=staging"
    Then I should find 0 services
