---
feature_name: telemetria-reactiva-eda-fase1
created: "2026-05-27"
process: feature
items_applied:
  - "1.A Topología fractal"
  - "1.B events-contract v1.1.0"
  - "1.A-prime Migración 7 ECST a domain/"
  - "1.C event-creator default domain (D1.9)"
  - "1.D Raw_Execution_Finished"
  - "1.E ecst_validation rglob + BOM fix + refs"
---

# Ejecución — Fase 1

## Aplicado

- Creadas subcarpetas `telemetry/`, `orchestration/`, `domain/` con Códices `index.md`.
- `events-contract.md` → v1.1.0 con `event_family` obligatorio en Clase y § Trinidad.
- `git mv` de 7 ECST a `domain/`; cabeceras `event_family: domain`, contrato v1.1.0.
- `event-creator` v1.1.0: fase Normalización; `effective_event_family` default `domain`; rutas `{family}/{name}.md`.
- Forjada `Raw_Execution_Finished` en `telemetry/` (uuid `5a02d313-685d-4464-84c1-ffe16ef6ba6d`).
- `ecst_validation.py`: carga por `rglob` + `utf-8-sig`; 8 tipos ECST verificados en smoke local.
- `test_eda_bus_v3plus`: 14 tests OK.
- BOM UTF-8 retirado de archivos migrados (artefacto PowerShell).

## Smoke retrocompat (D1.9)

| Caso | Resultado esperado | Evidencia |
|------|-------------------|-----------|
| Invocación sin `event_family` | `effective_event_family = domain` | Contrato `event-creator.md` Fase 0 |
| Invocación `event_family: telemetry` | Ruta `telemetry/{name}.md` | `raw-execution-finished.md` |
| Validación ECST dominio | Schemas cargados post-migración | `load_event_class_schemas` → 8 |

## Pendiente operador

- `entity-manager` / `eda-coverage` backfill si pre-commit reporta huérfanos tras bump `event-creator`.
- PR vía `delivery-close-cycle` (Fase 7 proceso feature).
