---
feature_name: sddia-installer-v3-smoke-lab-ac4-8
branch_name: fix/sddia-installer-v3-smoke-lab-ac4-8
persist_ref: docs/features/sddia-installer-v3-smoke-lab-ac4-8
pbi_ref: docs/todos/pending/[KAIZEN] Installer v3 — smoke lab AC-4 a AC-8.md
document_id: PBI-KAIZEN-INSTALLER-V3-SMOKE-LAB-AC4-8
uuid: "b93ca305-01e5-44ba-ab59-d4a70188d2e9"
---

# Plan

1. `SddIA/scripts/qa/test-sddia-installer-lab.sh` — matriz AC-4..AC-8; invocado al final de `test-sddia-installer.sh`.
2. Motor: fallo `build_bundle` con `error.step` / `child_exit`; `SDDIA_INSTALLER_BUNDLE_PROFILE` y `SDDIA_INSTALLER_LAB_SKIP_ENABLE` solo para lab/CI.
3. `installer_io.py`: en `fail` con `--step`, marcar paso `failed` y coherencia `steps[]`.
4. Evolution + cierre documental tras CI verde.
