---
feature_name: delivery-close-hook-eda-governance
created: "2026-05-22"
process: bug-fix
version_plan: "1.0.0"
---

# Plan de ejecución — delivery-close-hook-eda-governance

## Fase 0 — Inicialización ✅

- [x] Rama `fix/delivery-close-hook-eda-governance`
- [x] `persist_ref`: `docs/fixes/delivery-close-hook-eda-governance/`
- [x] Cascada documental: objectives, clarify, spec, plan, implementation, execution, validacion

## Fase 1 — Anti-recursión (O1, O5, O6) — Tekton ✅

| # | Tarea | Archivos |
|---|-------|----------|
| 1.1 | Guarda `SDDIA_HOOK_DELIVERY_CLOSE` en hook + skip temprano | `hook_common.py`, `pre_push_gate.py` |
| 1.2 | Push interno con `SDDIA_SKIP_HOOKS=1` acotado | `execute_process_capsules.py` |
| 1.3 | Skip PR MERGED + `resolve_persist_ref` fixes | `hook_common.py` |
| 1.4 | Smoke lab documentado | `execution.md`, `validacion.md` |

## Fase 2 — Retroactivo PR #20 (O2) — Tekton ✅

| # | Tarea | Detalle |
|---|-------|---------|
| 2.1 | Emit Presented retroactivo | `868d1b8f-0171-4f8f-ab72-19382941523d` |
| 2.2 | Emit Merged retroactivo | `75b8e950-9366-4ce5-bf22-b4b56430736e` |
| 2.3 | Watcher + evidencia | `validacion.md` |

## Fase 3 — Gobernanza (O3) — Dedalo/Tekton ✅

| # | Tarea | Archivo |
|---|-------|---------|
| 3.1 | Ley de Jurisdicción Delegada | `SddIA/norms/obediencia-procesos.md` v1.1 |
| 3.2 | Enlace desde `pull-request-orchestration.md` | §7 |
| 3.3 | Actualizar PBI §6 protocolo Kintsugi | PBI origen |

## Fase 4 — Kintsugi EDA + Autoconocimiento (O4) — Tekton ✅

| # | Tarea | Entregable |
|---|-------|------------|
| 4.1 | Contrato evento | `SddIA/events/system-fracture-detected.md` |
| 4.2 | Suscripción dual Cúmulo + Mayeuta | `event-subscriptions.json` |
| 4.3 | Acción Cúmulo + handler | `materialize-fracture-pbi.md` |
| 4.4 | Acción Mayeuta + handler | `enrich-fracture-pbi-kaizen.md` |
| 4.5 | Backfill Fase C | `orphan_count_after: 0` |
| 4.6 | Smoke fan-out dual | watcher → PBI enriquecido |

Fase 5 — Verificación y cierre — Argos ✅

- [x] `validacion.md` con checks CA-* y event_ids
- [x] PR #23 mergeado vía `accept-pr` (`620d94c`)
- [x] PBI movido a `docs/todos/done/`

## Orden de dependencias

```
Fase 1 ──► Fase 5 (smoke hook)
Fase 2 ──► Fase 5 (retroactivo)
Fase 3 ──► independiente (normativa)
Fase 4 ──► Fase 3 (protocolo referencia evento)
```

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| `SDDIA_SKIP_HOOKS=1` mal usado globalmente | Solo `extra_env` en subproceso push |
| Retroactivo sin correlación Argos | Incluir `correlation_id` / `persist_ref` en payload |
| Cúmulo PBI duplicado | Idempotencia por hash de `error_trace` en nombre archivo |
