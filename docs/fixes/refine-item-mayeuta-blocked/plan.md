---
feature_name: refine-item-mayeuta-blocked
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/refine-item-mayeuta-blocked
tracker_ref: AUT-34
execution_id: "9913ec04-8f19-4e24-920a-e689bf213c8b"
phases:
  - id: T1
    title: Prompt de handoff en Refinamiento Mayeuta
  - id: T2
    title: Workspace MCP fuera del git toplevel y binario por mtime
  - id: T3
    title: Parser Markdown y executed sin verdict.json
  - id: T4
    title: Mensaje, transcript y fractura
---

# Plan — AUT-34

Rama `Fix_Events_Multiple`. Un commit por tarea. Sin rama `fix/refine-item-mayeuta-blocked`.

Contraste con HEAD (2026-11-01): `role_brief` de Mayeuta sigue pidiendo `clarify.md` y `objectives.md`. `mcp list` en `.SddIA/workspaces/.agent-runtime-neutral` no ve el `mcp.json` que sí lista el servidor fuera del repo. `_VERDICT_RE` no admite `**`. `refine_item/run.rs` lee solo `entry.error`.

## T1 — Prompt

En `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py`:

`role_brief`: con proceso `refine-item` o fase `Refinamiento Mayeuta`, el texto pide escribir `verdict.json` (`verdict`: `refined` | `needs_clarification`) e `item.refined.md` bajo `workspace_path`, leyendo `item.md`. No menciona `clarify.md` ni `objectives.md`.

`build_prompt` imprime `workspace_path` en la cabecera. En esa fase no añade el bloque «Prohibido Write fuera del MCP», aunque el doc traiga `mcp_servers`.

El resto de agentes (Dedalo, Tekton, Argos) no cambia de brief.

## T2 — MCP y binario

El directorio de `--workspace` del CLI, cuando `use_mcp` es cierto, no es descendiente del git toplevel. Ahí se materializa `.cursor/mcp.json`. `run_cli` pasa `--workspace` a ese directorio.

Humo en el mismo arranque: `cursor-agent --workspace <dir> mcp list` debe contener `sddia-workspace-server`. Si responde `No MCP servers configured`, la fase queda `failed` con esa salida.

En `agent_runtime.rs`, el `command` del servidor es el binario con mtime más reciente entre `SddIA/target/release/sddia-workspace-server` y `SddIA/target/debug/sddia-workspace-server`.

## T3 — Veredicto

`parse_agent_verdict` acepta énfasis Markdown (`**`, `` ` ``) alrededor de `ok` y `blocked`. Test en `test_kalma2_runtime_timeout.py`: `Veredicto: **blocked**` → `blocked`.

En `refine_item/run.rs`, status `executed` sin `verdict.json` devuelve error `failed`, no `awaiting_agents`. Los status `simulated` y `awaiting_agents` del relevo IDE siguen igual.

## T4 — Colapso visible

En el handler, el detalle es `error`, si no `message`, si no `sin detalle`.

Si `persist_ref` va vacío y hay `workspace_path`, el runtime escribe `{workspace_path}/transcript.md` con el stdout del agente.

Ante `blocked` o `failed`, `materialize_pending_domain_event` emite `System_Fracture_Detected` con ese detalle antes de devolver el `Err`. Nota en `SddIA/evolution/` vinculada a AUT-34.

## Hecho que este plan no repite

`SDDIA_AGENT_RELAY_IDE=1` ya corta en `awaiting_agents` y el relevo reentra con el mismo `execution_id`. DoR, sellado Linear y el refactor AUT-19 no se reabren. `--approve-mcps` ya está en el argv del CLI.
