---
feature_name: kaizen-runtime-binary-parity-gate
created: "2026-10-29"
process: feature
phase: Verificación
branch: feat/kaizen-runtime-binary-parity-gate
branch_name: feat/kaizen-runtime-binary-parity-gate
persist_ref: docs/features/kaizen-runtime-binary-parity-gate
pbi_ref: MAN-25
tracker_ref: MAN-25
execution_id: "b4bc0b99-532a-4faa-8e57-dcd957d39b45"
global: APTO
pbi_archived: false
checks:
  KZ-CA1: APTO
  KZ-CA2: APTO
  KZ-CA3: APTO
  KZ-CA4: APTO
  KZ-CA5: APTO
  runtime_binary_parity: APTO
  sddia-runtime-parity-lib: APTO
  kalma2-bridge-runtime-tests: APTO
  execute-process-lib: APTO
git_changes:
  - SddIA/sddia-runtime-parity/
  - SddIA/ecosystem-health/
  - SddIA/engine/execute-process/
  - SddIA/interfaces/kalma2-bridge/
  - start-sddia.sh
  - docs/features/kaizen-runtime-binary-parity-gate/
---

# Validación — MAN-25

## Evidencia automatizada

```bash
cd SddIA && cargo test -p sddia-runtime-parity --lib
cd SddIA && cargo test -p kalma2-bridge runtime
cd SddIA && cargo test -p execute-process validacion_runtime_parity --lib
cd SddIA && cargo check -p execute-process -p kalma2-bridge -p sddia-ecosystem-health
```

## KZ-CA5

`pull_request_review` / triaje técnico: si `persist_ref` menciona `kalma2-bridge`, `workspace_init` o `kalma2-wui`, exige `runtime_binary_parity` ok (misma assess que system-health; bypass `SDDIA_ALLOW_STALE_RUNTIME=1`).

## Nota operativa

Tras merge, reciclar demonios (`accept-pr` KZ-CA4) o re-ignición para alinear ELF vivo con testigo; el gate de despacho detecta drift.
