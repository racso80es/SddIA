---
feature_name: aut-52-refine-fracture-route
created: "2026-11-02"
updated: "2026-11-02"
process: bug-fix
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-52-refine-fracture-route
pbi_ref: AUT-52
tracker_ref: AUT-52
execution_id: "f2880068-385c-4403-8c31-9d11bd4b08a1"
---
# Objetivos — aut-52-refine-fracture-route

## Misión

[FIX] refine-item — fractura sistémica al sellar AUT-25: contención de lock en `events.redb` sin fractura usable, fan-out mudo de `route-domain-event` (ruta `.redb` como JSON → `success: true` / `data: null`), envelope Linear opaco (`LINEAR_TRANSPORT` sin mensaje) y firma `F-REFINE-ITEM-REDB-LOCK` ausente en catálogo.

## Criterio de cierre (resumen)

1. Lock sellado acotado con fractura observable (`STORE_LOCK_CONTENTION` / `F-REFINE-ITEM-REDB-LOCK`).
2. Router redb reparte por `event_id` (no acusa éxito con path DB).
3. `tool_invoke_error_message` expone detalle HTTP del adaptador.
4. Firma en `fracture-signatures.json` + corpus/`unclassified` justificado.
5. AUT-52 Done en Linear; sin sembrar `docs/todos/` desde Tekton/Argos.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Materialización de artefactos del piloto solo vía MCP workspace-server.
