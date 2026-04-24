# PRD — phenotype-evaluation

## Overview

`phenotype-evaluation` is the evaluation framework for the Phenotype platform. It provides structured test harnesses, scoring pipelines, and result aggregation for evaluating agent behaviors, model outputs, and system contracts.

## Goals

- Define a reusable evaluation protocol usable across all Phenotype agents and services.
- Produce machine-readable evaluation reports with per-dimension scores.
- Integrate with CI to gate promotions on evaluation thresholds.

## Epics

### E1 — Evaluation Protocol
- E1.1 Define an `Evaluator` port with `evaluate(input, expected) -> Score`.
- E1.2 Support dimensions: correctness, latency, token efficiency, safety.
- E1.3 Allow composite evaluators that aggregate multiple dimension scores.

### E2 — Test Harnesses
- E2.1 Provide a dataset loader for JSONL evaluation sets.
- E2.2 Support few-shot prompting harnesses for LLM evaluations.
- E2.3 Provide a regression harness comparing current vs. baseline scores.

### E3 — Scoring and Reporting
- E3.1 Produce per-run score summaries in JSON and Markdown.
- E3.2 Track score history across runs for trend analysis.
- E3.3 Emit PASS/FAIL based on configurable thresholds per dimension.

### E4 — CI Integration
- E4.1 CLI entry point `phenotype-eval run --dataset <path> --threshold <cfg>`.
- E4.2 Exit code 1 when any threshold is breached.
- E4.3 Upload reports as CI artifacts.

## Acceptance Criteria

- Running `phenotype-eval run` on a sample dataset produces a scored report.
- Threshold breaches produce exit code 1 with a human-readable failure summary.
- Score history is persisted and queryable across multiple runs.
