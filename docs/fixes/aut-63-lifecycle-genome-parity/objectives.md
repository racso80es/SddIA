---
feature_name: aut-63-lifecycle-genome-parity
created: "2026-11-02"
process: bug-fix
branch_name: Fix_Events_Multiple_02
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/aut-63-lifecycle-genome-parity
pbi_ref: AUT-63
execution_id: "a6a8c74c-89df-46a7-ab0d-74d3ddb18814"
---

# Objetivos — aut-63-lifecycle-genome-parity

## Misión

AUT-63 — asimetrías genoma/código del ciclo de vida

Base: auditoría docs/audits/ciclo-vida-software-sddia.md §9 (A1–A4). Padre AUT-20. process: bug-fix. PBI-b0d4af60.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
