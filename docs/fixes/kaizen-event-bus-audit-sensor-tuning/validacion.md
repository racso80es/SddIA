---
feature_name: kaizen-event-bus-audit-sensor-tuning
created: "2026-09-05"
process: bug-fix
branch: fix/kaizen-event-bus-audit-sensor-tuning
persist_ref: docs/fixes/kaizen-event-bus-audit-sensor-tuning
global: APTO
pbi_archived: true
document_id: PBI-KAIZEN-EVENT-BUS-AUDIT-31867981
uuid: 46dde226-6672-420f-8d2a-a5f3b49cdea8
execution_id: "f9830175-0405-42fd-9e0c-e6de1c26201d"
checks:
  EBA-CA1_non_ecst_sink: APTO
  EBA-CA2_needs_kaizen_false_historico: APTO
  EBA-CA3_needs_kaizen_true_circuit_stale: APTO
  EBA-CA4_plantilla_bus: APTO
  EBA-CA5_dedupe_huella: APTO
  EBA-CA6_tests: APTO
  EBA-CA7_pbi_archivado: APTO
  EBA-CA8_cero_events: APTO
  DOC_SPEC: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_EVOLUTION: APTO
git_changes:
  - docs/fixes/kaizen-event-bus-audit-sensor-tuning/
  - docs/todos/done/PENDING_AUDIT_DOC_31867981.md
  - SddIA/tools/event-bus-audit/src/main.rs
  - SddIA/tools/event-bus-audit.md
  - SddIA/tools/index.md
  - SddIA/process/event-bus-audit.md
  - SddIA/process/index.md
  - SddIA/actions/materialize-kaizen-alert-doc.md
  - SddIA/actions/index.md
  - SddIA/events/domain/kaizen-alert-required.md
  - SddIA/events/domain/index.md
  - SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs
  - SddIA/evolution/65199eab-7ddf-47d1-b821-e865517f637b.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — kaizen-event-bus-audit-sensor-tuning

`global: APTO`. PBI en `docs/todos/done/PENDING_AUDIT_DOC_31867981.md`. Evolution `65199eab-7ddf-47d1-b821-e865517f637b` (`EVOL_OK`).

## Checks

| ID | Resultado | Evidencia |
|----|-----------|-----------|
| EBA-CA1 | APTO | `cargo test -p event-bus-audit` — `github_bridge_dump_is_non_ecst_sink_not_structural` |
| EBA-CA2 | APTO | `needs_kaizen_ignores_historical_dl_and_fracture_stale` — `needs_kaizen_actionable(false, 0) == false` |
| EBA-CA3 | APTO | mismo test: circuit o stale accionable → true |
| EBA-CA4 | APTO | `materialize_dedupes_open_doc_same_files_different_review` — cuerpo sin «fuga»/«sensor DIA» |
| EBA-CA5 | APTO | segundo `run` idempotente por huella con casilla bus abierta |
| EBA-CA6 | APTO | event-bus-audit 6/6; execute-process `materialize_kaizen` 4/4 |
| EBA-CA7 | APTO | `docs/todos/done/PENDING_AUDIT_DOC_31867981.md` `document_id` intacto |
| EBA-CA8 | APTO | diff sin `.events/` |

CI GitHub: verificación post-PR (mandato operador); no es gate de este `validacion.md`.
