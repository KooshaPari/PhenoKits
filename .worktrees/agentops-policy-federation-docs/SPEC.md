# agentops-policy-federation Specification

## Architecture
```
┌─────────────────────────────────────────────────────────────────────┐
│           Policy Federation                 │
├─────────────────────────────────────────────────────────────────────┤
│  ┌──────────┐   ┌────────────┐   ┌───────────┐  │
│  │ Scope    │──▶│ Resolver  │──▶│ Evaluator │  │
│  │ Parser  │   │ Engine   │   │ Engine   │  │
│  └──────────┘   └────────────┘   └───────────┘  │
│       │              │               │          │
│       ▼              ▼               ▼          │
│  ┌─────────────────────────────────────┐    │
│  │        Runtime Guards (exec/net/io)   │    │
│  └─────────────────────────────────────┘    │
└─────────────────────────────────────────────┘
```

## Components

| Component | Responsibility | Public API |
|-----------|----------------|-----------|
| ScopeParser | Parse 7-scope authorization | `parse()`, `validate()` |
| ResolverEngine | Resolve policies to decisions | `resolve()`, `compile()` |
| EvaluatorEngine | Evaluate rules at runtime | `evaluate()`, `check()` |
| RuntimeGuard | Enforce execution policies | `install()`, `uninstall()` |

## Data Models

```python
Scope = Literal["org", "workspace", "user", "agent", "session", "tool", "resource"]

Policy:
  scopes: List[Scope]
  rules: List[Rule]
  version: str

Rule:
  effect: "allow" | "deny"
  actions: List[str]
  conditions: Dict[str, Any]
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Policy resolve | <50ms |
| Rule evaluation | <1ms |
| Runtime install | <200ms |
| Concurrent policies | 100 max |