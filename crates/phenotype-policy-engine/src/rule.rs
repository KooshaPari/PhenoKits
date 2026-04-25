//! Policy rules - Allow, Deny, Require with pattern matching.

use crate::context::EvaluationContext;
use crate::error::PolicyEngineError;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Types of policy rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleType {
    /// Allow rule: value matching pattern is allowed.
    Allow,
    /// Deny rule: value matching pattern is denied.
    Deny,
    /// Require rule: fact must exist and match pattern.
    Require,
}

impl RuleType {
    /// Returns a string representation of the rule type.
    pub fn as_str(&self) -> &'static str {
        match self {
            RuleType::Allow => "Allow",
            RuleType::Deny => "Deny",
            RuleType::Require => "Require",
        }
    }
}

impl std::fmt::Display for RuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Matcher kind enumeration for flexible pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatcherKind {
    /// Glob pattern matching (e.g., "*.rs").
    Glob,
    /// Prefix matching (e.g., "dev_").
    Prefix,
    /// Exact string matching.
    Exact,
    /// Regular expression matching.
    Regex,
}

impl MatcherKind {
    /// Returns a string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            MatcherKind::Glob => "glob",
            MatcherKind::Prefix => "prefix",
            MatcherKind::Exact => "exact",
            MatcherKind::Regex => "regex",
        }
    }
}

impl std::fmt::Display for MatcherKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for MatcherKind {
    fn default() -> Self {
        MatcherKind::Regex
    }
}

/// Action to take when a rule pattern does not match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnMismatchAction {
    /// Continue evaluation (ignore mismatch).
    Continue,
    /// Log a warning but continue.
    Warn,
    /// Stop and fail immediately.
    Fail,
}

impl OnMismatchAction {
    /// Returns a string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            OnMismatchAction::Continue => "continue",
            OnMismatchAction::Warn => "warn",
            OnMismatchAction::Fail => "fail",
        }
    }
}

impl std::fmt::Display for OnMismatchAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for OnMismatchAction {
    fn default() -> Self {
        OnMismatchAction::Continue
    }
}

/// Metadata for a rule (rule_id, source).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMetadata {
    /// Unique rule identifier.
    pub rule_id: String,
    /// Source of the rule (e.g., "policy_lib.py", "system").
    pub source: String,
}

/// A policy rule with pattern matching and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// The type of rule (Allow, Deny, Require).
    pub rule_type: RuleType,
    /// The fact key to evaluate.
    pub fact: String,
    /// The regex pattern to match against the fact value.
    pub pattern: String,
    /// Optional human-readable description of the rule.
    pub description: Option<String>,
    /// Matcher kind used for pattern matching.
    pub matcher_kind: MatcherKind,
    /// Action to take on mismatch.
    pub on_mismatch: OnMismatchAction,
    /// Optional metadata (rule_id, source).
    pub metadata: Option<RuleMetadata>,
}

impl Rule {
    /// Creates a new rule.
    pub fn new(rule_type: RuleType, fact: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            rule_type,
            fact: fact.into(),
            pattern: pattern.into(),
            description: None,
            matcher_kind: MatcherKind::Regex,
            on_mismatch: OnMismatchAction::Continue,
            metadata: None,
        }
    }

    /// Sets the description of the rule.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the matcher kind.
    pub fn with_matcher(mut self, matcher_kind: MatcherKind) -> Self {
        self.matcher_kind = matcher_kind;
        self
    }

    /// Sets the on_mismatch action.
    pub fn with_on_mismatch(mut self, action: OnMismatchAction) -> Self {
        self.on_mismatch = action;
        self
    }

    /// Sets the rule metadata.
    pub fn set_metadata(&mut self, metadata: RuleMetadata) {
        self.metadata = Some(metadata);
    }

    /// Evaluates this rule against a context.
    ///
    /// Returns Ok(true) if the rule is satisfied, Ok(false) if violated.
    /// Returns Err if regex compilation or evaluation fails.
    pub fn evaluate(&self, context: &EvaluationContext) -> Result<bool, PolicyEngineError> {
        let regex =
            Regex::new(&self.pattern).map_err(|e| PolicyEngineError::RegexCompilationError {
                pattern: self.pattern.clone(),
                source: e,
            })?;

        let fact_value = context.get_string(&self.fact);

        match self.rule_type {
            RuleType::Allow => {
                // Allow: fact must match pattern, or fact not exist is OK
                match fact_value {
                    Some(value) => Ok(regex.is_match(&value)),
                    None => Ok(true), // Absence is allowed
                }
            }
            RuleType::Deny => {
                // Deny: fact must NOT match pattern
                match fact_value {
                    Some(value) => Ok(!regex.is_match(&value)),
                    None => Ok(true), // Absence is allowed (not denied)
                }
            }
            RuleType::Require => {
                // Require: fact must exist AND match pattern
                match fact_value {
                    Some(value) => Ok(regex.is_match(&value)),
                    None => Ok(false), // Missing fact fails Require
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_type_display() {
        assert_eq!(RuleType::Allow.as_str(), "Allow");
        assert_eq!(RuleType::Deny.as_str(), "Deny");
        assert_eq!(RuleType::Require.as_str(), "Require");
    }

    #[test]
    fn test_allow_rule_matching() {
        let rule = Rule::new(RuleType::Allow, "status", "^active$");
        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "active");

        assert!(rule.evaluate(&ctx).unwrap());
    }

    #[test]
    fn test_allow_rule_non_matching() {
        let rule = Rule::new(RuleType::Allow, "status", "^active$");
        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "inactive");

        assert!(!rule.evaluate(&ctx).unwrap());
    }

    #[test]
    fn test_allow_rule_missing_fact() {
        let rule = Rule::new(RuleType::Allow, "status", "^active$");
        let ctx = EvaluationContext::new();

        // Missing fact is allowed
        assert!(rule.evaluate(&ctx).unwrap());
    }

    #[test]
    fn test_deny_rule_matching() {
        let rule = Rule::new(RuleType::Deny, "status", "^banned$");
        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "banned");

        // Deny fails when pattern matches
        assert!(!rule.evaluate(&ctx).unwrap());
    }

    #[test]
    fn test_deny_rule_non_matching() {
        let rule = Rule::new(RuleType::Deny, "status", "^banned$");
        let mut ctx = EvaluationContext::new();
        ctx.set_string("status", "active");

        // Deny succeeds when pattern doesn't match
        assert!(rule.evaluate(&ctx).unwrap());
    }

    #[test]
    fn test_require_rule_matching() {
        let rule = Rule::new(RuleType::Require, "email", "^[a-z]+@example\\.com$");
        let mut ctx = EvaluationContext::new();
        ctx.set_string("email", "user@example.com");

        assert!(rule.evaluate(&ctx).unwrap());
    }

    #[test]
    fn test_require_rule_missing() {
        let rule = Rule::new(RuleType::Require, "email", "^[a-z]+@example\\.com$");
        let ctx = EvaluationContext::new();

        // Require fails when fact missing
        assert!(!rule.evaluate(&ctx).unwrap());
    }

    #[test]
    fn test_rule_with_description() {
        let rule = Rule::new(RuleType::Allow, "role", "admin|user")
            .with_description("User must have valid role");

        assert_eq!(
            rule.description,
            Some("User must have valid role".to_string())
        );
    }

    #[test]
    fn test_invalid_regex() {
        let rule = Rule::new(RuleType::Allow, "field", "[invalid");
        let ctx = EvaluationContext::new();

        let result = rule.evaluate(&ctx);
        assert!(result.is_err());
    }

    // Traces to: FR-POLICY-RULE-MATCHER-001 — Glob matcher kind
    #[test]
    fn test_rule_with_glob_matcher() {
        let rule = Rule::new(RuleType::Allow, "filename", "*.rs").with_matcher(MatcherKind::Glob);
        assert_eq!(rule.matcher_kind, MatcherKind::Glob);
    }

    // Traces to: FR-POLICY-RULE-MATCHER-002 — Prefix matcher kind
    #[test]
    fn test_rule_with_prefix_matcher() {
        let rule = Rule::new(RuleType::Allow, "var", "dev_").with_matcher(MatcherKind::Prefix);
        assert_eq!(rule.matcher_kind, MatcherKind::Prefix);
    }

    // Traces to: FR-POLICY-RULE-MATCHER-003 — Exact matcher kind
    #[test]
    fn test_rule_with_exact_matcher() {
        let rule = Rule::new(RuleType::Allow, "env", "production").with_matcher(MatcherKind::Exact);
        assert_eq!(rule.matcher_kind, MatcherKind::Exact);
    }

    // Traces to: FR-POLICY-RULE-MATCHER-004 — Regex matcher kind (default)
    #[test]
    fn test_rule_default_regex_matcher() {
        let rule = Rule::new(RuleType::Allow, "field", "^value$");
        assert_eq!(rule.matcher_kind, MatcherKind::Regex);
    }

    // Traces to: FR-POLICY-RULE-MISMATCH-001 — Continue on mismatch
    #[test]
    fn test_rule_on_mismatch_continue() {
        let rule = Rule::new(RuleType::Allow, "field", "^value$")
            .with_on_mismatch(OnMismatchAction::Continue);
        assert_eq!(rule.on_mismatch, OnMismatchAction::Continue);
    }

    // Traces to: FR-POLICY-RULE-MISMATCH-002 — Warn on mismatch
    #[test]
    fn test_rule_on_mismatch_warn() {
        let rule = Rule::new(RuleType::Allow, "field", "^value$")
            .with_on_mismatch(OnMismatchAction::Warn);
        assert_eq!(rule.on_mismatch, OnMismatchAction::Warn);
    }

    // Traces to: FR-POLICY-RULE-MISMATCH-003 — Fail on mismatch
    #[test]
    fn test_rule_on_mismatch_fail() {
        let rule = Rule::new(RuleType::Allow, "field", "^value$")
            .with_on_mismatch(OnMismatchAction::Fail);
        assert_eq!(rule.on_mismatch, OnMismatchAction::Fail);
    }

    // Traces to: FR-POLICY-RULE-METADATA-001 — Rule metadata
    #[test]
    fn test_rule_with_metadata() {
        let mut rule = Rule::new(RuleType::Allow, "status", "^active$");
        let metadata = RuleMetadata {
            rule_id: "rule-123".to_string(),
            source: "policy_lib".to_string(),
        };
        rule.set_metadata(metadata);

        assert!(rule.metadata.is_some());
        let m = rule.metadata.unwrap();
        assert_eq!(m.rule_id, "rule-123");
        assert_eq!(m.source, "policy_lib");
    }

    // Traces to: FR-POLICY-RULE-COMPLEX-001 — Rule with all metadata fields
    #[test]
    fn test_rule_full_configuration() {
        let mut rule = Rule::new(RuleType::Require, "env", "^(prod|staging)$")
            .with_description("Environment must be prod or staging")
            .with_matcher(MatcherKind::Regex)
            .with_on_mismatch(OnMismatchAction::Fail);

        rule.set_metadata(RuleMetadata {
            rule_id: "env-deploy-001".to_string(),
            source: "deploy_policy".to_string(),
        });

        assert_eq!(rule.rule_type, RuleType::Require);
        assert_eq!(rule.matcher_kind, MatcherKind::Regex);
        assert_eq!(rule.on_mismatch, OnMismatchAction::Fail);
        assert!(rule.description.is_some());
        assert!(rule.metadata.is_some());
    }

    // Traces to: FR-POLICY-RULE-DISPLAY-001 — OnMismatchAction display
    #[test]
    fn test_on_mismatch_action_display() {
        assert_eq!(OnMismatchAction::Continue.as_str(), "continue");
        assert_eq!(OnMismatchAction::Warn.as_str(), "warn");
        assert_eq!(OnMismatchAction::Fail.as_str(), "fail");
    }

    // Traces to: FR-POLICY-RULE-DISPLAY-002 — MatcherKind display
    #[test]
    fn test_matcher_kind_display() {
        assert_eq!(MatcherKind::Glob.as_str(), "glob");
        assert_eq!(MatcherKind::Prefix.as_str(), "prefix");
        assert_eq!(MatcherKind::Exact.as_str(), "exact");
        assert_eq!(MatcherKind::Regex.as_str(), "regex");
    }
}
