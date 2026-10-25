---
feature_name: ci-integrity-protoc-jobs
created: "2026-10-25"
process: bug-fix
branch: fix/ci-integrity-protoc-jobs
global: APTO
pbi_archived: false
tracker_ref: AUT-4
pr_url: https://github.com/racso80es/SddIA/pull/381
ci_run: https://github.com/racso80es/SddIA/actions/runs/40169210309
ci_head: ec38efe9db16610b6a2ca2419e85cda69b85ecf1
execution_id: "1d05f105-00ba-4ecd-b2ac-f295ff6ca047"
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  DISCOVERY: APTO
  STAGE_FAIL_CLOSED: APTO
  PROTOC_STEP: APTO
git_changes:
  - .github/workflows/sddia-index-qa.yml
  - SddIA/scripts/qa/stage-native-debug-bins.sh
  - docs/fixes/ci-integrity-protoc-jobs/objectives.md
  - docs/fixes/ci-integrity-protoc-jobs/spec.md
  - docs/fixes/ci-integrity-protoc-jobs/plan.md
  - docs/fixes/ci-integrity-protoc-jobs/implementation.md
  - docs/fixes/ci-integrity-protoc-jobs/execution.md
  - docs/fixes/ci-integrity-protoc-jobs/validacion.md
  - SddIA/evolution/9ce54472-0237-4d31-beb2-02de1a63d6eb.md
  - SddIA/evolution/Evolution_log.md
---
# Validación — AUT-4

`done_gate` del Core es `linear`. No hay PBI en `docs/todos/`. `pbi_archived` queda en false: el issue pasa a Done con el sello `PullRequest_Merged`, no con un fichero.

## Local

El descubrimiento del script incluye los 34 nombres ausentes en el run 40163380636 y los cuatro que el artefacto ya publicaba (38, sin duplicados). En esta máquina hay 14 binarios debug: el script sale 1 con `copiados=14 ausentes=24`.

`eda-bus-e2e-smoke` instala `protobuf-compiler` y `libprotobuf-dev` antes del laboratorio.

## CI

Run [40169210309](https://github.com/racso80es/SddIA/actions/runs/40169210309), head `ec38efe9`. `sddia-index-integrity` y `eda-bus-e2e-smoke` en verde. Los seis jobs que ya lo estaban siguen en verde: `native-build`, `wasi-build`, `sddia-installer-smoke`, `eda-iota-smoke-simulate`, `eda-iota-physical`, `wasi-runtime-smoke`.

El primer `delivery-close-cycle` bloqueó la aduana evolution (`EVOL_MATERIAL_UNREGISTERED`) y aun así abrió el PR. El registro `9ce54472-0237-4d31-beb2-02de1a63d6eb` cubre `SddIA/scripts/qa/stage-native-debug-bins.sh`.
