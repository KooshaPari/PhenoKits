//! Nested condition groups supporting all/any logic with required/optional flags.
//!
//! ConditionGroup enables complex nested policy rules: e.g., "(A AND B) OR (C AND D)"
//! where each condition can be required or optional.

use crate::context::EvaluationContext;
use crate::error::PolicyEngineError;
use serde::{Deserialize, Serialize};

/// Logical operator for combining conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOp {
    /// All conditions must evaluate to true.
    All,
    /// At least one condition must evaluate to true.
    Any,
}

/// A single condition to evaluate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// The fact key to evaluate.
    pub fact: String,
    /// The pattern to match (regex).
    pub pattern: String,
    /// Whether this condition is required to pass overall.
    pub required: bool,
    /// Optional description.
    pub description: Option<String>,
}

impl Condition {
    /// Creates a new condition.
    pub fn new(fact: impl Into<String>, pattern: impl Into<String>, required: bool) -> Self {
        Self {
            fact: fact.into(),
            pattern: pattern.into(),
            required,
            description: None,
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Evaluates this condition against a context.
    ///
    /// Returns (matches: bool, reason: Option<String>)
    /// where reason is Some if the condition failed or was missing.
    pub fn evaluate(&self, context: &EvaluationContext) -> Result<(bool, Option<String>), PolicyEngineError> {
        use regex::Regex;

        let regex = Regex::new(&self.pattern).map_err(|e| PolicyEngineError::RegexCompilationError {
            pattern: self.pattern.clone(),
            source: e,
        })?;

        match context.get_string(&self.fact) {
            Some(value) => {
                let matches = regex.is_match(&value);
                let reason = if !matches {
                    Some(format!("Fact '{}' = '{}' does not match pattern '{}'", self.fact, value, self.pattern))
                } else {
                    None
                };
                Ok((matches, reason))
            }
            None => {
                let reason = Some(format!("Fact '{}' is missing", self.fact));
                Ok((false, reason))
            }
        }
    }
}

/// A nested group of conditions/subgroups with logical operators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionGroup {
    /// Logical operator (All or Any).
    pub operator: LogicalOp,
    /// Conditions in this group.
    pub conditions: Vec<Condition>,
    /// Nested subgroups.
    pub subgroups: Vec<Box<ConditionGroup>>,
    /// Whether the group itself is required.
    pub required: bool,
}

impl ConditionGroup {
    /// Creates a new condition group.
    pub fn new(operator: LogicalOp) -> Self {
        Self {
            operator,
            conditions: Vec::new(),
            subgroups: Vec::new(),
            required: true,
        }
    }

    /// Creates an optional group.
    pub fn optional(operator: LogicalOp) -> Self {
        Self {
            operator,
            conditions: Vec::new(),
            subgroups: Vec::new(),
            required: false,
        }
    }

    /// Adds a condition to this group.
    pub fn add_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Adds a subgroup to this group.
    pub fn add_subgroup(mut self, subgroup: ConditionGroup) -> Self {
        self.subgroups.push(Box::new(subgroup));
        self
    }

    /// Evaluates this group against a context.
    ///
    /// Returns (satisfied: bool, reasons: Vec<String>)
    /// where reasons contains details about any failures.
    pub fn evaluate(&self, context: &EvaluationContext) -> Result<(bool, Vec<String>), PolicyEngineError> {
        let (satisfied, reasons) = self.evaluate_internal(context)?;
        Ok((satisfied, reasons))
    }

    /// Evaluates with quality score: (satisfied, required_failing, total)
    ///
    /// Useful for partial success scenarios.
    pub fn evaluate_with_quality(
        &self,
        context: &EvaluationContext,
    ) -> Result<(bool, usize, usize), PolicyEngineError> {
        let (satisfied, reasons) = self.evaluate_internal(context)?;
        let required_failing = reasons.len();
        let total = self.count_conditions();

        Ok((satisfied, required_failing, total))
    }

    fn evaluate_internal(
        &self,
        context: &EvaluationContext,
    ) -> Result<(bool, Vec<String>), PolicyEngineError> {
        let mut reasons = Vec::new();

        // Evaluate own conditions
        let mut required_failed = Vec::new();
        let mut any_required_match = false;

        for condition in &self.conditions {
            let (matches, reason) = condition.evaluate(context)?;
            if condition.required {
                if !matches {
                    required_failed.push(reason.unwrap_or_default());
                } else {
                    any_required_match = true;
                }
            }
        }

        // Evaluate subgroups
        for subgroup in &self.subgroups {
            let (satisfied, sub_reasons) = subgroup.evaluate_internal(context)?;
            if subgroup.required {
                if !satisfied {
                    required_failed.extend(sub_reasons);
                } else {
                    any_required_match = true;
                }
            }
        }

        // Apply logical operator
        let satisfied = match self.operator {
            LogicalOp::All => {
                // All required conditions/subgroups must be true
                required_failed.is_empty()
            }
            LogicalOp::Any => {
                // At least one required condition/subgroup must be true
                if required_failed.is_empty() {
                    true // All required passed, so Any passes
                } else {
                    any_required_match // Any passes if at least one required passed
                }
            }
        };

        if !satisfied {
            reasons.extend(required_failed);
        }

        // If this group is optional and failed, clear reasons (optional failure is OK)
        if !satisfied && !self.required {
            reasons.clear();
        }

        Ok((satisfied, reasons))
    }

    /// Count total conditions including nested ones.
    fn count_conditions(&self) -> usize {
        let own = self.conditions.len();
        let nested = self.subgroups.iter().map(|sg| sg.count_conditions()).sum::<usize>();
        own + nested
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-POLICY-CONDGROUP-001 — Nested condition group evaluation
    #[test]
    fn test_simple_all_group() {
        let group = ConditionGroup::new(LogicalOp::All)
            .add_condition(Condition::new("status", "^active$", true))
            .add_condition(Condition::new("role", "^admin$", true));

        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "active");
        ctx.set_string("role", "admin");

        let (satisfied, reasons) = group.evaluate(&ctx).unwrap();
        assert!(satisfied);
        assert!(reasons.is_empty());
    }

    // Traces to: FR-POLICY-CONDGROUP-002 — All group with one failing condition
    #[test]
    fn test_all_group_with_failure() {
        let group = ConditionGroup::new(LogicalOp::All)
            .add_condition(Condition::new("status", "^active$", true))
            .add_condition(Condition::new("role", "^admin$", true));

        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "active");
        ctx.set_string("role", "user");

        let (satisfied, reasons) = group.evaluate(&ctx).unwrap();
        assert!(!satisfied);
        assert!(!reasons.is_empty());
    }

    // Traces to: FR-POLICY-CONDGROUP-003 — Any group with one match
    #[test]
    fn test_any_group_partial_match() {
        let group = ConditionGroup::new(LogicalOp::Any)
            .add_condition(Condition::new("status", "^active$", true))
            .add_condition(Condition::new("role", "^admin$", true));

        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "active");
        ctx.set_string("role", "user");

        let (satisfied, reasons) = group.evaluate(&ctx).unwrap();
        assert!(satisfied);
        assert!(reasons.is_empty());
    }

    // Traces to: FR-POLICY-CONDGROUP-004 — Optional conditions are ignored on failure
    #[test]
    fn test_optional_condition() {
        let group = ConditionGroup::new(LogicalOp::All)
            .add_condition(Condition::new("status", "^active$", true))
            .add_condition(Condition::new("premium", "^yes$", false)); // optional

        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "active");
        // premium not set

        let (satisfied, reasons) = group.evaluate(&ctx).unwrap();
        assert!(satisfied);
        assert!(reasons.is_empty());
    }

    // Traces to: FR-POLICY-CONDGROUP-005 — Deeply nested groups
    #[test]
    fn test_nested_groups_3_levels() {
        // (A AND B) OR (C AND D)
        let inner_all_1 = ConditionGroup::new(LogicalOp::All)
            .add_condition(Condition::new("field1", "^val1$", true))
            .add_condition(Condition::new("field2", "^val2$", true));

        let inner_all_2 = ConditionGroup::new(LogicalOp::All)
            .add_condition(Condition::new("field3", "^val3$", true))
            .add_condition(Condition::new("field4", "^val4$", true));

        let outer = ConditionGroup::new(LogicalOp::Any)
            .add_subgroup(inner_all_1)
            .add_subgroup(inner_all_2);

        let mut ctx = EvaluationContext::new();
        ctx.set_string("field1", "val1");
        ctx.set_string("field2", "val2");
        // field3 and field4 not set

        let (satisfied, reasons) = outer.evaluate(&ctx).unwrap();
        assert!(satisfied); // First branch succeeds
        assert!(reasons.is_empty());
    }

    // Traces to: FR-POLICY-CONDGROUP-006 — Quality score evaluation
    #[test]
    fn test_evaluate_with_quality() {
        let group = ConditionGroup::new(LogicalOp::All)
            .add_condition(Condition::new("a", "^a$", true))
            .add_condition(Condition::new("b", "^b$", true))
            .add_condition(Condition::new("c", "^c$", true));

        let mut ctx = EvaluationContext::new();
        ctx.set_string("a", "a");
        // b and c missing

        let (satisfied, failing, total) = group.evaluate_with_quality(&ctx).unwrap();
        assert!(!satisfied);
        assert_eq!(total, 3);
        assert!(failing > 0); // At least b and c failed
    }

    // Traces to: FR-POLICY-CONDGROUP-007 — Optional group failures don't propagate
    #[test]
    fn test_optional_group_failure() {
        let optional_group = ConditionGroup::optional(LogicalOp::All)
            .add_condition(Condition::new("premium", "^yes$", true));

        let required_group = ConditionGroup::new(LogicalOp::All)
            .add_condition(Condition::new("status", "^active$", true))
            .add_subgroup(optional_group);

        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "active");
        // premium not set

        let (satisfied, reasons) = required_group.evaluate(&ctx).unwrap();
        assert!(satisfied); // optional subgroup failure is OK
        assert!(reasons.is_empty());
    }

    // Traces to: FR-POLICY-CONDGROUP-008 — Regex validation in conditions
    #[test]
    fn test_invalid_regex_in_condition() {
        let group = ConditionGroup::new(LogicalOp::All)
            .add_condition(Condition::new("field", "[invalid", true));

        let ctx = EvaluationContext::new();
        let result = group.evaluate(&ctx);
        assert!(result.is_err());
    }

    // Traces to: FR-POLICY-CONDGROUP-009 — Missing required fact in Any group
    #[test]
    fn test_any_group_all_missing() {
        let group = ConditionGroup::new(LogicalOp::Any)
            .add_condition(Condition::new("a", "^a$", true))
            .add_condition(Condition::new("b", "^b$", true));

        let ctx = EvaluationContext::new(); // empty context

        let (satisfied, reasons) = group.evaluate(&ctx).unwrap();
        assert!(!satisfied);
        assert!(!reasons.is_empty());
    }

    // Traces to: FR-POLICY-CONDGROUP-010 — Complex mixed nesting with optional conditions
    #[test]
    fn test_complex_mixed_nesting() {
        let inner = ConditionGroup::new(LogicalOp::All)
            .add_condition(Condition::new("x", "^x$", true))
            .add_condition(Condition::new("y", "^y$", false)); // optional

        let outer = ConditionGroup::new(LogicalOp::Any)
            .add_condition(Condition::new("z", "^z$", true))
            .add_subgroup(inner);

        let mut ctx = EvaluationContext::new();
        ctx.set_string("z", "z");
        // x and y not set, but z matches so Any should pass

        let (satisfied, reasons) = outer.evaluate(&ctx).unwrap();
        assert!(satisfied);
        assert!(reasons.is_empty());
    }
}
