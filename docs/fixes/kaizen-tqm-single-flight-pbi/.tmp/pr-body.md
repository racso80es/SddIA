## Summary
- El guard TQM clavea por `document_id` del PBI (o hash de ruta), no por `correlation_id`, evitando cadenas bug-fix duplicadas.
- Liveness endurecido (fail-closed ante lock vacío reciente; `starttime` en Linux).
- Descarte auditable: proof durable en `.SddIA/proofs/tqm-single-flight/` + evento `TQM_Dispatch_Discarded` (clase ECST nueva).

## Test plan
- [x] `cargo test -p execute-process task_queue_manager` (13 passed)
- [ ] Smoke CA12 post-merge: dos domain events sobre mismo PBI → un solo `cursor-agent`