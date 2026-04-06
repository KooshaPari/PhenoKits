package server

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"sync"
	"time"

	"github.com/maximhq/bifrost/core/schemas"
	agentapi "github.com/kooshapari/bifrost-extensions/wrappers/agentapi"
)

// AgentClient wraps the agentapi client for server use
type AgentClient struct {
	client   *agentapi.Client
	config   agentapi.Config
	mu       sync.RWMutex
	messages []AgentMessage
}

// AgentMessage represents a message in the agent conversation
type AgentMessage struct {
	ID        string    `json:"id"`
	Role      string    `json:"role"`
	Content   string    `json:"content"`
	Timestamp time.Time `json:"timestamp"`
}

// newAgentClient creates a new agent client
func newAgentClient(cfg agentapi.Config) (*AgentClient, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	client, err := agentapi.NewClient(ctx, cfg)
	if err != nil {
		return nil, err
	}

	return &AgentClient{
		client:   client,
		config:   cfg,
		messages: []AgentMessage{},
	}, nil
}

// AddMessage adds a message to the history
func (ac *AgentClient) AddMessage(msg AgentMessage) {
	ac.mu.Lock()
	defer ac.mu.Unlock()
	ac.messages = append(ac.messages, msg)
}

// GetMessages returns all messages
func (ac *AgentClient) GetMessages() []AgentMessage {
	ac.mu.RLock()
	defer ac.mu.RUnlock()
	return ac.messages
}

// GetMessageCount returns the number of messages
func (ac *AgentClient) GetMessageCount() int {
	ac.mu.RLock()
	defer ac.mu.RUnlock()
	return len(ac.messages)
}

// EmitEvent emits an event to the agent
func (ac *AgentClient) EmitEvent(eventType agentapi.EventType, data interface{}) {
	if ac.client != nil {
		ac.client.OnEvent(func(event agentapi.Event) {})
	}
}

// OnEvent registers an event handler
func (ac *AgentClient) OnEvent(handler agentapi.EventHandler) {
	if ac.client != nil {
		ac.client.OnEvent(handler)
	}
}

// Agent API handlers

// handleAgentStatus returns the agent status
func (s *Server) handleAgentStatus(w http.ResponseWriter, r *http.Request) {
	s.mu.RLock()
	agentStatus := "stable"
	messageCount := 0

	if s.agentClient != nil {
		metrics := s.agentClient.client.GetMetrics()
		if metrics.ErrorCount > 0 {
			agentStatus = "degraded"
		}
		messageCount = s.agentClient.GetMessageCount()
	}
	s.mu.RUnlock()

	s.writeJSON(w, http.StatusOK, map[string]interface{}{
		"status":    agentStatus,
		"messages":  messageCount,
		"timestamp": time.Now().Format(time.RFC3339),
	})
}

// handleAgentMessages returns conversation messages
func (s *Server) handleAgentMessages(w http.ResponseWriter, r *http.Request) {
	if s.agentClient == nil {
		s.writeJSON(w, http.StatusOK, []interface{}{})
		return
	}

	messages := s.agentClient.GetMessages()
	s.writeJSON(w, http.StatusOK, messages)
}

// handleAgentSendMessage sends a message to the agent
func (s *Server) handleAgentSendMessage(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Content string `json:"content"`
		Type    string `json:"type"` // "user" or "raw"
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		s.writeError(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	if s.agentClient == nil {
		s.writeError(w, http.StatusServiceUnavailable, "Agent not configured")
		return
	}

	// Add message to history
	msg := AgentMessage{
		ID:        fmt.Sprintf("msg-%d", time.Now().UnixNano()),
		Role:      req.Type,
		Content:   req.Content,
		Timestamp: time.Now(),
	}
	s.agentClient.AddMessage(msg)

	// Emit event to agent
	s.agentClient.EmitEvent(agentapi.EventTypeMessage, map[string]interface{}{
		"content": req.Content,
		"type":    req.Type,
	})

	s.writeJSON(w, http.StatusOK, map[string]interface{}{
		"status":  "sent",
		"message": msg,
	})
}

// handleAgentEvents handles SSE events from the agent
func (s *Server) handleAgentEvents(w http.ResponseWriter, r *http.Request) {
	// Set SSE headers
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")
	w.Header().Set("X-Accel-Buffering", "no")

	flusher, ok := w.(http.Flusher)
	if !ok {
		s.writeError(w, http.StatusInternalServerError, "Streaming not supported")
		return
	}

	// Send initial event
	_, _ = fmt.Fprintf(w, "event: status\ndata: {\"status\":\"connected\"}\n\n")
	flusher.Flush()

	// Register event handler if agent client is available
	if s.agentClient != nil {
		s.agentClient.OnEvent(func(event agentapi.Event) {
			data, _ := json.Marshal(event)
			_, _ = fmt.Fprintf(w, "event: %s\ndata: %s\n\n", event.Type, data)
			flusher.Flush()
		})
	}

	// Keep connection open until client disconnects
	<-r.Context().Done()
}

// Helper methods

// writeJSON writes a JSON response
func (s *Server) writeJSON(w http.ResponseWriter, status int, data interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	if err := json.NewEncoder(w).Encode(data); err != nil {
		s.logger.Error("failed to encode JSON response", "error", err)
	}
}

// writeError writes an error response
func (s *Server) writeError(w http.ResponseWriter, status int, message string) {
	s.writeJSON(w, status, map[string]interface{}{
		"error": map[string]interface{}{
			"message": message,
			"type":    "error",
			"code":    status,
		},
	})
}

// writeSSEError writes an error as SSE
func (s *Server) writeSSEError(w http.ResponseWriter, flusher http.Flusher, err *schemas.BifrostError) {
	msg := "Internal error"
	if err.Error != nil {
		msg = err.Error.Message
	}
	_, _ = fmt.Fprintf(w, "data: {\"error\":{\"message\":\"%s\"}}\n\n", msg)
	flusher.Flush()
}

// writeSSEChatResponse writes a chat response as SSE
func (s *Server) writeSSEChatResponse(w http.ResponseWriter, flusher http.Flusher, resp *schemas.BifrostChatResponse, model string) {
	if resp == nil {
		return
	}

	// Send content chunks
	for _, choice := range resp.Choices {
		content := ""
		if choice.ChatNonStreamResponseChoice != nil && choice.ChatNonStreamResponseChoice.Message != nil {
			if choice.ChatNonStreamResponseChoice.Message.Content != nil &&
				choice.ChatNonStreamResponseChoice.Message.Content.ContentStr != nil {
				content = *choice.ChatNonStreamResponseChoice.Message.Content.ContentStr
			}
		}

		chunk := map[string]interface{}{
			"id":      resp.ID,
			"object":  "chat.completion.chunk",
			"created": resp.Created,
			"model":   model,
			"choices": []map[string]interface{}{
				{
					"index": choice.Index,
					"delta": map[string]string{
						"role":    "assistant",
						"content": content,
					},
					"finish_reason": choice.FinishReason,
				},
			},
		}

		data, _ := json.Marshal(chunk)
		_, _ = fmt.Fprintf(w, "data: %s\n\n", data)
		flusher.Flush()
	}
}

// convertToBifrostChatRequest converts OpenAI request to Bifrost chat request
func (s *Server) convertToBifrostChatRequest(req *ChatCompletionRequest) *schemas.BifrostChatRequest {
	messages := make([]schemas.ChatMessage, 0, len(req.Messages))
	for _, msg := range req.Messages {
		content := msg.Content
		messages = append(messages, schemas.ChatMessage{
			Role:    schemas.ChatMessageRole(msg.Role),
			Content: &schemas.ChatMessageContent{ContentStr: &content},
		})
	}

	params := &schemas.ChatParameters{}
	if req.MaxTokens != nil {
		params.MaxCompletionTokens = req.MaxTokens
	}
	if req.Temperature != nil {
		params.Temperature = req.Temperature
	}
	if req.TopP != nil {
		params.TopP = req.TopP
	}

	return &schemas.BifrostChatRequest{
		Model:  req.Model,
		Input:  messages,
		Params: params,
	}
}

// Start starts the HTTP server
func (s *Server) Start() error {
	s.logger.Info("starting HTTP server", "addr", s.httpServer.Addr)
	return s.httpServer.ListenAndServe()
}

// Shutdown gracefully shuts down the server
func (s *Server) Shutdown(ctx context.Context) error {
	s.logger.Info("shutting down HTTP server")
	return s.httpServer.Shutdown(ctx)
}
