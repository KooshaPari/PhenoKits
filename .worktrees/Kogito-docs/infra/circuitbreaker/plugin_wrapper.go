// Package circuitbreaker provides plugin wrapper with circuit breaker protection
package circuitbreaker

import (
	"context"
	"fmt"
	"sync"

	"github.com/maximhq/bifrost/core/schemas"
)

// PluginWrapper wraps a plugin with circuit breaker protection
type PluginWrapper struct {
	plugin         schemas.Plugin
	circuitBreaker *CircuitBreaker
	mu             sync.RWMutex
}

// NewPluginWrapper creates a new plugin wrapper with circuit breaker
func NewPluginWrapper(plugin schemas.Plugin, config *Config) *PluginWrapper {
	name := fmt.Sprintf("plugin-%s", plugin.GetName())
	cb := New(name, config)
	return &PluginWrapper{
		plugin:         plugin,
		circuitBreaker: cb,
	}
}

// GetName returns the plugin name
func (pw *PluginWrapper) GetName() string {
	return pw.plugin.GetName()
}

// TransportInterceptor wraps plugin TransportInterceptor with circuit breaker
func (pw *PluginWrapper) TransportInterceptor(
	ctx *context.Context,
	url string,
	headers map[string]string,
	body map[string]any,
) (map[string]string, map[string]any, error) {
	var resultHeaders map[string]string
	var resultBody map[string]any
	var err error

	// Execute with circuit breaker protection
	cbErr := pw.circuitBreaker.Execute(*ctx, func() error {
		resultHeaders, resultBody, err = pw.plugin.TransportInterceptor(ctx, url, headers, body)
		return err
	})

	if cbErr != nil {
		// Circuit breaker is open - return original headers/body without modification
		// This allows graceful degradation
		return headers, body, nil
	}

	return resultHeaders, resultBody, err
}

// PreHook wraps plugin PreHook with circuit breaker
func (pw *PluginWrapper) PreHook(
	ctx *context.Context,
	req *schemas.BifrostRequest,
) (*schemas.BifrostRequest, *schemas.PluginShortCircuit, error) {
	var resultReq *schemas.BifrostRequest
	var shortCircuit *schemas.PluginShortCircuit
	var err error

	// Execute with circuit breaker protection
	cbErr := pw.circuitBreaker.Execute(*ctx, func() error {
		resultReq, shortCircuit, err = pw.plugin.PreHook(ctx, req)
		return err
	})

	// cbErr can be either:
	// 1. Circuit breaker is open (circuit breaker error)
	// 2. Plugin error (from the function)
	// We need to distinguish between these cases
	if cbErr != nil {
		// Check if this is a circuit breaker error (circuit is open)
		if cbErr.Error() == fmt.Sprintf("circuit breaker %s is open", pw.circuitBreaker.name) {
			// Circuit breaker is open - return original request without modification
			// This allows graceful degradation
			return req, nil, nil
		}
		// This is a plugin error - return it so FailFast can work
		return resultReq, shortCircuit, cbErr
	}

	// No circuit breaker error, return plugin result
	return resultReq, shortCircuit, err
}

// PostHook wraps plugin PostHook with circuit breaker
func (pw *PluginWrapper) PostHook(
	ctx *context.Context,
	resp *schemas.BifrostResponse,
	err *schemas.BifrostError,
) (*schemas.BifrostResponse, *schemas.BifrostError, error) {
	var resultResp *schemas.BifrostResponse
	var resultErr *schemas.BifrostError
	var hookErr error

	// Execute with circuit breaker protection
	cbErr := pw.circuitBreaker.Execute(*ctx, func() error {
		resultResp, resultErr, hookErr = pw.plugin.PostHook(ctx, resp, err)
		// Consider plugin error as failure for circuit breaker
		if hookErr != nil {
			return hookErr
		}
		// Also consider bifrost errors as failures
		if resultErr != nil && resultErr.StatusCode != nil {
			statusCode := *resultErr.StatusCode
			// 5xx errors are failures
			if statusCode >= 500 && statusCode < 600 {
				return fmt.Errorf("plugin returned 5xx error: %d", statusCode)
			}
		}
		return nil
	})

	if cbErr != nil {
		// Circuit breaker is open - return original response/error without modification
		// This allows graceful degradation
		return resp, err, nil
	}

	return resultResp, resultErr, hookErr
}

// Cleanup wraps plugin Cleanup
func (pw *PluginWrapper) Cleanup() error {
	return pw.plugin.Cleanup()
}

// GetCircuitBreaker returns the underlying circuit breaker
func (pw *PluginWrapper) GetCircuitBreaker() *CircuitBreaker {
	return pw.circuitBreaker
}

// WrapPlugins wraps multiple plugins with circuit breaker protection
func WrapPlugins(plugins []schemas.Plugin, config *Config) []schemas.Plugin {
	wrapped := make([]schemas.Plugin, 0, len(plugins))
	for _, plugin := range plugins {
		wrapped = append(wrapped, NewPluginWrapper(plugin, config))
	}
	return wrapped
}
