// Package workflows defines Hatchet workflow definitions for Bifrost.
// These workflows handle cold path operations like model evaluation,
// semantic research, and metrics synchronization.
package workflows

import (
	"fmt"
	"time"

	"github.com/hatchet-dev/hatchet/pkg/worker"
)

// ModelEvalInput represents input for model evaluation workflow
type ModelEvalInput struct {
	ModelID string   `json:"model_id"`
	Prompts []string `json:"prompts"`
}

// ModelEvalOutput represents output from model evaluation
type ModelEvalOutput struct {
	ModelID   string             `json:"model_id"`
	Scores    map[string]float64 `json:"scores"`
	Latencies map[string]int64   `json:"latencies_ms"`
	Errors    []string           `json:"errors,omitempty"`
	Timestamp time.Time          `json:"timestamp"`
}

// ModelEvalWorkflow defines the model evaluation workflow
type ModelEvalWorkflow struct {
	// Dependencies would be injected here
}

// NewModelEvalWorkflow creates a new model evaluation workflow
func NewModelEvalWorkflow() *ModelEvalWorkflow {
	return &ModelEvalWorkflow{}
}

// Register registers the workflow with a Hatchet worker
func (w *ModelEvalWorkflow) Register(wkr *worker.Worker) error {
	return wkr.RegisterWorkflow(
		&worker.WorkflowJob{
			Name: "model-eval",
			Description: "Evaluates a model against a set of prompts to update " +
				"performance metrics and bandit statistics",
			Steps: []*worker.WorkflowStep{
				{
					Name: "validate-input",
					Function: w.validateInput,
				},
				{
					Name:    "run-evaluations",
					Function:    w.runEvaluations,
					Parents: []string{"validate-input"},
				},
				{
					Name:    "compute-scores",
					Function:    w.computeScores,
					Parents: []string{"run-evaluations"},
				},
				{
					Name:    "update-metrics",
					Function:    w.updateMetrics,
					Parents: []string{"compute-scores"},
				},
			},
		},
	)
}

// validateInput validates the input for model evaluation
func (w *ModelEvalWorkflow) validateInput(ctx worker.HatchetContext) (interface{}, error) {
	var data ModelEvalInput
	if err := ctx.WorkflowInput(&data); err != nil {
		return nil, fmt.Errorf("parse input: %w", err)
	}

	if data.ModelID == "" {
		return nil, fmt.Errorf("model_id is required")
	}
	if len(data.Prompts) == 0 {
		return nil, fmt.Errorf("at least one prompt is required")
	}

	return &data, nil
}

// EvalResult represents the result of a single evaluation
type EvalResult struct {
	Prompt    string  `json:"prompt"`
	Response  string  `json:"response"`
	LatencyMS int64   `json:"latency_ms"`
	Error     string  `json:"error,omitempty"`
	Score     float64 `json:"score"`
}

// EvalResults holds all evaluation results
type EvalResults struct {
	ModelID string       `json:"model_id"`
	Results []EvalResult `json:"results"`
}

// runEvaluations runs the model against each prompt
func (w *ModelEvalWorkflow) runEvaluations(ctx worker.HatchetContext) (interface{}, error) {
	var data ModelEvalInput
	if err := ctx.WorkflowInput(&data); err != nil {
		return nil, fmt.Errorf("parse input: %w", err)
	}
	results := &EvalResults{
		ModelID: data.ModelID,
		Results: make([]EvalResult, 0, len(data.Prompts)),
	}

	for _, prompt := range data.Prompts {
		// In real implementation, this would call the model
		result := EvalResult{
			Prompt:    prompt,
			Response:  "placeholder response",
			LatencyMS: 100,
			Score:     0.85,
		}
		results.Results = append(results.Results, result)
	}

	return results, nil
}

// computeScores computes aggregate scores from evaluation results
func (w *ModelEvalWorkflow) computeScores(ctx worker.HatchetContext) (interface{}, error) {
	var data EvalResults
	if err := ctx.StepOutput("run-evaluations", &data); err != nil {
		return nil, fmt.Errorf("parse step output: %w", err)
	}

	output := &ModelEvalOutput{
		ModelID:   data.ModelID,
		Scores:    make(map[string]float64),
		Latencies: make(map[string]int64),
		Timestamp: time.Now(),
	}

	var totalScore float64
	var totalLatency int64
	for _, r := range data.Results {
		totalScore += r.Score
		totalLatency += r.LatencyMS
		if r.Error != "" {
			output.Errors = append(output.Errors, r.Error)
		}
	}

	n := float64(len(data.Results))
	output.Scores["average"] = totalScore / n
	output.Latencies["average"] = totalLatency / int64(len(data.Results))

	return output, nil
}

// updateMetrics updates the metrics store with evaluation results
func (w *ModelEvalWorkflow) updateMetrics(ctx worker.HatchetContext) (interface{}, error) {
	var data ModelEvalOutput
	if err := ctx.StepOutput("compute-scores", &data); err != nil {
		return nil, fmt.Errorf("parse step output: %w", err)
	}
	// In real implementation, this would update Postgres/Neo4j
	return &data, nil
}

