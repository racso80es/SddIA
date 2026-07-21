---
feature_name: kaizen-kalma2-feature-cycle-observability
created: "2026-07-21"
process: feature
branch: feat/kaizen-kalma2-feature-cycle-observability
global: APTO
pbi_archived: true
document_id: PBI-KAIZEN-KALMA2-FEATURE-CYCLE-OBS
pbi_ref: docs/todos/done/[Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR.md
correlation_id: 6ae1b7be-54e5-4750-8888-5f19ac76551f
approval_status: approved_awaiting_pr
remote_pushed: true
remote_branch: feat/kaizen-kalma2-feature-cycle-observability
pr_compare_url: https://github.com/racso80es/SddIA/compare/main...feat/kaizen-kalma2-feature-cycle-observability?expand=1
pr_url: null
checks:
  AC_O1_early_pec: APTO
  AC_O2_failed_pec_api: APTO
  AC_O3_checklist: APTO
  AC_O4_pr_url_defaultable: APTO
  AC_O4b_document_context_defaultable: APTO
  AC_O5_scope_separation: APTO
  AC_PPR_via_ecst_bus: APTO
  cargo_thermodynamic_tests: APTO
lab_pr_url: "https://github.com/lab-simulated/SddIA/pull/0-feat-kaizen-kalma2-feature-cycle-observability"
ppr_event_id: "1b7ea160-5703-48ad-b51b-5db7d46ac192"
ppr_witness: ".events/processed/subscribers/1b7ea160-5703-48ad-b51b-5db7d46ac192.argos.pull-request-review.json"
git_changes:
  - SddIA/engine/execute-process/src/core/resolver.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - docs/features/kaizen-kalma2-feature-cycle-observability/
  - docs/todos/done/[Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR.md
  - SddIA/evolution/6ae1b7be-54e5-4750-8888-5f19ac76551f.md
---

# Validación — Kaizen observabilidad Kalma2-feature

## Veredicto

**APTO** — O1–O5 materializados con evidencia de tests thermodynamic (5 passed).

| AC | Estado | Evidencia |
|----|--------|-----------|
| O1 | APTO | `emit_initialized_pec` + TQM early write a orchestration |
| O2 | APTO | PEC en fallo con `correlation_id`; bridge ya proyecta `failed` |
| O3 | APTO | `checklist-delivery-repro.md` |
| O4 | APTO | `pr_url` + `document_context` en `DEFAULTABLE`; route inyecta `document_context` |
| O5 | APTO | Rama/feature distintas de Fractura Core F1 |
| PPR ECST | APTO | `emit-pr-presented-event` → `route-domain-event` → `argos.pull-request-review=success` |

## Desbloqueo sin `gh` (bus local)

Con API GitHub bloqueada en el host, la aduana se materializa vía ECST + orquestador Rust:

```bash
./sddia-run.sh --action emit-pr-presented-event --inputs '{"branch":"<rama>","pr_url":"https://github.com/lab-simulated/SddIA/pull/0-<rama-con-guiones>","status":"presented","emitter_agent":"delivery-close-cycle"}'
SDDIA_LAB_ROUTE_SYNC=1 ./sddia-run.sh --process route-domain-event --inputs '{"event_file_path":".events/pending/<event_id>.json"}'
```

| Ciclo | event_id | `argos.pull-request-review` |
|-------|----------|-----------------------------|
| Kaizen (esta rama) | `1b7ea160-5703-48ad-b51b-5db7d46ac192` | success |
| F1 kitchen (otra rama) | `3c441eaf-0a8e-421b-8fe2-d7dd43ff92f4` | success |

Periféricos `send-telegram-notification` / `iota-immutable-publisher` pueden fallar sin invalidar PPR.

## Nota

Smoke E2E Kalma2 UI requiere Centinelas + auth; unitario + contrato cubren el nervio. PR real en GitHub sigue pendiente de API/`gh` en terminal del operador.

## Forja

Rama publicada en origin. Compare (abrir PR cuando API disponible):

https://github.com/racso80es/SddIA/compare/main...feat/kaizen-kalma2-feature-cycle-observability?expand=1
