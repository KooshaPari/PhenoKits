//! Generic policy evaluation engine for Phenotype.
//!
//! This crate provides a flexible, domain-agnostic policy engine that can evaluate
//! policies against evaluation contexts. It supports multiple rule types (Allow, Deny, Require)
//! and pattern matching via regular expressions.
//!
//! # Core Concepts
//!
//! - **Policy**: A named set of evaluation rules with a description.
//! - **Rule**: A constraint that can be evaluated against a context (Allow, Deny, Require).
//! - **ConditionGroup**: Nested conditions with all/any logic and required/optional flags.
//! - **RuleEvaluator**: Batch rule evaluation with decision tracing and metadata.
//! - **EvaluationContext**: A key-value map of facts used for policy evaluation.
//! - **PolicyResult**: The outcome of policy evaluation, including any violations.
//! - **PolicyEngine**: Orchestrator that evaluates contexts against a set of policies.

pub mod condition_group;
pub mod context;
pub mod engine;
pub mod error;
pub mod loader;
pub mod policy;
pub mod result;
pub mod rule;
pub mod rule_evaluator;

pub use condition_group::{Condition, ConditionGroup, LogicalOp};
pub use context::EvaluationContext;
pub use engine::PolicyEngine;
pub use error::PolicyEngineError;
pub use policy::Policy;
pub use result::{PolicyResult, Severity, Violation};
pub use rule::{MatcherKind, OnMismatchAction, Rule, RuleMetadata, RuleType};
pub use rule_evaluator::{Decision, DecisionMetadata, RuleEvaluationEngine, StandardRuleEvaluator};

/// Re-export commonly used types for convenience.
pub mod prelude {
    pub use crate::{
        condition_group::{Condition, ConditionGroup, LogicalOp},
        context::EvaluationContext,
        engine::PolicyEngine,
        error::PolicyEngineError,
        policy::Policy,
        result::{PolicyResult, Severity, Violation},
        rule::{MatcherKind, OnMismatchAction, Rule, RuleMetadata, RuleType},
        rule_evaluator::{Decision, DecisionMetadata, RuleEvaluationEngine, StandardRuleEvaluator},
    };
}
