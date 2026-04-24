# Phenotype Evaluation

> Comprehensive Evaluation Framework for AI Agents and LLM Systems

A robust evaluation framework for benchmarking AI agent performance, LLM outputs, and multi-agent system effectiveness with standardized metrics and reproducible test suites.

## Philosophy

**Evaluation should be rigorous, reproducible, and actionable.**

- **Standardized**: Consistent metrics across all evaluations
- **Reproducible**: Deterministic test suites with versioned datasets
- **Automated**: CI/CD integration for continuous evaluation
- **Extensible**: Plugin architecture for custom evaluators
- **Observable**: Detailed metrics, traces, and reports

## Features

| Feature | Description | Status |
|---------|-------------|--------|
| **Benchmark Suites** | Pre-built evaluation datasets | Stable |
| **Custom Metrics** | Define domain-specific metrics | Stable |
| **A/B Testing** | Compare model/agent versions | Stable |
| **Regression Testing** | Detect performance degradation | Stable |
| **Human-in-the-Loop** | Integrate human feedback | Beta |
| **Multi-Modal** | Evaluate text, code, images, audio | Alpha |
| **Distributed** | Scale evaluation across workers | Beta |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      Phenotype Evaluation Framework                           │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Test Suite Definition                          │   │
│  │                                                                      │   │
│  │   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │   │
│  │   │  Dataset │  │  Prompts │  │  Metrics │  │  Scoring │           │   │
│  │   └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘           │   │
│  │        │             │             │             │                    │   │
│  │        └─────────────┴─────────────┴─────────────┘                    │   │
│  │                      │                                                  │   │
│  └──────────────────────┼───────────────────────────────────────────────────┘   │
│                         │                                                   │
│  ┌──────────────────────┼───────────────────────────────────────────────────┐ │
│  │                    Evaluation Engine                                     │ │
│  │                                                                      │ │
│  │   ┌─────────────────────────────────────────────────────────────┐   │ │
│  │   │                    Agent Under Test                            │   │ │
│  │   │                                                              │   │ │
│  │   │   Input → [Agent/LLM] → Output → [Evaluator] → Score         │   │ │
│  │   │                                                              │   │ │
│  │   └─────────────────────────────────────────────────────────────┘   │ │
│  │                                                                      │ │
│  └──────────────────────────────────────────────────────────────────────┘ │
│                         │                                                   │
│  ┌──────────────────────┼───────────────────────────────────────────────────┐ │
│  │                      Results & Reporting                                 │ │
│  │                                                                      │ │
│  │   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │ │
│  │   │  Scores  │  │  Traces  │  │  Compare │  │  Export  │           │ │
│  │   └──────────┘  └──────────┘  └──────────┘  └──────────┘           │ │
│  │                                                                      │ │
│  └──────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Quick Start

```python
from phenotype_evaluation import Evaluator, Benchmark

# Load a benchmark
benchmark = Benchmark.load("code-generation-v1")

# Define your agent
async def my_agent(prompt: str) -> str:
    # Your agent logic here
    return await llm.complete(prompt)

# Run evaluation
results = await Evaluator(
    benchmark=benchmark,
    agent=my_agent,
    metrics=["accuracy", "latency", "token_usage"]
).run()

# View results
print(f"Accuracy: {results.accuracy:.2%}")
print(f"Avg Latency: {results.latency_ms:.0f}ms")

# Export report
results.export("evaluation-report.html")
```

## Evaluation Types

| Type | Description | Example Use Case |
|------|-------------|------------------|
| **Correctness** | Output matches expected | Unit test generation |
| **Similarity** | Semantic/textual similarity | Paraphrase detection |
| **Safety** | Harmful content detection | Content moderation |
| **Performance** | Speed and resource usage | Latency benchmarking |
| **Human Eval** | Subjective quality ratings | Creative writing |
| **Agent Eval** | Multi-step task completion | Autonomous agents |

## Metrics

### Built-in Metrics

| Metric | Description | Range |
|--------|-------------|-------|
| `accuracy` | Exact match accuracy | 0-1 |
| `f1_score` | F1 for classification | 0-1 |
| `bleu` | BLEU for generation | 0-1 |
| `rouge_l` | ROUGE-L for summarization | 0-1 |
| `semantic_similarity` | Embedding cosine similarity | 0-1 |
| `latency_ms` | Response time | ms |
| `token_usage` | Tokens consumed | count |
| `cost_usd` | Estimated cost | USD |

### Custom Metrics

```python
from phenotype_evaluation import Metric

class MyMetric(Metric):
    name = "custom_score"
    
    async def compute(self, input, output, expected) -> float:
        # Custom scoring logic
        score = custom_scoring_function(output, expected)
        return score

# Use in evaluation
results = await Evaluator(
    benchmark=benchmark,
    agent=my_agent,
    metrics=["accuracy", MyMetric()]
).run()
```

## Benchmark Suites

### Included Benchmarks

| Benchmark | Domain | Size | Metrics |
|-----------|--------|------|---------|
| `human-eval` | Code generation | 164 | Pass@k |
| `mbpp` | Python programming | 974 | Pass@k |
| `gsm8k` | Math reasoning | 8.5K | Accuracy |
| `mmlu` | General knowledge | 15.9K | Accuracy |
| `truthfulqa` | Truthfulness | 817 | Truth/info |
| `bbh` | Big bench hard | 6.5K | Accuracy |
| `swde` | Web data extraction | 2.4K | F1 |
| `toolbench` | Tool use | 16.4K | Success rate |

### Loading Benchmarks

```python
# Load by name
benchmark = Benchmark.load("human-eval")

# Load from file
benchmark = Benchmark.from_file("./my-benchmark.jsonl")

# Create custom
benchmark = Benchmark(
    name="my-eval",
    tasks=[
        Task(input="What is 2+2?", expected="4"),
        Task(input="What is the capital of France?", expected="Paris")
    ]
)
```

## A/B Testing

```python
from phenotype_evaluation import ABTest

# Compare two agents
test = ABTest(
    name="model-comparison",
    benchmark=benchmark,
    variant_a=agent_v1,
    variant_b=agent_v2,
    metrics=["accuracy", "latency"]
)

results = await test.run(n_samples=100)

# Statistical analysis
print(f"Winner: {results.winner}")
print(f"Confidence: {results.confidence:.1%}")
print(f"Effect size: {results.effect_size:.2f}")
```

## Configuration

```yaml
# evaluation.yaml
version: "1.0"

evaluation:
  parallel: true
  workers: 4
  timeout: 30s
  retry: 2
  
metrics:
  - accuracy
  - latency_ms
  - token_usage
  
benchmarks:
  - name: code-generation
    weight: 0.5
  - name: reasoning
    weight: 0.3
  - name: safety
    weight: 0.2

reporting:
  format: html
  output: ./reports/
  include_traces: true
  include_comparisons: true

thresholds:
  accuracy:
    min: 0.85
    target: 0.95
  latency_ms:
    max: 5000
    target: 2000
```

## CI/CD Integration

```yaml
# .github/workflows/evaluate.yml
name: Evaluation

on:
  push:
    branches: [ main ]

jobs:
  evaluate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Run Evaluation
        run: |
          phenotype-eval run \
            --config evaluation.yaml \
            --output reports/
      
      - name: Check Thresholds
        run: |
          phenotype-eval check \
            --report reports/latest.json \
            --thresholds evaluation.yaml
      
      - name: Upload Report
        uses: actions/upload-artifact@v3
        with:
          name: evaluation-report
          path: reports/
```

## References

- EleutherAI LM Eval: https://github.com/EleutherAI/lm-evaluation-harness
- OpenAI Evals: https://github.com/openai/evals
- BIG-bench: https://github.com/google/BIG-bench
- HELM: https://github.com/stanford-crfm/helm
- AgentBench: https://github.com/THUDM/AgentBench
- SWE-bench: https://github.com/princeton-nlp/SWE-bench
