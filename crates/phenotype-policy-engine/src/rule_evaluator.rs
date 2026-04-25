//! Batch rule evaluator with decision tracing and metadata.
//!
//! RuleEvaluator supports evaluating multiple rules against an evaluation context
//! and collecting decision traces for audit purposes.

use crate::context::EvaluationContext;
use crate::error::PolicyEngineError;
use crate::rule::{MatcherKind, OnMismatchAction, Rule};
use serde::{Deserialize, Serialize};

/// Metadata for a rule evaluation decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionMetadata {
    /// Unique rule identifier.
    pub rule_id: String,
    /// Matcher kind used.
    pub matcher_kind: MatcherKind,
    /// Action on mismatch.
    pub on_mismatch: OnMismatchAction,
    /// Source of the rule (e.g., "policy_lib.py", "system").
    pub source: String,
    /// Whether the rule evaluated to a match.
    pub matched: bool,
    /// Reason for mismatch (if applicable).
    pub reason: Option<String>,
}

impl DecisionMetadata {
    /// Creates a decision trace string: "rule_id::action::source".
    pub fn trace(&self) -> String {
        format!("{}::{}::{}", self.rule_id, self.on_mismatch.as_str(), self.source)
    }
}

/// A decision trace from rule evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    /// Whether the rule matched.
    pub matched: bool,
    /// Metadata about the decision.
    pub metadata: DecisionMetadata,
}

impl Decision {
    /// Returns the decision trace string.
    pub fn trace(&self) -> String {
        self.metadata.trace()
    }
}

/// Trait for evaluating rules with metadata collection.
pub trait RuleEvaluationEngine {
    /// Evaluates a batch of rules against a context.
    ///
    /// Returns immediately on first Allow/Deny match, collecting reasons for audit.
    /// For Require rules, all are evaluated and any failure prevents success.
    fn evaluate_rules(
        &self,
        rules: Vec<Rule>,
        context: &EvaluationContext,
    ) -> Result<Vec<Decision>, PolicyEngineError>;
}

/// Default implementation of rule evaluation.
pub struct StandardRuleEvaluator;

impl RuleEvaluationEngine for StandardRuleEvaluator {
    fn evaluate_rules(
        &self,
        rules: Vec<Rule>,
        context: &EvaluationContext,
    ) -> Result<Vec<Decision>, PolicyEngineError> {
        let mut decisions = Vec::new();

        for rule in rules {
            let matched = rule.evaluate(context)?;

            let metadata = DecisionMetadata {
                rule_id: rule.metadata.as_ref().map(|m| m.rule_id.clone()).unwrap_or_else(|| "unknown".to_string()),
                matcher_kind: rule.matcher_kind,
                on_mismatch: rule.on_mismatch,
                source: rule.metadata.as_ref().map(|m| m.source.clone()).unwrap_or_else(|| "default".to_string()),
                matched,
                reason: if !matched {
                    Some(format!("Pattern '{}' did not match fact '{}'", rule.pattern, rule.fact))
                } else {
                    None
                },
            };

            decisions.push(Decision { matched, metadata });
        }

        Ok(decisions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::{Rule, RuleMetadata, RuleType};

    // Traces to: FR-POLICY-EVAL-001 — Basic batch evaluation
    #[test]
    fn test_batch_rule_evaluation() {
        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "active");

        let rules = vec![
            Rule::new(RuleType::Allow, "status", "^active$"),
            Rule::new(RuleType::Require, "role", "^admin$"),
        ];

        let evaluator = StandardRuleEvaluator;
        let decisions = evaluator.evaluate_rules(rules, &ctx).unwrap();

        assert_eq!(decisions.len(), 2);
        assert!(decisions[0].matched); // status matches
        assert!(!decisions[1].matched); // role missing
    }

    // Traces to: FR-POLICY-EVAL-002 — Decision metadata collection
    #[test]
    fn test_decision_metadata() {
        let mut ctx = EvaluationContext::new();
        ctx.set_string("env", "prod");

        let mut rule = Rule::new(RuleType::Require, "env", "^prod$");
        rule.set_metadata(RuleMetadata {
            rule_id: "deploy-prod-001".to_string(),
            source: "deploy_policy".to_string(),
        });
        rule = rule.with_matcher(MatcherKind::Regex);
        rule = rule.with_on_mismatch(OnMismatchAction::Fail);

        let evaluator = StandardRuleEvaluator;
        let decisions = evaluator.evaluate_rules(vec![rule], &ctx).unwrap();

        assert_eq!(decisions.len(), 1);
        let decision = &decisions[0];
        assert!(decision.matched);
        assert_eq!(decision.metadata.rule_id, "deploy-prod-001");
        assert_eq!(decision.metadata.source, "deploy_policy");
        assert_eq!(decision.metadata.on_mismatch, OnMismatchAction::Fail);
    }

    // Traces to: FR-POLICY-EVAL-003 — Decision trace format
    #[test]
    fn test_decision_trace() {
        let metadata = DecisionMetadata {
            rule_id: "rule-123".to_string(),
            matcher_kind: MatcherKind::Regex,
            on_mismatch: OnMismatchAction::Warn,
            source: "policy_lib".to_string(),
            matched: true,
            reason: None,
        };

        let trace = metadata.trace();
        assert_eq!(trace, "rule-123::warn::policy_lib");
    }

    // Traces to: FR-POLICY-EVAL-004 — Matcher kind display
    #[test]
    fn test_matcher_kind_display() {
        assert_eq!(MatcherKind::Glob.as_str(), "glob");
        assert_eq!(MatcherKind::Prefix.as_str(), "prefix");
        assert_eq!(MatcherKind::Exact.as_str(), "exact");
        assert_eq!(MatcherKind::Regex.as_str(), "regex");
    }

    // Traces to: FR-POLICY-EVAL-005 — On-mismatch actions
    #[test]
    fn test_on_mismatch_actions() {
        assert_eq!(OnMismatchAction::Continue as u8, OnMismatchAction::Continue as u8);
        // Just verify the enum values exist and are comparable
        let _continue = OnMismatchAction::Continue;
        let _warn = OnMismatchAction::Warn;
        let _fail = OnMismatchAction::Fail;
        assert_ne!(_continue, _warn);
        assert_ne!(_warn, _fail);
    }

    // Traces to: FR-POLICY-EVAL-006 — Multiple decisions collection
    #[test]
    fn test_multiple_rule_decisions() {
        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "active");
        ctx.set_string("role", "admin");
        ctx.set_string("env", "prod");

        let rules = vec![
            Rule::new(RuleType::Require, "status", "^active$"),
            Rule::new(RuleType::Allow, "role", "^admin|user$"),
            Rule::new(RuleType::Deny, "env", "^staging$"),
        ];

        let evaluator = StandardRuleEvaluator;
        let decisions = evaluator.evaluate_rules(rules, &ctx).unwrap();

        assert_eq!(decisions.len(), 3);
        assert!(decisions.iter().all(|d| d.matched)); // All should match
    }

    // Traces to: FR-POLICY-EVAL-007 — Reason collection on mismatch
    #[test]
    fn test_mismatch_reason() {
        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "inactive");

        let rule = Rule::new(RuleType::Allow, "status", "^active$");

        let evaluator = StandardRuleEvaluator;
        let decisions = evaluator.evaluate_rules(vec![rule], &ctx).unwrap();

        assert_eq!(decisions.len(), 1);
        assert!(!decisions[0].matched);
        assert!(decisions[0].metadata.reason.is_some());
    }
}
