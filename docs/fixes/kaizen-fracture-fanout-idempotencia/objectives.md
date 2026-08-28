---
feature_name: kaizen-fracture-fanout-idempotencia
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-fracture-fanout-idempotencia
persist_ref: docs/fixes/kaizen-fracture-fanout-idempotencia
pbi_ref: docs/todos/pending/[KAIZEN] Fan-out de fractura sin idempotencia real — PBI cerrados resucitados y Mayeuta en dead-letter.md
execution_id: "57a510b9-f288-4569-8ff5-067c0c614d1a"
---

# Objetivos — kaizen-fracture-fanout-idempotencia

## Misión

Fan-out System_Fracture_Detected sin idempotencia real: materialize-fracture-pbi ignora done/, identidad fuera del genoma YAML, Mayeuta recalcula ruta por nombre y va a dead-letter. Criterios FPBI-CA1..CA13 del PBI. Parada de este ciclo: Diseño (spec.md + plan.md) y commit; sin ejecución de código.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
