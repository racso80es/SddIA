---
feature_name: telemetria-reactiva-eda-fase0
created: "2026-05-27"
process: feature
phases:
  - "0.A Inventario de acoplamientos"
  - "0.B Matriz de gaps vs. PBI"
  - "0.C Decisiones y refinamiento"
  - "0.D impact-analysis.md"
---

# Plan — Fase 0 · Análisis de afectaciones

## Secuencia

| Paso | Actividad | Salida |
|------|-----------|--------|
| **0.A** | Barrido grep / índice Cúmulo: `.events/`, suscripciones, `persist_ref`, `featurePath`/`fixPath`, emisores, specs de procesos | Tabla ubicación × fase (1–6) × severidad |
| **0.B** | Contrastar hallazgos con tareas PBI Fases 1–6 | Clasificación (a) cubierto (b) ampliar (c) nueva subtarea (d) fuera de alcance |
| **0.C** | Resolver/escalar bloqueantes (V3+ `pending/` vs. bus fractal; DLT Cúmulo/Radamanto) | Decisiones en `clarify.md` + subtareas en PBI maestro si aplica |
| **0.D** | Redactar `impact-analysis.md` | AC0.1–AC0.4 |
| **Cierre** | Mayeuta/validación ejecutabilidad Fases 1–6 | AC0.5 |

## Criterios de aceptación (PBI)

- **AC0.1** — `impact-analysis.md` completo
- **AC0.2** — Bloqueantes con decisión o subtarea en Fase 1–6
- **AC0.3** — `featurePath`/`fixPath` clasificados en matriz
- **AC0.4** — Jurisdicción DLT explicitada
- **AC0.5** — Fases 1–6 ejecutables sin ambigüedad bloqueante

## Post-Fase 0

Abrir **feature independiente** para Fase 1 (`telemetria-reactiva-eda-fase1` convención) solo tras cumplir AC0.x y merge de esta feature.
