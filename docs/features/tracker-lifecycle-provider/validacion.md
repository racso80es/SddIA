---
feature_name: tracker-lifecycle-provider
created: "2026-10-22"
process: feature
branch: feat/tracker-lifecycle-provider
branch_name: feat/tracker-lifecycle-provider
persist_ref: docs/features/tracker-lifecycle-provider
pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-02 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
uuid: "4b7aae4b-1fc6-477f-be07-d03216258493"
execution_id: "7a0b47aa-3afa-424e-afc9-a9cb3c67a038"
global: APTO
pbi_archived: true
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
git_changes:
  - SddIA/tools/linear-tracker-adapter.md
  - SddIA/engine/execute-process/src/engine/tracker_outbound.rs
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/core/eda-coverage.json
  - SddIA/evolution/75b4d30c-520e-446d-9b0f-6539ed14d4f2.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/tracker-lifecycle-provider/objectives.md
  - docs/features/tracker-lifecycle-provider/clarify.md
  - docs/features/tracker-lifecycle-provider/spec.md
  - docs/features/tracker-lifecycle-provider/plan.md
  - docs/features/tracker-lifecycle-provider/implementation.md
  - docs/features/tracker-lifecycle-provider/execution.md
  - docs/features/tracker-lifecycle-provider/validacion.md
  - docs/todos/done/[ARQUITECTURA] HU-003 PBI-02 — tracker lifecycle.md
---

# Validación — tracker-lifecycle-provider

AC-1: `provides_matches_binding_row` pasó. La fila `tracker:lifecycle` resuelve a `linear-tracker-adapter`. El gate, con `delegates_to: tool:linear-tracker-adapter` y `requires_capability: tracker:lifecycle`, acepta la fase. `outputs` declara `success` y `exitCode`.

AC-2: `ac2_update_issue_description_writes_jsonl` pasó. La línea queda en `.SddIA/state/tracker-outbound/OSC-15.jsonl` con `operation` e `issue_ref`. `records_only_the_resolved_provider` rechaza otro nombre de tool y rechaza `fetch_issue`.

AC-3: el crate no cambió. El `match` sigue en `fetch_issue`, `list_issues`, `update_issue_state`, `create_issue`, `create_comment`, `update_issue_description`.

El sello `Domain_Entity_Updated` de esta tool ya existía (`036e080a-3fe9-44b1-8155-9f59873c960e`). El motor lo trató como idempotente. La cobertura quedó en `sha256:455541dc257756b2ec330b27d84e87c6f92c7754549729efa25450e7890eb0eb`. Evolución `75b4d30c-520e-446d-9b0f-6539ed14d4f2`, hash `sha256:48ac398430b8ee607359ef0be8274200ac9c8840cf159956efec3407980155e9`.
