---
feature_name: event-bus-audit
created: "2026-07-11"
process: feature
branch_name: feat/event-bus-audit-process
persist_ref: docs/features/event-bus-audit
pbi_ref: docs/todos/pending/[FEATURE] Auditoría empírica del bus de eventos.md
impacts_doc: true
---

# Especificación técnica — event-bus-audit

## 1. Contexto

El bus EDA persiste instancias ECST en `./.events/` (DLT V3+ y bus fractal). Existen auditorías parciales (`telemetry-compliance-audit`, `daemon-heartbeat-audit`) pero no un proceso on-demand que inspeccione empíricamente el estado global del bus: pendientes, procesados, dead-letter y familias fractales.

**Objetivo:** proceso `event-bus-audit` invocable a petición que escanea `./.events/`, valida coherencia ECST, detecta anomalías (staleness, huérfanos, tipos desconocidos) y emite informe + `Kaizen_Alert_Required` cuando proceda.

## 2. Entidades

| Entidad | Tipo | Ruta |
|---------|------|------|
| `event-bus-audit` | process | `SddIA/process/event-bus-audit.md` |
| `event-bus-audit` | tool | `SddIA/tools/event-bus-audit/` |

## 3. Inputs del proceso

| Campo | Tipo | Default | Descripción |
|-------|------|---------|-------------|
| `stale_threshold_hours` | number | 24 | Umbral de antigüedad para pending estancados |
| `emit_kaizen_alert` | boolean | true | Emitir `Kaizen_Alert_Required` si hay anomalías |

## 4. Outputs

| Campo | Descripción |
|-------|-------------|
| `audit_summary` | Conteos por estado y familia |
| `anomalies` | Lista de anomalías detectadas |
| `report_path` | Ruta del informe Markdown en workspace |
| `kaizen_event_id` | UUID del evento Kaizen emitido (si aplica) |

## 5. Fases

1. **Auditoría empírica** — delega a `tool:event-bus-audit` (cápsula Rust).

## 6. Criterios de aceptación

- CA1: Escaneo de todas las rutas `eda_bus.*` y `eda_fractal.*` desde `cumulo.paths.json`.
- CA2: Validación ECST mínima (campos obligatorios, JSON parseable, coherencia event_id ↔ filename).
- CA3: Detección de pending estancados (> umbral) y testigos huérfanos.
- CA4: Informe `audit-report.md` en workspace del proceso.
- CA5: Emisión `Kaizen_Alert_Required` en `eda_bus.pending` cuando hay dead-letters o anomalías estructurales.
- CA6: `./sddia-run.sh --process event-bus-audit --inputs '{}'` retorna `success: true`.

## 7. Impacto en Documentación

- Feature: `docs/features/event-bus-audit/`
- Evolución: registro en `SddIA/evolution/`
