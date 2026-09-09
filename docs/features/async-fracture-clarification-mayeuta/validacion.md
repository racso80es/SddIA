---
feature_name: async-fracture-clarification-mayeuta
created: "2026-09-09"
process: feature
branch: feat/async-fracture-clarification-mayeuta
global: PENDIENTE-CI
pbi_archived: true
checks:
  ASYNC-CLARIFY-CA1: APTO
  ASYNC-CLARIFY-CA2: APTO
  ASYNC-CLARIFY-CA3: APTO
  ASYNC-CLARIFY-CA4: APTO
  ASYNC-CLARIFY-CA5: APTO
  ASYNC-CLARIFY-CA6: APTO
  CA-CI: PENDIENTE-CI
git_changes:
  - SddIA/events/orchestration/fracture-clarification-requested.md
  - SddIA/events/orchestration/index.md
  - SddIA/events/index.md
  - SddIA/actions/append-mayeuta-hypothesis.md
  - SddIA/actions/index.md
  - SddIA/core/event-orchestration-subscriptions.json
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/engine/execute-process/src/engine/append_mayeuta_hypothesis.rs
  - SddIA/engine/execute-process/src/engine/actions.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/engine/ecst_validation.rs
  - SddIA/engine/execute-process/src/forges/factory.rs
  - SddIA/engine/execute-process/src/forges/common.rs
  - docs/features/async-fracture-clarification-mayeuta/
  - docs/todos/done/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md
  - SddIA/evolution/67a74480-f2b3-440d-a467-7dfe32726705.md
---

# Validación — async-fracture-clarification-mayeuta

## Resultado

CA1–CA6 verificados en `cargo test -p execute-process --lib` (29 passed, filtro `enrich_fracture_pbi_kaizen` / `append_mayeuta_hypothesis` / `loads_fracture_clarification`). `global` no es APTO hasta check GitHub verde (`run_id`).

| ID | Resultado | Evidencia |
|----|-----------|-----------|
| CA1 | APTO | `enrich_unclassified_emits_*` / `enrich_classified_colaps_does_not_emit_*` |
| CA2 | APTO | `loads_fracture_clarification_requested_schema` + validate payload REQUIRED |
| CA3 | APTO | `upsert_inserts_before_criterio_and_is_idempotent` |
| CA4 | APTO | upsert conserva YAML / traza / Conclusión |
| CA5 | APTO | `run_without_llm_cli_fail_open_leaves_pbi` |
| CA6 | APTO | `prompt_contains_kernel_and_forbids_invented_paths` + `truncate_caps_at_15_lines` |
| CA-CI | PENDIENTE-CI | GitHub Actions post-PR |
