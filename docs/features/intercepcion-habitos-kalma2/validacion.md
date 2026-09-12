---
feature_name: intercepcion-habitos-kalma2
created: "2026-09-12"
process: feature
branch: feat/intercepcion-habitos-kalma2
branch_name: feat/intercepcion-habitos-kalma2
persist_ref: docs/features/intercepcion-habitos-kalma2
pbi_ref: docs/todos/done/[NÚCLEO] Intercepción Ontológica y Delegación de Hábitos desde Kalma2.md
document_id: PBI-NUCLEO-INTERCEPCION-HABITOS-KALMA2
uuid: "8f3d1b22-6b9c-4e89-a512-9c3e4f7a1102"
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/290
ci_run_id: "34684154603"
ci_run_url: https://github.com/racso80es/SddIA/actions/runs/34684154603
ci_head_sha: "2093ccc8a979125fdfe764d506d3254a2ae8c6ff"
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  CA-5: APTO
  CA-6: APTO
  CA-7: APTO
  CA-8: APTO
  CA-9: APTO
  CA-10: APTO
  CA-CI: APTO
git_changes:
  - SddIA/actions/dispatch-aiua-intent.md
  - SddIA/actions/index.md
  - SddIA/conscience/aiua_core.md
  - SddIA/core/eda-coverage.json
  - SddIA/engine/execute-process/src/engine/aiua_intent.rs
  - SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/engine/execute-process/src/engine/handlers/user_preference.rs
  - SddIA/user-preference-core/src/lib.rs
  - SddIA/evolution/89d3ce2b-4bde-49af-8945-8aeb9c4e84cf.md
  - docs/features/intercepcion-habitos-kalma2/
  - docs/todos/done/[NÚCLEO] Intercepción Ontológica y Delegación de Hábitos desde Kalma2.md
---

# Validación — intercepcion-habitos-kalma2

Tests locales: `cargo test -p user-preference-core --lib` (12 passed); `cargo test -p execute-process --lib -- aiua_intent email_triage user_preference aiua_stimulus` (filtros verdes, incl. CA-8 hint computrabajo y CA-9 overlay sin join a ingest).

CA-CI: run [34684154603](https://github.com/racso80es/SddIA/actions/runs/34684154603) sobre `2093ccc8a979125fdfe764d506d3254a2ae8c6ff` — `sddia-index-integrity`, `eda-iota-smoke-simulate`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-physical` en pass. Cero fail.
