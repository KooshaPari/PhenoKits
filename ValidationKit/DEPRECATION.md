# ValidationKit — Deprecation Notice

**Effective Date:** 2026-04-25

ValidationKit has been archived effective 2026-04-25 per W-68B audit findings:
- **Real Source:** 2 LOC
- **Active Consumers:** 0
- **Test Coverage:** Minimal
- **Maintenance Burden:** Low, but insufficient value to justify continued development

## Migration Guidance

Users of ValidationKit should:
1. Evaluate alternative validation frameworks (zod, joi, valibot, yup for TypeScript; pydantic, marshmallow for Python)
2. Migrate validation logic to the framework of choice
3. Contact phenotype-infra if custom validation patterns are required

## Audit Reference

Full audit details: `/docs/org-audit-2026-04/validationkit_audit.md`

See also: W-68B W-68D audit wave findings.

## Contact

For questions about this deprecation, refer to the audit or raise an issue in the org governance tracker.
