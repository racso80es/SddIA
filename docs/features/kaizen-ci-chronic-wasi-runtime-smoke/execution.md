---
feature_name: kaizen-ci-chronic-wasi-runtime-smoke
created: "2026-09-25"
process: feature
items_applied:
  - evolution-rehash
  - gate-universe
  - gate-range
branch_name: feat/kaizen-ci-chronic-wasi-runtime-smoke
persist_ref: docs/features/kaizen-ci-chronic-wasi-runtime-smoke
execution_id: "915ff612-6aad-4d82-b92a-6d455d2cfe3d"
document_id: PBI-KAIZEN-CI-CHRONIC-WASI-RUNTIME-SMOKE
---

# Ejecución — kaizen-ci-chronic-wasi-runtime-smoke

## Rehash

`sddia-qa evolution-rehash --id 83d6eb73-0936-4acb-9f1e-5d519987f1ab --json`

`success: true`, `exitCode: 0`, `idempotent: false`, `hash_integrity: sha256:7a76a140e8f111bed85904c5fcbf38f6eb88ad260db3d866b35edcf4b7f06dd8`.

El detalle que escribió la cápsula no traía newline final (`join("\n")`). Se añadió un newline final: `canonical_hash` usa `lines()`, así que no altera el hash. El diff contra `main` es una sola línea.

## Gates sobre `1992513`

Universo:

```json
{"exitCode":0,"message":"universo conforme","success":true,"result":{"reason_codes":["EVOL_OK"],"findings":[]}}
```

Rango `origin/main...HEAD` con `--sync-base --require-synced-base`:

```json
{"exitCode":0,"message":"L-SELF / sin material","success":true,"result":{"reason_codes":["EVOL_OK"],"base_resolution":{"mode":"synced","fetch_outcome":"ok","ref":"origin/main"}}}
```

## CI

CA3 queda `PENDIENTE-CI` hasta el run `pull_request` del PR.
