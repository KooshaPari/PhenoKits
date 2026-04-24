# Wave1-F Adoption Session Overview

## Session Metadata
- Session ID: `20260303-lane-f-adoption`
- Lane: `Wave1-F`
- Repository: `agentops-policy-federation`
- Owner: `TBD`
- Start Date: `2026-03-03`
- Target Completion: `2026-03-14`

## Sprint Goals
- Complete Wave1-F policy adoption inventory across all in-scope services.
- Assign accountable owner and backup owner for each rollout item.
- Drive all blocking items to explicit decision (`go`, `defer`, or `remove`).
- Achieve `adopted` or `waived` state for all Wave1-F scope items by sprint end.

## Status Legend
| Status | Meaning | Exit Criteria |
|---|---|---|
| `not_started` | Item identified but work has not begun | Owner assigned and kickoff recorded |
| `in_progress` | Active implementation/configuration work | Evidence link and PR/issue attached |
| `blocked` | Cannot progress due to dependency or decision | Blocker + unblock owner + ETA documented |
| `in_review` | Implementation complete, awaiting approval | Reviewer assigned and review SLA date set |
| `adopted` | Change approved and live in target scope | Verification evidence linked |
| `waived` | Explicit exception granted | Waiver approver, reason, and expiry recorded |

## Rollout Inventory + Owner Map Template
| Item ID | Policy/Control | Scope (Repo/Service) | Primary Owner | Backup Owner | Current Status | Dependency | Evidence Link | Notes |
|---|---|---|---|---|---|---|---|---|
| F-001 | `<control_name>` | `<repo_or_service>` | `<owner_handle>` | `<backup_handle>` | `not_started` | `<dep_or_none>` | `<url_or_path>` | `<notes>` |
| F-002 | `<control_name>` | `<repo_or_service>` | `<owner_handle>` | `<backup_handle>` | `not_started` | `<dep_or_none>` | `<url_or_path>` | `<notes>` |
| F-003 | `<control_name>` | `<repo_or_service>` | `<owner_handle>` | `<backup_handle>` | `not_started` | `<dep_or_none>` | `<url_or_path>` | `<notes>` |

## Target Dates
| Milestone | Target Date | Owner | Success Signal |
|---|---|---|---|
| Inventory freeze and ownership lock | `2026-03-04` | `Wave1-F Lead (TBD)` | 100% items have primary + backup owner |
| Dependency clearance checkpoint | `2026-03-06` | `Lane Owners` | All blockers have unblock owner + ETA |
| Implementation complete cutoff | `2026-03-10` | `Implementers` | No items in `not_started` |
| Review and approvals complete | `2026-03-12` | `Reviewers` | All pending items moved out of `in_review` |
| Sprint closeout | `2026-03-14` | `Wave1-F Lead (TBD)` | All items are `adopted` or `waived` |

## Notes
- Update this file as the source of truth for Wave1-F session scope and ownership.
- Add links to supporting artifacts as rollout evidence is produced.
