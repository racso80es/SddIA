---
feature_name: aut20-ecst-router-verify
created: "2026-11-01"
process: bug-fix
tracker_ref: AUT-57
---

# Implementación — AUT-57

## Touchpoints

| Área | Cambio |
|------|--------|
| `route_domain_core.rs` | Despacho ECST por catálogo; `pull-request-review` aislado con `branch`; `execute-suite` con `suite_id` |
| `mod.rs` | `memory-evolution-ingest` vía `ingest_from_process_inputs`; handlers nativos `cerbero-governance-react` y `fix-tool-process` |
| `memory_evolution_ingest_core.rs` | `ingest_ecst_body` + carga por `load_ecst_from_inputs` |
| `fix_tool_process_core.rs` | `process_fix_tool_from_inputs` / `load_ecst_from_rel` |
| `tracker_sync_replay.rs` / `tracker_markdown_apply.rs` | `verify_ecst_matches_bus` → `failed-bus-mismatch` |

## Tests

- `domain_process_catalog_inputs_include_ecst` (lib)
- `run_rejects_tampered_inline_event_against_store` (replay + markdown)
- `route_domain_*_not_blocked_by_missing_branch` (integration `eda_subscriber_ecst_redb`)
