# Service Registry BDD Tests
# Traces to: E1.1, E1.2, E1.3

Feature: Service Registration and Management
  As a platform engineer
  I want to register and manage service instances
  So that I can maintain an accurate service catalog

  Background:
    Given a clean service registry

  Scenario: Register a new service instance
    Given a service "user-service" running at "localhost:8080"
    When I register the service
    Then the service should be available in the registry
    And the service should be marked as healthy

  Scenario: Register multiple instances of the same service
    Given a service "api-gateway" with instances at "localhost:8081" and "localhost:8082"
    When I register both instances
    Then both instances should be discoverable
    And the service count should be 2

  Scenario: Deregister a service
    Given a registered service "cache-service" at "localhost:6379"
    When I deregister the service
    Then the service should not be found in the registry

  Scenario: Update service health status
    Given a registered service "db-service" at "localhost:5432"
    When I mark the service as unhealthy
    Then the service should be marked as unhealthy
    And the service should not appear in healthy discovery

  Scenario: Re-register an existing service
    Given a registered service "worker-service" at "localhost:9090"
    When I re-register the service at "localhost:9091"
    Then the service address should be updated to "localhost:9091"

  Scenario: List all registered services
    Given the following services are registered:
      | name          | address           |
      | auth-service  | localhost:8080    |
      | log-service   | localhost:8081    |
      | metrics-svc   | localhost:8082    |
    When I list all services
    Then I should see 3 services
    And the service names should include "auth-service", "log-service", "metrics-svc"
