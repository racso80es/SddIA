---
feature_name: refine-item-mayeuta-envelope
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/refine-item-mayeuta-envelope
pbi_ref: AUT-37
tracker_ref: AUT-37
document_id: PBI-79e0810b
execution_id: "1f6f7316-ed62-4ded-922a-6b95fbb9d44a"
historia_ref: AUT-19
---

# Objetivos — refine-item-mayeuta-envelope

## Misión

Cerrar el residuo de AUT-37 en HEAD `28a16d61`. El cuerpo del 2026-10-29 ya no describe este corte: AUT-34 cerró el prompt, el fallback `error → message`, `transcript.md` en el camino ok y la fractura. Sigue abierto el contrato que el criterio pide y el código no cumple.

## Alcance

- `agent_runtime.rs`: con `blocked` o `failed` y `body.error` vacío, copiar `data.message` a `entry.error`. Test de mock.
- `kalma2-agent-runtime-cursor.py`: el camino de fallo del CLI escribe `{workspace_path}/transcript.md` si `persist_ref` va vacío. Test del veredicto `blocked` con persist vacío.
- Spec: `forge-pbi` Recepción no invoca Mayeuta.
- Nota en `SddIA/evolution/` vinculada a AUT-37.

## Fuera

- Prompt, MCP, parser Markdown y fractura de AUT-34.
- `PARENT_REF_REQUIRED` en `forge-pbi`.
- Mover AUT-34 a Done.

## Ley aplicada

- Git vía `skill:git-manager` en la entrega. Esta sesión no hace checkout: `SDDIA_LAB_SKIP_GIT=1`. HEAD es `Fix_Events_Multiple`.
- El motor nombra `fix/refine-item-mayeuta-envelope`. Esa rama no se abre.
- Provider `linear`: cierre en el issue, sin fichero nuevo en `docs/todos/pending/` ni `docs/todos/done/`.
