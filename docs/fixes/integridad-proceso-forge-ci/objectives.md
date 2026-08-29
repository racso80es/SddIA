---
feature_name: integridad-proceso-forge-ci
created: "2026-08-29"
process: bug-fix
branch_name: fix/integridad-proceso-forge-ci
persist_ref: docs/fixes/integridad-proceso-forge-ci
execution_id: "1dd48b02-251c-433a-85f8-bcfd7e93336e"
---

# Objetivos — integridad-proceso-forge-ci

## Misión

PBI-FIX-INTEGRIDAD-PROCESO-FORGE-CI uuid d6387831-0e57-4bee-b402-a49f782e6837. F1 parse_frontmatter ciego y F2 hash forge divergente saldados en 76be459; restan tests CA1/CA2. F3 job CI verify-tools-index nombre engañoso. F4 DCC sin aduana local verify-process-integrity+verify-tools-index antes del push (DA-6). CA5 opcional workspace_template ---. Detener tras Diseño (spec.md + plan.md) y commit.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
