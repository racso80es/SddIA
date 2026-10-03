---
document_id: PBI-ARQUITECTURA-PROJECT-TRACKER-CONTRACT
uuid: "ee3e2a61-c905-4e63-af24-7b816e597849"
title: "[ARQUITECTURA] Tracker — contrato de proyecto 1.2.0 (tracker.*)"
format: markdown
version: "1.0.0"
status: done
closed: "2026-10-02"
execution_branch: feat/tracker-operations-context
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: project-tracker-contract
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
  - PBI-ARQUITECTURA-KALMA2-BACKLOG
  - PBI-ARQUITECTURA-TRACKER-STAMP
baseline_decisiones:
  - "D2: labels hu/pbi configurables (tracker.labels.*)"
  - "D3: ausente tracker = proyecto sin tracker; no hay provider markdown"
---

# Contrato de proyecto 1.2.0 — `tracker.*`

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` §4.A, F7.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| Editar `project-config-contract.md` a mano | Genoma de códice. Vía cadena de entidad (`entity-manager` / creator de contrato). |
| `project_binding.rs` ya admite 1.2.0 | Hoy solo `1.0.0` \| `1.1.0` (`CONTRACT_VERSION`). Hay que abrir 1.2.0. |
| `LINEAR_API_TOKEN` vive en el manifiesto | Prohibido. La clave se resuelve SO > proyecto (`env_ref`) > instancia > global. El manifiesto solo declara `tracker.*`. |
| `docs_layout.todos_stories` es obligatorio | Esta HU ya no lee markdown de historias (D3). No añadir `todos_stories` en este PBI. |

## 1. Intención

Que un proyecto declare opt-in Linear en su manifiesto, y que el Core valide la forma sin interpretar negocio.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-CTR-1 | `project-config-contract` 1.1.0 → **1.2.0**. Campos opcionales: `tracker.provider` (`linear`), `tracker.team_key` (obligatorio si hay provider), `tracker.project_id`, `tracker.state_map`, `tracker.labels.hu` / `tracker.labels.pbi` (default `hu` / `pbi`). |
| R-BIND-1 | `project_binding.rs` admite `1.0.0` \| `1.1.0` \| `1.2.0`. Manifiestos 1.1.0 sin `tracker` siguen válidos. `provider` distinto de `linear` → `PROJECT_CONFIG_INVALID`. `provider: linear` sin `team_key` → `PROJECT_CONFIG_INVALID`. |
| R-BIND-2 | `state_map` y `labels` son objetos de strings. Claves de `state_map` ∈ `{backlog, in_progress, in_review, done, cancelled}`. |
| R-SEC-1 | Cero secretos en el manifiesto. `rg` no encuentra `LINEAR_` en `project.md` de laboratorio. |

## 3. Plan

1. Init `feature` `project-tracker-contract`.
2. Bump del contrato vía entidad. Admisión 1.2.0 en `project_binding`.
3. Tests de binding (válido, sin tracker, provider ilegal, team_key ausente).
4. Evolution + cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-F7-1 | Manifiesto 1.2.0 con `tracker.provider: linear` + `team_key` pasa el binding. | Test `project_binding` |
| AC-F7-2 | Manifiesto 1.1.0 sin `tracker` sigue pasando. | Regresión |
| AC-F7-3 | `provider: jira` o `linear` sin `team_key` → `PROJECT_CONFIG_INVALID`. | Test |
| AC-19 (parcial) | Evolution del contrato y del binding. | `gate-evolution --range` |

## 5. Fuera de alcance

- Cargar `LINEAR_API_TOKEN` (cápsula / runtime).
- Panel Kalma2, sello de estado, spike del gate de Done.

## 6. Dependencias

Ninguna. Desbloquea backlog Kalma2 y `tracker-stamp`.
