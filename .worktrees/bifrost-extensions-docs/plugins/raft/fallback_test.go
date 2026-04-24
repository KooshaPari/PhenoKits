package smartfallback

import (
	"context"
	"testing"
	"time"

	"github.com/maximhq/bifrost/core/schemas"
)

func TestNewPlugin(t *testing.T) {
	cfg := DefaultConfig()
	p := New(cfg)

	if p == nil {
		t.Fatal("New returned nil")
	}
	if p.GetName() != "smart-fallback" {
		t.Errorf("expected name 'smart-fallback', got %s", p.GetName())
	}
}

func TestPluginPreHook(t *testing.T) {
	cfg := DefaultConfig()
	p := New(cfg)

	ctx := context.Background()
	content := "write a function to sort an array"
	req := &schemas.BifrostRequest{
		ChatRequest: &schemas.BifrostChatRequest{
			Model: "gpt-4",
			Input: []schemas.ChatMessage{
				{
					Role:    schemas.ChatMessageRoleUser,
					Content: &schemas.ChatMessageContent{ContentStr: &content},
				},
			},
		},
	}

	result, shortCircuit, err := p.PreHook(&ctx, req)
	if err != nil {
		t.Fatalf("PreHook returned error: %v", err)
	}
	if shortCircuit != nil {
		t.Error("PreHook should not short-circuit")
	}
	if result == nil {
		t.Error("PreHook should return a request")
	}
}

func TestPluginPostHook(t *testing.T) {
	cfg := DefaultConfig()
	p := New(cfg)

	ctx := context.Background()
	resp := &schemas.BifrostResponse{
		ChatResponse: &schemas.BifrostChatResponse{
			ID: "test-id",
		},
	}

	result, bifrostErr, err := p.PostHook(&ctx, resp, nil)
	if err != nil {
		t.Fatalf("PostHook returned error: %v", err)
	}
	if bifrostErr != nil {
		t.Error("PostHook should not return BifrostError on success")
	}
	if result != resp {
		t.Error("PostHook should return the same response")
	}
}

func TestPluginCleanup(t *testing.T) {
	cfg := DefaultConfig()
	p := New(cfg)

	err := p.Cleanup()
	if err != nil {
		t.Errorf("Cleanup returned error: %v", err)
	}
}

func TestExponentialBackoff(t *testing.T) {
	cfg := DefaultConfig()
	b := NewExponentialBackoffStrategy("test", cfg)

	// Initially no delay (wait time for attempt 0 is min)
	delay := b.GetWaitTime(0)
	if delay != cfg.GeminiBackoffMin && delay != 100*time.Millisecond {
		t.Errorf("expected min delay initially, got %v", delay)
	}
}

func TestBudgetStrategy(t *testing.T) {
	cfg := DefaultConfig()
	budget := 100.0
	s := NewBudgetAwareStrategy(cfg, budget)

	s.UpdateBudget(60.0)
	usage := s.GetBudgetUsage()
	if usage != 0.6 {
		t.Errorf("expected 0.6 usage, got %f", usage)
	}

	if s.IsBlocked() {
		t.Error("should not be blocked at 60% usage")
	}

	s.UpdateBudget(50.0)
	if !s.IsBlocked() {
		t.Error("should be blocked at >100% usage")
	}
}

func TestTaskRuleEngine(t *testing.T) {
	engine := NewTaskRuleEngine()

	// Test code generation classification
	content := "implement a sorting algorithm"
	req := &schemas.BifrostRequest{
		ChatRequest: &schemas.BifrostChatRequest{
			Model: "gpt-4",
			Input: []schemas.ChatMessage{
				{
					Role:    schemas.ChatMessageRoleUser,
					Content: &schemas.ChatMessageContent{ContentStr: &content},
				},
			},
		},
	}

	taskType := engine.ClassifyTask(req)
	if taskType != TaskTypeCodeGen {
		t.Errorf("expected TaskTypeCodeGen, got %v", taskType)
	}

	fallbacks := engine.GetFallbackModels(taskType)
	if len(fallbacks) == 0 {
		t.Error("expected fallbacks for code generation task")
	}
}

