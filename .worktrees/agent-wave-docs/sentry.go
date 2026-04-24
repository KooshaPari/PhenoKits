package sentry

/**
 * Sentry configuration for agent-wave
 *
 * Traces to: FR-AGENTWAVE-SENTRY-001
 *
 * Error tracking for agent orchestration and wave management
 */

import (
	"os"

	"github.com/getsentry/sentry-go"
)

// Init initializes Sentry for the agent-wave service
func Init() error {
	dsn := os.Getenv("SENTRY_DSN")
	if dsn == "" {
		return nil // Sentry disabled
	}

	env := os.Getenv("SENTRY_ENVIRONMENT")
	if env == "" {
		env = "development"
	}

	release := os.Getenv("AGENT_WAVE_VERSION")
	if release == "" {
		release = "dev"
	}

	err := sentry.Init(sentry.ClientOptions{
		Dsn:              dsn,
		Environment:      env,
		Release:          release,
		AttachStacktrace: true,
		SampleRate:       1.0,
		TracesSampleRate: 0.1,
		BeforeSend: func(event *sentry.Event, hint *sentry.EventHint) *sentry.Event {
			// Filter out agent heartbeat noise
			if event.Message != "" && contains(event.Message, "heartbeat") {
				return nil
			}
			return event
		},
	})

	return err
}

// CaptureAgentError captures an error with agent context
func CaptureAgentError(err error, agentID string, waveID string) {
	sentry.WithScope(func(scope *sentry.Scope) {
		scope.SetTag("agent.id", agentID)
		scope.SetTag("wave.id", waveID)
		scope.SetLevel(sentry.LevelError)
		sentry.CaptureException(err)
	})
}

// CaptureWaveEvent captures a wave lifecycle event
func CaptureWaveEvent(waveID string, event string, metadata map[string]interface{}) {
	sentry.WithScope(func(scope *sentry.Scope) {
		scope.SetTag("wave.id", waveID)
		scope.SetTag("wave.event", event)
		scope.SetLevel(sentry.LevelInfo)
		
		for k, v := range metadata {
			scope.SetExtra(k, v)
		}
		
		sentry.CaptureMessage("Wave event: " + event)
	})
}

func contains(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr || (len(s) > len(substr) && containsAt(s, substr)))
}

func containsAt(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}
