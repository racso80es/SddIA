---
feature_name: refine-item-mayeuta-blocked
created: "2026-11-01"
base: main
scope: handoff Mayeuta de refine-item
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/refine-item-mayeuta-blocked
tracker_ref: AUT-34
execution_id: "9913ec04-8f19-4e24-920a-e689bf213c8b"
---

# Especificación — handoff Mayeuta de refine-item

## Corte

HEAD, fase `Refinamiento Mayeuta`:

```text
role_brief(mayeuta) → clarify.md + objectives.md bajo persist_ref
build_prompt        → no imprime workspace_path
persist_ref vacío   → append_handoff no escribe
project_root ligado → cwd .agent-runtime-neutral dentro del repo
                      + prompt «prohibido Write fuera del MCP»
mcp list            → No MCP servers configured
_VERDICT_RE         → no casa Veredicto: **blocked**
                      → status executed
refine_item/run.rs  → error ausente ⇒ «sin detalle»
                      executed sin verdict.json ⇒ awaiting_agents
                      blocked/failed ⇒ sin System_Fracture_Detected
```

El payload de `invoke_agent_phase` ya lleva `workspace_path`. El prompt no lo usa. `release/sddia-workspace-server` gana a `debug` si el fichero existe (`agent_runtime.rs`).

## Decisión

1. Si `process_name` es `refine-item` y la fase es `Refinamiento Mayeuta`, el brief nombra `workspace_path`, la entrada `{workspace}/item.md` y la salida `verdict.json` + `item.refined.md`. No pide cascada documental de feature. No activa el bloqueo «solo MCP».
2. Cuando el MCP de fase siga activo, el `--workspace` del CLI es un directorio que no cuelga del git toplevel del Core, con el `.cursor/mcp.json` de la fase. `mcp list` sobre ese workspace nombra `sddia-workspace-server`.
3. Comando del servidor: el binario con mtime mayor entre `SddIA/target/release/sddia-workspace-server` y `SddIA/target/debug/sddia-workspace-server`. Si solo uno existe, ese.
4. `parse_agent_verdict` ignora énfasis Markdown entre los dos puntos y el token. `**blocked**` y `` `blocked` `` cuentan. Sin token, no se inventa `executed` para esta fase: el handler trata `executed` sin `verdict.json` como `failed`.
5. Texto de fallo: `entry.error` si trae cuerpo; si no, `entry.message`; si no, `sin detalle`. Transcript en `{workspace}/transcript.md` cuando no hay `persist_ref`. `System_Fracture_Detected` con ese texto en el payload.

## Regresión

| Caso | Esperado |
|------|----------|
| `Veredicto: **blocked**` | `blocked` |
| `refine-item` + `executed` + sin `verdict.json` | `failed`, no `awaiting_agents` |
| `blocked` con `message` y sin `error` | el acuse cita `message` |
| `mcp list` del workspace de fase | contiene `sddia-workspace-server` |
| `SDDIA_AGENT_RELAY_IDE=1` | sigue en `awaiting_agents` hasta el relevo |

## Fuera de alcance

AUT-19, DoR, `create_issue` de HU, DCC, y reescribir el contrato de Workspace 1×N para fases que ya tienen `persist_ref`.

## Criterios de aceptación

Los del cuerpo sellado de AUT-34: prompt de handoff, MCP visible fuera del git toplevel, binario por mtime, parser Markdown, `failed` sin veredicto, mensaje y fractura, `validacion.md` APTO, AUT-34 en Done.
