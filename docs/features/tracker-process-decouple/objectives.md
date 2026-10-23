---
feature_name: tracker-process-decouple
created: "2026-10-23"
process: refactorization
branch_name: refactor/tracker-process-decouple
persist_ref: docs/features/tracker-process-decouple
execution_id: "90e151ce-3830-4fa2-985a-784a93c90330"
---

# Objetivos — tracker-process-decouple

## Misión

MAN-20. Desacoplar el ciclo documental: tracker:lifecycle sin nombrar producto ni path de backlog en los procesos.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
