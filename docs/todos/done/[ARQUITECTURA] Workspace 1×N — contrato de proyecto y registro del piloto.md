---
document_id: PBI-ARQUITECTURA-WS-PILOT-REGISTRY
uuid: "a4a70f84-4caf-4aac-90df-af2edc206207"
title: "[ARQUITECTURA] Workspace 1×N — contrato de proyecto 1.1.0 y registro del piloto"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
feature_name: sddia-project-pilot-registry
historia_ref: "docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
historia_document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
historia_uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
created: "2026-09-28"
author: tekton
unblocks:
  - PBI-ARQUITECTURA-WS-SERVER
  - PBI-ARQUITECTURA-KALMA2-PROJECT-SLUG
  - PBI-ARQUITECTURA-BX-KALMA2-E2E
baseline_decisiones:
  - "D4 (contrato): project-config-contract 1.0.0 → 1.1.0; env_ref opcional, relativo, sin .."
  - "D3 (manifiesto): codex_slug del piloto = codex-software-engineering exacto"
  - "qa_gates y mcp.allowed_executables opcionales en 1.1.0; el servidor que los consume es otro PBI"
---

# Contrato de proyecto 1.1.0 y registro de BarcelonaXplorer

Historia madre: `HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN` §4.A, fase F0, AC-10. Cierra G5.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| `project_binding.rs` ya acepta 1.1.0 | No. `CONTRACT_VERSION` es el literal `"1.0.0"` (`project_binding.rs`). Un manifiesto 1.1.0 hoy falla con `PROJECT_CONFIG_INVALID`. Este PBI abre la admisión a `1.0.0 \| 1.1.0`. |
| Registrar el piloto crea un Workspace Registry JSON | Prohibido (HU §2.3). Solo `.SddIA/projects/{slug}.md` + `{project_root}/.SddIA/project.md`. |
| `forge-pbi` vive en `SddIA/process/` | Vive en el códice: `SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md`. Siembra en el `todos_pending` **del proyecto**, no en `docs/todos/` de la forja. |

## 1. Intención

Que un proyecto externo se dé de alta con dos markdown y que el Core acepte el contrato 1.1.0 (`env_ref`) sin romper manifiestos 1.0.0. BarcelonaXplorer queda registrado y `forge-pbi` siembra un PBI en su layout.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-CTR-1 | `project-config-contract` **1.1.0** vía cadena de entidad (prohibido editar `SddIA/library/` a mano). Campo opcional `env_ref` (relativo a `project_root`, sin `..`). Campos opcionales `qa_gates` y `mcp.allowed_executables`. Obligatorios 1.0.0 intactos. |
| R-BIND-1 | `project_binding.rs` admite `contract_version` `1.0.0` o `1.1.0`. `env_ref` con `..` o absoluto → `PROJECT_CONFIG_INVALID`. Manifiesto 1.0.0 sin `env_ref` sigue válido. |
| R-REG-1 | Índice `.SddIA/projects/barcelonaxplorer.md` (`instance.projects`) + `BarcelonaXplorer/.SddIA/project.md`. Mismo `uuid`. `codex_slug: codex-software-engineering`. `project_root` solo en el índice. |
| R-REG-2 | `delivery_mode` y `docs_layout` (`features`, `fixes`, `todos_pending`, `todos_done`) relativos, sin `..`. |
| R-SEED-1 | `execute-process --process forge-pbi` con `project_slug=barcelonaxplorer` escribe un PBI bajo el `todos_pending` de BX y emite `PBI_Forged`. |
| R-AC10 | Alta de un segundo proyecto de laboratorio = otros dos `.md`. Cero diff de genoma respecto del alta del piloto. |

## 3. Plan

1. Init `feature` `sddia-project-pilot-registry` con este PBI como `pbi_ref`.
2. Bump del contrato y de `CONTRACT_VERSION` / admisión dual.
3. Registrar el piloto (índice en la forja; manifiesto en el árbol de BX, que no es este repo: el manifiesto se entrega como artefacto de la feature o se escribe en el piloto y se cita en `validacion.md`).
4. Smoke `forge-pbi`.
5. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-10 | Segundo proyecto ficticio = dos `.md`, sin cambio de Core adicional. | Lab. |
| AC-F0-1 | Manifiesto `contract_version: "1.1.0"` con `env_ref` válido pasa el binding. | Test `project_binding`. |
| AC-F0-2 | Manifiesto 1.0.0 existente sigue pasando. | Test de regresión. |
| AC-F0-3 | `env_ref: ../.env` → `PROJECT_CONFIG_INVALID`. | Test. |
| AC-F0-4 | `forge-pbi` materializa un fichero en el `todos_pending` de BX. | Inspección + evento. |

## 5. Fuera de alcance

- Crate MCP, `filesystem-manager`, runtime, UI Kalma2, `software_forge`.
- Cargar `env_ref` en proceso (lo hace el Workspace Server).
- Saneo de la instancia Aplicaciones.

## 6. Dependencias

- Ningún PBI antecesor. Desbloquea el servidor MCP y el selector Kalma2.
