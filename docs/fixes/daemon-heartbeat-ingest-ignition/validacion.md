---
feature_name: daemon-heartbeat-ingest-ignition
created: "2026-07-23"
process: bug-fix
branch: fix/daemon-heartbeat-ingest-ignition
global: APTO
pbi_archived: true
document_id: PBI-FIX-FRACTURE-d22645cea40c
related_document_ids:
  - PBI-FIX-FRACTURE-bb5d18128823
  - PBI-FIX-FRACTURE-da29db92ed52
pbi_ref: docs/todos/done/[FIX] event-sweeper — fractura sistémica (d22645cea40c).md
execution_id: 21d465b1-940f-45e2-879d-9cb8f773e230
checks:
  - id: CA1
    result: APTO
    evidence: "_ingest_telemetry_heartbeats → last_heartbeat_at 2026-07-23T05:54:32Z missed_cycles=0 (obligatorios)"
  - id: CA2
    result: APTO
    evidence: "start-sddia.sh IGNITION_OK@6s; Ecosistema S+ Grade; Kalma HTTP 200"
  - id: CA3
    result: APTO
    evidence: "event-bus-audit phase executed capsule-tool-event-bus-audit (21 anomalies estructurales reportadas)"
  - id: CA4
    result: APTO
    evidence: "di.binding.schema.json pattern ^(skill|action|tool):"
  - id: CA5
    result: APTO
    evidence: "3 PBI FIX fractura en docs/todos/done/; este validacion pbi_archived true"
git_changes:
  - start-sddia.sh
  - SddIA/library/norms/capability-contracts/di.binding.schema.json
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - docs/fixes/daemon-heartbeat-ingest-ignition/
  - docs/todos/done/[FIX] event-sweeper — fractura sistémica (d22645cea40c).md
  - docs/todos/done/[FIX] github-bridge-watcher — fractura sistémica (bb5d18128823).md
  - docs/todos/done/[FIX] telegram-watcher — fractura sistémica (da29db92ed52).md
---

# Validación — daemon-heartbeat-ingest-ignition

**global: APTO**

Causa raíz: ingest térmico ausente en gate de ignición + residual H9 (`tool:` en schema / validación envelope). Centinelas keepalive no tocados.
