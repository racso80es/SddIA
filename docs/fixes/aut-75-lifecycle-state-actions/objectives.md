---
feature_name: aut-75-lifecycle-state-actions
created: "2026-11-04"
process: bug-fix
branch_name: Fix_Events_Multiple_03
sighting_branch: Fix_Events_Multiple_03
persist_ref: docs/fixes/aut-75-lifecycle-state-actions
pbi_ref: AUT-75
document_id: PBI-38a1cf1a
uuid: bd2a24b3-9800-46e5-9724-303359242323
parent: AUT-20
execution_id: "e7138ac5-71fe-4407-b025-ab1eca978f35"
---

# Objetivos — aut-75-lifecycle-state-actions

## Misión

Kaizen **AUT-75** (historia **AUT-20**, `PBI-38a1cf1a`): el estado y los metadatos del artefacto documental (issue Linear o fichero) los escribe la acción de la fase del ciclo. Un evento del bus no es el mediador de ese cambio.

## Alcance (manifiesto)

- Acción de ciclo para los hitos del laudo: fin de forja, Argos, apertura de PR, resultado de CI. El refino ya sella Todo en origen.
- Estado nuevo Validated y metadatos de fecha, agente y status en el frontmatter del artefacto.
- Suscriptor `tracker-stamp` deja de ser quien mueve esos estados.
- Nueva versión de `docs/audits/ciclo-vida-software-sddia.md`.
- Rama de trabajo: `Fix_Events_Multiple_03`. Provider Linear: sin `docs/todos/`.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Parada actual: planificación L0 (`spec.md` + `plan.md`); código en estímulo Tekton posterior.
