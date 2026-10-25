---
feature_name: ci-integrity-protoc-jobs
created: "2026-10-25"
process: bug-fix
branch: fix/ci-integrity-protoc-jobs
global: NO_APTO
pbi_archived: false
tracker_ref: AUT-4
execution_id: "1d05f105-00ba-4ecd-b2ac-f295ff6ca047"
checks:
  AC-1: PENDIENTE-CI
  AC-2: PENDIENTE-CI
  AC-3: PENDIENTE-CI
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

`done_gate` del Core es `linear`. AUT-4 sigue en Backlog. No hay PBI en `docs/todos/`.

## Local

El descubrimiento del script incluye los 34 nombres ausentes en el run 40163380636 y los cuatro que el artefacto ya publicaba (38, sin duplicados). En esta máquina hay 14 binarios debug: el script sale 1 con `copiados=14 ausentes=24`.

`eda-bus-e2e-smoke` instala `protobuf-compiler` y `libprotobuf-dev` antes del laboratorio.

## CI

AC-1, AC-2 y AC-3 se ven en el workflow de GitHub Actions. Sin run verde de este head, `global` no es `APTO`.

El primer `delivery-close-cycle` bloqueó la aduana evolution (`EVOL_MATERIAL_UNREGISTERED`) y aun así abrió el PR. El registro `9ce54472-0237-4d31-beb2-02de1a63d6eb` cubre `SddIA/scripts/qa/stage-native-debug-bins.sh`.
