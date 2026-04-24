package oauthproxy

import (
	"strings"
	"testing"
	"time"
)

func TestGeneratePKCE(t *testing.T) {
	pkce, err := GeneratePKCE()
	if err != nil {
		t.Fatalf("GeneratePKCE failed: %v", err)
	}

	if pkce.CodeVerifier == "" {
		t.Error("expected non-empty verifier")
	}
	if pkce.CodeChallenge == "" {
		t.Error("expected non-empty challenge")
	}
	if pkce.CodeVerifier == pkce.CodeChallenge {
		t.Error("verifier and challenge should be different")
	}
}

func TestClaudeOAuth(t *testing.T) {
	oauth := NewClaudeOAuth()

	if oauth.GetName() != "claude" {
		t.Errorf("expected name 'claude', got %s", oauth.GetName())
	}

	pkce, _ := GeneratePKCE()
	authURL, _ := oauth.GenerateAuthURL("test-state", pkce)

	if authURL == "" {
		t.Error("expected non-empty auth URL")
	}
	if !strings.Contains(authURL, "claude.ai") {
		t.Error("auth URL should contain claude.ai domain")
	}
	if !strings.Contains(authURL, "code_challenge=") {
		t.Error("auth URL should contain code_challenge")
	}
	if !strings.Contains(authURL, "state=test-state") {
		t.Error("auth URL should contain state")
	}
}

func TestCodexOAuth(t *testing.T) {
	oauth := NewCodexOAuth()

	if oauth.GetName() != "codex" {
		t.Errorf("expected name 'codex', got %s", oauth.GetName())
	}

	pkce, _ := GeneratePKCE()
	authURL, _ := oauth.GenerateAuthURL("test-state", pkce)

	if authURL == "" {
		t.Error("expected non-empty auth URL")
	}
	if !strings.Contains(authURL, "openai.com") {
		t.Error("auth URL should contain openai domain")
	}
}

func TestOAuthManager(t *testing.T) {
	tmpDir := t.TempDir()

	manager := NewOAuthManager(tmpDir)

	// Register provider
	claude := NewClaudeOAuth()
	manager.RegisterProvider(claude)

	// Load tokens (initially empty)
	tokens, err := manager.LoadTokens()
	if err != nil {
		t.Fatalf("LoadTokens failed: %v", err)
	}
	if len(tokens) != 0 {
		t.Error("expected zero tokens initially")
	}

	// Set token
	testToken := &TokenData{
		AccessToken:  "test-access-token",
		RefreshToken: "test-refresh-token",
		ExpiresAt:    time.Now().Add(time.Hour),
	}
	tokens["claude"] = testToken
	err = manager.SaveTokens(tokens)
	if err != nil {
		t.Fatalf("SaveTokens failed: %v", err)
	}

	// Load again and verify
	manager2 := NewOAuthManager(tmpDir)
	tokens2, err := manager2.LoadTokens()
	if err != nil {
		t.Fatalf("LoadTokens failed: %v", err)
	}
	retrieved := tokens2["claude"]
	if retrieved == nil {
		t.Fatal("expected token to be retrieved")
	}
	if retrieved.AccessToken != testToken.AccessToken {
		t.Errorf("expected access token %s, got %s", testToken.AccessToken, retrieved.AccessToken)
	}
}

func TestTokenDataExpiry(t *testing.T) {
	// Test expired token
	expiredToken := &TokenData{
		AccessToken: "expired",
		ExpiresAt:   time.Now().Add(-time.Hour),
	}
	if !expiredToken.IsExpired() {
		t.Error("token should be expired")
	}

	// Test valid token
	validToken := &TokenData{
		AccessToken: "valid",
		ExpiresAt:   time.Now().Add(time.Hour),
	}
	if validToken.IsExpired() {
		t.Error("token should not be expired")
	}
}

