---
document_id: PBI-SPIKE-LINEAR-DONE-GATE
uuid: "fb35cfa0-a7a8-485c-a87e-2a6bc934e9d3"
title: "[SPIKE] Tracker — migración del gate de Done a Linear"
format: markdown
version: "1.0.0"
status: done
closed: "2026-10-02"
execution_branch: feat/tracker-operations-context
priority: media
type: spike
process: feature
dispatch: true
feature_name: spike-linear-done-gate
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
execution_mode: autonomo
especificacion_cerrada: "2026-10-02"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-STAMP
unblocks: []
baseline_decisiones:
  - "D1: Linear es SSOT objetivo; este spike investiga el gate; no cambia producción"
---

# Spike — gate de Done en Linear

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` §7.1, D1.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| El gate lo aplican Argos o `delivery-close-cycle` | Lo aplica `feature-pbi-archive` en `phase_capsules.rs` (`validacion.md` APTO + `pbi_archived: true` + move a `docs/todos/done/`, ruta **cableada**). |
| Quitar el gate bloquea PRs | Los dejaría pasar sin evidencia. El bloqueo aparece si el PBI vive solo en Linear y el handler sigue exigiendo markdown. |
| Este PBI cambia el gate | Spike documental. Cero mutación del handler en producción. |

## 1. Intención

Informe + propuesta de HU para desacoplar Done de Git, sin romper proyectos sin tracker.

## 2. Requisitos (preguntas del spike)

| ID | Pregunta |
|----|----------|
| Q-1 | ¿Cómo `pull-request-review` (Argos) consulta `fetch_issue` antes del veredicto, con `tracker-operations`? |
| Q-2 | ¿Qué sustituye a `feature-pbi-archive` / `pbi_archived: true` si el PBI vive solo en Linear? |
| Q-3 | ¿`validacion.md` sigue en Git o se ancla en Linear? |
| Q-4 | Impacto en `task-closure-documental.mdc`, `features-documentation-pattern`, `feature`, `bug-fix`, `refactorization`, aduana pre-push. |
| Q-5 | Plan para la deuda: `phase_capsules.rs` ignora `docs_layout.todos_done`. |
| Q-6 | Convivencia con proyectos sin tracker (D3). |

Entregable: `{persist_ref}/informe.md` + borrador de HU hija. Sin parche de gate.

## 3. Plan

1. Tras `tracker-stamp` en main.
2. Init `feature` `spike-linear-done-gate`.
3. Informe contrastado con código. Propuesta de HU.
4. Cierre documental (el spike se archiva como el resto).

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-SPK-1 | Informe responde Q-1–Q-6 con rutas y números de línea reales. | Revisión |
| AC-SPK-2 | Cero diff de `phase_capsules.rs` / hooks de Done. | `git diff` |
| AC-SPK-3 | Borrador de HU hija en el `persist_ref` o en `docs/todos/historias/`. | Existencia |

## 5. Fuera de alcance

- Implementar el nuevo gate. Mutar Cerbero. Webhooks.

## 6. Dependencias

Bloqueado por `PBI-ARQUITECTURA-TRACKER-STAMP` (hace falta espejo activo para medir el gap).
