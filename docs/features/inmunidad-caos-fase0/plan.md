---
feature_name: inmunidad-caos-fase0
created: "2026-05-28"
process: feature
phases:
  - "0.A Inventario de acoplamientos"
  - "0.B Matriz de gaps vs. PBI"
  - "0.C Decisiones y refinamiento PBI v2.1.0"
  - "0.D impact-analysis.md"
---

# Plan — Fase 0 · Análisis de implicaciones (Caos / Suite)

## Secuencia

| Paso | Actividad | Salida |
|------|-----------|--------|
| **0.A** | Barrido: `entity-manager`, tools, sandbox, workspaces, Cerbero/RBAC, bus domain, Radamanto, compliance, subprocesses | Tabla H01–H28 |
| **0.B** | Contrastar con Fases 1–5 del PBI | Clasificación gap (a–d) |
| **0.C** | Cerrar D0.1–D0.9; reordenar Fases 4–5; subtareas inline | `clarify.md` + PBI v2.1.0 |
| **0.D** | Redactar `impact-analysis.md` | AC0.1–AC0.4 |
| **Cierre** | Validar ejecutabilidad Fases 1–5 | AC0.5 |

## Criterios de aceptación (PBI)

- **AC0.1** — `impact-analysis.md` completo
- **AC0.2** — Bloqueantes con decisión o subtarea en Fase 1–5
- **AC0.3** — Conflictos genómicos (`suite`, tools ofensivas, sandbox) en matriz
- **AC0.4** — Jurisdicción DLT `System_Immunity_Certified` explicitada
- **AC0.5** — Fases 1–5 ejecutables sin ambigüedad bloqueante

## Post-Fase 0

Abrir **`inmunidad-caos-fase1`** solo tras merge de esta feature con `validacion.md` APTO.
