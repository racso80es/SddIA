---
feature_name: sddia-installer-v2-clean-deploy
created: "2026-09-26"
process: feature
branch_name: feat/sddia-installer-v2-clean-deploy
persist_ref: docs/features/sddia-installer-v2-clean-deploy
global: PENDIENTE-CI
pbi_archived: false
document_id: PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO
checks:
  local_smoke:
    status: APTO
    evidence: SddIA/scripts/qa/test-sddia-installer.sh
  eda_coverage:
    status: APTO
    evidence: execute-process --audit-eda-coverage --scan orphan_count=0
  ci:
    status: PENDIENTE
    run_id: null
---

# Validación — sddia-installer-v2-clean-deploy

Smoke installer y EDA locales APTO. **global** permanece `PENDIENTE-CI` hasta run_id verde post-PR.
