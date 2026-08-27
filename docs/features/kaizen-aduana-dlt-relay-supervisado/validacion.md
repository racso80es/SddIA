---
feature_name: kaizen-aduana-dlt-relay-supervisado
created: "2026-08-27"
updated: "2026-08-27"
process: feature
branch: feat/kaizen-aduana-dlt-relay-supervisado
branch_name: feat/kaizen-aduana-dlt-relay-supervisado
persist_ref: docs/features/kaizen-aduana-dlt-relay-supervisado
pbi_ref: docs/todos/done/[KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch.md
document_id: PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO
uuid: "1243c58b-8e93-4897-ba3e-3efc26564673"
execution_id: "cdd000a0-75d3-4bf9-9a4b-c1d889860ed2"
global: APTO
pbi_archived: true
checks:
  DLT-CA1: APTO
  DLT-CA2: APTO
  DLT-CA3: APTO
  DLT-CA4: APTO
  DLT-CA5: APTO
  DLT-CA6: APTO
  DLT-CA7: APTO
  DLT-CA8: APTO
  DLT-CA9: APTO
  DLT-CA10: APTO
  UNIT-BATCH-ANCHOR: APTO
git_changes:
  - SddIA/core/cumulo.paths.json
  - SddIA/daemons/iota-publish-relay.md
  - SddIA/daemons/iota-publish-relay.sh
  - SddIA/daemons/iota-publish-relay/
  - SddIA/daemons/index.md
  - SddIA/scripts/daemons/iota-publish-relay.sh
  - SddIA/engine/execute-process/src/forges/factory.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs
  - start-sddia.sh
  - SddIA/Cargo.lock
  - SddIA/evolution/1243c58b-8e93-4897-ba3e-3efc26564673.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/kaizen-aduana-dlt-relay-supervisado/
  - docs/todos/done/[KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch.md
---

# Validación — kaizen-aduana-dlt-relay-supervisado

**Veredicto global: APTO** — Aduana DLT supervisada, causa real en batch, rescate Merkle 28 eventos, cola re-anclaje. PBI archivado en `docs/todos/done/`.

## Criterios PBI (DLT-CA*)

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| DLT-CA1 | Daemon forjado + launcher + lock/heartbeat | ✅ | `iota-publish-relay.md` uuid `78e94d53-…`; launchers; índice; binario debug |
| DLT-CA2 | Sin `ACTIVO` sin `/health` | ✅ | `start-sddia.sh` `_wait_http` en `_start_daemon` y `_systemd_ignite` |
| DLT-CA3 | Causa real en dead-letter | ✅ | `batch_anchor_error_trace` + tests; prohibido literal único |
| DLT-CA4 | Fractura×1 por lote | ✅ | `emit_dlt_batch_fracture` → `eda_bus.pending` |
| DLT-CA5 | Reinicio supervisor | ✅ | `iota-publish-relay/src/main.rs` restart loop + kill on health fail |
| DLT-CA6 | Acta + lote único | ✅ | `.SddIA/proofs/merkle-acta-dlt-backfill-20260827.json` (28 UUID) |
| DLT-CA7 | Sin `batched-digest` | ✅ | digest on-chain `45EdSejCRHkZTWNzkrECVPAqn4K1HdwbELYABdMVEFNv` |
| DLT-CA8 | `anchored_retroactively: true` | ✅ | rescate `dlt-backfill-rescue.py` / `fase0-rescue-merkle.sh` |
| DLT-CA9 | Sin reinyección `pending/` | ✅ | rescate mueve DL subscriber → `processed/subscribers/` |
| DLT-CA10 | Cola re-anclaje | ✅ | `eda_instance.dlt_reanchor` + `try_drain_dlt_reanchor_queue` |

## Tests

```text
cargo test -p execute-process batch_anchor -- —nocapture  → 2 passed
```

## Rescate (instancia)

| Campo | Valor |
|-------|--------|
| Ventana | 2026-08-25 … 2026-08-27 |
| Censo | 28 |
| Merkle root | `d14432fd1ce17e5b0e1895a7ffb08c5cb9b1c7f14d2d317adeb7dc6f7e758353` |
| Digest | `45EdSejCRHkZTWNzkrECVPAqn4K1HdwbELYABdMVEFNv` |

## Régimen

Fail-soft con cola persistente; `invoke_iota_publisher` conservado (L-FOSSIL).
