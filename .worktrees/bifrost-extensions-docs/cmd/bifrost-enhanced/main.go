// Package main provides the entry point for the enhanced Bifrost server
// with intelligent routing, learning, and smart fallback capabilities.
package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	bifrost "github.com/maximhq/bifrost/core"
	"github.com/maximhq/bifrost/core/schemas"

	"github.com/kooshapari/BifrostGo/account"
	"github.com/kooshapari/BifrostGo/plugins/nexus"
	"github.com/kooshapari/BifrostGo/plugins/cortex"
	"github.com/kooshapari/BifrostGo/plugins/raft"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Handle shutdown signals
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	go func() {
		<-sigChan
		log.Println("Shutting down...")
		cancel()
	}()

	// Create enhanced account
	acct := account.NewEnhancedAccount(nil)

	// Add standard providers from environment
	if key := os.Getenv("OPENAI_API_KEY"); key != "" {
		acct.AddStandardProvider(schemas.OpenAI, key, []string{})
	}
	if key := os.Getenv("ANTHROPIC_API_KEY"); key != "" {
		acct.AddStandardProvider(schemas.Anthropic, key, []string{})
	}
	if key := os.Getenv("GOOGLE_API_KEY"); key != "" {
		acct.AddStandardProvider(schemas.Gemini, key, []string{})
	}

	// Add extension providers
	acct.AddAgentCLIProvider("claude", 3284)
	acct.AddAgentCLIProvider("cursor", 3285)
	acct.AddAgentCLIProvider("auggie", 3286)
	acct.AddOAuthProxyProvider("claude", 8080)
	acct.AddOAuthProxyProvider("codex", 8081)

	// Create plugins
	routerPlugin := intelligentrouter.New(intelligentrouter.DefaultConfig())
	learningPlugin := learning.New(learning.DefaultConfig())
	fallbackPlugin := smartfallback.New(smartfallback.DefaultConfig())

	// Start learning plugin background processes
	learningPlugin.Start(ctx)

	// Initialize Bifrost with plugins
	bf, err := bifrost.Init(ctx, schemas.BifrostConfig{
		Account: acct,
		Plugins: []schemas.Plugin{
			routerPlugin,
			learningPlugin,
			fallbackPlugin,
		},
		Logger:          bifrost.NewDefaultLogger(schemas.LogLevelInfo),
		InitialPoolSize: 100,
	})
	if err != nil {
		log.Fatalf("Failed to initialize Bifrost: %v", err)
	}

	fmt.Println("Enhanced Bifrost initialized successfully!")
	fmt.Println("Plugins loaded:")
	fmt.Printf("  - %s (intelligent routing)\n", routerPlugin.GetName())
	fmt.Printf("  - %s (performance learning)\n", learningPlugin.GetName())
	fmt.Printf("  - %s (smart fallback)\n", fallbackPlugin.GetName())

	// Wait for shutdown
	<-ctx.Done()

	// Cleanup
	bf.Shutdown()

	fmt.Println("Shutdown complete")
}

