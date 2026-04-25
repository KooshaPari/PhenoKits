//! PyO3 bindings for phenotype-policy-engine.
//!
//! Exposes ConditionGroup, RuleEvaluator, MatcherKind, and core types to Python.
//! This scaffolding is for integration testing — no runtime adoption yet (awaits user approval).

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use phenotype_policy_engine::{
    condition_group::{Condition, ConditionGroup, LogicalOp},
    context::EvaluationContext,
    rule::{MatcherKind, Rule, RuleType},
    rule_evaluator::{RuleEvaluationEngine, StandardRuleEvaluator},
};

/// PyO3 wrapper for LogicalOp enum.
#[pyclass(name = "LogicalOp")]
#[derive(Clone, Copy)]
pub struct PyLogicalOp {
    inner: LogicalOp,
}

#[pymethods]
impl PyLogicalOp {
    /// Create an "All" logical operator (AND).
    #[staticmethod]
    fn all() -> Self {
        PyLogicalOp {
            inner: LogicalOp::All,
        }
    }

    /// Create an "Any" logical operator (OR).
    #[staticmethod]
    fn any() -> Self {
        PyLogicalOp {
            inner: LogicalOp::Any,
        }
    }

    fn __repr__(&self) -> String {
        match self.inner {
            LogicalOp::All => "LogicalOp.All".to_string(),
            LogicalOp::Any => "LogicalOp.Any".to_string(),
        }
    }
}

/// PyO3 wrapper for MatcherKind enum.
#[pyclass(name = "MatcherKind")]
#[derive(Clone, Copy)]
pub struct PyMatcherKind {
    inner: MatcherKind,
}

#[pymethods]
impl PyMatcherKind {
    /// Glob pattern matching (e.g., "*.rs").
    #[staticmethod]
    fn glob() -> Self {
        PyMatcherKind {
            inner: MatcherKind::Glob,
        }
    }

    /// Prefix matching (e.g., "dev_").
    #[staticmethod]
    fn prefix() -> Self {
        PyMatcherKind {
            inner: MatcherKind::Prefix,
        }
    }

    /// Exact string matching.
    #[staticmethod]
    fn exact() -> Self {
        PyMatcherKind {
            inner: MatcherKind::Exact,
        }
    }

    /// Regular expression matching.
    #[staticmethod]
    fn regex() -> Self {
        PyMatcherKind {
            inner: MatcherKind::Regex,
        }
    }

    fn __repr__(&self) -> String {
        format!("MatcherKind.{}", self.inner.as_str())
    }
}

/// PyO3 wrapper for Condition.
#[pyclass(name = "Condition")]
#[derive(Clone)]
pub struct PyCondition {
    inner: Condition,
}

#[pymethods]
impl PyCondition {
    /// Create a new condition.
    ///
    /// Args:
    ///     fact: The fact key to evaluate.
    ///     pattern: The regex pattern to match.
    ///     required: Whether this condition is required.
    ///     description: Optional description.
    #[new]
    fn new(fact: String, pattern: String, required: bool, description: Option<String>) -> Self {
        let mut cond = Condition::new(fact, pattern, required);
        if let Some(desc) = description {
            cond = cond.with_description(desc);
        }
        PyCondition { inner: cond }
    }

    fn fact(&self) -> String {
        self.inner.fact.clone()
    }

    fn pattern(&self) -> String {
        self.inner.pattern.clone()
    }

    fn required(&self) -> bool {
        self.inner.required
    }

    fn description(&self) -> Option<String> {
        self.inner.description.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Condition(fact={}, pattern={}, required={})",
            self.inner.fact, self.inner.pattern, self.inner.required
        )
    }
}

/// PyO3 wrapper for ConditionGroup.
#[pyclass(name = "ConditionGroup")]
pub struct PyConditionGroup {
    inner: ConditionGroup,
}

#[pymethods]
impl PyConditionGroup {
    /// Create a new ConditionGroup.
    ///
    /// Args:
    ///     logical_op: LogicalOp.all() or LogicalOp.any().
    ///     required: Whether this group is required.
    #[new]
    fn new(logical_op: PyLogicalOp, required: bool) -> Self {
        PyConditionGroup {
            inner: ConditionGroup::new(logical_op.inner, required),
        }
    }

    /// Add a condition to this group.
    ///
    /// Args:
    ///     condition: A Condition object.
    fn add_condition(&mut self, condition: &PyCondition) {
        self.inner = self.inner.clone().add_condition(condition.inner.clone());
    }

    /// Evaluate this group against an evaluation context.
    ///
    /// Args:
    ///     context: Dict of fact key-value pairs.
    ///
    /// Returns:
    ///     (matched: bool, reason: Optional[str])
    fn evaluate(&self, py: Python, context_dict: &PyDict) -> PyResult<(bool, Option<String>)> {
        let mut ctx = EvaluationContext::new();
        for (key, value) in context_dict.iter() {
            let key_str: String = key.extract()?;
            if let Ok(val_str) = value.extract::<String>() {
                ctx = ctx.with_fact(key_str, val_str);
            }
        }

        self.inner
            .evaluate(&ctx)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn __repr__(&self) -> String {
        format!(
            "ConditionGroup(logical_op={:?}, required={})",
            self.inner.logical_op, self.inner.required
        )
    }
}

/// PyO3 wrapper for DecisionMetadata.
#[pyclass(name = "DecisionMetadata")]
#[derive(Clone)]
pub struct PyDecisionMetadata {
    rule_id: String,
    matcher_kind: String,
    on_mismatch: String,
    source: String,
    matched: bool,
    reason: Option<String>,
}

#[pymethods]
impl PyDecisionMetadata {
    fn rule_id(&self) -> String {
        self.rule_id.clone()
    }

    fn matcher_kind(&self) -> String {
        self.matcher_kind.clone()
    }

    fn on_mismatch(&self) -> String {
        self.on_mismatch.clone()
    }

    fn source(&self) -> String {
        self.source.clone()
    }

    fn matched(&self) -> bool {
        self.matched
    }

    fn reason(&self) -> Option<String> {
        self.reason.clone()
    }

    /// Generate a decision trace string: "rule_id::action::source".
    fn trace(&self) -> String {
        format!("{}::{}::{}", self.rule_id, self.on_mismatch, self.source)
    }

    fn __repr__(&self) -> String {
        format!(
            "DecisionMetadata(rule_id={}, matched={})",
            self.rule_id, self.matched
        )
    }
}

/// PyO3 wrapper for Decision.
#[pyclass(name = "Decision")]
#[derive(Clone)]
pub struct PyDecision {
    matched: bool,
    metadata: PyDecisionMetadata,
}

#[pymethods]
impl PyDecision {
    fn matched(&self) -> bool {
        self.matched
    }

    fn metadata(&self) -> PyDecisionMetadata {
        self.metadata.clone()
    }

    /// Return the decision trace string.
    fn trace(&self) -> String {
        self.metadata.trace()
    }

    fn __repr__(&self) -> String {
        format!("Decision(matched={}, trace={})", self.matched, self.trace())
    }
}

/// PyO3 wrapper for RuleEvaluator.
#[pyclass(name = "RuleEvaluator")]
pub struct PyRuleEvaluator {
    evaluator: StandardRuleEvaluator,
}

#[pymethods]
impl PyRuleEvaluator {
    /// Create a new RuleEvaluator.
    #[new]
    fn new() -> Self {
        PyRuleEvaluator {
            evaluator: StandardRuleEvaluator,
        }
    }

    /// Evaluate a batch of rules against a context.
    ///
    /// Args:
    ///     rules: List of rule dicts with keys: rule_type, pattern, matcher_kind, required, source.
    ///     context: Dict of fact key-value pairs.
    ///
    /// Returns:
    ///     List of Decision objects with metadata.
    fn evaluate_rules(
        &self,
        rules_list: &PyList,
        context_dict: &PyDict,
    ) -> PyResult<Vec<PyDecision>> {
        // Build evaluation context
        let mut ctx = EvaluationContext::new();
        for (key, value) in context_dict.iter() {
            let key_str: String = key.extract()?;
            if let Ok(val_str) = value.extract::<String>() {
                ctx = ctx.with_fact(key_str, val_str);
            }
        }

        // Parse rules from Python list
        let mut rules = Vec::new();
        for rule_obj in rules_list.iter() {
            let rule_dict = rule_obj.downcast::<PyDict>()?;

            let rule_type_str: String = rule_dict.get_item("rule_type")?.extract()?;
            let rule_type = match rule_type_str.as_str() {
                "Allow" => RuleType::Allow,
                "Deny" => RuleType::Deny,
                "Require" => RuleType::Require,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Unknown rule type: {}", rule_type_str),
                )),
            };

            let pattern: String = rule_dict.get_item("pattern")?.extract()?;
            let matcher_kind_str: String = rule_dict.get_item("matcher_kind")?.extract()?;
            let matcher_kind = match matcher_kind_str.as_str() {
                "glob" => MatcherKind::Glob,
                "prefix" => MatcherKind::Prefix,
                "exact" => MatcherKind::Exact,
                "regex" => MatcherKind::Regex,
                _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Unknown matcher kind: {}", matcher_kind_str),
                )),
            };

            let required: bool = rule_dict.get_item("required")?.extract()?;
            let _source: String = rule_dict.get_item("source")?.extract()?;

            let rule = Rule {
                rule_type,
                fact: rule_dict.get_item("fact").ok().and_then(|v| v.extract::<String>().ok()),
                pattern,
                matcher_kind,
                required,
                metadata: None,
            };

            rules.push(rule);
        }

        // Evaluate rules
        let decisions = self
            .evaluator
            .evaluate_rules(rules, &ctx)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        // Convert Rust decisions to Python decisions
        let py_decisions = decisions
            .into_iter()
            .map(|d| PyDecision {
                matched: d.matched,
                metadata: PyDecisionMetadata {
                    rule_id: d.metadata.rule_id,
                    matcher_kind: d.metadata.matcher_kind.as_str().to_string(),
                    on_mismatch: d.metadata.on_mismatch.as_str().to_string(),
                    source: d.metadata.source,
                    matched: d.metadata.matched,
                    reason: d.metadata.reason,
                },
            })
            .collect();

        Ok(py_decisions)
    }

    fn __repr__(&self) -> String {
        "RuleEvaluator()".to_string()
    }
}

/// Module initialization for PyO3.
#[pymodule]
fn policy_engine_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyLogicalOp>()?;
    m.add_class::<PyMatcherKind>()?;
    m.add_class::<PyCondition>()?;
    m.add_class::<PyConditionGroup>()?;
    m.add_class::<PyDecisionMetadata>()?;
    m.add_class::<PyDecision>()?;
    m.add_class::<PyRuleEvaluator>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Traces to: FR-POLICY-001 (ConditionGroup PyO3 exposure).
    #[test]
    fn test_py_logical_op_creation() {
        let all_op = PyLogicalOp::all();
        assert_eq!(all_op.inner, LogicalOp::All);

        let any_op = PyLogicalOp::any();
        assert_eq!(any_op.inner, LogicalOp::Any);
    }

    /// Traces to: FR-POLICY-002 (MatcherKind PyO3 exposure).
    #[test]
    fn test_py_matcher_kind_creation() {
        let glob = PyMatcherKind::glob();
        assert_eq!(glob.inner, MatcherKind::Glob);

        let exact = PyMatcherKind::exact();
        assert_eq!(exact.inner, MatcherKind::Exact);
    }

    /// Traces to: FR-POLICY-003 (Condition PyO3 binding).
    #[test]
    fn test_py_condition_creation() {
        let cond = PyCondition::new(
            "environment".to_string(),
            "prod|staging".to_string(),
            true,
            Some("Check environment".to_string()),
        );

        assert_eq!(cond.fact(), "environment");
        assert_eq!(cond.pattern(), "prod|staging");
        assert!(cond.required());
    }

    /// Traces to: FR-POLICY-004 (RuleEvaluator PyO3 binding).
    #[test]
    fn test_py_rule_evaluator_instantiation() {
        let evaluator = PyRuleEvaluator::new();
        assert_eq!(evaluator.__repr__(), "RuleEvaluator()");
    }

    /// Traces to: FR-POLICY-005 (DecisionMetadata PyO3 binding).
    #[test]
    fn test_py_decision_metadata() {
        let metadata = PyDecisionMetadata {
            rule_id: "rule-001".to_string(),
            matcher_kind: "regex".to_string(),
            on_mismatch: "FAIL".to_string(),
            source: "policy.py".to_string(),
            matched: true,
            reason: None,
        };

        assert_eq!(metadata.trace(), "rule-001::FAIL::policy.py");
    }
}
    }
}
