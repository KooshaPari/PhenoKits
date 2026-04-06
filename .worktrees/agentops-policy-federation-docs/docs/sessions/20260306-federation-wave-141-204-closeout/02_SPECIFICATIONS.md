# Specifications

Scope specification for this closeout:

- Treat lane `a` range `141-204` as active closeout scope.
- Treat lanes `b-f` range `169-204` as active closeout scope.
- For every in-scope script:
  - filename number must match primary `E###` marker
  - lane `c` files must also match companion `C###` marker
  - file must remain executable
  - file must parse successfully as Python

Acceptance criteria:
- Continuous in-scope inventory
- No in-scope token mismatches
- No in-scope execute-bit gaps
- No in-scope syntax failures

Out of scope:
- Older lane `a` debt below `141`
- Defining a new upper bound beyond `204`
