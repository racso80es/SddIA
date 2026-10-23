---
feature_name: tracker-process-decouple
created: "2026-10-23"
audited_at: "2026-10-23T18:58:00+02:00"
process: refactorization
phase: Verificación
agent: argos
agents: agent:argos
branch: refactor/tracker-process-decouple
branch_name: refactor/tracker-process-decouple
persist_ref: docs/features/tracker-process-decouple
pbi_ref: "docs/todos/pending/[KAIZEN] Desacoplamiento del ciclo documental — inversión y ceguera espacial.md"
document_id: PBI-KAIZEN-TRACKER-PROCESS-DECOUPLE
uuid: "34a1e5b7-721d-455b-861a-de926e7f591a"
execution_id: "90e151ce-3830-4fa2-985a-784a93c90330"
global: APTO
pbi_archived: false
approval_status: aprobado
verdict: aprobado
git_manager_invoked: true
git_evidence_source: skill:git-manager
branch_worktree_observed: refactor/tracker-process-decouple
checks:
  DOC_OBJECTIVES: APTO
  DOC_CLARIFY: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  CASCADE_COMPLETE: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  PROCESS_INTEGRITY: APTO
  KZ-CA1: APTO
  KZ-CA2: APTO
  KZ-CA3: APTO
  KZ-CA4: APTO
  KZ-CA5: APTO
  KZ-CA6: APTO
  KZ-CA7: APTO
  ARGOS_NO_KM_WRITE: APTO
git_changes:
  - SddIA/core/eda-coverage.json
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/engine/execute-process/src/engine/tracker_lifecycle_port.rs
  - SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs
  - SddIA/engine/execute-process/src/engine/handlers/refine.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_backlog_query.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_linear_markdown_sync.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_markdown_apply.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs
  - SddIA/engine/execute-process/src/engine/materialize_ci_chronic_failure_pbi.rs
  - SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs
  - SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/events/domain/pull-request-audited.md
  - SddIA/events/domain/pull-request-merged.md
  - SddIA/events/domain/pull-request-presented.md
  - SddIA/events/domain/tracker-issue-changed.md
  - SddIA/events/domain/tracker-sync-failed.md
  - SddIA/events/domain/work-initiated.md
  - SddIA/evolution/Evolution_log.md
  - SddIA/evolution/0cb0e520-bd14-4d81-a46d-ff357987ce3b.md
  - SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md
  - SddIA/process/index.md
  - SddIA/process/refine-hu.md
  - SddIA/process/refine-pbi.md
  - SddIA/process/tracker-backlog-query.md
  - SddIA/process/tracker-description-sync.md
  - SddIA/process/tracker-linear-markdown-sync.md
  - SddIA/process/tracker-markdown-apply.md
  - SddIA/process/tracker-stamp.md
  - SddIA/process/tracker-sync-replay.md
  - docs/features/tracker-process-decouple/
---

# Validación — tracker-process-decouple

## approval_status

**aprobado.** Re-auditoría sobre el árbol posterior al `NO_APTO` de cascada. Proveedor efectivo del manifiesto Core: `linear`. El fichero del PBI permanece en `pending/`. `pbi_archived` queda en `false`.

## Cápsulas

- `skill:git-manager` `status` y `diff_name_only` (`ref_spec: HEAD`): `exitCode: 0`. Rama de trabajo con el corte sin commit. `tracker-description-sync.md`, `tracker_lifecycle_port.rs`, el registro `0cb0e520-bd14-4d81-a46d-ff357987ce3b.md` y `docs/features/tracker-process-decouple/` aparecen como no rastreados en `status`.
- `sddia-qa verify-process-integrity`: `OK`.

## Criterios

| ID | Veredicto | Evidencia |
|---|---|---|
| KZ-CA1 | APTO | Los ocho procesos, incluido `forge-pbi`, no contienen `Linear`, `linear`, `docs/todos`, `historias/`, `todos_pending`, `Realizado`, `pending/` ni `done/`. |
| KZ-CA2 | APTO | `refine-hu`: `tracker_ref` opaco, fase `Registro en Tracker`, un `create_issue`. |
| KZ-CA3 | APTO | El código de producción de los ocho handlers no bifurca por `docs/todos`, `"linear"`, `provider file` ni `Sello Linear`. El sustrato lo resuelve `substrate()`. El test `substrate_file_reads_buckets_remote_does_not` cubre los dos sustratos. |
| KZ-CA4 | APTO | `persist_ref` y el layout de features/fixes siguen en el genoma. |
| KZ-CA5 | APTO | Los procesos no reescriben los buckets. |
| KZ-CA6 | APTO | Sin adaptador GitHub nuevo. Los verbos del contrato no se renombran. |
| KZ-CA7 | APTO | Topología `docs/features/tracker-process-decouple/`. `entity-manager` acusó el sello de los ocho procesos y de las seis clases ECST. Registro `0cb0e520-bd14-4d81-a46d-ff357987ce3b`. |

## Conclusión

`global: APTO`. Sin escritura bajo `docs/todos/`. El cierre del issue en `state_map.done` y `delivery-close-cycle` quedan fuera de esta fase.
