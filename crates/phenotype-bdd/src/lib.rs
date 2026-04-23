//! Behavior-Driven Development (BDD) testing utilities for Phenotype
//!
//! Provides types and traits for writing Given-When-Then style tests
//! compatible with Gherkin syntax.
//!
//! # Example
//!
//! ```rust
//! use phenotype_bdd::{Scenario, StepType, Step};
//!
//! let scenario = Scenario::new("User login")
//!     .given("user has valid credentials")
//!     .when("user submits login form")
//!     .then("user is authenticated");
//! ```
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

/// Errors that can occur during BDD execution
#[derive(Debug, Clone, PartialEq)]
pub enum BddError {
    /// Step definition not found
    StepNotFound(String),
    /// Step execution failed
    StepFailed {
        /// The step that failed
        step: String,
        /// The reason for failure
        reason: String,
    },
    /// Parse error in scenario
    ParseError {
        /// Line number where error occurred
        line: usize,
        /// Error message
        message: String,
    },
    /// Missing step type
    MissingStepType(String),
    /// Duplicate scenario name
    DuplicateScenario(String),
    /// Hook execution failed
    HookFailed {
        /// The hook that failed
        hook: String,
        /// The reason for failure
        reason: String,
    },
    /// Invalid parameter in step
    InvalidParameter {
        /// The step with invalid parameter
        step: String,
        /// The invalid parameter
        param: String,
    },
    /// Scenario timeout
    Timeout(String),
}

impl fmt::Display for BddError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BddError::StepNotFound(step) => write!(f, "Step definition not found: {step}"),
            BddError::StepFailed { step, reason } => {
                write!(f, "Step '{step}' failed: {reason}")
            }
            BddError::ParseError { line, message } => {
                write!(f, "Parse error at line {line}: {message}")
            }
            BddError::MissingStepType(step) => {
                write!(f, "Missing step type for: {step}")
            }
            BddError::DuplicateScenario(name) => {
                write!(f, "Duplicate scenario name: {name}")
            }
            BddError::HookFailed { hook, reason } => {
                write!(f, "Hook '{hook}' failed: {reason}")
            }
            BddError::InvalidParameter { step, param } => {
                write!(f, "Invalid parameter '{param}' in step: {step}")
            }
            BddError::Timeout(scenario) => {
                write!(f, "Scenario '{scenario}' timed out")
            }
        }
    }
}

impl std::error::Error for BddError {}

/// Step types in BDD scenarios
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StepType {
    /// Setup/precondition step
    Given,
    /// Action step
    When,
    /// Assertion step
    Then,
    /// Additional precondition (avoids repeating Given)
    And,
    /// Alternative precondition or assertion
    But,
    /// Background steps (run before each scenario)
    Background,
}

impl fmt::Display for StepType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepType::Given => write!(f, "Given"),
            StepType::When => write!(f, "When"),
            StepType::Then => write!(f, "Then"),
            StepType::And => write!(f, "And"),
            StepType::But => write!(f, "But"),
            StepType::Background => write!(f, "Background"),
        }
    }
}

impl StepType {
    /// Parse a step type from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "Given" => Some(StepType::Given),
            "When" => Some(StepType::When),
            "Then" => Some(StepType::Then),
            "And" => Some(StepType::And),
            "But" => Some(StepType::But),
            "Background" => Some(StepType::Background),
            _ => None,
        }
    }
}

/// A single step in a BDD scenario
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    /// Type of step
    pub step_type: StepType,
    /// The step text/pattern
    pub text: String,
    /// Table data attached to the step (optional)
    pub table: Option<Table>,
    /// Doc string attached to the step (optional)
    pub doc_string: Option<String>,
    /// Line number for error reporting
    pub line: usize,
}

impl Step {
    /// Create a new step
    #[must_use]
    pub fn new(step_type: StepType, text: impl Into<String>) -> Self {
        Self {
            step_type,
            text: text.into(),
            table: None,
            doc_string: None,
            line: 0,
        }
    }

    /// Add a table to the step
    #[must_use]
    pub fn with_table(mut self, table: Table) -> Self {
        self.table = Some(table);
        self
    }

    /// Add a doc string to the step
    #[must_use]
    pub fn with_doc_string(mut self, doc: impl Into<String>) -> Self {
        self.doc_string = Some(doc.into());
        self
    }

    /// Set line number
    #[must_use]
    pub fn with_line(mut self, line: usize) -> Self {
        self.line = line;
        self
    }

    /// Get step type in canonical form (Given/When/Then)
    pub fn canonical_type(&self) -> StepType {
        match self.step_type {
            StepType::And | StepType::But => StepType::Given, // Inherit from previous
            other => other,
        }
    }
}

/// A Gherkin-style data table
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Table {
    /// Table header row
    pub header: Vec<String>,
    /// Table data rows
    pub rows: Vec<Vec<String>>,
}

impl Table {
    /// Create a new empty table
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a table from header and rows
    pub fn from_rows(header: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self { header, rows }
    }

    /// Add a row to the table
    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    /// Get row as a map (column name -> value)
    pub fn row_as_map(&self, row_index: usize) -> Option<HashMap<String, String>> {
        self.rows.get(row_index).map(|row| {
            self.header
                .iter()
                .cloned()
                .zip(row.iter().cloned())
                .collect()
        })
    }

    /// Convert table to list of maps
    pub fn to_maps(&self) -> Vec<HashMap<String, String>> {
        (0..self.rows.len())
            .filter_map(|i| self.row_as_map(i))
            .collect()
    }
}

/// A BDD scenario containing multiple steps
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scenario {
    /// Scenario name
    pub name: String,
    /// Steps in the scenario
    pub steps: Vec<Step>,
    /// Tags for filtering
    pub tags: Vec<String>,
    /// Background steps (if any)
    pub background: Option<Vec<Step>>,
    /// Examples for scenario outlines
    pub examples: Option<Table>,
}

impl Scenario {
    /// Create a new scenario
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            steps: Vec::new(),
            tags: Vec::new(),
            background: None,
            examples: None,
        }
    }

    /// Add a tag
    #[must_use]
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add multiple tags
    #[must_use]
    pub fn with_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags.extend(tags.into_iter().map(|t| t.into()));
        self
    }

    /// Add a Given step
    #[must_use]
    pub fn given(mut self, text: impl Into<String>) -> Self {
        self.steps.push(Step::new(StepType::Given, text));
        self
    }

    /// Add a When step
    #[must_use]
    pub fn when(mut self, text: impl Into<String>) -> Self {
        self.steps.push(Step::new(StepType::When, text));
        self
    }

    /// Add a Then step
    #[must_use]
    pub fn then(mut self, text: impl Into<String>) -> Self {
        self.steps.push(Step::new(StepType::Then, text));
        self
    }

    /// Add an And step
    #[must_use]
    pub fn and(mut self, text: impl Into<String>) -> Self {
        self.steps.push(Step::new(StepType::And, text));
        self
    }

    /// Add a But step
    #[must_use]
    pub fn but(mut self, text: impl Into<String>) -> Self {
        self.steps.push(Step::new(StepType::But, text));
        self
    }

    /// Set background steps
    #[must_use]
    pub fn with_background(mut self, steps: Vec<Step>) -> Self {
        self.background = Some(steps);
        self
    }

    /// Set examples table for scenario outlines
    #[must_use]
    pub fn with_examples(mut self, examples: Table) -> Self {
        self.examples = Some(examples);
        self
    }

    /// Get all effective steps (background + scenario steps)
    pub fn effective_steps(&self) -> Vec<&Step> {
        let mut result = Vec::new();
        if let Some(ref bg) = self.background {
            result.extend(bg.iter());
        }
        result.extend(self.steps.iter());
        result
    }

    /// Mark this scenario as skipped
    ///
    /// This is useful when a scenario should not be executed due to
    /// preconditions not being met or when it is temporarily disabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::Scenario;
    ///
    /// let scenario = Scenario::new("Feature X").skip("Feature not ready");
    /// ```
    #[must_use]
    pub fn skip(mut self, reason: impl Into<String>) -> Self {
        self.tags.push(format!("skip: {}", reason.into()));
        self
    }

    /// Check if this scenario is marked to be skipped
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::Scenario;
    ///
    /// let scenario = Scenario::new("Test").skip("not ready");
    /// assert!(scenario.is_skipped());
    ///
    /// let normal = Scenario::new("Normal");
    /// assert!(!normal.is_skipped());
    /// ```
    #[must_use]
    pub fn is_skipped(&self) -> bool {
        self.tags.iter().any(|t| t.starts_with("skip:"))
    }
}

/// A BDD feature containing multiple scenarios
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Feature {
    /// Feature name
    pub name: String,
    /// Feature description
    pub description: Option<String>,
    /// Scenarios in the feature
    pub scenarios: Vec<Scenario>,
    /// Background steps shared by all scenarios
    pub background: Option<Vec<Step>>,
    /// Tags for all scenarios
    pub tags: Vec<String>,
}

impl Feature {
    /// Create a new feature
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            scenarios: Vec::new(),
            background: None,
            tags: Vec::new(),
        }
    }

    /// Add description
    #[must_use]
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Add a tag
    #[must_use]
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add a scenario
    pub fn add_scenario(&mut self, scenario: Scenario) {
        self.scenarios.push(scenario);
    }

    /// Add a scenario (builder style)
    #[must_use]
    pub fn with_scenario(mut self, scenario: Scenario) -> Self {
        self.add_scenario(scenario);
        self
    }

    /// Set background steps
    #[must_use]
    pub fn with_background(mut self, steps: Vec<Step>) -> Self {
        self.background = Some(steps);
        self
    }

    /// Apply background to all scenarios that don't have one
    pub fn apply_background(&mut self) {
        if let Some(ref bg) = self.background {
            for scenario in &mut self.scenarios {
                if scenario.background.is_none() {
                    scenario.background = Some(bg.clone());
                }
            }
        }
    }

    /// Get scenarios filtered by tags
    pub fn scenarios_with_tags(&self, tags: &[String]) -> Vec<&Scenario> {
        self.scenarios
            .iter()
            .filter(|s| tags.iter().all(|t| s.tags.contains(t)))
            .collect()
    }
}

/// The result of executing a step
#[derive(Debug, Clone, PartialEq)]
pub enum StepResult {
    /// Step passed
    Passed,
    /// Step failed with error
    Failed(String),
    /// Step was skipped
    Skipped(String),
    /// Step is pending (not implemented)
    Pending(String),
}

impl StepResult {
    /// Check if the step passed
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::StepResult;
    ///
    /// assert!(StepResult::Passed.is_passed());
    /// assert!(!StepResult::Failed("error".into()).is_passed());
    /// ```
    #[must_use]
    pub fn is_passed(&self) -> bool {
        matches!(self, Self::Passed)
    }

    /// Check if the step failed
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::StepResult;
    ///
    /// assert!(StepResult::Failed("error".into()).is_failed());
    /// assert!(!StepResult::Passed.is_failed());
    /// ```
    #[must_use]
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed(_))
    }

    /// Check if the step was skipped
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::StepResult;
    ///
    /// assert!(StepResult::Skipped("reason".into()).is_skipped());
    /// assert!(!StepResult::Passed.is_skipped());
    /// ```
    #[must_use]
    pub fn is_skipped(&self) -> bool {
        matches!(self, Self::Skipped(_))
    }

    /// Check if the step is pending (not implemented)
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::StepResult;
    ///
    /// assert!(StepResult::Pending("TODO".into()).is_pending());
    /// assert!(!StepResult::Passed.is_pending());
    /// ```
    #[must_use]
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending(_))
    }

    /// Get the error message if the step did not pass
    ///
    /// Returns `Some` with the message if the step failed, was skipped, or is pending.
    /// Returns `None` if the step passed.
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::StepResult;
    ///
    /// assert_eq!(StepResult::Passed.error_message(), None);
    /// assert_eq!(
    ///     StepResult::Failed("timeout".into()).error_message(),
    ///     Some("timeout")
    /// );
    /// ```
    #[must_use]
    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Failed(msg) | Self::Skipped(msg) | Self::Pending(msg) => Some(msg),
            Self::Passed => None,
        }
    }
}

/// The result of executing a scenario
#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioResult {
    /// Scenario name
    pub scenario_name: String,
    /// Results for each step
    pub step_results: Vec<(Step, StepResult)>,
    /// Overall duration
    pub duration_ms: u64,
    /// Whether the scenario passed
    pub passed: bool,
    /// Error message if failed
    pub error: Option<String>,
}

impl ScenarioResult {
    /// Get the count of passed steps
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::{ScenarioResult, Step, StepResult, StepType};
    ///
    /// let result = ScenarioResult {
    ///     scenario_name: "Test".into(),
    ///     step_results: vec![
    ///         (Step::new(StepType::Given, "step1"), StepResult::Passed),
    ///         (Step::new(StepType::When, "step2"), StepResult::Passed),
    ///     ],
    ///     duration_ms: 100,
    ///     passed: true,
    ///     error: None,
    /// };
    ///
    /// assert_eq!(result.passed_step_count(), 2);
    /// ```
    #[must_use]
    pub fn passed_step_count(&self) -> usize {
        self.step_results.iter().filter(|(_, r)| r.is_passed()).count()
    }

    /// Get the count of failed steps
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::{ScenarioResult, Step, StepResult, StepType};
    ///
    /// let result = ScenarioResult {
    ///     scenario_name: "Test".into(),
    ///     step_results: vec![
    ///         (Step::new(StepType::Given, "step1"), StepResult::Passed),
    ///         (Step::new(StepType::When, "step2"), StepResult::Failed("err".into())),
    ///     ],
    ///     duration_ms: 100,
    ///     passed: false,
    ///     error: Some("err".into()),
    /// };
    ///
    /// assert_eq!(result.failed_step_count(), 1);
    /// ```
    #[must_use]
    pub fn failed_step_count(&self) -> usize {
        self.step_results.iter().filter(|(_, r)| r.is_failed()).count()
    }

    /// Get the count of pending steps
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::{ScenarioResult, Step, StepResult, StepType};
    ///
    /// let result = ScenarioResult {
    ///     scenario_name: "Test".into(),
    ///     step_results: vec![
    ///         (Step::new(StepType::Given, "step1"), StepResult::Passed),
    ///         (Step::new(StepType::When, "step2"), StepResult::Pending("TODO".into())),
    ///     ],
    ///     duration_ms: 100,
    ///     passed: false,
    ///     error: None,
    /// };
    ///
    /// assert_eq!(result.pending_step_count(), 1);
    /// ```
    #[must_use]
    pub fn pending_step_count(&self) -> usize {
        self.step_results.iter().filter(|(_, r)| r.is_pending()).count()
    }

    /// Get total step count
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::{ScenarioResult, Step, StepResult, StepType};
    ///
    /// let result = ScenarioResult {
    ///     scenario_name: "Test".into(),
    ///     step_results: vec![
    ///         (Step::new(StepType::Given, "step1"), StepResult::Passed),
    ///         (Step::new(StepType::When, "step2"), StepResult::Passed),
    ///         (Step::new(StepType::Then, "step3"), StepResult::Failed("err".into())),
    ///     ],
    ///     duration_ms: 100,
    ///     passed: false,
    ///     error: Some("err".into()),
    /// };
    ///
    /// assert_eq!(result.total_step_count(), 3);
    /// ```
    #[must_use]
    pub fn total_step_count(&self) -> usize {
        self.step_results.len()
    }
}

/// Context passed between steps in a scenario
#[derive(Debug, Clone, Default)]
pub struct StepContext {
    /// Key-value storage for sharing data between steps
    pub data: HashMap<String, serde_json::Value>,
    /// Current step index
    pub step_index: usize,
    /// Scenario name
    pub scenario_name: String,
}

impl StepContext {
    /// Create a new context for a scenario
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::StepContext;
    ///
    /// let ctx = StepContext::new("Login Test");
    /// assert_eq!(ctx.scenario_name, "Login Test");
    /// ```
    #[must_use]
    pub fn new(scenario_name: impl Into<String>) -> Self {
        Self {
            data: HashMap::new(),
            step_index: 0,
            scenario_name: scenario_name.into(),
        }
    }

    /// Get a value from context
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::StepContext;
    ///
    /// let ctx = StepContext::new("Test");
    /// // ctx.set("key", "value");
    /// // assert_eq!(ctx.get("key"), Some(&serde_json::json!("value")));
    /// ```
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    /// Set a value in context
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) {
        self.data.insert(key.into(), value.into());
    }

    /// Check if a key exists in the context
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::StepContext;
    ///
    /// let ctx = StepContext::new("Test");
    /// assert!(!ctx.has("missing"));
    /// ```
    #[must_use]
    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}

/// Type alias for async step functions
pub type AsyncStepFn = Box<
    dyn Fn(StepContext, Step) -> Pin<Box<dyn Future<Output = Result<StepContext, BddError>> + Send>>
        + Send
        + Sync,
>;

/// A step definition that can be executed
pub struct StepDefinition {
    /// Step pattern (regex or exact string)
    pub pattern: String,
    /// Step type this definition handles
    pub step_type: StepType,
    /// The function to execute
    pub handler: AsyncStepFn,
}

impl fmt::Debug for StepDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StepDefinition")
            .field("pattern", &self.pattern)
            .field("step_type", &self.step_type)
            .finish_non_exhaustive()
    }
}

/// Registry of step definitions
#[derive(Default)]
pub struct StepRegistry {
    definitions: Vec<StepDefinition>,
}

impl StepRegistry {
    /// Create a new empty step registry
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::StepRegistry;
    ///
    /// let registry = StepRegistry::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a step definition
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use phenotype_bdd::{StepRegistry, StepDefinition, StepType};
    ///
    /// let mut registry = StepRegistry::new();
    /// // registry.register(StepDefinition { ... });
    /// ```
    pub fn register(&mut self, definition: StepDefinition) {
        self.definitions.push(definition);
    }

    /// Find a step definition matching the given step
    ///
    /// Returns `Some(&StepDefinition)` if a matching pattern is found,
    /// or `None` if no definition matches.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use phenotype_bdd::{StepRegistry, Step, StepType};
    ///
    /// let registry = StepRegistry::new();
    /// let step = Step::new(StepType::Given, "user is logged in");
    /// // let def = registry.find_match(&step);
    /// ```
    #[must_use]
    pub fn find_match(&self, step: &Step) -> Option<&StepDefinition> {
        self.definitions.iter().find(|def| {
            def.step_type == step.canonical_type()
                && Self::pattern_matches(&def.pattern, &step.text)
        })
    }

    /// Check if a pattern matches step text
    ///
    /// Supports exact match and parameterized patterns like `user {name} logs in`.
    fn pattern_matches(pattern: &str, text: &str) -> bool {
        // Simple exact match or regex match
        if pattern == text {
            return true;
        }
        // Check for parameterized patterns like "user {name} logs in"
        let regex_pattern = pattern.replace(r"{", r"(?P<").replace(r"}", r">[^\s]+)");
        if let Ok(re) = regex::Regex::new(&regex_pattern) {
            return re.is_match(text);
        }
        false
    }

    /// Extract named parameters from step text using pattern
    ///
    /// Patterns can use `{param_name}` syntax to capture values.
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::StepRegistry;
    ///
    /// let registry = StepRegistry::new();
    /// let params = registry.extract_params("user {name} logs in", "user alice logs in");
    /// assert_eq!(params.get("name"), Some(&"alice".to_string()));
    /// ```
    #[must_use]
    pub fn extract_params(&self, pattern: &str, text: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        let regex_pattern = pattern.replace(r"{", r"(?P<").replace(r"}", r">[^\s]+)");
        if let Ok(re) = regex::Regex::new(&regex_pattern) {
            if let Some(caps) = re.captures(text) {
                for name in re.capture_names().flatten() {
                    if let Some(val) = caps.name(name) {
                        params.insert(name.to_string(), val.as_str().to_string());
                    }
                }
            }
        }
        params
    }
}

/// BDD Runner for executing scenarios
pub struct BddRunner {
    step_registry: StepRegistry,
    before_hooks: Vec<Box<dyn Fn(&Scenario) -> Result<(), BddError> + Send + Sync>>,
    after_hooks: Vec<Box<dyn Fn(&ScenarioResult) -> Result<(), BddError> + Send + Sync>>,
    fail_fast: bool,
    timeout_ms: u64,
}

impl Default for BddRunner {
    fn default() -> Self {
        Self {
            step_registry: StepRegistry::new(),
            before_hooks: Vec::new(),
            after_hooks: Vec::new(),
            fail_fast: true,
            timeout_ms: 30000, // 30 seconds default
        }
        }
}

impl BddRunner {
    /// Create a new BDD runner with default settings
    ///
    /// Default configuration:
    /// - `fail_fast`: true
    /// - `timeout_ms`: 30 seconds
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::BddRunner;
    ///
    /// let runner = BddRunner::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure fail-fast behavior
    ///
    /// When enabled (default), the runner stops on the first failure.
    /// When disabled, all steps are executed and failures are collected.
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::BddRunner;
    ///
    /// let runner = BddRunner::new().with_fail_fast(false);
    /// ```
    #[must_use]
    pub fn with_fail_fast(mut self, fail_fast: bool) -> Self {
        self.fail_fast = fail_fast;
        self
    }

    /// Configure the timeout for scenario execution
    ///
    /// Default is 30 seconds (30000 ms).
    ///
    /// # Examples
    ///
    /// ```
    /// use phenotype_bdd::BddRunner;
    ///
    /// let runner = BddRunner::new().with_timeout(60000); // 60 seconds
    /// ```
    #[must_use]
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Register a step definition with the runner
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use phenotype_bdd::{BddRunner, StepDefinition, StepType, AsyncStepFn};
    ///
    /// let mut runner = BddRunner::new();
    /// // runner.register_step(StepDefinition { ... });
    /// ```
    pub fn register_step(&mut self, definition: StepDefinition) {
        self.step_registry.register(definition);
    }

    /// Add a hook to run before each scenario
    ///
    /// Hooks are executed in the order they are added.
    /// If any hook fails, the scenario is marked as failed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use phenotype_bdd::{BddRunner, Scenario, BddError};
    ///
    /// let mut runner = BddRunner::new();
    /// runner.before(|scenario: &Scenario| {
    ///     println!("Starting: {}", scenario.name);
    ///     Ok(())
    /// });
    /// ```
    pub fn before<F>(&mut self, hook: F)
    where
        F: Fn(&Scenario) -> Result<(), BddError> + Send + Sync + 'static,
    {
        self.before_hooks.push(Box::new(hook));
    }

    /// Add a hook to run after each scenario
    ///
    /// Hooks are executed in the order they are added.
    /// After hooks always run, even if the scenario fails.
    /// Errors in after hooks are logged but do not affect the scenario result.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use phenotype_bdd::{BddRunner, ScenarioResult, BddError};
    ///
    /// let mut runner = BddRunner::new();
    /// runner.after(|result: &ScenarioResult| {
    ///     if result.passed {
    ///         println!("✓ {}", result.scenario_name);
    ///     } else {
    ///         println!("✗ {}", result.scenario_name);
    ///     }
    ///     Ok(())
    /// });
    /// ```
    pub fn after<F>(&mut self, hook: F)
    where
        F: Fn(&ScenarioResult) -> Result<(), BddError> + Send + Sync + 'static,
    {
        self.after_hooks.push(Box::new(hook));
    }

    /// Execute a single scenario
    ///
    /// Runs all steps in the scenario, including any background steps.
    /// Returns a `ScenarioResult` containing the results of each step
    /// and the overall pass/fail status.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use phenotype_bdd::{BddRunner, Scenario};
    ///
    /// # async fn example() {
    /// let runner = BddRunner::new();
    /// let scenario = Scenario::new("Login Test")
    ///     .given("user exists")
    ///     .when("user logs in")
    ///     .then("login succeeds");
    ///
    /// let result = runner.run_scenario(&scenario).await;
    /// println!("Scenario passed: {}", result.passed);
    /// # }
    /// ```
    #[must_use]
    pub async fn run_scenario(&self, scenario: &Scenario) -> ScenarioResult {
        let start = std::time::Instant::now();
        let mut context = StepContext::new(&scenario.name);
        let mut step_results = Vec::new();
        let mut passed = true;
        let mut error = None;

        // Run before hooks
        for hook in &self.before_hooks {
            if let Err(e) = hook(scenario) {
                error = Some(e.to_string());
                passed = false;
                break;
            }
        }

        if passed {
            // Execute steps
            for (idx, step) in scenario.effective_steps().iter().enumerate() {
                context.step_index = idx;

                // Find matching step definition
                match self.step_registry.find_match(step) {
                    Some(def) => {
                        // Execute the step handler
                        let result = (def.handler)(context.clone(), (*step).clone()).await;
                        match result {
                            Ok(ctx) => {
                                context = ctx;
                                step_results.push(((*step).clone(), StepResult::Passed));
                            }
                            Err(e) => {
                                step_results.push(((*step).clone(), StepResult::Failed(e.to_string())));
                                if self.fail_fast {
                                    passed = false;
                                    error = Some(e.to_string());
                                    break;
                                }
                            }
                        }
                    }
                    None => {
                        let msg = format!("No step definition found for: {}", step.text);
                        step_results.push(((*step).clone(), StepResult::Pending(msg.clone())));
                        if self.fail_fast {
                            passed = false;
                            error = Some(msg);
                            break;
                        }
                    }
                }
            }
        }

        let duration_ms = start.elapsed().as_millis() as u64;

        let result = ScenarioResult {
            scenario_name: scenario.name.clone(),
            step_results,
            duration_ms,
            passed,
            error,
        };

        // Run after hooks
        for hook in &self.after_hooks {
            let _ = hook(&result); // Ignore errors in after hooks
        }

        result
    }

    /// Execute all scenarios in a feature
    ///
    /// Runs each scenario in the feature sequentially and returns
    /// a vector of `ScenarioResult` for each scenario.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use phenotype_bdd::{BddRunner, Feature, Scenario};
    ///
    /// # async fn example() {
    /// let runner = BddRunner::new();
    /// let mut feature = Feature::new("User Management")
    ///     .with_scenario(Scenario::new("Create User").given("admin exists"))
    ///     .with_scenario(Scenario::new("Delete User").given("user exists"));
    ///
    /// let results = runner.run_feature(&feature).await;
    /// let passed_count = results.iter().filter(|r| r.passed).count();
    /// println!("Passed: {}/{}", passed_count, results.len());
    /// # }
    /// ```
    #[must_use]
    pub async fn run_feature(&self, feature: &Feature) -> Vec<ScenarioResult> {
        let mut results = Vec::new();
        for scenario in &feature.scenarios {
            results.push(self.run_scenario(scenario).await);
        }
        results
    }
}

/// Parser for Gherkin feature files
pub mod parser {
    use super::*;

    /// Track where the last step was added
    #[derive(Debug, Clone, Copy)]
    enum LastStepLocation {
        Scenario(usize), // index in scenario.steps
        Background(usize), // index in background vec
        None,
    }

    /// Parse a Gherkin feature file content
    pub fn parse_feature(content: &str) -> Result<Feature, BddError> {
        let mut lines = content.lines().enumerate().peekable();
        let mut feature = Feature::default();
        let mut current_scenario: Option<Scenario> = None;
        let mut current_background: Option<Vec<Step>> = None;
        let mut in_doc_string = false;
        let mut doc_string_lines: Vec<String> = Vec::new();
        let mut doc_string_delimiter: String = String::new();
        let mut last_step_location: LastStepLocation = LastStepLocation::None;
        let mut pending_tags: Vec<String> = Vec::new();

        while let Some((_line_num, line)) = lines.next() {
            let trimmed = line.trim();

            // Skip empty lines and comments (except when in doc string)
            if !in_doc_string {
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
            }

            // Handle doc strings
            if in_doc_string {
                if trimmed == doc_string_delimiter {
                    in_doc_string = false;
                    let doc_content = doc_string_lines.join("\n");
                    // Apply doc string to last step
                    match last_step_location {
                        LastStepLocation::Scenario(idx) => {
                            if let Some(ref mut scenario) = current_scenario {
                                if let Some(step) = scenario.steps.get_mut(idx) {
                                    step.doc_string = Some(doc_content);
                                }
                            }
                        }
                        LastStepLocation::Background(idx) => {
                            if let Some(ref mut bg) = current_background {
                                if let Some(step) = bg.get_mut(idx) {
                                    step.doc_string = Some(doc_content);
                                }
                            }
                        }
                        LastStepLocation::None => {}
                    }
                    doc_string_lines.clear();
                } else {
                    doc_string_lines.push(line.to_string());
                }
                continue;
            }

            // Feature declaration
            if trimmed.starts_with("Feature:") {
                feature.name = trimmed["Feature:".len()..].trim().to_string();
                // Collect description lines
                let mut description_lines = Vec::new();
                while let Some((_, next_line)) = lines.peek() {
                    let next_trimmed = next_line.trim();
                    if next_trimmed.is_empty()
                        || next_trimmed.starts_with("Scenario")
                        || next_trimmed.starts_with("Background")
                        || next_trimmed.starts_with('@')
                    {
                        break;
                    }
                    description_lines.push(lines.next().unwrap().1.trim().to_string());
                }
                if !description_lines.is_empty() {
                    feature.description = Some(description_lines.join("\n"));
                }
                // Apply any pending tags to the feature
                if !pending_tags.is_empty() {
                    feature.tags.extend(pending_tags.drain(..));
                }
                continue;
            }

            // Tags - collect into pending_tags, will be applied to next scenario or feature
            if trimmed.starts_with('@') {
                let tags: Vec<String> = trimmed
                    .split_whitespace()
                    .map(|t| t.trim_start_matches('@').to_string())
                    .collect();
                pending_tags.extend(tags);
                continue;
            }

            // Background
            if trimmed.starts_with("Background:") {
                current_background = Some(Vec::new());
                continue;
            }

            // Scenario or Scenario Outline
            if trimmed.starts_with("Scenario Outline:") || trimmed.starts_with("Scenario:") {
                // Save current scenario if any
                if let Some(scenario) = current_scenario.take() {
                    feature.scenarios.push(scenario);
                }

                let name = if trimmed.starts_with("Scenario Outline:") {
                    trimmed["Scenario Outline:".len()..].trim().to_string()
                } else {
                    trimmed["Scenario:".len()..].trim().to_string()
                };

                let mut scenario = Scenario::new(name);
                if let Some(ref bg) = current_background {
                    scenario.background = Some(bg.clone());
                }
                // Apply any pending tags to this scenario
                if !pending_tags.is_empty() {
                    scenario.tags.extend(pending_tags.drain(..));
                }
                current_scenario = Some(scenario);
                last_step_location = LastStepLocation::None;
                continue;
            }

            // Examples (for Scenario Outline)
            if trimmed.starts_with("Examples:") {
                // Parse table following Examples
                if let Some((_, next_line)) = lines.peek() {
                    if next_line.trim().starts_with('|') {
                        let table = parse_table(&mut lines)?;
                        if let Some(ref mut scenario) = current_scenario {
                            scenario.examples = Some(table);
                        }
                    }
                }
                continue;
            }

            // Step definitions
            if let Some(step_type) = StepType::from_str(trimmed.split_whitespace().next().unwrap_or("")) {
                let text = trimmed.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");
                let step = Step::new(step_type, text).with_line(_line_num + 1);

                if let Some(ref mut scenario) = current_scenario {
                    let idx = scenario.steps.len();
                    scenario.steps.push(step);
                    last_step_location = LastStepLocation::Scenario(idx);
                } else if let Some(ref mut bg) = current_background {
                    let idx = bg.len();
                    bg.push(step);
                    last_step_location = LastStepLocation::Background(idx);
                }
                continue;
            }

            // Data table attached to step
            if trimmed.starts_with('|') {
                let table = parse_table(&mut lines)?;
                match last_step_location {
                    LastStepLocation::Scenario(idx) => {
                        if let Some(ref mut scenario) = current_scenario {
                            if let Some(step) = scenario.steps.get_mut(idx) {
                                step.table = Some(table);
                            }
                        }
                    }
                    LastStepLocation::Background(idx) => {
                        if let Some(ref mut bg) = current_background {
                            if let Some(step) = bg.get_mut(idx) {
                                step.table = Some(table);
                            }
                        }
                    }
                    LastStepLocation::None => {}
                }
                continue;
            }

            // Doc string start
            if trimmed.starts_with("\"") || trimmed.starts_with("```") {
                in_doc_string = true;
                doc_string_delimiter = if trimmed.starts_with("\"") {
                    "\"\"\"".to_string()
                } else {
                    "```".to_string()
                };
                continue;
            }
        }

        // Save last scenario
        if let Some(scenario) = current_scenario.take() {
            feature.scenarios.push(scenario);
        }

        // Save background to feature
        if let Some(bg) = current_background {
            feature.background = Some(bg);
        }

        // Apply any remaining pending tags to the feature
        if !pending_tags.is_empty() {
            feature.tags.extend(pending_tags);
        }

        if feature.name.is_empty() {
            return Err(BddError::ParseError {
                line: 0,
                message: "No Feature declaration found".to_string(),
            });
        }

        Ok(feature)
    }

    /// Parse a data table from lines
    fn parse_table<'a>(
        lines: &mut std::iter::Peekable<impl Iterator<Item = (usize, &'a str)>>,
    ) -> Result<Table, BddError> {
        let mut table = Table::new();
        let mut is_first = true;

        while let Some((_line_num, line)) = lines.peek() {
            let trimmed = line.trim();
            if !trimmed.starts_with('|') {
                break;
            }

            let cells: Vec<String> = trimmed
                .split('|')
                .skip(1)
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect();

            if is_first {
                table.header = cells;
                is_first = false;
            } else {
                table.rows.push(cells);
            }

            lines.next();
        }

        Ok(table)
    }

    /// Parse a single scenario from text
    pub fn parse_scenario(name: impl Into<String>, content: &str) -> Result<Scenario, BddError> {
        let mut scenario = Scenario::new(name);

        for (_line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(step_type) = StepType::from_str(trimmed.split_whitespace().next().unwrap_or("")) {
                let text = trimmed.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");
                scenario.steps.push(Step::new(step_type, text).with_line(_line_num + 1));
            }
        }

        Ok(scenario)
    }
}

/// Builder macros for creating BDD scenarios
#[macro_export]
macro_rules! scenario {
    ($name:expr) => {
        $crate::Scenario::new($name)
    };
    ($name:expr, $($step_type:ident $text:expr),+ $(,)?) => {{
        let mut scenario = $crate::Scenario::new($name);
        $(
            scenario.steps.push($crate::Step::new(
                $crate::StepType::$step_type,
                $text
            ));
        )+
        scenario
    }};
}

/// Macro to define a step
#[macro_export]
macro_rules! step {
    ($step_type:ident $text:expr) => {
        $crate::Step::new($crate::StepType::$step_type, $text)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_type_display() {
        assert_eq!(StepType::Given.to_string(), "Given");
        assert_eq!(StepType::When.to_string(), "When");
        assert_eq!(StepType::Then.to_string(), "Then");
    }

    #[test]
    fn test_scenario_builder() {
        let scenario = Scenario::new("Test Login")
            .given("user has valid credentials")
            .when("user submits login form")
            .then("user is logged in");

        assert_eq!(scenario.name, "Test Login");
        assert_eq!(scenario.steps.len(), 3);
        assert_eq!(scenario.steps[0].step_type, StepType::Given);
        assert_eq!(scenario.steps[1].step_type, StepType::When);
        assert_eq!(scenario.steps[2].step_type, StepType::Then);
    }

    #[test]
    fn test_feature_builder() {
        let feature = Feature::new("User Management")
            .with_description("Managing users in the system")
            .with_scenario(Scenario::new("Create User").given("admin is logged in"));

        assert_eq!(feature.name, "User Management");
        assert_eq!(feature.description.as_ref().unwrap(), "Managing users in the system");
        assert_eq!(feature.scenarios.len(), 1);
    }

    #[test]
    fn test_table_operations() {
        let mut table = Table::new();
        table.header = vec!["name".to_string(), "age".to_string()];
        table.rows = vec![
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];

        let maps = table.to_maps();
        assert_eq!(maps.len(), 2);
        assert_eq!(maps[0].get("name"), Some(&String::from("Alice")));
        assert_eq!(maps[1].get("age"), Some(&String::from("25")));
    }

    #[test]
    fn test_parse_feature() {
        let content = r#"
Feature: User Login
  As a user
  I want to log in
  So that I can access my account

  Scenario: Valid login
    Given user has valid credentials
    When user submits login form
    Then user is logged in
"#;

        let feature = parser::parse_feature(content).unwrap();
        assert_eq!(feature.name, "User Login");
        assert!(feature.description.is_some());
        assert_eq!(feature.scenarios.len(), 1);
        assert_eq!(feature.scenarios[0].name, "Valid login");
        assert_eq!(feature.scenarios[0].steps.len(), 3);
    }

    #[test]
    fn test_bdd_error_display() {
        let err = BddError::StepNotFound("Given user is logged in".to_string());
        assert!(err.to_string().contains("Step definition not found"));

        let err = BddError::StepFailed {
            step: "Click button".to_string(),
            reason: "Element not found".to_string(),
        };
        assert!(err.to_string().contains("Click button"));
        assert!(err.to_string().contains("Element not found"));
    }

    #[test]
    fn test_step_registry_pattern_matching() {
        let registry = StepRegistry::new();

        // Test that patterns with parameters work
        let pattern = "user {name} logs in";
        let text = "user alice logs in";

        let params = registry.extract_params(pattern, text);
        assert_eq!(params.get("name"), Some(&"alice".to_string()));
    }

    #[test]
    fn test_step_context() {
        let mut ctx = StepContext::new("Test Scenario");
        ctx.set("user_id", "12345");
        ctx.set("authenticated", true);

        assert_eq!(ctx.get("user_id"), Some(&serde_json::json!("12345")));
        assert_eq!(ctx.get("authenticated"), Some(&serde_json::json!(true)));
        assert!(ctx.has("user_id"));
        assert!(!ctx.has("nonexistent"));
    }
}
