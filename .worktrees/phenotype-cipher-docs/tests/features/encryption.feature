# Encryption
#
# Traces to: FR-CIPHER-001, FR-CIPHER-002

Feature: AEAD Encryption
  As a security engineer
  I want to encrypt and decrypt data
  So that sensitive information remains confidential

  Background:
    Given a secure random number generator

  Scenario: AES-GCM encryption round-trip
    Given a 256-bit AES key
    And plaintext data "Hello, World!"
    When I encrypt with AES-GCM
    Then the ciphertext should be different from plaintext
    And the ciphertext should have authentication tag
    When I decrypt the ciphertext
    Then the decrypted data should be "Hello, World!"

  Scenario: ChaCha20-Poly1305 encryption round-trip
    Given a 256-bit ChaCha20 key
    And plaintext data "Secret message"
    When I encrypt with ChaCha20-Poly1305
    Then the ciphertext should be different from plaintext
    When I decrypt the ciphertext
    Then the decrypted data should be "Secret message"

  Scenario: Different keys produce different ciphertexts
    Given plaintext data "Same data"
    And key A
    And key B (different from key A)
    When I encrypt with key A
    And I encrypt with key B
    Then the two ciphertexts should be different

  Scenario: Different nonces produce different ciphertexts
    Given a fixed key
    And plaintext data "Test data"
    When I encrypt with nonce 1
    And I encrypt with nonce 2
    Then the two ciphertexts should be different

  Scenario: Empty plaintext encryption
    Given a 256-bit AES key
    And empty plaintext ""
    When I encrypt with AES-GCM
    Then the ciphertext should not be empty
    When I decrypt the ciphertext
    Then the result should be empty ""

  Scenario: Large data encryption
    Given a 256-bit AES key
    And 1MB of random plaintext
    When I encrypt with AES-GCM
    Then the ciphertext size should be approximately 1MB + overhead
    When I decrypt the ciphertext
    Then the decrypted data should match original

  @security
  Scenario: Tampered ciphertext detection
    Given encrypted data with AES-GCM
    When an attacker flips a bit in the ciphertext
    And I attempt to decrypt
    Then decryption should fail with authentication error

  @security
  Scenario: Wrong key decryption failure
    Given data encrypted with key A
    When I attempt to decrypt with key B
    Then decryption should fail
    And no partial plaintext should be revealed
