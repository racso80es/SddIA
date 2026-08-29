---
feature_name: route-domain-event-fracture-b3a715381787
created: "2026-08-29"
process: bug-fix
branch_name: fix/route-domain-event-fracture-b3a715381787
persist_ref: docs/fixes/route-domain-event-fracture-b3a715381787
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (b3a715381787).md
execution_id: "b27989e4-99d8-48a1-b05e-fb130c644f02"
---

# Objetivos — route-domain-event-fracture-b3a715381787

## Misión

Fractura b3a715381787: merkle-batch-preseal falló con iota-relay-unreachable HTTP 500 en POST /v1/publish. El relay está vivo (no connection refused); ureq envuelve no-2xx como unreachable y classify_batch_anchor_friction lo mapea a F-DLT-RELAY-SIN-SUPERVISOR. Distinto de 6a49e0ad310e (payload inválido por relay caído). Alcance: (1) desambiguar relay-down vs relay-publish-error (prefijo iota-relay-publish-error + friction F-DLT-PUBLISH-ERROR, propagar cuerpo error/feedback del 500); (2) documentar causa raíz del 500 (wallet/package/red) sin bypass de entrega. Prohibido mutar genoma tools/ a mano: iota-immutable-publisher vía entity-manager si toca contrato.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
