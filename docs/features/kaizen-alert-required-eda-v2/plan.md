---
feature_name: kaizen-alert-required-eda-v2
created: "2026-05-25"
process: feature
branch_name: feat/kaizen-alert-required-eda-v2
---

# Plan — Kaizen_Alert_Required (EDA v2)

## Fases de trabajo

| Fase | Actividad | Resultado |
|------|-----------|-----------|
| 1 | Forja ECST + índice events | `kaizen-alert-required.md` |
| 2 | Suscripción Sistema Nervioso | `event-subscriptions.json` |
| 3 | Acción Cúmulo + handler execute-action | `materialize-kaizen-alert-doc` |
| 4 | Emisión Aduana + poda puente v1 | `_emit_kaizen_alert_required`, limpieza cápsula Kaizen |
| 5 | Genoma aduana + Cúmulo | `pull-request-review` v2.2.0, `cumulo.md` |
| 6 | Smoke E2E + integridad | `execution.md`, `validacion.md` APTO |
| 7 | Cierre documental en rama | PBI → `done/` |

## Secuencia de verificación

1. `pull-request-review` lab con DIA alerta → evento en `.events/pending/`.
2. `event-watcher.py --once` → `PENDING_AUDIT_DOC_*.md` materializado.
3. Grep: cero `_dia_audit_hash`, cero escritura DIA en `capsule_pr_review_kaizen`.
4. `verify-process-integrity.py` OK.

## Artefactos tocados

| Artefacto | Acción |
|-----------|--------|
| `SddIA/events/kaizen-alert-required.md` | Crear |
| `SddIA/events/index.md` | Fila ECST |
| `SddIA/actions/materialize-kaizen-alert-doc.md` | Crear |
| `SddIA/actions/index.md` | Fila acción |
| `SddIA/core/event-subscriptions.json` | Entrada Kaizen_Alert_Required |
| `SddIA/scripts/qa/execute_process_capsules.py` | Emisión + poda |
| `SddIA/scripts/qa/execute-action.py` | Handler Cúmulo |
| `SddIA/process/pull-request-review.md` | v2.2.0 DIA-2/3 |
| `SddIA/agents/cumulo.md` | § reactivo EDA |
| `SddIA/agents/cumulo.instructions.json` | Mandato machine-readable |
| `docs/features/kaizen-alert-required-eda-v2/*` | Cascada documental |
| `docs/todos/pending/kaizen-alert-required-eda-v2.md` | → `done/` |
