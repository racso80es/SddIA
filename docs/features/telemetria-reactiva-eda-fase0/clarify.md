---
feature_name: telemetria-reactiva-eda-fase0
created: "2026-05-27"
purpose: Cierre de decisiones Fase 0 e incorporación al PBI v1.1.0
---

# Clarificación — Fase 0

## Decisiones cerradas (incorporadas al PBI maestro v1.1.0)

| ID | Pregunta | Resolución |
|----|----------|------------|
| D0.1 | ¿Quién firma DLT tras Radamanto? | Coexistencia: Cúmulo en PR/ECST hasta handoff Fase 4.0; Radamanto en gobernanza de herramientas |
| D0.2 | ¿Big-bang del bus? | No — V3+ `pending/` convive con rutas fractal |
| D0.3 | ¿Dónde vive `featurePath`? | Declarar `paths.workspacesRoot`; deprecar feature/fix en SSOT y scripts |
| D0.4 | ¿Apagar `event-watcher` monolítico? | No hasta validar PR Presented → review (3.C.1) |
| D0.5 | ¿Quién emite telemetría? | Solo CLI (Peaje Termodinámico); clase `Raw_Execution_Finished` en Fase 1.D |
| D0.6 | ¿Archivar PBI maestro en Fase 0? | No — permanece `pending/`; esta feature cierra con `pbi_archived: false` |

## Ejecutabilidad Fases 1–6 (AC0.5)

Tras refinamiento inline (§ 1.D, 1.E, 2.D, 3.C.1, 4.0), las fases siguientes son ejecutables sin ambigüedad bloqueante pendiente de Vértice Biológico.

## Referencia

- Inventario: `impact-analysis.md` (H01–H26)
- PBI: `docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md` v1.1.0
