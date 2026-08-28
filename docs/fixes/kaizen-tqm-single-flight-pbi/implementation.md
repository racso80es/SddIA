---
feature_name: kaizen-tqm-single-flight-pbi
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-tqm-single-flight-pbi
persist_ref: docs/fixes/kaizen-tqm-single-flight-pbi
pbi_ref: docs/todos/done/[KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera.md
document_id: PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI
items:
  - L1-lock-key-pbi
  - L2-liveness-hardening
  - L3-discard-proof-event
  - L4-detach-invariant
---

# Implementación — TQM single-flight por PBI

| ID | Artefacto | Cambio |
|----|-----------|--------|
| L1 | `handlers/task_queue_manager.rs` | `resolve_lock_identity` (`document_id`/`uuid` FM o `path:sha256`); lock `{lock_hex}.lock`; adquisición por `pbi_ref` sin depender de `correlation_id` |
| L2 | `task_queue_manager.rs` | Payload JSON `{pid,starttime,holder_correlation_id}` + `sync_all`; `lock_occupancy` fail-closed (vacío reciente = Held); Linux `starttime` en `/proc/{pid}/stat` |
| L3 | `task_queue_manager.rs` | Proof durable `.SddIA/proofs/tqm-single-flight/{lock_hex}.json`; emisión `TQM_Dispatch_Discarded` en `./.events/orchestration/` |
| L3 | `entity-manager` → `event-creator` | Clase `SddIA/events/orchestration/tqm-dispatch-discarded.md` (`be28d7c5-…`) |
| L4 | `task_queue_manager.rs` | Error si hijo `detached: true`; test `DISPATCHABLE ∩ cli_detach allowlist = ∅` |

No se unifican las copias de `normalize_rel` en otros crates (fuera de alcance).
