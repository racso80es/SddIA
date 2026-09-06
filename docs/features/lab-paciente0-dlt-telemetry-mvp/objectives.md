---
feature_name: lab-paciente0-dlt-telemetry-mvp
created: "2026-09-06"
process: feature
branch_name: feat/lab-paciente0-dlt-telemetry-mvp
persist_ref: docs/features/lab-paciente0-dlt-telemetry-mvp
pbi_ref: docs/todos/pending/PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP.md
execution_id: "17ed4fb6-e729-4dbe-9813-cf9985aa9bce"
document_id: PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
pbi_uuid: "17380fcf-0630-45d3-9813-611d80beec0d"
pbi_version: "1.2.0"
status: in-progress
---

# Objetivos — lab-paciente0-dlt-telemetry-mvp

## Misión

Cablear el snapshot `Domain_Entity_Telemetry_Captured` al anclaje DLT existente (`iota-immutable-publisher`) sin mutar Peaje ni emisores ECST. El digest debe sobrevivir al `purge_after` fractal. Instancias sin bóveda IOTA no contaminan `dead-letter`.

## Alcance (manifiesto)

- Ciclo `feature` `execution_id` `17ed4fb6-e729-4dbe-9813-cf9985aa9bce`.
- SSOT `event-domain-subscriptions.json` + paridad `event-subscriptions.json`.
- Persistencia `{eda_instance.proofs}/dlt-telemetry/{event_id}.json`.
- `skipped-config-missing` exclusivo de este `event_type`.
- Clase ECST vía `entity-manager`. Tests `execute-process` + `SDDIA_LAB_SIMULATE_IOTA`.
- Plano B (Testnet físico Paciente 0) fuera del gate CI.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal `main`.
- DA-2/DA-4: topología `objectives.md` en rama antes de mutar `SddIA/events/`.
- JSON de suscripciones y `route_*_core.rs` no son genoma DA-2.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CA-CI verde (`run_id`).
- Agnosticismo Core: cero paths de host Paciente 0.

## Criterios (PBI v1.2.0)

Plano A: CA-1…CA-6. Plano B: CA-B1…CA-B3 (lab instancia, no GitHub).
