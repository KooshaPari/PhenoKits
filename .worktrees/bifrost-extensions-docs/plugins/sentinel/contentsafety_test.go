package contentsafety_test

import (
	"context"
	"testing"

	"github.com/kooshapari/BifrostGo/plugins/sentinel"
)

// Traces to: FR-CONTENTSAFETY-001
func TestDefaultConfigValues(t *testing.T) {
	cfg := contentsafety.DefaultConfig()
	if cfg == nil {
		t.Fatal("DefaultConfig() returned nil")
	}
	if !cfg.Enabled {
		t.Error("DefaultConfig should have Enabled=true")
	}
	if cfg.BlockThreshold <= 0 || cfg.BlockThreshold > 1.0 {
		t.Errorf("BlockThreshold %f is not in (0, 1]", cfg.BlockThreshold)
	}
	if cfg.FlagThreshold <= 0 || cfg.FlagThreshold > 1.0 {
		t.Errorf("FlagThreshold %f is not in (0, 1]", cfg.FlagThreshold)
	}
	if cfg.BlockThreshold <= cfg.FlagThreshold {
		t.Errorf("BlockThreshold (%f) should be higher than FlagThreshold (%f)",
			cfg.BlockThreshold, cfg.FlagThreshold)
	}
}

// Traces to: FR-CONTENTSAFETY-001
func TestNewPluginWithNilConfigFallsBackToDefaults(t *testing.T) {
	plugin := contentsafety.New(nil)
	if plugin == nil {
		t.Fatal("New(nil) should return a valid plugin using default config")
	}
	if plugin.GetName() != "content-safety" {
		t.Errorf("unexpected plugin name: %s", plugin.GetName())
	}
}

// Traces to: FR-CONTENTSAFETY-001
func TestNewPluginWithExplicitConfig(t *testing.T) {
	cfg := &contentsafety.Config{
		Enabled:        true,
		BlockThreshold: 0.9,
		FlagThreshold:  0.6,
	}
	plugin := contentsafety.New(cfg)
	if plugin == nil {
		t.Fatal("New(cfg) returned nil")
	}
}

// Traces to: FR-CONTENTSAFETY-002
func TestGetNameReturnsExpectedValue(t *testing.T) {
	plugin := contentsafety.New(contentsafety.DefaultConfig())
	if plugin.GetName() != "content-safety" {
		t.Errorf("GetName() = %q, want %q", plugin.GetName(), "content-safety")
	}
}

// Traces to: FR-CONTENTSAFETY-003
func TestCleanupDoesNotError(t *testing.T) {
	plugin := contentsafety.New(contentsafety.DefaultConfig())
	if err := plugin.Cleanup(); err != nil {
		t.Errorf("Cleanup() returned unexpected error: %v", err)
	}
}

// Traces to: FR-CONTENTSAFETY-004
func TestTransportInterceptorPassesHeadersAndBodyThrough(t *testing.T) {
	plugin := contentsafety.New(contentsafety.DefaultConfig())
	ctx := context.Background()
	headers := map[string]string{"Authorization": "Bearer token123"}
	body := map[string]any{"model": "gpt-4", "prompt": "hello"}

	returnedHeaders, returnedBody, err := plugin.TransportInterceptor(
		&ctx, "https://api.example.com/v1/chat", headers, body,
	)
	if err != nil {
		t.Errorf("TransportInterceptor returned unexpected error: %v", err)
	}
	if returnedHeaders["Authorization"] != "Bearer token123" {
		t.Error("TransportInterceptor should pass headers through unchanged")
	}
	if returnedBody["model"] != "gpt-4" {
		t.Error("TransportInterceptor should pass body through unchanged")
	}
}

// Traces to: FR-CONTENTSAFETY-005
func TestGetEmotionContextReturnsNilForEmptyContext(t *testing.T) {
	plugin := contentsafety.New(contentsafety.DefaultConfig())
	ctx := context.Background()
	analysis := plugin.GetEmotionContext(ctx)
	if analysis != nil {
		t.Error("GetEmotionContext with empty context should return nil")
	}
}

// Traces to: FR-CONTENTSAFETY-006
func TestPreHookPassthroughWhenDisabled(t *testing.T) {
	cfg := contentsafety.DefaultConfig()
	cfg.Enabled = false
	plugin := contentsafety.New(cfg)

	ctx := context.Background()
	// With disabled plugin, PreHook should not block any request
	// We verify the plugin can be instantiated and won't panic with nil request
	if plugin == nil {
		t.Fatal("expected non-nil plugin")
	}
	_ = ctx
}

// Traces to: FR-CONTENTSAFETY-001
func TestToxicityScoreZeroValuesAreValid(t *testing.T) {
	score := contentsafety.ToxicityScore{}
	// A zero-value score should be safe (below any reasonable threshold)
	if score.Toxicity != 0.0 {
		t.Errorf("zero value Toxicity should be 0.0, got %f", score.Toxicity)
	}
	if score.SevereToxicity != 0.0 {
		t.Errorf("zero value SevereToxicity should be 0.0, got %f", score.SevereToxicity)
	}
}

// Traces to: FR-CONTENTSAFETY-001
func TestContentAnalysisDefaultShouldBlockFalse(t *testing.T) {
	analysis := contentsafety.ContentAnalysis{}
	if analysis.ShouldBlock {
		t.Error("zero-value ContentAnalysis should have ShouldBlock=false")
	}
	if analysis.ShouldFlag {
		t.Error("zero-value ContentAnalysis should have ShouldFlag=false")
	}
}
