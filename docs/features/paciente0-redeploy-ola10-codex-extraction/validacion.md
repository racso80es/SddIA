---
feature_name: paciente0-redeploy-ola10-codex-extraction
created: "2026-09-25"
process: feature
branch: feat/paciente0-redeploy-ola10-codex-extraction
branch_name: feat/paciente0-redeploy-ola10-codex-extraction
persist_ref: docs/features/paciente0-redeploy-ola10-codex-extraction
pbi_document_id: PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION
pbi_ref: docs/todos/done/[OPERATIVO] Paciente 0 SddIA_AP — redeploy ola 10 post-extracción del códice de ingeniería de software.md
execution_id: "3098fa27-fb71-48e7-8ee2-aab2bee2c468"
global: PENDIENTE-CI
pbi_archived: true
evolution_id: faa18af8-60e1-4d3f-b824-990f89cee208
audit_ref: docs/audits/paciente0-deploy-20260925T114629Z.md
checks:
  CA-PREREQ: APTO
  CA-OLA10: APTO
  CA-FILTRO-C: APTO
  CA-EDA-ROOTS: APTO
  CA-AUTHORITY: APTO
  CA-OLA9-ABSORBIDA: APTO
  CA-TORMENTOSA: APTO
  CA-LLM-REGISTRY: APTO
  CA-AUDIT: APTO
  CA-DEUDA-1.7.0: APTO
  CA-DA5: APTO
  CA-CI: PENDIENTE-CI
git_changes:
  - docs/features/paciente0-redeploy-ola10-codex-extraction/
  - docs/audits/paciente0-deploy-20260925T114629Z.md
  - docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
  - docs/todos/done/[OPERATIVO] Paciente 0 SddIA_AP — redeploy ola 10 post-extracción del códice de ingeniería de software.md
  - SddIA/scripts/build-release-bundle.sh
  - SddIA/scripts/qa/test-build-release-bundle-filtro-c.sh
  - start-sddia.sh
  - SddIA/evolution/faa18af8-60e1-4d3f-b824-990f89cee208.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — paciente0-redeploy-ola10-codex-extraction

Gates locales APTO. `global` no es APTO hasta `run_id` CI verde (features-documentation-pattern v1.2.1).
