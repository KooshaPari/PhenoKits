package toolrouter

import (
	"context"

	"github.com/kooshapari/BifrostGo/slm"
	"github.com/maximhq/bifrost/core/schemas"
)

// getToolProfile gets tool profile from context
func (tr *ToolRouter) getToolProfile(ctx context.Context) slm.ToolProfile {
	if profile, ok := ctx.Value(toolProfileKey).(slm.ToolProfile); ok {
		return profile
	}
	return slm.ToolProfile{}
}

// filterTools filters and prioritizes tools based on profile
func (tr *ToolRouter) filterTools(
	ctx context.Context,
	req *schemas.BifrostRequest,
	profile slm.ToolProfile,
) *schemas.BifrostRequest {
	if req.ChatRequest == nil || req.ChatRequest.Params == nil {
		return req
	}

	tools := req.ChatRequest.Params.Tools
	if tools == nil || len(tools) == 0 {
		return req
	}

	// If no profile, try to get one from SLM
	if len(profile.Preferred) == 0 && tr.slmClients != nil {
		profile = tr.classifyTools(ctx, req)
	}

	// Filter and reorder tools
	filteredTools := tr.applyProfile(tools, profile)

	// Limit number of tools
	if len(filteredTools) > tr.config.MaxToolsPerRequest {
		filteredTools = filteredTools[:tr.config.MaxToolsPerRequest]
	}

	// Create modified request
	modifiedReq := *req
	modifiedChat := *req.ChatRequest
	modifiedReq.ChatRequest = &modifiedChat
	modifiedReq.ChatRequest.Params.Tools = filteredTools

	return &modifiedReq
}

// classifyTools uses SLM to classify tools for this request
func (tr *ToolRouter) classifyTools(ctx context.Context, req *schemas.BifrostRequest) slm.ToolProfile {
	if tr.slmClients == nil || tr.slmClients.Router == nil {
		return slm.ToolProfile{}
	}

	// Get last user message for context
	var userMessage string
	if req.ChatRequest != nil && req.ChatRequest.Input != nil {
		for i := len(req.ChatRequest.Input) - 1; i >= 0; i-- {
			msg := req.ChatRequest.Input[i]
			if msg.Role == schemas.ChatMessageRoleUser && msg.Content != nil && msg.Content.ContentStr != nil {
				userMessage = *msg.Content.ContentStr
				break
			}
		}
	}

	// Call router SLM to classify
	resp, err := tr.slmClients.Classify(ctx, &slm.ClassifyRequest{
		UserMessage: userMessage,
	})
	if err != nil || resp == nil {
		return slm.ToolProfile{}
	}

	// Build profile from classification - use the role as a hint for preferred tools
	return slm.ToolProfile{
		Preferred: []string{resp.Role},
	}
}

// applyProfile reorders tools based on profile
func (tr *ToolRouter) applyProfile(tools []schemas.ChatTool, profile slm.ToolProfile) []schemas.ChatTool {
	if len(profile.Preferred) == 0 {
		return tools
	}

	// Create priority map
	priority := make(map[string]int)
	for i, name := range profile.Preferred {
		priority[name] = i
	}

	// Separate prioritized and other tools
	prioritized := make([]schemas.ChatTool, 0)
	other := make([]schemas.ChatTool, 0)

	for _, tool := range tools {
		if tool.Function != nil {
			if _, ok := priority[tool.Function.Name]; ok {
				prioritized = append(prioritized, tool)
			} else if tr.config.FallbackToAllTools {
				other = append(other, tool)
			}
		}
	}

	// Combine: prioritized first, then others
	result := make([]schemas.ChatTool, 0, len(prioritized)+len(other))
	result = append(result, prioritized...)
	result = append(result, other...)

	return result
}

// trackToolUsage tracks which tools were used for learning
func (tr *ToolRouter) trackToolUsage(ctx context.Context, resp *schemas.BifrostResponse) {
	// TODO: Extract tool calls from response and record metrics
}
