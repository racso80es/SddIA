---
feature_name: eda-coverage-ssot-bus-isolation
created: "2026-05-25"
process: feature
version_implementation: "1.0.0"
---

# Implementación — SSOT eda-coverage y desacople bus EDA

## F1 — SSOT esqueleto

| Archivo | Cambio |
|---------|--------|
| `SddIA/core/eda-coverage.json` | SSOT con `coverage_matrix` |
| `SddIA/core/cumulo.paths.json` | Clave `eda_coverage` |

## F2 — Parametrización bus

| Archivo | Cambio |
|---------|--------|
| `eda_bus_utils.load_eda_bus()` | Precedencia `EVENT_BUS_PATH` → cumulo → `.events` |
| `env_loader.load_test_env_overlay()` | Carga `.env.test` o `.env.test.example` |
| `run-eda-e2e-lab.py` | Jerarquía bóvedas + overlay test al inicio |
| `.dev/.env.test.example` | Plantilla lab/CI |
| `.dev/.env.example` | Documentación `EVENT_BUS_PATH` |

## F3 — Módulo coverage

| Archivo | Cambio |
|---------|--------|
| `eda_coverage_utils.py` | load / upsert / remove / is_entity_covered; escritura atómica |

## F4 — Emisión doble fase

| Archivo | Cambio |
|---------|--------|
| `execute-action.py` → `_run_emit_domain_mutation` | Upsert SSOT antes de `_write_pending_event`; delete elimina entrada |

## F5–F6 — Backfill y audit

| Archivo | Cambio |
|---------|--------|
| `audit-entity-eda-coverage.py` | `--backfill-coverage`; `--scan` vía SSOT (`scan_source: eda-coverage.json`) |

## F7 — Sweep y router

| Archivo | Cambio |
|---------|--------|
| `eda_bus_utils.archive_event_after_sweep` | Sweep vacío (sin retención cabeceras) |
| `eda_bus_utils.applicable_subscriber_ids_for_event` | Filtro topológico sin fallback |
| `eda_bus_utils.try_sweep_event` | Purga si aplicables vacíos |
| `route_domain_event_core.py` | Sweep cuando `subscribers == []` |
| `test_eda_bus_v3plus.py` | Regresión sweep vacío + EVENT_BUS_PATH + topología local |

## Normativa

| Archivo | Cambio |
|---------|--------|
| `features-documentation-pattern.md` | § Cobertura EDA — SSOT eda-coverage |
