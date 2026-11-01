---
feature_name: refine-item-mayeuta-blocked
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/refine-item-mayeuta-blocked
pbi_ref: AUT-34
tracker_ref: AUT-34
document_id: PBI-0372d98e
execution_id: "9913ec04-8f19-4e24-920a-e689bf213c8b"
historia_ref: AUT-20
---

# Objetivos — refine-item-mayeuta-blocked

## Misión

Cerrar AUT-34: la fase `Refinamiento Mayeuta` de `refine-item` debe terminar en `refined` o `needs_clarification` con motivo explícito. El prompt, el MCP del CLI y el colapso del handler tienen que coincidir con `verdict.json` e `item.refined.md`.

## Alcance

`kalma2-agent-runtime-cursor.py` (brief, prompt, parser, transcript, workspace del CLI). Selección del binario `sddia-workspace-server` en `agent_runtime.rs`. Handler `refine_item/run.rs`: `failed` si `executed` no deja `verdict.json`; mensaje desde `error` o `message`; `System_Fracture_Detected`.

## Ley aplicada

- Git solo vía `skill:git-manager`. Sin bypass raw para cerrar el defecto.
- `SddIA/scripts/tools/` y `SddIA/engine/` no son genoma de las claves `tools` / `skills` / `actions` / `process` / `events` / `norms`.
- Rama de trabajo: `Fix_Events_Multiple`. El init etiquetó `fix/refine-item-mayeuta-blocked`; esa rama no se abre.
- Provider `linear`: cierre en el issue, sin fichero nuevo en `docs/todos/pending/` ni `docs/todos/done/`.
