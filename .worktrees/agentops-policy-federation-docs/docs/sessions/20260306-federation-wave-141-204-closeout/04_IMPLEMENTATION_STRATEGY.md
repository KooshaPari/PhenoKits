# Implementation Strategy

Strategy used:

- Keep generation in bounded 24-file waves using six child workers, one per lane.
- Verify every wave boundary with deterministic inventory checks.
- When defects surfaced, fix forward with one bounded mechanical repair pass instead of re-generating the entire range.

Repair strategy:

- Replace stale copied `E###` markers with the filename number for in-scope generated families.
- Replace stale `C###` markers in lane `c` with the filename number.
- Repair the copied indentation defect in lane `a` files `a156`, `a160`, ..., `a204`.
- Restore execute bits on the recent lane `a` files that missed permission normalization.

Reasoning:

- The failures were systematic copy-forward defects, so a bounded bulk repair was safer and faster than ad hoc per-file edits.
