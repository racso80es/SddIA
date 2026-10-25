---

feature_name: pr-merged-witness-store
created: "2026-10-24"
process: bug-fix
branch_name: fix/pr-merged-witness-store
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/pr-merged-witness-store
tracker_ref: MAN-23
document_id: PBI-FIX-PR-MERGED-WITNESS-STORE
execution_id: "f0d1f9df-997e-4739-b698-2771dc57867a"
---
# Objetivos — pr-merged-witness-store

## Misión

MAN-23. PullRequest_Merged 862d0270-9796-4306-8965-af811c31eba4 (PR 374) quedó sin fila en el store: el alta escribió el JSON y require_event respondió sin evento. El enrutado ya inserta la fila antes del fan-out.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
