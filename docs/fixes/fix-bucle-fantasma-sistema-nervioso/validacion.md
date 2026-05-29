---
feature_name: fix-bucle-fantasma-sistema-nervioso
created: "2026-05-29"
process: bug-fix
branch: fix/bucle-fantasma-sistema-nervioso
global: PENDIENTE
pbi_archived: false
index_prefix: "[ARQUITECTURA]"
checks:
  CA1-watcher-no-concurrent-duplicate: pass
  CA2-watcher-skip-routed-ok-file-persists: pass
  CA3-safe-remove-retries: pass
  CA4-purge-stale-dry-run: pass
  CA5-sweeper-no-regression: pass
  CA6-kaizen-dead-letter: pass
git_changes:
  - SddIA/scripts/daemons/event-watcher.py
  - SddIA/scripts/qa/eda_bus_utils.py
  - SddIA/scripts/qa/route_fractal_event_core.py
  - SddIA/scripts/qa/purge_stale_events.py
  - SddIA/scripts/qa/test_bucle_fantasma_bus.py
  - docs/fixes/fix-bucle-fantasma-sistema-nervioso/
---

# Validación — [ARQUITECTURA] fix-bucle-fantasma-sistema-nervioso

**Veredicto global: PENDIENTE** (implementación y unittest OK; falta PR merge y cierre Argos formal).

## Criterios de aceptación (spec.md)

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA1 | Watcher: sin `execute-process` concurrente duplicado por UUID | ✅ | `test_skip_in_flight_and_routed_ok` |
| CA2 | D3: tras route OK y archivo persistente, no re-despacho | ✅ | `_watcher_skip_reason` routed-ok |
| CA3 | `safe_remove_path` con reintentos | ✅ | `test_safe_remove_retries_then_succeeds` |
| CA4 | `purge_stale_events --dry-run` detecta stale | ✅ | `test_detects_delivery_complete` |
| CA5 | `event-sweeper` / E2E lab sin regresión | ✅ | `test_eda_bus_v3plus` + `test_eda_fractal_bus` OK |
| CA6 | Kaizen dead-letter preservado | ✅ | Sin cambio en rama DL del watcher |

## Objetivos PBI

| ID | Estado |
|----|--------|
| O1 Idempotencia en caliente | ✅ |
| O2 Liberación determinista | ✅ |
| O3 Absorción latencia E/S | ✅ |
| O4 Telemetría fallo purga | ✅ (`purge_failed` en route fractal) |
| O5 Purga zona cero (lab) | ✅ |
| O6 No regresión sweeper | ✅ |
| O7 UX logs watcher | ✅ |
| O8 Prefijo [ARQUITECTURA] en PR | ⏳ |

## Cierre documental (pre-merge)

| Paso | Estado |
|------|--------|
| PBI → `docs/todos/done/` | ⏳ |
| `pbi_archived: true` en este archivo | ⏳ |
| `implementation.md` / `execution.md` completos | ⏳ |
| PR único mergeado en `main` | ⏳ |
