---
feature_name: lab-paciente0-dlt-telemetry-mvp
created: "2026-09-06"
process: feature
items:
  - json-subscriptions
  - digest-proof-skip-tests
  - entity-manager-event
  - evolution-register
document_id: PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
execution_id: "17ed4fb6-e729-4dbe-9813-cf9985aa9bce"
---

# Implementación — lab-paciente0-dlt-telemetry-mvp

| Item | Path | Nota |
| :--- | :--- | :--- |
| CA-3 | `SddIA/core/event-domain-subscriptions.json` + `event-subscriptions.json` | Segundo suscriptor Cúmulo/`iota-immutable-publisher`. Ingest intacto. |
| CA-1, CA-4, CA-5 | `dlt_telemetry_anchor.rs` + `route_domain_core.rs` + `route_fractal_core.rs` | Proof `{eda_instance.proofs}/dlt-telemetry/{event_id}.json`. `skipped-config-missing` solo este `event_type`. PR DLT sigue `failed` sin bóveda. |
| CA-2 | `entity-manager` `ecfa4740-a96a-4aac-aa34-f71599983574` | uuid `54a49fa7-8d45-4376-9aa1-deeebeb301ea` inmutable. hash `sha256:01884d5cb77da51e5b931314a7b3a3505369d53bca4b1c0aafa491c1ffcc25cd`. Versión 1.0.0. Sello `8c70e6c7-ff87-4928-8ed8-06deda7d9e8e`. |
| Evolution | `ad46c2d6-30fc-451e-8e74-5b19f4f2602e` | `EVOL_OK` alta. |
