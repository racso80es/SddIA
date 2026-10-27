---
feature_name: domain-authority-deny-fracture
created: "2026-10-27"
process: bug-fix
branch_name: fix/domain-authority-deny-fracture
persist_ref: docs/fixes/domain-authority-deny-fracture
pbi_ref: AUT-8
document_id: PBI-50be8903
tracker_ref: AUT-8
execution_id: "383795cc-6dee-478f-bcb1-6cbbc670f10c"
---

# Objetivos — domain-authority-deny-fracture

## Misión

DOMAIN_AUTHORITY_DENIED debe materializar System_Fracture_Detected antes del envelope de fallo.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
