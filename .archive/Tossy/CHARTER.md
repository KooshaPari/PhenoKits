# Tossy Charter

## 1. Mission Statement

**Tossy** is a task orchestration and workflow management system designed to simplify complex multi-step operations into reliable, composable, and observable workflows. The mission is to provide a declarative yet flexible framework for defining, executing, and monitoring task pipelines—enabling developers to automate intricate processes with confidence, retry logic, and comprehensive observability.

The project exists to make task orchestration as simple as defining what needs to happen, while handling the complexity of execution ordering, failure recovery, parallelization, and state management automatically.

---

## 2. Tenets (Unless You Know Better Ones)

### Tenet 1: Declarative Definition, Imperative Control

Workflows are defined declaratively (what to do), but controlled imperatively (how to react). The system handles execution; developers control flow. YAML for definition, code for logic.

### Tenet 2: Reliable by Default

Tasks retry automatically with exponential backoff. Failures are isolated. Partial successes are handled gracefully. The system degrades elegantly, never catastrophically.

### Tenet 3: Observable Everything

Every task execution is visible. Progress tracking, timing, logs, and state—all accessible. No black boxes. Debugging workflows is straightforward.

### Tenet 4: Composable Primitives

Complex workflows built from simple, reusable tasks. Tasks compose like functions. Higher-order workflows emerge from primitive building blocks.

### Tenet 5: Language Agnostic

Tasks can be written in any language. Shell scripts, Python, Rust—Tossy orchestrates them all. No lock-in to a specific runtime or ecosystem.

### Tenet 6: Resource Aware

Workflows respect resource constraints. Parallelism is bounded. Memory and CPU usage are monitored. Tasks can specify resource requirements.

### Tenet 7. Stateless Core, Stateful Optional

Core engine is stateless for reliability. State can be externalized (database, object storage) when needed. No required state complexity for simple use cases.

---

## 3. Scope & Boundaries

### In Scope

**Core Workflow Engine:**
- Task definition and registration
- Dependency graph resolution
- Parallel execution with dependency awareness
- Serial fallback when dependencies require
- Retry logic with configurable strategies

**Execution Models:**
- Local execution for development
- Distributed execution for scale
- Container-based task isolation
- Serverless integration (AWS Lambda, etc.)

**Workflow Features:**
- Conditional task execution
- Dynamic task generation
- Task loops and iterations
- Sub-workflow composition
- Event-driven triggers

**Observability:**
- Execution logging
- Progress tracking
- Performance metrics
- State persistence options
- Alerting and notification

**Integrations:**
- CI/CD pipeline integration
- Message queue triggers
- Schedule-based execution
- Webhook triggers
- API for external control

### Out of Scope

- Business process management (BPM) features
- Human task management (approvals, etc.)
- Complex state machines (use dedicated state machine libraries)
- Long-running transaction management (Saga pattern—use dedicated libraries)
- Visual workflow designer (CLI-first, API-driven)

### Boundaries

- Tossy orchestrates; tasks implement logic
- No implicit state sharing between tasks
- Tasks are isolated and communicate through defined interfaces
- Resource limits are enforced, not advisory

---

## 4. Target Users & Personas

### Primary Persona: Developer Drew

**Role:** Engineer building data pipelines and automation
**Goals:** Reliable task execution, clear dependencies, easy debugging
**Pain Points:** Brittle scripts, unclear failure points, manual retry logic
**Needs:** Declarative workflows, automatic retries, good observability
**Tech Comfort:** High, comfortable with YAML and scripting

### Secondary Persona: Data Engineer Dana

**Role:** Data pipeline developer
**Goals:** Complex pipeline orchestration, data lineage tracking
**Pain Points:** Pipeline failures, hard to debug distributed issues
**Needs:** Visual execution graphs, clear error context, retry strategies
**Tech Comfort:** Very high, expert in data processing

### Tertiary Persona: DevOps Derek

**Role:** Infrastructure automation engineer
**Goals:** Reliable deployment pipelines, infrastructure orchestration
**Pain Points:** Complex deployment steps, rollback complexity
**Needs:** Conditional execution, rollback workflows, notification integration
**Tech Comfort:** Very high, expert in automation

### Persona: Platform Patty

**Role:** Platform engineer providing tools to teams
**Goals:** Consistent workflow patterns across organization
**Pain Points:** Inconsistent automation, hard to share best practices
**Needs:** Reusable workflow templates, standard patterns, governance
**Tech Comfort:** Very high, expert in platform engineering

---

## 5. Success Criteria (Measurable)

### Reliability Metrics

- **Task Success Rate:** 99.9% of tasks complete successfully (with retries)
- **Retry Effectiveness:** 80% of transient failures resolved by automatic retry
- **Failure Isolation:** 100% of task failures contained to failed task
- **Recovery Time:** Failed workflows restartable within 1 minute

### Performance Metrics

- **Startup Time:** Workflow engine starts in <5 seconds
- **Task Dispatch:** Task execution begins within 100ms of scheduling
- **Parallel Efficiency:** Near-linear speedup up to resource limits
- **Memory Efficiency:** Constant memory overhead per workflow

### Developer Experience

- **Definition Time:** New workflow defined in <30 minutes
- **Debuggability:** Average issue debugged within 15 minutes
- **Documentation:** 100% of features documented with examples
- **Learning Curve:** New user productive within 2 hours

### Operational Metrics

- **Uptime:** 99.9% orchestrator availability
- **Deployment Frequency:** New workflow deployed within 5 minutes
- **Rollback Time:** Workflow version rolled back within 1 minute
- **Alert Clarity:** 95% of alerts require no runbook lookup

---

## 6. Governance Model

### Component Organization

```
Tossy/
├── engine/          # Core workflow engine
├── runtime/         # Task execution runtimes
├── scheduler/       # Task scheduling and distribution
├── storage/         # State persistence
├── observability/   # Metrics, logging, tracing
├── api/             # External API
└── cli/             # Command-line interface
```

### Development Process

**New Features:**
- RFC for significant features
- Backward compatibility review
- Performance impact assessment
- Documentation requirements

**Breaking Changes:**
- Major version bump
- Migration guide required
- Deprecation period

**Security:**
- Security review for new execution features
- Sandboxing review
- Audit logging requirements

---

## 7. Charter Compliance Checklist

### For New Workflows

- [ ] Workflow has clear owner
- [ ] Retry strategy defined
- [ ] Resource requirements specified
- [ ] Error handling documented
- [ ] Observability configured

### For Engine Changes

- [ ] Backward compatibility maintained
- [ ] Performance regression tested
- [ ] Documentation updated
- [ ] Security implications assessed

### For Deprecations

- [ ] Migration path documented
- [ ] Users notified
- [ ] Grace period defined

---

## 8. Decision Authority Levels

### Level 1: Workflow Owner Authority

**Scope:** Individual workflow modifications
**Process:** Owner approval

### Level 2: Engine Maintainer Authority

**Scope:** Non-breaking engine changes
**Process:** Maintainer approval

### Level 3: Technical Steering Authority

**Scope:** Breaking changes, new paradigms
**Process:** Written proposal, steering approval

### Level 4: Executive Authority

**Scope:** Strategic direction, major investments
**Process:** Business case, executive approval

---

*This charter governs Tossy, the workflow orchestration system. Reliable automation enables reliable operations.*

*Last Updated: April 2026*
*Next Review: July 2026*
