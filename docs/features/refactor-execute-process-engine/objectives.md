---
feature_name: refactor-execute-process-engine
created: "2026-05-20"
process: feature
branch_name: feat/refactor-execute-process-engine
persist_ref: docs/features/refactor-execute-process-engine
pbi_ref: PBI-005
---

# Objetivos — refactor-execute-process-engine

## Misión

Refactorizacion de execute-process.py para convertirlo en un interprete dinamico y ciego basado en contratos MD

## Alcance (manifiesto)

Ver `docs/todos/PBI-005-Hito2-TODO.md` — Asalto 1: motor de acciones, anatomía de capas Skills/Tools y desacoplamiento del watcher.

## Ley aplicada

- Proceso `feature` v1.2.0; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill agrupadora → Tools atómicas.
