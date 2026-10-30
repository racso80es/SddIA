---
feature_name: bridge-presented-tracker-ref
created: "2026-10-30"
process: bug-fix
branch_name: fix/bridge-presented-tracker-ref
persist_ref: docs/fixes/bridge-presented-tracker-ref
pbi_ref: AUT-25
tracker_ref: AUT-25
execution_id: "ceb7ecb1-5e05-4aef-b6f9-c58ed2b6540e"
---

# Objetivos — bridge-presented-tracker-ref

## Misión

El PullRequest_Presented de github-bridge-watcher no lleva tracker_ref leído del objectives.md del commit cabeza y precarga delivery_state, así que tracker-stamp no mueve la issue a in_review.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
