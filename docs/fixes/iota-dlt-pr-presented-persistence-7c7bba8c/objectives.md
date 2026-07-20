---
feature_name: iota-dlt-pr-presented-persistence-7c7bba8c
created: "2026-07-20"
process: bug-fix
branch_name: fix/iota-dlt-pr-presented-persistence-7c7bba8c
persist_ref: docs/fixes/iota-dlt-pr-presented-persistence-7c7bba8c
---

# Objetivos — iota-dlt-pr-presented-persistence-7c7bba8c

## Misión

DLT iota-immutable-publisher opaco en PullRequest_Presented 7c7bba8c-4286-4302-bb31-f8928b81b132: error_trace='iota publish failed'. Capsula emite fallo en campo error; route-domain lee feedback/message y pierde causa. Payload ECST incompleto (sin pr_url). Corregir propagacion de error + persistencia/anclaje DLT y validar. Evidencia: .events/dead-letter/subscribers/7c7bba8c-4286-4302-bb31-f8928b81b132.cumulo.iota-immutable-publisher.json

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
