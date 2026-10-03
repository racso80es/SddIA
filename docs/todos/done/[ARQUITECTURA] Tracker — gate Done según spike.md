---
document_id: PBI-ARQUITECTURA-TRACKER-DONE-GATE
uuid: "2085a7f7-578a-43df-a5b7-08b4a25ff18f"
title: "[ARQUITECTURA] Tracker — laudo de no migrar el gate de Done"
format: markdown
version: "1.1.0"
status: pending
priority: media
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-done-gate
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
tracker_ref: OSC-11
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
refined: "2026-10-02"
especificacion_cerrada: "2026-10-02"
execution_mode: autonomo
cola_ejecucion: docs/todos/pending/
derived_from:
  - "docs/todos/done/[SPIKE] Tracker — migración del gate de Done a Linear.md"
  - "docs/features/spike-linear-done-gate/informe.md"
blocked_by: []
unblocks: []
---

# Laudo: el gate de Done no migra

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` (D1). Spike archivado: `PBI-SPIKE-LINEAR-DONE-GATE`.

## 0. Filtro A

| Afirmación de la semilla | Realidad | Corrección |
|--------------------------|----------|------------|
| El informe no elige y hay que pedir laudo | `informe.md` recomienda mantener el gate y usar Linear como espejo. Condiciona una revisita a un E2E mock del ciclo (AC-8), que no existe. | Este PBI ejecuta esa recomendación: no migrar. No abre otra ronda de laudo. |
| El spike respondió Q-1–Q-6 | El informe no cita rutas ni líneas y no trae borrador de HU (AC-SPK-1 y AC-SPK-3 del spike incumplidos). | Este PBI no rehace el spike. |
| Q-5 es «el motor ignora `docs_layout`» | El handler `feature-pbi-archive` cablea `docs/todos/pending/` y `docs/todos/done/` (`phase_capsules.rs`, ramas `already_archived`, «fuera de pending» y `done_dir`). `fracture_pbi::resolve_todos_done_rel` sí resuelve el layout. | La deuda nombrada es solo ese handler. |
| Si hay cambio de gate, tests de los dos mundos | No hay cambio de gate. | Sin tests nuevos del handler. |

## 1. Intención

Dejar escrito el laudo «no migrar» y un puntero a Q-5. El gate local sigue exigiendo `validacion.md` con `global: APTO` y `pbi_archived: true`, y el markdown en `done/`.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Entregable documental en `docs/features/tracker-done-gate/`: el laudo cita la recomendación del informe y declara que la migración no se implementa en este PBI. |
| R-2 | Puntero de Q-5: un párrafo en ese entregable con el handler y las tres ramas cableadas. Cero diff de `phase_capsules.rs`, de hooks de Done y de `task-closure-documental.mdc`. |
| R-3 | La HU futura de migración queda nombrada como bloqueada por el E2E de `PBI-DEUDA-TRACKER-STAMP-PARIDAD` (AC-8). No se forja esa HU aquí. |

## 3. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | El entregable dice «no migrar» y apunta a `feature-pbi-archive`. |
| AC-2 | `git diff` de este PBI no toca `phase_capsules.rs` ni los hooks de pre-push. |

## 4. Fuera de alcance

- Paridad de `tracker-stamp`.
- Hacer que `feature-pbi-archive` lea `docs_layout.todos_done`.
- Reescribir `informe.md` del spike para cubrir Q-1–Q-6.
