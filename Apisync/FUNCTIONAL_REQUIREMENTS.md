# Functional Requirements — Apisync

Traces to: PRD.md epics E1–E5.
ID format: FR-APISYNC-{NNN}.

---

## GitHub-Discord Integration

**FR-APISYNC-001**: The system SHALL synchronize GitHub repository issues to Discord thread channels bidirectionally.
Traces to: E1.1

**FR-APISYNC-002**: When a GitHub issue is created or updated, the system SHALL emit a Discord message update to the corresponding thread channel within [TTL] seconds.
Traces to: E1.2

**FR-APISYNC-003**: When a Discord thread message is posted, the system SHALL post a comment to the corresponding GitHub issue preserving thread context and attribution.
Traces to: E1.3

**FR-APISYNC-004**: The system SHALL maintain a mapping table of (GitHub issue ID, Discord thread ID) to enable bidirectional lookups.
Traces to: E1.4

---

## Discord Bot Interface

**FR-APISYNC-005**: The system SHALL register Discord slash commands for [sync, create-issue, list-issues] to allow users to manage GitHub issues via Discord.
Traces to: E2.1

**FR-APISYNC-006**: The system SHALL validate Discord user credentials against GitHub OAuth before allowing issue mutation operations.
Traces to: E2.2

---

## Trace & Test Guidance

All tests MUST reference a Functional Requirement (FR) using the pattern below:

```rust
// Traces to: FR-APISYNC-NNN
#[test]
fn test_github_issue_creates_discord_thread() { ... }
```

Every FR in this document MUST have ≥1 test. Every test MUST reference ≥1 FR.
