package agentcli

import (
	"context"
	"testing"

	"github.com/maximhq/bifrost/core/schemas"
)

func TestDefaultConfig(t *testing.T) {
	cfg := DefaultConfig()

	if cfg.BaseURL != "http://localhost" {
		t.Errorf("expected base URL http://localhost, got %s", cfg.BaseURL)
	}
	if cfg.Port != 3284 {
		t.Errorf("expected port 3284, got %d", cfg.Port)
	}
	if cfg.AgentType != AgentTypeClaude {
		t.Errorf("expected default agent claude, got %s", cfg.AgentType)
	}
	if cfg.TerminalWidth != 80 {
		t.Errorf("expected term width 80, got %d", cfg.TerminalWidth)
	}
	if cfg.TerminalHeight != 1000 {
		t.Errorf("expected term height 1000, got %d", cfg.TerminalHeight)
	}
}

func TestNew(t *testing.T) {
	cfg := DefaultConfig()
	p := New(cfg)

	if p == nil {
		t.Fatal("New returned nil")
	}
	if p.GetProviderKey() != "agentcli-claude" {
		t.Errorf("expected provider key 'agentcli-claude', got %s", p.GetProviderKey())
	}
}

func TestChatCompletionNilRequest(t *testing.T) {
	cfg := DefaultConfig()
	p := New(cfg)

	ctx := context.Background()
	req := &schemas.BifrostRequest{
		ChatRequest: nil,
	}

	resp, err := p.ChatCompletion(ctx, req)
	if resp != nil {
		t.Error("expected nil response for nil ChatRequest")
	}
	if err == nil {
		t.Error("expected error for nil ChatRequest")
	}
	if err.StatusCode == nil || *err.StatusCode != 400 {
		t.Error("expected 400 status code")
	}
}

func TestBaseURL(t *testing.T) {
	cfg := DefaultConfig()
	p := New(cfg)

	if p == nil {
		t.Fatal("New returned nil")
	}
	expectedURL := "http://localhost:3284"
	if p.baseURL() != expectedURL {
		t.Errorf("expected base URL %s, got %s", expectedURL, p.baseURL())
	}
}

