---
feature_name: nucleo-aiua-tormentosa-motor
created: "2026-09-08"
process: feature
phase: validate
agents: argos
branch: feat/nucleo-aiua-tormentosa-motor
branch_name: feat/nucleo-aiua-tormentosa-motor
persist_ref: docs/features/nucleo-aiua-tormentosa-motor
pbi_ref: docs/todos/done/PBI_Arranque_Aiua.md
document_id: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
uuid: "2a4e6c88-1f3b-4d0e-9a2f-7e4b5401a892"
global: APTO
pbi_archived: true
pr_url: "https://github.com/racso80es/SddIA/pull/270"
ci_run_id: "34224347775"
ci_run_url: "https://github.com/racso80es/SddIA/actions/runs/34224347775"
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
  CA-11: APTO
  CA-CI: APTO
git_changes:
  - SddIA/conscience/
  - README.md
  - SddIA/CONSTITUTION_CORE.md
  - SddIA/core/cumulo.paths.json
  - SddIA/tools/thought-graph-access.md
  - SddIA/tools/thought-graph-access/
  - SddIA/actions/retrieve-active-context.md
  - SddIA/actions/invoke-aiua-core.md
  - SddIA/actions/persist-thought-record.md
  - SddIA/process/aiua-stimulus-processing.md
  - SddIA/infrastructure/adapters/lancedb_thought_repo/
  - SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs
  - SddIA/scripts/qa/build-wasi-capsules.sh
  - docs/features/nucleo-aiua-tormentosa-motor/
  - docs/todos/done/PBI_Arranque_Aiua.md
---

# Validacion — nucleo-aiua-tormentosa-motor

`global: APTO`. PBI archivado en `docs/todos/done/` en esta rama. CA-CI: run `34224347775` (PR #270, head `fab3256`).

## Checks

| CA | Veredicto | Evidencia |
|----|-----------|-----------|
| CA-1 | APTO | `aiua_core.md` uuid `942aa727-…`, v1.1.0, secciones 1–5. |
| CA-2 | APTO | `conscience/index.md` fila uuid coincidente. |
| CA-3 | APTO | README fila Aiúa; proceso sin `agent:`. |
| CA-4 | APTO | `cumulo.paths.json` `directories.conscience`; version 1.10.0. |
| CA-5 | APTO | Constitución → `paths.directories.conscience`. |
| CA-6 | APTO | `aiua-stimulus-processing.md` 4 fases, `workspace_template`, sin `agent:`. |
| CA-7 | APTO | Tres acciones forjadas; persist no emite ECST. |
| CA-8 | APTO | Crate `thought-graph-access` search/store; 3 tests. |
| CA-9 | APTO | `store_thought_emits_thought_persisted_when_bus_configured`. |
| CA-10 | APTO | `lab_mock_empty_memories_yields_duration_and_thought_id`. |
| CA-11 | APTO | `process_genome_has_no_kalma2_ui_coupling`. |
| CA-CI | APTO | Run [34224347775](https://github.com/racso80es/SddIA/actions/runs/34224347775): `sddia-index-integrity`, `wasi-runtime-smoke`, `eda-iota-smoke-simulate`, `eda-bus-e2e-smoke`, `eda-iota-physical` SUCCESS. |

## Tests

```text
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p thought-graph-access -p sddia-infrastructure-lancedb-thought
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib -- aiua_stimulus
```
