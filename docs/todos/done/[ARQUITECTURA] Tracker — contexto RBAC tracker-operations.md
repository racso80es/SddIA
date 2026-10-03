---
document_id: PBI-ARQUITECTURA-TRACKER-OPERATIONS-CONTEXT
uuid: "12aeb72e-f7b4-4b39-ba1a-81fc19bed0a5"
title: "[ARQUITECTURA] Tracker — contexto RBAC tracker-operations"
format: markdown
version: "1.0.0"
status: done
closed: "2026-10-02"
execution_branch: feat/tracker-operations-context
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-operations-context
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
execution_mode: autonomo
especificacion_cerrada: "2026-10-02"
cola_ejecucion: docs/todos/pending/
blocked_by: []
unblocks:
  - PBI-ARQUITECTURA-LINEAR-TRACKER-ADAPTER
baseline_decisiones:
  - "D4: contexto nuevo tracker-operations; allowed_policies de Tekton y Argos"
  - "D4.1: Tekton ejecutor de registro; Cerbero en fases tool: valida context[] del proceso"
---

# Contexto RBAC `tracker-operations`

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` §4.E, D4, AC-14, AC-15.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| Editar `execution-contexts.md` / `tekton.md` / `argos.md` a mano | Genoma. Mutar vía `entity-manager`. |
| Basta con el `allowed_policies` del agente | En fases `tool:` Cerbero usa `context[]` del **proceso** (`cerbero_di_rbac.rs`). El contexto del agente rige fases `agent:`. Doble cerrojo. |
| `system-operations` ya cubre Linear | Tekton ya lo tiene; usarlo no acota la tool. Cualquier agente con esa política podría invocarla. |

## 1. Intención

Que Cerbero reconozca `tracker-operations` y que solo Tekton y Argos lo declaren. La tool y los procesos de tracker lo usarán en PBIs posteriores.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-CTX-1 | `execution-contexts.md` 1.1.0 → 1.2.0. Nueva §2.10 `tracker-operations`: leer, listar, transicionar y comentar issues. Fuera: crear/borrar issues, admin de equipos, webhooks. Vía `entity-manager`. |
| R-AGT-1 | `allowed_policies` de Tekton y Argos ganan `tracker-operations`. No se toca `system-operations`. Vía `entity-manager`. |
| R-VAL-1 | `policy_validator` sigue parseando la norma; el test de contextos exige `>= 10`. |

## 3. Plan

1. Init `feature` `tracker-operations-context` con este PBI como `pbi_ref`.
2. Bump de la norma y de los dos agentes vía `entity-manager`.
3. Evolution vinculada a los `uuid` mutados.
4. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-15 | `policy_validator` reconoce `tracker-operations` (`>= 10` contextos). | `cargo test` en `execute-process` |
| AC-14 (parcial) | Un agente sin `tracker-operations` no declara la política. Tekton y Argos sí. | Diff de genoma + test de parseo |
| AC-19 (parcial) | Entrada en `SddIA/evolution/` con los uuid de norma y agentes. | `gate-evolution --range` |

## 5. Fuera de alcance

- Forja de la tool, procesos `tracker-*`, UI Kalma2.
- Derivar `target_executor_rbac` desde el genoma del agente (mejora de motor, L17).

## 6. Dependencias

Ninguna. Desbloquea `PBI-ARQUITECTURA-LINEAR-TRACKER-ADAPTER`.
