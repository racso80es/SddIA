---
feature_name: kaizen-aduana-dlt-relay-supervisado
created: "2026-08-27"
process: feature
branch_name: feat/kaizen-aduana-dlt-relay-supervisado
persist_ref: docs/features/kaizen-aduana-dlt-relay-supervisado
document_id: PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO
uuid: "1243c58b-8e93-4897-ba3e-3efc26564673"
execution_id: "cdd000a0-75d3-4bf9-9a4b-c1d889860ed2"
items:
  - T1-forge-daemon
  - T2-supervisor-crate
  - T3-ignicion
  - T4-rescate-merkle
  - T5-causa-fractura
  - T6-cola-reanchor
  - T7-fossil-regime
---

# Implementation — kaizen-aduana-dlt-relay-supervisado

## Touchpoints

| Área | Artefacto |
|------|-----------|
| Forja daemon | `SddIA/daemons/iota-publish-relay.md` vía `daemon-creator` (`SDDIA_LAB_SKIP_CAPABILITY_DI=1`) |
| Porte forja | `run_daemon_forge` en `factory.rs` |
| Supervisor | `SddIA/daemons/iota-publish-relay/` + launchers `SddIA/daemons/iota-publish-relay.sh`, `SddIA/scripts/daemons/iota-publish-relay.sh` |
| Ignición | `start-sddia.sh` L-REQUIRED, `_wait_http` `/health`, systemd factory + `_systemd_ignite` |
| instance-creator | `SYSTEMD_FACTORY_DAEMONS` + `iota-publish-relay` |
| Cúmulo | `eda_instance.dlt_reanchor` en `cumulo.paths.json` v1.6.5 |
| Motor | `route_domain_core.rs`: causa real, fractura×1, cola, drain |
| Rescate | `dlt-backfill-rescue.py` + acta en `.SddIA/proofs/` |

## Régimen (L-PEAJE / L-FOSSIL)

- Fail-soft con cola en `.SddIA/dlt/reanchor-queue`.
- `invoke_iota_publisher` **permanece**; sin fallback unitario en error de transporte.
- Deuda: `DT-DLT-RELAY-NODE` (publisher Node); F-01 `entity_class: daemon` en `entity-manager`.

## Rescate Fase 0

- Censo: 28 eventos DL ventana 2026-08-25..27.
- Lote Merkle único; digest `45EdSejCRHkZTWNzkrECVPAqn4K1HdwbELYABdMVEFNv`.
- Acta: `merkle-acta-dlt-backfill-20260827.json`.
- `anchored_retroactively: true` en cada evento.
