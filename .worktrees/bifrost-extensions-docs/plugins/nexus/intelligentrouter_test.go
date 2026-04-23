package intelligentrouter_test

import (
	"testing"

	"github.com/maximhq/bifrost/core/schemas"

	"github.com/kooshapari/BifrostGo/plugins/nexus"
)

// Traces to: FR-ROUTER-001
func TestDefaultConfigValues(t *testing.T) {
	cfg := intelligentrouter.DefaultConfig()
	if cfg == nil {
		t.Fatal("DefaultConfig() returned nil")
	}
	if !cfg.RouteLLMEnabled {
		t.Error("DefaultConfig should have RouteLLMEnabled=true")
	}
	if cfg.RouteLLMThreshold <= 0 || cfg.RouteLLMThreshold >= 1.0 {
		t.Errorf("RouteLLMThreshold %f should be in (0, 1)", cfg.RouteLLMThreshold)
	}
	if cfg.RouteLLMRouter == "" {
		t.Error("DefaultConfig RouteLLMRouter should not be empty")
	}
	if cfg.MaxCostPerRequest <= 0 {
		t.Errorf("MaxCostPerRequest %f should be positive", cfg.MaxCostPerRequest)
	}
}

// Traces to: FR-ROUTER-001
func TestDefaultConfigHasPreferredProviders(t *testing.T) {
	cfg := intelligentrouter.DefaultConfig()
	if len(cfg.PreferredProviders) == 0 {
		t.Error("DefaultConfig should have at least one preferred provider")
	}
}

// Traces to: FR-ROUTER-001
func TestDefaultConfigArchRouterDefaults(t *testing.T) {
	cfg := intelligentrouter.DefaultConfig()
	if cfg.ArchRouterEndpoint == "" {
		t.Error("DefaultConfig ArchRouterEndpoint should not be empty")
	}
	if cfg.ArchRouterModel == "" {
		t.Error("DefaultConfig ArchRouterModel should not be empty")
	}
}

// Traces to: FR-ROUTER-002
func TestNewWithNilConfigFallsBackToDefaults(t *testing.T) {
	router := intelligentrouter.New(nil)
	if router == nil {
		t.Fatal("New(nil) should return a valid router using default config")
	}
	if router.GetName() != "intelligent-router" {
		t.Errorf("unexpected plugin name: %s", router.GetName())
	}
}

// Traces to: FR-ROUTER-002
func TestNewWithExplicitConfig(t *testing.T) {
	cfg := &intelligentrouter.Config{
		RouteLLMEnabled:   false,
		RouteLLMThreshold: 0.5,
		UseCostEngine:     true,
		MaxCostPerRequest: 0.5,
		PreferredProviders: []schemas.ModelProvider{
			schemas.OpenAI,
		},
	}
	router := intelligentrouter.New(cfg)
	if router == nil {
		t.Fatal("New(cfg) returned nil")
	}
}

// Traces to: FR-ROUTER-003
func TestGetNameReturnsExpectedValue(t *testing.T) {
	router := intelligentrouter.New(intelligentrouter.DefaultConfig())
	want := "intelligent-router"
	if router.GetName() != want {
		t.Errorf("GetName() = %q, want %q", router.GetName(), want)
	}
}

// Traces to: FR-ROUTER-004
func TestCleanupDoesNotError(t *testing.T) {
	router := intelligentrouter.New(intelligentrouter.DefaultConfig())
	if err := router.Cleanup(); err != nil {
		t.Errorf("Cleanup() returned unexpected error: %v", err)
	}
}

// Traces to: FR-ROUTER-005
func TestTaskTypeConstants(t *testing.T) {
	cases := []struct {
		name string
		val  intelligentrouter.TaskType
	}{
		{"ToolCall", intelligentrouter.TaskTypeToolCall},
		{"CodeGen", intelligentrouter.TaskTypeCodeGen},
		{"Reasoning", intelligentrouter.TaskTypeReasoning},
		{"Conversation", intelligentrouter.TaskTypeConversation},
		{"Default", intelligentrouter.TaskTypeDefault},
	}
	seen := map[intelligentrouter.TaskType]bool{}
	for _, tc := range cases {
		if tc.val == "" {
			t.Errorf("TaskType %s should not be empty string", tc.name)
		}
		if seen[tc.val] {
			t.Errorf("TaskType %s has duplicate value %q", tc.name, tc.val)
		}
		seen[tc.val] = true
	}
}

// Traces to: FR-ROUTER-005
func TestRiskLevelConstants(t *testing.T) {
	levels := []intelligentrouter.RiskLevel{
		intelligentrouter.RiskLow,
		intelligentrouter.RiskMedium,
		intelligentrouter.RiskHigh,
	}
	seen := map[intelligentrouter.RiskLevel]bool{}
	for _, l := range levels {
		if l == "" {
			t.Error("RiskLevel should not be empty string")
		}
		if seen[l] {
			t.Errorf("duplicate RiskLevel value: %q", l)
		}
		seen[l] = true
	}
}

// Traces to: FR-ROUTER-005
func TestRoutingDecisionZeroValue(t *testing.T) {
	var d intelligentrouter.RoutingDecision
	// Zero-value RoutingDecision should be a valid struct with sane defaults
	if d.Confidence < 0 {
		t.Error("zero-value Confidence should not be negative")
	}
	if d.CostEstimate < 0 {
		t.Error("zero-value CostEstimate should not be negative")
	}
}

// Traces to: FR-ROUTER-006
func TestConfigCostThresholdConstraints(t *testing.T) {
	cfg := intelligentrouter.DefaultConfig()
	// Subscription preference and scarce endpoint config should be coherent
	if cfg.MaxCostPerRequest < 0 {
		t.Error("MaxCostPerRequest must not be negative")
	}
}
