---

feature_name: linear-update-state-graphql-types
created: "2026-11-06"
process: bug-fix
branch_name: fix/linear-update-state-graphql-types
persist_ref: docs/fixes/linear-update-state-graphql-types
document_id: PBI-cd8405c2
uuid: cd8405c2-2394-4d3e-b846-ab2ffc5f38cc
execution_id: "65f1bd41-ca2d-4139-83b3-edc774251e56"
---
# Objetivos — linear-update-state-graphql-types

## Misión

Alinear las mutaciones `issueUpdate` de `linear-tracker-adapter` con el esquema GraphQL de Linear: `id: String!`, `stateId: String`. Restaurar el paso Backlog → Todo en el sellado de origen de `refine-item`.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio). Rama `fix/linear-update-state-graphql-types`.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Genoma `SddIA/tools/` solo en fase Ejecución del `bug-fix` (no a mano fuera de proceso).
