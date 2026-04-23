# Digital Signatures
#
# Traces to: FR-CIPHER-003

Feature: Ed25519 Digital Signatures
  As a security engineer
  I want to sign and verify messages
  So that I can ensure authenticity and integrity

  Background:
    Given the Ed25519 signature scheme

  Scenario: Sign and verify message
    Given an Ed25519 keypair
    And message "Important announcement"
    When I sign the message
    Then the signature should be 64 bytes
    And signature verification should succeed

  Scenario: Signature verification with wrong key
    Given a signed message
    And a different public key
    When I verify with the different key
    Then verification should fail

  Scenario: Modified message detection
    Given a signed message "Original"
    When an attacker changes it to "Modified"
    And I verify the signature
    Then verification should fail

  Scenario: Deterministic signatures
    Given a keypair
    And message "Test"
    When I sign twice
    Then both signatures should verify
    But signatures may be different (randomized)

  @performance
  Scenario: Batch verification
    Given 1000 signed messages
    When I batch verify them
    Then all valid signatures should be accepted
    And invalid signatures should be rejected
    And performance should exceed 10,000 ops/sec
