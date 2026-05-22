---
feature_name: refactor-topologia-eventos-ola-c-v3
created: "2026-05-22"
process: refactorization
branch_name: feat/refactor-topologia-eventos-ola-c-v3
persist_ref: docs/features/refactor-topologia-eventos-ola-c-v3
pbi_ref: docs/todos/pending/TODO_Refactor_Topologia_Eventos_Ola_C_V3.md
supersedes_partial: docs/features/ola-c-v3-coreografia
priority: kaizen-eda-topologia
updated: "2026-05-22"
status: en_curso
implementation: docs/features/refactor-topologia-eventos-ola-c-v3/implementation.md
validacion: docs/features/refactor-topologia-eventos-ola-c-v3/validacion.md
---

# Objetivos — Refactor topología eventos Ola C V3+

## Misión

Evolucionar la **gestión runtime de eventos EDA** desde el modelo V3 (padre inmutable en `pending/` + testigos aislados bajo `subscribers/`) hacia una topología **simétrica por estado**, donde cada fase del bus contiene **cabecera del evento** y subcarpeta **`subscribers/`** con un fichero por proceso suscriptor. Promover **`route-domain-event`** de acción a **proceso** orquestador con fan-out asíncrono y promoción de testigos según resultado.

## Contexto (delta sobre V3 entregado)

| Aspecto | V3 actual (`ola-c-v3-coreografia`) | Objetivo Kaizen (este PBI) |
|---------|-------------------------------------|----------------------------|
| Padre en `pending/` | Inmutable ✅ | Se mantiene como entrada |
| Estados del evento | Solo testigos en `subscribers/{processing,processed,dead-letter}/` | **Cabecera del evento** también en `processing/`, `processed/`, `dead-letter/` |
| Subscriptores | Ficheros planos `[UUID].[subscriber].json` | Bajo `{estado}/subscribers/` |
| Orquestador | Acción `route-domain-event` | **Proceso** `route-domain-event` (artefactos SddIA) |
| Convivencia multi-estado | Parcial (sweeper + testigos) | Evento puede coexistir en carpetas según estado de cada suscriptor |
| Watcher | `event-watcher.py` → acción | Adaptar al nuevo procedimiento |

## Hitos

| Hito | Contenido | Entregable |
|------|-----------|------------|
| **K1** | Topología SSOT | `cumulo.paths.json`, `eda_bus_utils`, `.gitignore`, bootstrap |
| **K2** | Proceso orquestador | `SddIA/process/route-domain-event.md` + handler lab / runtime |
| **K3** | Retirada acción legacy | Deprecar/migrar `SddIA/actions/route-domain-event.md` e índice |
| **K4** | Watcher + sweeper | `event-watcher.py`, `event-sweeper.py` alineados a K1–K2 |
| **K5** | Fan-out asíncrono + contrato testigo | Dispatcher async, decoración §5, promoción por callback |
| **K6** | Documentación + E2E | `implementation.md`, `validacion.md`, cierre PBI |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | **Topología simétrica** | Árbol `./.events/{pending,processing,processed,dead-letter}` con `subscribers/` en processing/processed/dead-letter |
| O2 | **Proceso orquestador** | Fan-out asíncrono; creación de testigos en `processing/subscribers/`; promoción a processed/dead-letter con decoración de resultado |
| O3 | **Cabecera replicada** | Si en destino no existe copia del evento padre, crearla al promover el último suscriptor o al entrar en processing |
| O4 | **Limpieza processing** | Al cerrar el último suscriptor del evento, eliminar cabecera de `processing/` |
| O5 | **Paridad SddIA** | Forja vía `entity-manager` si aplica; contratos process/actions/events coherentes |
| O6 | **Watcher adaptado** | `event-watcher.py` invoca proceso, no acción legacy |
| O7 | **Sin regresión V3** | Suscripciones en `event-subscriptions.json` siguen operativas; idempotencia y filtro topológico preservados |

## Fuera de alcance

- Cambios al genoma de Clases ECST (`SddIA/events/`) salvo contratos de emisión.
- Nuevos tipos de evento o suscriptores de negocio.
- Migración de histórico en bus runtime (colas volátiles).

## Manifiesto operativo

Origen: `docs/todos/pending/TODO_Refactor_Topologia_Eventos_Ola_C_V3.md`

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Proceso de entrega: **`refactorization`** v1.2.0 → `delivery-close-cycle` (`source_process: refactorization`).
- Laboratorio: fase 1 física (`workspace-init`); fases Mayeuta–Argos simuladas hasta runtime IDE completo.
