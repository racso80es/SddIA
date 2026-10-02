---
document_id: PBI-ARQUITECTURA-AGENT-RUNTIME-MCP
uuid: "b8b69287-6508-46d6-86c5-e63dac9f4867"
title: "[ARQUITECTURA] Workspace 1×N — runtime de agentes con proyecto y MCP"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
feature_name: kalma2-agent-runtime-project-mcp
historia_ref: "docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
historia_document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
historia_uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
created: "2026-09-28"
author: tekton
blocked_by:
  - PBI-ARQUITECTURA-WS-SERVER
unblocks:
  - PBI-ARQUITECTURA-BX-KALMA2-E2E
baseline_decisiones:
  - "D1: el cliente MCP es el runtime de agentes, no kalma2-bridge ni el navegador"
  - "Backend sin MCP queda excluido para fases sobre proyecto; error MCP_BACKEND_UNSUPPORTED; sin fallback --add-dir"
  - "mcp_servers no transporta valores de secretos"
---

# Runtime de agentes anclado al proyecto

Historia madre: §4.D, fase F3, G2, AC-1 (payload), AC-5, AC-15. El binario del servidor ya existe cuando este PBI arranca.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| `AGENT_PHASE` ya lleva `project_root` | No. El payload fija `repo_root`, `workspace_path`, `persist_ref`, `inputs`. `build_agent_command` hace `current_dir(repo)`. `project_root` no entra al payload (`agent_runtime.rs`). |
| `cursor-agent` y `agy` ya negocian MCP dentro de SddIA | No hay referencia MCP en `kalma2-agent-runtime-cursor.py` ni en `antigravity-cli-executor`. La capacidad, si existe, es del binario externo. Este PBI la **mide** con smoke `initialize`. |
| Clave de smoke `SDDIA_AGENT_RUNTIME_COMMAND` | AC-15 de la historia cita `SDDIA_AGENT_RUNTIME_CLI`. En `.dev/.env.example` conviven las dos: `SDDIA_AGENT_RUNTIME_COMMAND` es el proceso que spawnea `agent_runtime.rs`; `SDDIA_AGENT_RUNTIME_CLI` es el binario interno (`cursor-agent --print`). El smoke golpea el binario de `SDDIA_AGENT_RUNTIME_CLI` (el que hablaría MCP). |
| El puente HTTP abre la sesión MCP | El frontend es inerte (HU §2.3). El cliente MCP es este runtime. |

## 1. Intención

Una fase Tekton sobre `project_slug` recibe `project_root` y el descriptor del Workspace Server, arranca ese servidor como único MCP y no tiene el Core como `cwd`. Si el backend no negocia MCP, la fase aborta con error tipado.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-PAY-1 | Con `project_slug` resuelto, `AGENT_PHASE` incluye `project_root` y `mcp_servers`. Sin `project_slug`, el payload y el `cwd` actuales no cambian (AC-7, la suite de Core-self la cierra el PBI de transporte). |
| R-DESC-1 | Cada entrada de `mcp_servers` es `{name, command, args, env_keys}`. `env_keys` son nombres, nunca valores. `command`/`args` salen de la resolución de cápsula, no de una ruta de BX escrita en el agente. |
| R-CWD-1 | El hijo del runtime, en fase sobre proyecto, no tiene `cwd` = raíz del Core. `--add-dir` hacia `project_root` ausente en `antigravity-cli-executor` para ese caso. |
| R-SMOKE-1 | Smoke `initialize` contra el backend de `SDDIA_AGENT_RUNTIME_CLI`. Éxito: `capabilities` con `tools` y `resources`. Fallo: la fase Tekton sobre proyecto termina `MCP_BACKEND_UNSUPPORTED`. Prohibido el fallback `--add-dir`. |
| R-ONE-1 | El único servidor MCP de esa fase es `sddia-workspace-server`. |
| R-AC1-1 | El payload hacia el agente y el frontend no contienen el literal `project_root` de BarcelonaXplorer. La ruta vive en `.SddIA/projects/{slug}.md` y en el `--root` del servidor. |

### Residuales que cierra Dédalo aquí

- Forma definitiva del descriptor `mcp_servers` (HU §7, residual 2), sobre el mínimo de R-DESC-1.
- `--add-dir` en proyectos: se elimina (residual 3). No queda como lectura redundante.

## 3. Plan

1. Esperar `PBI-ARQUITECTURA-WS-SERVER` en `done/`.
2. Init `feature` `kalma2-agent-runtime-project-mcp`.
3. Extender `agent_runtime.rs` y los runtimes `kalma2-agent-runtime-cursor` y `antigravity-cli-executor` (forja de skill si el cuerpo cambia).
4. Smoke por backend configurado. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-5 | `AGENT_PHASE` sobre `project_slug` registrado contiene `project_root` y `mcp_servers`. El hijo no tiene `cwd` = Core. | Trazas. |
| AC-15 | Smoke `initialize` devuelve `capabilities.tools` y `capabilities.resources`. Si falla, la fase aborta `MCP_BACKEND_UNSUPPORTED` y no usa `--add-dir`. | Smoke F3. |
| AC-1 (runtime) | Cero literales de `project_root` de BX en el payload del agente. | `rg` sobre trazas de payload. |

## 5. Fuera de alcance

- Implementar el servidor MCP.
- Selector de la UI y el evento `Kalma2_Process_Requested`.
- Sustituir el backend LLM.
- Redeploy de Aplicaciones.

## 6. Dependencias

- `PBI-ARQUITECTURA-WS-SERVER`.
