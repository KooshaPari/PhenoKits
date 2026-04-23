# Hash Functions
#
# Traces to: FR-CIPHER-004

Feature: Cryptographic Hashing
  As a developer
  I want to hash data
  So that I can verify integrity and create identifiers

  Background:
    Given a test dataset

  Scenario: SHA-256 hash
    Given data "Hello"
    When I compute SHA-256
    Then the result should be 32 bytes
    And should match known SHA-256 value

  Scenario: BLAKE3 hash
    Given data "Hello"
    When I compute BLAKE3
    Then the result should be 32 bytes
    And should match known BLAKE3 value

  Scenario: Hash uniqueness
    Given data A = "test1"
    And data B = "test2"
    When I hash both
    Then the hashes should be different

  Scenario: Small input change, large output change (avalanche effect)
    Given input "abc"
    And input "abd" (differs by 1 bit)
    When I hash both
    Then the two hashes should differ by approximately 50% of bits

  Scenario: Hash of empty input
    Given empty input ""
    When I compute SHA-256
    Then the result should be the known empty string hash

  Scenario: Large data hashing
    Given 1GB of random data
    When I compute BLAKE3
    Then it should complete in under 10 seconds
    And the result should be 32 bytes

  @security
  Scenario: Collision resistance (property test)
    Given 100,000 random inputs
    When I compute hashes
    Then all hashes should be unique (no collisions)
