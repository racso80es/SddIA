---
feature_name: aiua-antigravity-cli-vector
created: "2026-09-09"
process: feature
branch_name: feat/aiua-antigravity-cli-vector
persist_ref: docs/features/aiua-antigravity-cli-vector
execution_id: "e35155cd-fdfa-4612-92a4-121e12b4c3f9"
document_id: PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR
items_applied:
  - forge-process-em
  - handler-antigravity
  - bridge-sanitize-timeout
  - starter-kit-env
  - tests-lab-mock
---

# Ejecución — aiua-antigravity-cli-vector

## Init

`execution_id` `e35155cd-fdfa-4612-92a4-121e12b4c3f9`. Relé IDE (`SDDIA_AGENT_RELAY_IDE=1`). Commit planificación `aae408d`.

## L1 genoma

| Entidad | EM execution_id | Sello Domain_Entity_Updated | hash_new | UUID |
|---------|-----------------|------------------------------|----------|------|
| aiua-stimulus-processing 1.1.0 phases | `5c96c2e6-5dea-4e20-b7af-bb63417069fb` | `caeadda9-a163-4240-b2cd-79dc044535e8` | `sha256:c7671b27…` | `6c595785-…` |
| cuerpo replacements | `0d7cd115-cf75-4702-ac4b-ebce5639d873` | idempotente (hash fases) | `sha256:c7671b27…` | inmutable |

## L2 handler

`infer_antigravity_cli` + `invoke_capsule_json`. Effort whitelist. Telemetría `result.usage`. Prefijo lab-mock `lab-mock-agy:`.

## L3 bridge

Sanitize auth/timeout CLI. `resolve_client_timeout_secs(client, gemini, agy)`. `app.js` intacto.

## Tests

```text
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib -- aiua_stimulus
# 7 passed

cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test --bin kalma2-bridge -- sanitize_maps_agy -- client_timeout
# 2 passed (suite completa se corre en CI)
```

`sddia-qa evolution-register` → `1dc4055c-b0c8-40ff-a30d-d257152fb8df` (`EVOL_OK`, `alta`).

## Fuera

Crates HTTP/CLI. Mayeuta. `--print`. Kitchen router. Anatomía motora.
