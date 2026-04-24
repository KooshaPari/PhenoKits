# Tracera FR-to-Test Traceability Scaffold

**Status:** explicit-gap scaffold; every stub below is `pytest.mark.skip`
by design. The skip markers ARE the debt signal.

## Coverage snapshot

- FRs in catalog: **76**
- FRs with at least one referencing test: **21**
- FRs without tests (stubbed here): **55**
- Stub files generated: **55**
- Potentially un-implementable FRs (target module absent): **0**
- Placeholder/synthetic IDs excluded (FR-XXX-*, FR-FAKE-*): **5**

## Parse note

The repository has NO single canonical FR catalogue file — the referenced
`docs/reference/FUNCTIONAL_REQUIREMENTS.md` does not exist. The
`FUNCTIONAL_REQUIREMENTS_MANIFEST.txt` at the repo root is a manifest of
*deliverable documents*, not an FR catalogue. This scaffold therefore
treats the **union of all `FR-<DOMAIN>-<NN>` IDs referenced in source,
tests, specs, and scripts** as the working catalog, with titles harvested
best-effort from `FR-ID: Title` annotations. Known placeholders
(`FR-XXX-*`, `FR-FAKE-*`) are excluded.

## By-domain breakdown

| Domain | Total | Covered | Missing | Coverage % |
|--------|------:|--------:|--------:|-----------:|
| FR-QUAL | 10 | 3 | 7 | 30% |
| FR-APP | 10 | 10 | 0 | 100% |
| FR-RPT | 9 | 1 | 8 | 11% |
| FR-AI | 9 | 0 | 9 | 0% |
| FR-DISC | 8 | 5 | 3 | 62% |
| FR-INFRA | 7 | 0 | 7 | 0% |
| FR-COLLAB | 7 | 1 | 6 | 14% |
| FR-TRAC | 6 | 0 | 6 | 0% |
| FR-VERIF | 5 | 1 | 4 | 20% |
| FR-MCP | 3 | 0 | 3 | 0% |
| FR-AUTH | 1 | 0 | 1 | 0% |
| FR-GRAPH | 1 | 0 | 1 | 0% |

## FRs WITH existing test coverage

These FRs already have at least one test file referencing the ID. No
stub was generated; the scaffold defers to the existing test.

| FR | Title | Test files |
|----|-------|------------|
| FR-APP-001 | Bidirectional Link Navigation | `tests/integration/links/test_bidirectional_navigation.py`, `tests/integration/services/test_link_service.py`, `tests/unit/api/test_items_api.py`, +2 more |
| FR-APP-002 | Retrieve Traceability Item | `tests/api/test_specifications_router.py`, `tests/integration/test_specification_repositories.py`, `tests/unit/api/test_items_api.py`, +2 more |
| FR-APP-003 | Update Traceability Item | `tests/integration/links/test_dependency_detection.py`, `tests/integration/services/test_link_service.py`, `tests/integration/services/test_services_simple_full_coverage.py`, +2 more |
| FR-APP-004 | Delete Traceability Item | `tests/unit/api/test_items_api.py`, `tests/unit/services/test_item_service.py` |
| FR-APP-005 | List Traceability Items | `tests/unit/api/test_items_api.py`, `tests/unit/services/test_item_service.py` |
| FR-APP-006 | Create Traceability Link | `tests/unit/api/test_links_api.py` |
| FR-APP-007 | Retrieve Traceability Link | `tests/unit/api/test_link_endpoints_comprehensive.py`, `tests/unit/api/test_links_api.py` |
| FR-APP-008 | Update Traceability Link | `tests/unit/api/test_link_endpoints_comprehensive.py`, `tests/unit/api/test_links_api.py` |
| FR-APP-009 | Delete Traceability Link | `tests/unit/api/test_links_api.py` |
| FR-APP-010 | List Traceability Links | `tests/unit/api/test_link_endpoints_comprehensive.py`, `tests/unit/api/test_links_api.py` |
| FR-COLLAB-001 | External Tool Integration | `tests/unit/repositories/test_github_project_repository.py`, `tests/unit/repositories/test_webhook_repository.py` |
| FR-DISC-001 | GitHub Issue Import | `tests/integration/services/test_services_simple_full_coverage.py`, `tests/integration/test_webhook_handler.py`, `tests/unit/api/test_export_import_endpoints.py`, +2 more |
| FR-DISC-002 | Specification Parsing | `tests/api/test_specifications_router.py`, `tests/integration/services/test_services_simple_full_coverage.py`, `tests/integration/test_item_spec_service.py`, +4 more |
| FR-DISC-003 | Auto-Link Suggestion | `tests/integration/links/test_auto_linking.py`, `tests/integration/services/test_link_service.py`, `tests/unit/api/test_links_api.py`, +1 more |
| FR-DISC-004 | Commit Linking | `tests/integration/links/test_auto_linking.py`, `tests/integration/services/test_services_simple_full_coverage.py` |
| FR-DISC-005 | Webhook Ingestion | `tests/integration/test_webhook_handler.py`, `tests/unit/repositories/test_webhook_repository.py`, `tests/unit/test_gap_coverage_import_services.py` |
| FR-QUAL-001 | Requirement Quality Assessment | `tests/api/test_specifications_router.py`, `tests/integration/test_item_spec_service.py`, `tests/integration/test_specification_repositories.py`, +1 more |
| FR-QUAL-002 | Link Quality Scoring | `tests/integration/services/test_link_service.py` |
| FR-QUAL-003 | Dependency Analysis | `tests/integration/links/test_dependency_detection.py`, `tests/integration/test_item_spec_service.py` |
| FR-RPT-003 | Export to External Formats | `tests/unit/api/test_export_import_endpoints.py` |
| FR-VERIF-001 | Test Execution Tracking | `tests/integration/services/test_services_simple_full_coverage.py` |

## FRs WITHOUT existing coverage (stubbed with explicit skip)

| FR | Title | Stub file |
|----|-------|-----------|
| FR-AI-001 | (title not declared) | `test_fr_ai_001.py` |
| FR-AI-002 | (title not declared) | `test_fr_ai_002.py` |
| FR-AI-003 | (title not declared) | `test_fr_ai_003.py` |
| FR-AI-004 | (title not declared) | `test_fr_ai_004.py` |
| FR-AI-005 | (title not declared) | `test_fr_ai_005.py` |
| FR-AI-006 | (title not declared) | `test_fr_ai_006.py` |
| FR-AI-007 | (title not declared) | `test_fr_ai_007.py` |
| FR-AI-009 | (title not declared) | `test_fr_ai_009.py` |
| FR-AI-010 | (title not declared) | `test_fr_ai_010.py` |
| FR-AUTH-001 | (title not declared) | `test_fr_auth_001.py` |
| FR-COLLAB-002 | (title not declared) | `test_fr_collab_002.py` |
| FR-COLLAB-003 | (title not declared) | `test_fr_collab_003.py` |
| FR-COLLAB-004 | (title not declared) | `test_fr_collab_004.py` |
| FR-COLLAB-006 | (title not declared) | `test_fr_collab_006.py` |
| FR-COLLAB-007 | (title not declared) | `test_fr_collab_007.py` |
| FR-COLLAB-008 | (title not declared) | `test_fr_collab_008.py` |
| FR-DISC-006 | (title not declared) | `test_fr_disc_006.py` |
| FR-DISC-007 | (title not declared) | `test_fr_disc_007.py` |
| FR-DISC-008 | (title not declared) | `test_fr_disc_008.py` |
| FR-GRAPH-001 | (title not declared) | `test_fr_graph_001.py` |
| FR-INFRA-001 | (title not declared) | `test_fr_infra_001.py` |
| FR-INFRA-002 | (title not declared) | `test_fr_infra_002.py` |
| FR-INFRA-004 | (title not declared) | `test_fr_infra_004.py` |
| FR-INFRA-005 | (title not declared) | `test_fr_infra_005.py` |
| FR-INFRA-006 | (title not declared) | `test_fr_infra_006.py` |
| FR-INFRA-007 | (title not declared) | `test_fr_infra_007.py` |
| FR-INFRA-010 | (title not declared) | `test_fr_infra_010.py` |
| FR-MCP-001 | (title not declared) | `test_fr_mcp_001.py` |
| FR-MCP-002 | (title not declared) | `test_fr_mcp_002.py` |
| FR-MCP-009 | (title not declared) | `test_fr_mcp_009.py` |
| FR-QUAL-004 | (title not declared) | `test_fr_qual_004.py` |
| FR-QUAL-005 | (title not declared) | `test_fr_qual_005.py` |
| FR-QUAL-006 | (title not declared) | `test_fr_qual_006.py` |
| FR-QUAL-007 | (title not declared) | `test_fr_qual_007.py` |
| FR-QUAL-008 | (title not declared) | `test_fr_qual_008.py` |
| FR-QUAL-009 | (title not declared) | `test_fr_qual_009.py` |
| FR-QUAL-010 | (title not declared) | `test_fr_qual_010.py` |
| FR-RPT-001 | (title not declared) | `test_fr_rpt_001.py` |
| FR-RPT-002 | (title not declared) | `test_fr_rpt_002.py` |
| FR-RPT-004 | (title not declared) | `test_fr_rpt_004.py` |
| FR-RPT-005 | (title not declared) | `test_fr_rpt_005.py` |
| FR-RPT-006 | (title not declared) | `test_fr_rpt_006.py` |
| FR-RPT-007 | (title not declared) | `test_fr_rpt_007.py` |
| FR-RPT-008 | (title not declared) | `test_fr_rpt_008.py` |
| FR-RPT-009 | (title not declared) | `test_fr_rpt_009.py` |
| FR-TRAC-001 | Span Ingestion API | `test_fr_trac_001.py` |
| FR-TRAC-002 | Trace Visualization UI | `test_fr_trac_002.py` |
| FR-TRAC-003 | AI Anomaly Detection | `test_fr_trac_003.py` |
| FR-TRAC-004 | Alerting System | `test_fr_trac_004.py` |
| FR-TRAC-005 | Historical Search | `test_fr_trac_005.py` |
| FR-TRAC-006 | Custom Dashboards | `test_fr_trac_006.py` |
| FR-VERIF-002 | (title not declared) | `test_fr_verif_002.py` |
| FR-VERIF-005 | (title not declared) | `test_fr_verif_005.py` |
| FR-VERIF-008 | (title not declared) | `test_fr_verif_008.py` |
| FR-VERIF-010 | (title not declared) | `test_fr_verif_010.py` |

## Maintenance

1. When an FR is implemented, delete the matching stub file (or replace
   its body with a real test) and remove the entry from this README's
   *Without coverage* table.
2. When a new FR is added to the catalog, re-run the scaffold generator
   and commit the new stub alongside the FR.
3. Never replace a skip with `assert True` — passing stubs hide the
   gap that this scaffold exists to surface.

