"""Type stubs for phenotype-policy-engine-py PyO3 bindings."""

from typing import Any, Dict, List, Optional, Tuple

class LogicalOp:
    """Logical operator for combining conditions."""

    @staticmethod
    def all() -> LogicalOp:
        """All conditions must evaluate to true."""
        ...

    @staticmethod
    def any() -> LogicalOp:
        """At least one condition must evaluate to true."""
        ...

class MatcherKind:
    """Pattern matching kind enumeration."""

    @staticmethod
    def glob() -> MatcherKind:
        """Glob pattern matching (e.g., '*.rs')."""
        ...

    @staticmethod
    def prefix() -> MatcherKind:
        """Prefix matching (e.g., 'dev_')."""
        ...

    @staticmethod
    def exact() -> MatcherKind:
        """Exact string matching."""
        ...

    @staticmethod
    def regex() -> MatcherKind:
        """Regular expression matching."""
        ...

class Condition:
    """A single condition to evaluate against a fact."""

    def __init__(
        self,
        fact: str,
        pattern: str,
        required: bool,
        description: Optional[str] = None,
    ) -> None:
        """Create a new condition.

        Args:
            fact: The fact key to evaluate.
            pattern: The regex pattern to match.
            required: Whether this condition is required.
            description: Optional description.
        """
        ...

    def fact(self) -> str:
        """Get the fact key."""
        ...

    def pattern(self) -> str:
        """Get the regex pattern."""
        ...

    def required(self) -> bool:
        """Check if this condition is required."""
        ...

    def description(self) -> Optional[str]:
        """Get the description."""
        ...

class ConditionGroup:
    """A nested group of conditions with logical operators."""

    def __init__(self, logical_op: LogicalOp, required: bool) -> None:
        """Create a new ConditionGroup.

        Args:
            logical_op: LogicalOp.all() or LogicalOp.any().
            required: Whether this group is required.
        """
        ...

    def add_condition(self, condition: Condition) -> ConditionGroup:
        """Add a condition to this group.

        Args:
            condition: A Condition object.

        Returns:
            Self for chaining.
        """
        ...

    def evaluate(self, context: Dict[str, str]) -> Tuple[bool, Optional[str]]:
        """Evaluate this group against a context.

        Args:
            context: Dict of fact key-value pairs.

        Returns:
            (matched, reason) tuple.
        """
        ...

class DecisionMetadata:
    """Metadata for a rule evaluation decision."""

    def rule_id(self) -> str:
        """Get the rule ID."""
        ...

    def matcher_kind(self) -> str:
        """Get the matcher kind."""
        ...

    def on_mismatch(self) -> str:
        """Get the on-mismatch action."""
        ...

    def source(self) -> str:
        """Get the source of the rule."""
        ...

    def matched(self) -> bool:
        """Check if the rule matched."""
        ...

    def reason(self) -> Optional[str]:
        """Get the reason for mismatch (if any)."""
        ...

    def trace(self) -> str:
        """Generate decision trace: 'rule_id::action::source'."""
        ...

class Decision:
    """A decision trace from rule evaluation."""

    def matched(self) -> bool:
        """Check if the rule matched."""
        ...

    def metadata(self) -> DecisionMetadata:
        """Get the decision metadata."""
        ...

    def trace(self) -> str:
        """Return the decision trace string."""
        ...

class RuleEvaluator:
    """Batch rule evaluator with decision tracing."""

    def __init__(self) -> None:
        """Create a new RuleEvaluator."""
        ...

    def evaluate_rules(
        self,
        rules: List[Dict[str, Any]],
        context: Dict[str, str],
    ) -> List[Decision]:
        """Evaluate a batch of rules against a context.

        Args:
            rules: List of rule dicts with keys:
                - rule_type: "Allow" | "Deny" | "Require"
                - pattern: str
                - matcher_kind: "glob" | "prefix" | "exact" | "regex"
                - required: bool
                - source: str
                - fact: Optional[str]
            context: Dict of fact key-value pairs.

        Returns:
            List of Decision objects with metadata and traces.
        """
        ...
