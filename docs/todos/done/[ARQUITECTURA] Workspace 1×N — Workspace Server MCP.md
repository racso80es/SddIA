---
document_id: PBI-ARQUITECTURA-WS-SERVER
uuid: "e78e2a29-12fb-4175-99e8-2d360de5b4dc"
title: "[ARQUITECTURA] Workspace 1×N — Workspace Server MCP (stdio)"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
feature_name: sddia-workspace-server
historia_ref: "docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
historia_document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
historia_uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
created: "2026-09-28"
author: tekton
blocked_by:
  - PBI-ARQUITECTURA-WS-PILOT-REGISTRY
  - PBI-ARQUITECTURA-FS-MANAGER-PHYSICAL
unblocks:
  - PBI-ARQUITECTURA-AGENT-RUNTIME-MCP
  - PBI-ARQUITECTURA-BX-KALMA2-E2E
baseline_decisiones:
  - "D1: MCP JSON-RPC 2.0 sobre stdio, sin adaptador externo"
  - "D2: crate SddIA/tools/sddia-workspace-server; io_mode mcp-stdio; binario nativo, no WASI"
  - "D4: el servidor carga env_ref y lo inyecta solo a cápsulas hijas; SO > proyecto > instancia > global"
  - "D5: el servidor no expone prompts MCP ni interpreta normas"
tools_contract_version_actual: "1.5.0"
---

# Workspace Server MCP

Historia madre: §4.B y §4.G, fases F1–F2 (adaptadores), D1 D2 D4 D5, AC-2 (lectura), AC-3, AC-4, AC-12, AC-14. Cierra G6. El spawn desde el runtime de agentes es otro PBI.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| `tools-contract` §5 exige «un único envelope por stdout» a secas | El texto es «un único envelope JSON (por stdout o canal equivalente)». Un servidor MCP emite N mensajes JSON-RPC. Hace falta bump que reconozca `io_mode: mcp-stdio`. |
| `antigravity-cli-executor` es precedente de tool con spawn | Es un **skill** (`skills-contract`). Analogía, no precedente de `tools-contract`. |
| Sustrato `wasm32-wasip1` (§8) | El servidor mantiene stdio de sesión y spawnea cápsulas. WASI bloquea subprocess. Delivery: **binario nativo** (`tools-contract` §3 lo admite). Desviación de §8 declarada en el `{name}.md`. |
| El servidor interpreta el códice de BX | D5: no hay `prompts` MCP. `active_norm_pack` lo inyecta el proceso. El servidor solo hace `resources/read` del fichero. |

## 1. Intención

Un proceso Rust, spawn por fase, con `--root` resuelto por Cúmulo, que expone resources de lectura y tools que delegan en `filesystem-manager`, `git-manager` y `shell-executor`. Cerbero antes de cada `tools/call`. La bóveda del proyecto no sale de sus hijos.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-IO-1 | Bump de `tools-contract` que admite `io_mode: mcp-stdio`. Cada `tools/call` produce además un envelope `capsule-json-io` en el `workspace_path` de la fase y un `Raw_Execution_Finished` (`entity` = este servidor, `correlation_id` de la ejecución). |
| R-ENT-1 | `{name}.md` `type: tool` vía `tool-creator`. `context`: `filesystem-ops`, `source-control`, `system-operations`. `io_mode: mcp-stdio`. Desviación §8 (binario nativo) escrita en el cuerpo. |
| R-LIFE-1 | Spawn por fase. Muere al cerrar la fase o por timeout de barrera. Prohibido unidad systemd por proyecto. |
| R-RES-1 | `initialize`, `resources/list`, `resources/read`: `project://tree`, `project://file/{rel}`, `project://git-status`, `project://docs/{key}`. |
| R-TOOL-1 | `fs_*` (incluye `PATCH_FILE`) → `filesystem-manager` 2.0.0. `git_*` → `git-manager` con el esquema congelado vigente. `run_check` → `shell-executor` con whitelist `mcp.allowed_executables` / `qa_gates` de `project.md`. Cero reimplementación de git o shell. |
| R-RBAC-1 | Cerbero antes de cada `tools/call`. Ejecutable fuera de whitelist → deny, `exitCode ≠ 0`, sin spawn. |
| R-SAND-1 | `resources/read` con `../` o symlink fuera de `--root` → error tipado, sin panic. Escritura que escape → propagar `PROJECT_SCOPE_ESCAPE` (emisor de fractura: el ya existente en `workspace_init`; no duplicar). |
| R-ENV-1 | Carga `env_ref` al arrancar. La inyecta como entorno efímero solo a cápsulas hijas. Precedencia **SO > proyecto > instancia > global**, salvo `VAULT_PRECEDENCE_KEYS` (`SDDIA_LAB_SIMULATE_IOTA`, `SDDIA_IOTA_TIMEOUT_SECONDS`), que sí pisa el SO. La variable de `env_ref` no aparece en el entorno del orquestador ni del proceso LLM. |

### Residual que cierra Dédalo aquí

Nombre y forma del campo `io_mode` en `tools-contract` (HU §7, residual 1). Mínimo: un campo en el `{name}.md` que el contrato reconozca.

## 3. Plan

1. Esperar a que los dos PBI de `blocked_by` estén en `done/`.
2. Init `feature` `sddia-workspace-server`.
3. Bump de contrato, entidad, crate nativo, resources, adaptadores, Cerbero, telemetría, `env_ref`.
4. Tests de escape, deny y entorno. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-2 (lectura) | `resources/read` con `../` o symlink exterior → error tipado, sin panic. | Test. |
| AC-3 | `tools/call` fuera de whitelist → deny Cerbero, sin ejecución. | Test. |
| AC-4 | Cada `tools/call` emite `Raw_Execution_Finished` con `entity` del servidor y `correlation_id`. | `./.events/telemetry`. |
| AC-12 | Clave en SO y en `env_ref` → gana el SO (salvo `VAULT_PRECEDENCE_KEYS`). Clave de `env_ref` ausente en el orquestador y en el LLM; presente en el hijo. | `shell-executor` con `env` en whitelist vs `/proc/{pid}/environ`. |
| AC-14 | `{name}.md` declara `io_mode: mcp-stdio`. Cada `tools/call` deja envelope `capsule-json-io` en el `workspace_path` de la fase. | Inspección. |

## 5. Fuera de alcance

- Montar el servidor en `cursor-agent` / `agy` (PBI de runtime).
- Selector Kalma2 y `project_slug` en el evento.
- `software_forge` y redeploy de Aplicaciones.
- MCP por red. Autenticación multiusuario.

## 6. Dependencias

- `PBI-ARQUITECTURA-WS-PILOT-REGISTRY` (contrato `env_ref` y whitelist en `project.md`).
- `PBI-ARQUITECTURA-FS-MANAGER-PHYSICAL` (cápsula 2.0.0).
