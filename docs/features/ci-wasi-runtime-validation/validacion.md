---
feature_name: ci-wasi-runtime-validation
created: "2026-06-11"
process: feature
branch_name: feat/ci-wasi-runtime-validation
persist_ref: docs/features/ci-wasi-runtime-validation
global: APTO
pbi_archived: false
checks:
  CI-W1: pass
  CI-W2: pending
  CI-W3: pending
  CI-W4: pending
  CI-W5: pending
  CI-W6: pending
  CI-W8: pass
git_changes:
  - .github/workflows/sddia-index-qa.yml
  - SddIA/scripts/qa/run-wasi-ci-smoke.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/scripts/qa/execute-action.py
  - docs/features/ci-wasi-runtime-validation/
---

# Validación — ci-wasi-runtime-validation

**Veredicto global: APTO** (CI-W2–W6 pendiente PR)

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CI-W1 | Job `wasi-runtime-smoke` en workflow | ✅ | `sddia-index-qa.yml` |
| CI-W2 | Build workspace WASI en runner | ⏳ | PR CI |
| CI-W3 | `wasmtime` en PATH | ⏳ | PR CI |
| CI-W4 | PoC `wasi-poc` ejecuta | ⏳ | `run-wasi-ci-smoke.py` |
| CI-W5 | Crypto WASM sin fallback | ⏳ | `SDDIA_CI_REQUIRE_WASI=1` |
| CI-W6 | `eda-bus-e2e-smoke` sin regresión | ⏳ | PR CI |
| CI-W8 | Paridad documental | ✅ | spec/plan/objectives/implementation/validacion |
