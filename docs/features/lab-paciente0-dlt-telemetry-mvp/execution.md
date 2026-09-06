---
feature_name: lab-paciente0-dlt-telemetry-mvp
created: "2026-09-06"
process: feature
branch_name: feat/lab-paciente0-dlt-telemetry-mvp
persist_ref: docs/features/lab-paciente0-dlt-telemetry-mvp
execution_id: "17ed4fb6-e729-4dbe-9813-cf9985aa9bce"
items_applied:
  - json-subscriptions
  - digest-proof-skip-tests
  - entity-manager-event
  - evolution-register
---

# Ejecución — lab-paciente0-dlt-telemetry-mvp

## Init

`SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 ./sddia-run.sh --process feature --inputs-file .tmp/feature-lab-paciente0-dlt-telemetry-mvp-17380fcf.json`

`execution_id` `17ed4fb6-e729-4dbe-9813-cf9985aa9bce`. workspace-init **executed**. Mayeuta simulated; Dedalo…DCC phase-barrier skipped. Relevo IDE. Commit planificación `8aaff0f`.

## Tests

```text
cd SddIA && cargo test -p execute-process --lib -- dlt_telemetry
# 8 passed

cd SddIA && cargo test -p execute-process --test dlt_telemetry
# 2 passed

cd SddIA && cargo test -p execute-process --lib -- email_triage_does_not_invoke pull_request_iota_config
# 2 passed
```

## Genoma (L3)

`./sddia-run.sh --process entity-manager --inputs-file .tmp/em-domain-entity-telemetry-captured.json`

`execution_id` `ecfa4740-a96a-4aac-aa34-f71599983574`. Sello `Domain_Entity_Updated` `8c70e6c7-ff87-4928-8ed8-06deda7d9e8e`. uuid `54a49fa7-8d45-4376-9aa1-deeebeb301ea`. hash_new `sha256:01884d5cb77da51e5b931314a7b3a3505369d53bca4b1c0aafa491c1ffcc25cd`.

## Evolution

`sddia-qa evolution-register` → `ad46c2d6-30fc-451e-8e74-5b19f4f2602e` (`EVOL_OK`, `alta`).
