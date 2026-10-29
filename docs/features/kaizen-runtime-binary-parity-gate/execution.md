---
feature_name: kaizen-runtime-binary-parity-gate
created: "2026-10-29"
process: feature
branch_name: feat/kaizen-runtime-binary-parity-gate
persist_ref: docs/features/kaizen-runtime-binary-parity-gate
execution_id: "b4bc0b99-532a-4faa-8e57-dcd957d39b45"
---

# Ejecución — MAN-25

## Init

`SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 ./sddia-run.sh --process feature --inputs-file .tmp/feature-man25-init.json`

`execution_id` `b4bc0b99-532a-4faa-8e57-dcd957d39b45`. Rama `feat/kaizen-runtime-binary-parity-gate`.

## Verificación prevista

```bash
cd SddIA && cargo test -p sddia-runtime-parity --lib
cd SddIA && cargo test -p kalma2-bridge stale_runtime system_health
cd SddIA && cargo test -p execute-process accept_pr
```
