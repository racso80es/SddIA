---
feature_name: refine-item-mayeuta-blocked
created: "2026-11-01"
purpose: decisiones de alcance AUT-34
process: bug-fix
branch_name: Fix_Events_Multiple
tracker_ref: AUT-34
execution_id: "9913ec04-8f19-4e24-920a-e689bf213c8b"
---

# Clarificación — AUT-34

## Decisiones

1. El corte es el runtime real de Mayeuta en `refine-item`. El relevo `SDDIA_AGENT_RELAY_IDE=1` que devuelve `awaiting_agents` antes de exigir `verdict.json` se queda.
2. Con `project_slug: core` el engine ata `project_root` y activa MCP. Para `refine-item` los artefactos viven en `workspace_path` del Core. El prompt de esa fase no prohíbe Write ni pide `clarify.md` / `objectives.md`.
3. El MCP de Workspace 1×N sigue siendo obligatorio cuando la fase no es el handoff de `refine-item`. `cursor-agent mcp list` no ve `.cursor/mcp.json` si el workspace es descendiente del git toplevel. El mismo fichero fuera del repo lista `sddia-workspace-server` (`not loaded (needs approval)` sin `--approve-mcps`). El spawn ya añade `--approve-mcps`.
4. El binario del servidor es el de mtime más reciente entre `target/release` y `target/debug`. Hoy el engine prefiere `release` en cuanto el fichero existe.
5. `Veredicto: **blocked**` es `blocked`. Un `executed` sin `verdict.json` en `refine-item` es `failed`, no espera. El motivo sale de `error` o de `message`. Hay fractura y transcript en el workspace.
6. Rama: `Fix_Events_Multiple`. AUT-19 y `docs/refactors/refine-item-mayeuta` quedan fuera.

## Evidencia de descubrimiento MCP

| Invocación | Resultado |
|------------|-----------|
| `cursor-agent mcp list` en el repo y con `--workspace` en `.SddIA/workspaces/.agent-runtime-neutral` | `No MCP servers configured` |
| El mismo `mcp.json` en un directorio fuera del git toplevel | `sddia-workspace-server: not loaded (needs approval)` |
