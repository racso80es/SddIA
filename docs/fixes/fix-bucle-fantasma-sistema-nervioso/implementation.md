---
feature_name: fix-bucle-fantasma-sistema-nervioso
created: "2026-05-29"
process: bug-fix
branch_name: fix/bucle-fantasma-sistema-nervioso
version_implementation: "1.0.0"
status: implementado
index_prefix: "[ARQUITECTURA]"
---

# Implementación — [ARQUITECTURA] fix-bucle-fantasma-sistema-nervioso

## Cambios de código

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/daemons/event-watcher.py` | `processing_uuids`, `routed_ok_pending_absent` (D3), `_watcher_skip_reason`, `_prune_routed_ok_pending_absent`, logs skip |
| `SddIA/scripts/qa/eda_bus_utils.py` | `safe_remove_path`; `archive_event_after_sweep` y `maybe_purge_fractal_telemetry_when_terminal` |
| `SddIA/scripts/qa/route_fractal_event_core.py` | Purga vía `safe_remove_path`; `purged` / `purge_failed` honestos |
| `SddIA/scripts/qa/purge_stale_events.py` | **Nuevo** — CLI `--dry-run` / `--apply` / `--json` |
| `SddIA/scripts/qa/test_bucle_fantasma_bus.py` | **Nuevo** — T1–T4 (unittest) |

## `safe_remove_path`

- Hasta 3 intentos, 50 ms entre intentos.
- Captura `PermissionError` y `OSError`.
- Retorna `True` si el archivo no existe o se eliminó.

## Watcher (F1 + D3)

| Set | Semántica |
|-----|-----------|
| `processing_uuids` | UUID en vuelo hasta retorno de `execute-process` |
| `routed_ok_pending_absent` | Route exit 0 y archivo aún en disco — no re-despachar |

`_prune_routed_ok_pending_absent` limpia UUIDs cuando el archivo desaparece de todas las colas vigiladas.

## Sin cambios

- `event-sweeper.py`, `route_domain_event_core.py` (sweep pending V3+)
- Suscripciones JSON
- README / `events-contract.md` (pendiente Hito 5 documental opcional en PR)
