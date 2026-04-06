# Functional Requirements — phenotype-evaluation

## FR-EVAL — Evaluation Protocol

| ID | Requirement |
|----|-------------|
| FR-EVAL-001 | The system SHALL define an Evaluator port with a standard evaluate interface. |
| FR-EVAL-002 | The system SHALL support correctness, latency, token efficiency, and safety dimensions. |
| FR-EVAL-003 | The system SHALL support composite evaluators that aggregate dimension scores. |
| FR-EVAL-004 | The system SHALL allow custom evaluator adapters to be registered by plug-in. |

## FR-HARN — Test Harnesses

| ID | Requirement |
|----|-------------|
| FR-HARN-001 | The system SHALL load evaluation datasets from JSONL files. |
| FR-HARN-002 | The system SHALL support few-shot prompting harnesses for LLM evaluation. |
| FR-HARN-003 | The system SHALL provide a regression harness comparing current vs. baseline scores. |
| FR-HARN-004 | The system SHALL support parallel evaluation of dataset rows. |

## FR-SCORE — Scoring

| ID | Requirement |
|----|-------------|
| FR-SCORE-001 | The system SHALL produce per-run score summaries in JSON format. |
| FR-SCORE-002 | The system SHALL produce human-readable Markdown reports. |
| FR-SCORE-003 | The system SHALL persist score history for trend analysis. |
| FR-SCORE-004 | The system SHALL emit PASS or FAIL based on configured thresholds per dimension. |

## FR-CLI — CLI

| ID | Requirement |
|----|-------------|
| FR-CLI-001 | The system SHALL provide a CLI command `phenotype-eval run`. |
| FR-CLI-002 | The CLI SHALL accept `--dataset`, `--threshold`, and `--output` flags. |
| FR-CLI-003 | The CLI SHALL exit with code 1 when any configured threshold is breached. |
