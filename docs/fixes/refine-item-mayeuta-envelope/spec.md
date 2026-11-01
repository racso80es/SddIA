---
feature_name: refine-item-mayeuta-envelope
created: "2026-11-01"
base: main
scope: sobre de error y transcript de Mayeuta
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/refine-item-mayeuta-envelope
tracker_ref: AUT-37
execution_id: "1f6f7316-ed62-4ded-922a-6b95fbb9d44a"
---

# Especificación — sobre de Mayeuta (AUT-37)

## Corte

HEAD `28a16d61`, después de AUT-34 (`dd13986e`, `4e0ad528`):

```text
role_brief / build_prompt     → handoff refine-item (cerrado)
refine_item/run.rs            → error, si no message, si no «sin detalle» (cerrado)
write_workspace_transcript    → solo en el camino ok de run_agent_phase
append_handoff                → None si persist_ref vacío
invoke_agent_phase            → data.message → entry.message
                                body.error vacío → entry.error ausente
configured_cli_can_block      → status blocked, sin afirmar message ni error
forge-pbi Recepción           → el handler la marca executed; no hay invoke_agent_phase
```

## Decisión

1. Tras normalizar el status, si es `blocked` o `failed`, `body.error` no trae texto y `data.message` sí, `entry.error` recibe ese texto. `entry.message` no se borra. Un `blocked` sin mensaje sigue sin `error`.
2. En el camino en que `run_cli` / `run_sdk` no devuelve ok, si `resolve_persist_ref` está vacío y hay `workspace_path`, se escribe `{workspace_path}/transcript.md` con el stdout y, si no hay stdout, con el texto de error. El camino ok (incluido `Veredicto: blocked`) ya escribe ese fichero; no se cambia `append_handoff`.
3. `forge-pbi` fase Recepción no es una fase de agente. El brief de Mayeuta no aplica. No hay cambio en `forge_pbi.rs`.

## Regresión

| Caso | Esperado |
|------|----------|
| Mock `blocked` + `data.message` + sin `error` | `entry.error` y `entry.message` iguales al mensaje |
| Mock `blocked` sin mensaje | `entry.error` ausente |
| `write_workspace_transcript` con persist vacío y texto | existe `transcript.md` |
| `Veredicto: blocked`, persist vacío, CLI ok | `transcript.md` presente (test nuevo; el write ya existe) |
| CLI no ok, persist vacío, `workspace_path` | `transcript.md` presente |
| `SDDIA_AGENT_RELAY_IDE=1` | sigue en `awaiting_agents` |

## Fuera de alcance

AUT-34 (prompt, MCP, parser, fractura), `PARENT_REF_REQUIRED`, DoR y el sello de `tracker_ref`.

## Criterios de aceptación

Los del cuerpo refinado de AUT-37: propagación a `entry.error`, transcript en el fallo, nota de Recepción, evolution, `validacion.md` APTO y AUT-37 en Done.
