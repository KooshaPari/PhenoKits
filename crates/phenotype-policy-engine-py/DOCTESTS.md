# PyO3 Binding Doctests — policy_engine_py

This document contains 5 doctests demonstrating Python usage of the phenotype-policy-engine PyO3 binding.
Each test is ready to run once `maturin develop` is executed.

## Setup

```bash
cd /repos/crates/phenotype-policy-engine-py
maturin develop
```

## Doctest 1: Import and LogicalOp Creation

```python
import policy_engine_py

# Create logical operators
all_op = policy_engine_py.LogicalOp.all()
any_op = policy_engine_py.LogicalOp.any()

print(f"All operator: {all_op}")  # LogicalOp.All
print(f"Any operator: {any_op}")  # LogicalOp.Any
```

## Doctest 2: Construct ConditionGroup with Conditions

```python
import policy_engine_py

# Create matcher kinds
exact_matcher = policy_engine_py.MatcherKind.exact()
regex_matcher = policy_engine_py.MatcherKind.regex()

# Create conditions
cond_env = policy_engine_py.Condition(
    fact="environment",
    pattern="^(prod|staging)$",
    required=True,
    description="Environment must be prod or staging"
)

cond_role = policy_engine_py.Condition(
    fact="role",
    pattern="admin|operator",
    required=False,
    description="User role (optional)"
)

# Build a condition group
group = policy_engine_py.ConditionGroup(
    logical_op=policy_engine_py.LogicalOp.all(),
    required=True
)
group.add_condition(cond_env)
group.add_condition(cond_role)

print(f"Created group: {group}")
```

## Doctest 3: Evaluate ConditionGroup Against Context

```python
import policy_engine_py

# Create a condition
cond = policy_engine_py.Condition(
    fact="environment",
    pattern="prod",
    required=True
)

# Create group
group = policy_engine_py.ConditionGroup(
    logical_op=policy_engine_py.LogicalOp.all(),
    required=True
)
group.add_condition(cond)

# Evaluate against context
context = {
    "environment": "prod",
    "region": "us-west-2"
}

matched, reason = group.evaluate(context)
print(f"Matched: {matched}, Reason: {reason}")  # Matched: True, Reason: None

# Evaluate with non-matching context
context_fail = {"environment": "dev"}
matched_fail, reason_fail = group.evaluate(context_fail)
print(f"Matched (fail): {matched_fail}")  # False
print(f"Reason (fail): {reason_fail}")    # "Fact 'environment' = 'dev' does not match pattern 'prod'"
```

## Doctest 4: RuleEvaluator with Decision Traces

```python
import policy_engine_py

evaluator = policy_engine_py.RuleEvaluator()

# Define rules
rules = [
    {
        "rule_type": "Allow",
        "fact": "environment",
        "pattern": "prod",
        "matcher_kind": "exact",
        "required": True,
        "source": "policy.py"
    },
    {
        "rule_type": "Require",
        "fact": "role",
        "pattern": "admin",
        "matcher_kind": "exact",
        "required": True,
        "source": "policy.py"
    }
]

context = {
    "environment": "prod",
    "role": "admin"
}

# Evaluate rules
decisions = evaluator.evaluate_rules(rules, context)

for decision in decisions:
    print(f"Decision: matched={decision.matched()}")
    print(f"  Trace: {decision.trace()}")
    metadata = decision.metadata()
    print(f"  Rule ID: {metadata.rule_id()}")
    print(f"  Source: {metadata.source()}")
```

## Doctest 5: Inspect Decision Metadata

```python
import policy_engine_py

evaluator = policy_engine_py.RuleEvaluator()

# Single rule evaluation
rules = [
    {
        "rule_type": "Allow",
        "fact": "tier",
        "pattern": ".*",  # Match any tier
        "matcher_kind": "regex",
        "required": False,
        "source": "bootstrap.py"
    }
]

context = {"tier": "enterprise"}

decisions = evaluator.evaluate_rules(rules, context)

if decisions:
    decision = decisions[0]
    metadata = decision.metadata()
    
    print(f"Rule ID: {metadata.rule_id()}")
    print(f"Matched: {metadata.matched()}")
    print(f"Matcher: {metadata.matcher_kind()}")
    print(f"On Mismatch: {metadata.on_mismatch()}")
    print(f"Reason: {metadata.reason()}")
    print(f"Full Trace: {metadata.trace()}")
```

## Running Doctests

Each test can be run independently in a Python REPL after `maturin develop`:

```bash
python3 << 'EOF'
# Copy doctest code here
EOF
```

## Expected Outputs

- **Doctest 1:** Logical operators print correctly with string representation
- **Doctest 2:** ConditionGroup created and added with 2 conditions
- **Doctest 3:** Context evaluation returns (True, None) for match; (False, "reason") for mismatch
- **Doctest 4:** Multiple decisions traced with rule IDs and sources
- **Doctest 5:** DecisionMetadata shows all fields including trace string

## Notes

- All doctests assume `policy_engine_py` is installed via `maturin develop`
- ConditionGroup.add_condition() returns Self for chaining (test demonstrates single-call usage)
- RuleEvaluator.evaluate_rules() returns a list of Decision objects with full metadata
- Traces follow format: "rule_id::on_mismatch_action::source"
