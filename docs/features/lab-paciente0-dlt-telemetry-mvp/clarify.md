---
feature_name: lab-paciente0-dlt-telemetry-mvp
created: "2026-09-06"
process: feature
purpose: Estabilización Filtro A del PBI v1.2.0 tras init lab
execution_id: "17ed4fb6-e729-4dbe-9813-cf9985aa9bce"
pbi_ref: docs/todos/pending/PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP.md
document_id: PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
pbi_uuid: "17380fcf-0630-45d3-9813-611d80beec0d"
pbi_version: "1.2.0"
---

# Clarificación — lab-paciente0-dlt-telemetry-mvp

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `17ed4fb6-e729-4dbe-9813-cf9985aa9bce`. Rama `feat/lab-paciente0-dlt-telemetry-mvp`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

Semilla: PBI v1.2.0 (purga de alucinaciones v1.0/v1.1).

## Decisiones

| ID | Laudo |
|----|-------|
| L-SSOT | Suscripción en `event-domain-subscriptions.json`. Paridad `event-subscriptions.json`. Intent sin ruido. Ingest no se mueve. |
| L-BUS | Evento en `./.events/domain/`. Router `route-domain` fractal (`purge_after=true`). No V3+ `processed/subscribers/` como SSOT del digest. |
| L-PAYLOAD | Ancla = JSON ECST completo (`invoke_iota_publisher`). No Merkle de métricas. `network` literal `testnet`. |
| L-SKIP | Solo `Domain_Entity_Telemetry_Captured` + config-missing → `skipped-config-missing` (OK fractal). PR/CRUD DLT intactos. |
| L-PROOF | Proof `{eda_instance.proofs}/dlt-telemetry/{event_id}.json` + `delivery_state.transaction_digest` pre-unlink. |
| L-FORGE | Clase `domain-entity-telemetry-captured` vía `entity-manager` `update` + `markdown_body_replacements`. UUID `54a49fa7-8d45-4376-9aa1-deeebeb301ea` inmutable. Payload schema intacto → versión Clase **1.0.0**. |
| L-CORE | JSON suscripciones y motores `route_*` / proofs no son DA-2. |
| L-PLANOS | CI cierra Plano A (simulado). Plano B (explorador Testnet) es lab de instancia. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo tras checks verdes del PR. |
| L-FREQ | Alta frecuencia Testnet aceptada en MVP. Sin throttle. |

## Fuera

Peaje Termodinámico; firma MoveVM en Rust; overlays Vía C; Gas Station; path `/home/racso/Proyectos/SddIA_AP`; fósil TypeScript en `iota-immutable-publisher.md`.
