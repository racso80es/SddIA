---
feature_name: telemetria-reactiva-eda-fase1
created: "2026-05-27"
process: feature
items:
  - id: "1.A"
    touchpoint: "SddIA/events/{telemetry,orchestration,domain}/"
    proposal: "Topología fractal + Códices index.md"
  - id: "1.B"
    touchpoint: "SddIA/events/events-contract.md"
    proposal: "v1.1.0 + event_family + § Trinidad"
  - id: "1.A-prime"
    touchpoint: "SddIA/events/domain/*.md"
    proposal: "Migración 7 ECST + cabecera event_family"
  - id: "1.C"
    touchpoint: "SddIA/process/event-creator.md"
    proposal: "default domain + effective_event_family"
  - id: "1.D"
    touchpoint: "SddIA/events/telemetry/raw-execution-finished.md"
    proposal: "Clase ECST telemetría"
  - id: "1.E"
    touchpoint: "SddIA/scripts/qa/ecst_validation.py"
    proposal: "rglob genoma fractal utf-8-sig"
---

# Implementación — Fase 1

| Paso | Archivos | Cambio |
|------|----------|--------|
| 1.A | `events/index.md`, `*/index.md` | Índice familias + Códices |
| 1.B | `events-contract.md` | v1.1.0, trinidad, rutas `{family}/` |
| 1.A′ | `domain/*.md` (git mv) | 7 clases migradas |
| 1.C | `event-creator.md` v1.1.0 | Fase 0 normalización; default `domain` |
| 1.D | `telemetry/raw-execution-finished.md` | Nueva Clase |
| 1.E | `ecst_validation.py`, plantilla EDA, refs Core | Regresión + rutas `domain/` |

Referencias Core actualizadas: `emit-pr-presented-event.md`, `cumulo.md`, `execute-action.py`, `obediencia-procesos.md`.
