# xDD and Engineering Methodology Catalog

This catalog provides a practical set of xDD methods, process patterns, and software engineering best practices for architecture modernization, polyrepo decomposition, and long-term maintainability.

## xDD Family (50+)

1. Test-Driven Development (TDD)
2. Behavior-Driven Development (BDD)
3. Specification-Driven Development (SDD)
4. Acceptance-Test-Driven Development (ATDD)
5. Domain-Driven Design (DDD)
6. Contract-Driven Development (CDD)
7. API-First Development
8. Schema-First Development
9. Documentation-Driven Development
10. Security-Driven Development
11. Threat-Model-Driven Development
12. Performance-Driven Development
13. Reliability-Driven Development
14. Resilience-Driven Development
15. Observability-Driven Development
16. Telemetry-Driven Development
17. Data-Driven Development
18. Metrics-Driven Development
19. Event-Driven Development
20. Workflow-Driven Development
21. Policy-Driven Development
22. Compliance-Driven Development
23. Model-Driven Development
24. Type-Driven Development
25. Property-Driven Development
26. Mutation-Driven Development
27. Example-Driven Development
28. User-Journey-Driven Development
29. Use-Case-Driven Development
30. Scenario-Driven Development
31. Interface-Driven Development
32. Protocol-Driven Development
33. Adapter-Driven Development
34. Port-Driven Development
35. Hexagonal-Driven Development
36. Plugin-Driven Development
37. Extension-Point-Driven Development
38. Integration-Driven Development
39. Consumer-Driven Contract Development
40. Service-Level-Objective-Driven Development
41. Quality-Gate-Driven Development
42. Risk-Driven Development
43. Failure-Mode-Driven Development
44. Chaos-Driven Development
45. Recovery-Driven Development
46. Migration-Driven Development
47. Versioning-Driven Development
48. Compatibility-Driven Development
49. Release-Train-Driven Development
50. Value-Driven Development
51. Outcome-Driven Development
52. Lean-Driven Development
53. Platform-Driven Development
54. Productization-Driven Development
55. Reuse-Driven Development
56. Library-First Development
57. Service-First Development
58. CLI-First Development
59. Docs-Site-Driven Development
60. Governance-Driven Development

## Architecture and Design Practices

61. Hexagonal Architecture (Ports and Adapters)
62. Clean Architecture
63. Layered Architecture with explicit dependency direction
64. Onion Architecture
65. Modular Monolith to Microservice evolutionary design
66. Bounded Context decomposition
67. CQRS where read/write concerns differ materially
68. Event Sourcing where auditability and replay are core needs
69. Saga orchestration for distributed workflows
70. Anti-Corruption Layer integration pattern
71. Plugin Registry and capability negotiation
72. Shared Kernel minimization
73. Explicit domain invariants in model boundaries
74. Stable public API surface with internal volatility allowed
75. Backward-compatibility policy by integration tier

## Code Quality and Delivery Practices

76. SOLID principles
77. KISS design style
78. DRY refactoring discipline
79. YAGNI guardrails
80. Single Responsibility at module level
81. Refactor-before-rewrite preference
82. Forward-only migration strategy
83. Trunk health ownership and CI fail-fast behavior
84. Small-batch PR and stacked PR discipline
85. Mandatory changelog and semantic versioning
86. Automated dependency and vulnerability hygiene
87. Security scanning in CI (SAST, secrets, deps)
88. Architecture conformance tests (imports/layers/contracts)
89. Golden tests for contract outputs
90. Progressive delivery with explicit rollback plans

## Recommended minimum adoption baseline

- Core xDD set: TDD, BDD, SDD, CDD, DDD.
- Architecture set: Hexagonal + Clean + bounded contexts + plugin contracts.
- Delivery set: CI quality gates, semantic versioning, changelog automation, security pipeline.
- Governance set: reuse-first extraction, documented migration order, and measurable SLO validation.
