---
feature_name: tracker-lifecycle-provider
created: "2026-10-22"
process: feature
items:
  - provides
  - outbound-resolved
  - recorded-description
branch_name: feat/tracker-lifecycle-provider
persist_ref: docs/features/tracker-lifecycle-provider
execution_id: "7a0b47aa-3afa-424e-afc9-a9cb3c67a038"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
---

# Implementation — tracker-lifecycle-provider

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/tools/linear-tracker-adapter.md` | `provides` `tracker:lifecycle` / `tracker.lifecycle` / `1.0.0`. `outputs` de `emit_v2`. Hash `sha256:455541dc257756b2ec330b27d84e87c6f92c7754549729efa25450e7890eb0eb`. |
| `SddIA/engine/execute-process/src/engine/tracker_outbound.rs` | Sin `const TOOL`. Nombre leído de la fila. `update_issue_description` en `RECORDED_OPS`. |
| `SddIA/engine/execute-process/src/engine/capsules.rs` | El llamador pasa `repo` a `should_record_linear_tool`. `LINEAR_TRACKER_TOOL` no se toca. |

`entity-manager` update `hash_refresh_only`. Hash previo `sha256:8722fcf349c933f3a7af8eb00f4fb1912591f48cef8c1944a787c3ccac3c46d7`. El sello `Domain_Entity_Updated` `036e080a-3fe9-44b1-8155-9f59873c960e` fue idempotente. La cobertura quedó en el hash nuevo.
