---
feature_name: core-software-forge-profile
created: "2026-10-29"
process: refactorization
phase: Verificación
branch: refactor/core-software-forge-profile
global: APTO
tracker_ref: AUT-33
document_id: PBI-497d7023
execution_id: "c214dcd6-4e70-4c23-bfc7-8931780582df"
pbi_archived: false
checks:
  domain-authority-test: APTO
  refine-item-gate-smoke: APTO
  CI-GITHUB: PENDIENTE-CI
git_changes:
  - .SddIA/active-domain-profile.json
  - SddIA/engine/execute-process/src/engine/domain_authority.rs
  - docs/refactors/core-software-forge-profile/
---

# Validación — AUT-33 (local)

## Evidencia

- `cargo test -p execute-process --lib domain_authority::tests::core_project_slug_allowed_with_instance_software_forge_profile` → **ok**
- `./sddia-run.sh --process refine-item --inputs '{"item_ref":"AUT-33","project_slug":"core"}'` → pasa Domain Authority; falla solo en Mayeuta `blocked` (runtime agente, no regresión del PBI)

## Alcance entregado

Perfil de instancia Core-self + test AUT-33. Propuestas 2–3 del kaizen documentadas como fuera de corte en `objectives.md`.

## Pendiente cierre

Commit, PR y `delivery-close-cycle` con `tracker_ref: AUT-33` (Linear `done_gate`).
