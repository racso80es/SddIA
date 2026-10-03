---
document_id: PBI-DEUDA-FEATURES-F2-DOC
uuid: "21f245f0-bf5b-4640-8aee-bddc095e65fa"
title: "[DEUDA] Tracker — patrón documental F2 (spec, plan, implementation)"
format: markdown
version: "1.1.0"
status: pending
priority: alta
type: deuda
process: feature
dispatch: true
feature_name: tracker-features-f2
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
tracker_ref: OSC-10
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
refined: "2026-10-02"
especificacion_cerrada: "2026-10-02"
execution_mode: autonomo
cola_ejecucion: docs/todos/pending/
derived_from_pr: "https://github.com/racso80es/SddIA/pull/316"
blocked_by: []
unblocks:
  - PBI-DEUDA-PREPUSH-ARGOS-QA
---

# Patrón F2 de las features Tracker

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001`.

## 0. Filtro A

| Afirmación de la semilla | Realidad | Corrección |
|--------------------------|----------|------------|
| Faltan `spec.md`, `plan.md` e `implementation.md` | Cierto en las ocho carpetas. El gate que falló es la fase «Triaje documental» de `pull-request-review` v2.3.0: exige esos tres **y** `objectives.md`. | `objectives.md` entra en el criterio. Siete carpetas ya lo tienen. |
| El spike se alinea «al patrón de spikes» o se documenta una excepción en la norma | `features-documentation-pattern` v1.2.1 no define artefacto `informe.md` ni excepción de spike. `spike-linear-done-gate/` tiene `informe.md` y `validacion.md`, y no tiene `objectives.md`. | No se muta la norma. El spike gana los cuatro archivos F2; `informe.md` se queda como entregable del spike y los cuatro lo referencian. |
| Un review de laboratorio de una carpeta absuelve la serie | El triaje mira el `persist_ref` de esa corrida. | Hace falta el cuarteto en las ocho. Una corrida verde sobre una carpeta no cubre las otras. |
| Hay que rellenar también `clarify.md` y `execution.md` | Están en la norma de fases. No están en el intent de Triaje documental. | Fuera de este PBI. |

## 1. Alcance

Backfill del código ya en PR #316. No es un rediseño.

| Carpeta | Hueco F2 |
|---------|----------|
| `tracker-operations-context` | spec, plan, implementation |
| `project-tracker-contract` | spec, plan, implementation |
| `linear-tracker-adapter` | spec, plan, implementation |
| `kalma2-tracker-backlog` | spec, plan, implementation |
| `work-initiated-event` | spec, plan, implementation |
| `tracker-sync-failed` | spec, plan, implementation |
| `tracker-stamp` | spec, plan, implementation |
| `spike-linear-done-gate` | objectives, spec, plan, implementation (conservar `informe.md`) |

Frontmatter mínimo según la tabla de `features-documentation-pattern` v1.2.1 (`feature_name`, `created`, y `base` en `spec.md`).

## 2. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | Las ocho carpetas contienen `objectives.md`, `spec.md`, `plan.md` e `implementation.md` con frontmatter. El cuerpo describe el código de PR #316, no un diseño nuevo. |
| AC-2 | `spike-linear-done-gate/informe.md` sigue siendo el entregable del spike. |
| AC-3 | La fase Triaje documental, apuntando a una de las carpetas de implementación, no falla por ausencia de esos cuatro archivos. |

## 3. Fuera de alcance

- Mutar `features-documentation-pattern` o `pull-request-review`.
- Crear `clarify.md` o `execution.md`.
- Cambiar el veredicto ya archivado de PR #316.
