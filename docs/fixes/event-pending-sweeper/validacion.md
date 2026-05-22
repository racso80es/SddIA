---
feature_name: event-pending-sweeper
created: "2026-05-22"
process: bug-fix
branch: fix/event-pending-sweeper
global: APTO
merged_pr: 29
merge_commit: 0ba2ac7e608db36321a51aefe4e9c1550a3d22c6
closed: "2026-05-22"
checks:
  CA1-emit-watcher-purge: pass
  CA2-dead-letter-kaizen: pass
  CA3-sweeper-idempotent: pass
  CA4-lab-route-sync: pass
  eda-e2e-lab: pass
git_changes:
  - SddIA/scripts/qa/eda_bus_utils.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/scripts/daemons/event-sweeper.py
  - SddIA/scripts/daemons/event-watcher.py
  - SddIA/scripts/qa/run-eda-e2e-lab.py
  - SddIA/events/events-contract.md
  - README.md
  - docs/fixes/event-pending-sweeper/
---

# Validación — event-pending-sweeper

**Veredicto global: APTO**

## CA1 — Purga automática post-route

| Check | Evidencia |
|-------|-----------|
| `try_sweep_event` en cierre route | `route_domain_event_core.py` → `data.sweep.status: purged` |
| Padre ausente tras E2E | `run-eda-e2e-lab.py` → `parent_purged: true`, exit 0 |
| Log watcher | `"enrutado y purgado de pending"` cuando `sweep.status == purged` |

## CA2 — Dead-letter / Kaizen

| Check | Evidencia |
|-------|-----------|
| `try_sweep_event` con dead-letter | Retorna `status: kaizen`, `purged: false` |
| Sweeper alerta | `_emit_kaizen_alert` preservado en `event-sweeper.py` |

## CA3 — Sweeper idempotente

| Check | Evidencia |
|-------|-----------|
| Refactor delegación | `sweep_once` → `try_sweep_event` por UUID |
| Segunda invocación | `status: absent` si padre ya purgado |

## CA4 — Modo sync lab

| Check | Evidencia |
|-------|-----------|
| `SDDIA_LAB_ROUTE_SYNC=1` | Smoke tool E2E exit 0 con `dispatch_mode: sync` |

## Objetivos PBI

| ID | Estado |
|----|--------|
| O1 Reproducibilidad | ✅ Baseline documentado en `clarify.md` |
| O2 Cierre automático | ✅ `try_sweep_event` integrado |
| O3 Semántica dead-letter | ✅ Kaizen preservado |
| O4 Compatibilidad sweeper | ✅ Daemon refactorizado, sin cambio externo |
| O5 UX operador | ✅ Logs watcher actualizados |
