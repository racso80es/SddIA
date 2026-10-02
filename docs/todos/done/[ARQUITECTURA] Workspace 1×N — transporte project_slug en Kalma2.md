---
document_id: PBI-ARQUITECTURA-KALMA2-PROJECT-SLUG
uuid: "9177d689-2957-45e0-ba50-8772ec39750d"
title: "[ARQUITECTURA] Workspace 1×N — transporte de project_slug en Kalma2"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
feature_name: kalma2-project-slug-transport
historia_ref: "docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
historia_document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
historia_uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
created: "2026-09-28"
author: tekton
blocked_by:
  - PBI-ARQUITECTURA-WS-PILOT-REGISTRY
unblocks:
  - PBI-ARQUITECTURA-BX-KALMA2-E2E
baseline_decisiones:
  - "El frontend no negocia MCP ni resuelve project_root (Dogma O3)"
  - "Sin project_slug el comportamiento Core-self queda idéntico (AC-7)"
---

# Transporte de project_slug de punta a punta

Historia madre: §4.E, fase F4, G4, AC-1 (UI), AC-6, AC-7. No espera al servidor MCP: solo mueve el slug.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| `kalma2-interact` ya acepta el proyecto | Input único: `prompt` (`kalma2-interact` v1.1.1). Allowlist: `bug-fix`, `feature`, `refactorization`, `task-queue-manager`. |
| El botón es «Forjar» | El botón es `Forjar Proceso` (`#forge`), junto a `Chat`, `Hablar con Tormentosa` y `Sincronizar Genoma`. |
| El navegador elige el servidor MCP | El selector es inerte. Pide slugs a una ruta de lectura (`/api/status` o ruta nueva de índice). No recibe `project_root`. |

## 1. Intención

Elegir un proyecto registrado en Kalma2 y que ese `project_slug` llegue a `bug-fix` / `feature` / `refactorization`. Sin selección, el ciclo actual sobre el Core no cambia.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-UI-1 | Selector inerte en `interfaces/kalma2/`. Opciones = slugs del índice `instance.projects`. Cero `project_root` en HTML, JS o respuestas que alimentan la UI. |
| R-API-1 | `POST /api/execute` (y el camino de Forjar Proceso) acepta `project_slug` además de `process` y `prompt`. `kalma2-bridge` lo reenvía. Ausente → no se inventa slug. |
| R-PROC-1 | `kalma2-interact` gana input opcional `project_slug`. `Kalma2_Process_Requested.payload.project_slug` lo lleva. `task-queue-manager` lo copia a `inputs.project_slug` del proceso allowlisted. |
| R-VER-1 | Bumps de proceso/evento por `process-creator` / `event-creator`. Prohibido editar `SddIA/process/` o `SddIA/events/` a mano. |
| R-REG-1 | Sin `project_slug`, la suite Core-self (AC-CORE-SELF de ABSTRACT-04) permanece verde. |

## 3. Plan

1. Esperar el registro del piloto (hace falta un índice real para el selector).
2. Init `feature` `kalma2-project-slug-transport`.
3. Ruta de lectura, UI, puente, proceso, evento, TQM.
4. Verificar el flujo en el cliente Kalma2 (selector → Forjar Proceso → evento). Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-6 | Selector en `barcelonaxplorer` + Forjar Proceso → `payload.project_slug = barcelonaxplorer` y TQM lo propaga a `bug-fix`. | Evento en bus. |
| AC-7 | Sin `project_slug`, comportamiento actual idéntico. | Suite existente. |
| AC-1 (UI) | Cero coincidencias del `project_root` literal de BX en `interfaces/kalma2/` y en el JSON que pinta el selector. | `rg`. |

## 5. Fuera de alcance

- Arrancar el Workspace Server o cambiar el `cwd` del agente.
- Resolver autoridad `software_forge` (otro PBI). El slug se transporta aunque la autoridad luego deniegue.
- Autenticación multiusuario.

## 6. Dependencias

- `PBI-ARQUITECTURA-WS-PILOT-REGISTRY`.
- Paralelo al PBI del servidor y al del runtime.
