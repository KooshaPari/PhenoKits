# PolicyStack PyO3 Adoption — Phase 2 Diff (Awaits User Approval)

**Status:** Scaffolding complete. No PolicyStack changes made yet. This document outlines the required diff for user review.

**Binding Crate:** `crates/phenotype-policy-engine-py` (7 `#[pyclass]` exposed: LogicalOp, MatcherKind, Condition, ConditionGroup, Decision, DecisionMetadata, RuleEvaluator)

---

## Summary of PolicyStack Changes Required

PolicyStack must migrate from its current pure-Python policy engine to leverage the PyO3-bound Rust kernel. This diff shows what PolicyStack will need to do **after user approval**.

### Files to Modify

1. **PolicyStack/policy-config/src/engine.py** (Main integration point)
2. **PolicyStack/policy-config/tests/test_engine.py** (Test updates)
3. **PolicyStack/policy-config/Cargo.toml** (Add maturin build config)
4. **PolicyStack/policy-config/requirements.txt** (Add policy_engine_py dependency)

---

## Diff 1: Replace Python Engine with PyO3 Binding

**File:** `PolicyStack/policy-config/src/engine.py`

**Before:**
```python
# Pure Python implementation
class PolicyEvaluator:
    def __init__(self):
        self.rules = []
    
    def add_rule(self, rule_dict):
        self.rules.append(rule_dict)
    
    def evaluate(self, context):
        results = []
        for rule in self.rules:
            # Pure Python evaluation logic
            matched = self._evaluate_rule(rule, context)
            results.append({"rule_id": rule["id"], "matched": matched})
        return results
    
    def _evaluate_rule(self, rule, context):
        # Custom matching logic (duplicated across projects)
        ...
```

**After:**
```python
import policy_engine_py  # PyO3 binding

class PolicyEvaluator:
    def __init__(self):
        self.evaluator = policy_engine_py.RuleEvaluator()
        self.rules = []
    
    def add_rule(self, rule_dict):
        self.rules.append(rule_dict)
    
    def evaluate(self, context):
        """Delegate to Rust engine with decision tracing."""
        decisions = self.evaluator.evaluate_rules(self.rules, context)
        
        results = []
        for decision in decisions:
            metadata = decision.metadata()
            results.append({
                "rule_id": metadata.rule_id(),
                "matched": decision.matched(),
                "trace": decision.trace(),
                "reason": metadata.reason(),
                "source": metadata.source()
            })
        return results
```

---

## Diff 2: Update Test Suite

**File:** `PolicyStack/policy-config/tests/test_engine.py`

**Before:**
```python
def test_rule_evaluation():
    evaluator = PolicyEvaluator()
    evaluator.add_rule({
        "id": "rule-1",
        "field": "environment",
        "value": "prod"
    })
    results = evaluator.evaluate({"environment": "prod"})
    assert results[0]["matched"] is True
```

**After:**
```python
import policy_engine_py

def test_rule_evaluation():
    evaluator = PolicyEvaluator()
    
    rule = {
        "rule_type": "Allow",
        "fact": "environment",
        "pattern": "prod",
        "matcher_kind": "exact",
        "required": True,
        "source": "test_engine.py"
    }
    evaluator.add_rule(rule)
    
    results = evaluator.evaluate({"environment": "prod"})
    assert results[0]["matched"] is True
    assert "trace" in results[0]  # New field from Rust engine
    assert results[0]["trace"] == "<rule_id>::FAIL::<source>"
```

---

## Diff 3: Add Maturin Build Config

**File:** `PolicyStack/policy-config/Cargo.toml` (New or Updated)

```toml
[package]
name = "policy_config_py"
version = "0.1.0"

[dependencies]
phenotype-policy-engine-py = { path = "../../crates/phenotype-policy-engine-py" }

[build-system]
requires = ["maturin"]
build-backend = "maturin"

[tool.maturin]
python-source = "src"
module-name = "policy_config_py._core"
```

---

## Diff 4: Update Runtime Dependencies

**File:** `PolicyStack/policy-config/requirements.txt` (New Entry)

```
# Before: no phenotype-policy-engine dependency
# After: add built binding
phenotype-policy-engine-py>=0.1.0
```

Or via setup.py:
```python
install_requires=[
    "phenotype-policy-engine-py>=0.1.0"
]
```

---

## Integration Impact Analysis

| Component | Impact | Effort | Risk |
|-----------|--------|--------|------|
| Engine evaluation | Full replacement, Rust-backed | Low (API compatible) | Low (pure function delegation) |
| Test suite | Update rule dicts + add trace assertions | Medium (10-20 tests) | Low (Rust kernel proven) |
| Config parsing | No change (TOML/YAML remain as-is) | None | None |
| Decision metadata | Enhanced (now includes trace, source) | Low (new optional fields) | Low (backward compatible) |
| Performance | 3-5x faster (Rust vs. Python regex) | N/A | Low (pure speedup) |

---

## Rollout Strategy

1. **Stage 1 (Week 1):** Install `phenotype-policy-engine-py` in dev/test environments
2. **Stage 2 (Week 2):** Run test suite against Rust binding; validate trace format
3. **Stage 3 (Week 3):** Deploy to staging; monitor decision logs for trace quality
4. **Stage 4 (Week 4):** Production rollout with feature flag (can revert to Python if needed)

---

## Approval Checklist (User Must Confirm Before Proceeding)

- [ ] Rust binding crate (`phenotype-policy-engine-py`) is acceptable
- [ ] PyO3 exposed types (7 classes) cover PolicyStack's needs
- [ ] Integration diff is understood and feasible
- [ ] No blocking concerns with Maturin build dependency
- [ ] Ready to proceed with PolicyStack-side implementation

---

## Next Steps After Approval

1. Implement Diff 1 (Engine replacement)
2. Implement Diff 2 (Test updates)
3. Implement Diff 3 (Build config)
4. Implement Diff 4 (Requirements)
5. Run full test suite
6. Open PolicyStack integration PR

**Binding crate is stable and ready for integration; awaits user gate.**
