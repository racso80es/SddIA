---
feature_name: ola-c-v3-coreografia
created: "2026-05-22"
process: feature
branch_name: feat/ola-c-v3-coreografia
persist_ref: docs/features/ola-c-v3-coreografia
---

# Objetivos — Ola C V3: Coreografía Asíncrona

## Misión

Refactorizar el bus EDA a topología de **Estado de Suscriptores** bajo `/.events/`, eliminando el Evento Padre Mutante y habilitando recolección diferida vía `event-sweeper.py`.

## Hitos

| Hito | Contenido | Estado |
|------|-----------|--------|
| C3.1 | SSOT `event_bus`, `.gitignore`, `eda_bus_utils` | ✅ |
| C3.2 | Bootstrap topología al arranque | ✅ |
| C3.3 | Testigos `[UUID].[SUSCRIPTOR].json` | ✅ |
| C3.4 | `event-sweeper.py` + alerta Kaizen | ✅ |

## Criterio de éxito

- Padre ECST inmutable en `.events/pending/` durante procesamiento.
- Testigos en `subscribers/{processing,processed,dead-letter}/`.
- Sweeper purga padre solo con consenso `processed/`; dead-letter → alerta sin purga.
