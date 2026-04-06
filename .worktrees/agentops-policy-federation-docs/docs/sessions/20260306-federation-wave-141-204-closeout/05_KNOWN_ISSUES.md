# Known Issues

Resolved in this session:

- Recent generated wave families had stale `E###` markers.
- Lane `c` recent generated files had stale `C###` markers.
- Lane `a` threshold-window derived files had a copied indentation defect.
- `a149-a156` missed execute-bit normalization.
- Older lane `a` execute-bit gaps below `141` were repaired.
- `a82_schema_drift_correlation_gate.py` syntax was repaired.

Still open outside this scope:

- Lane `a` numeric gap remains at `77-80` with no discoverable spec or prior session artifact defining intended filenames/content.
- Earlier lane `a` files below `141` use legacy `A###` style identifiers rather than `E###`, so broad whole-directory `E###` checks are not a valid invariant.

Next follow-up if continuing cleanup:

- Resolve or author the missing `a77-a80` wave from source requirements
- Then rerun whole-directory continuity and compile validation
