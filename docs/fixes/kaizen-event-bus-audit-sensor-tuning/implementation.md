---
feature_name: kaizen-event-bus-audit-sensor-tuning
created: "2026-09-05"
process: bug-fix
branch_name: fix/kaizen-event-bus-audit-sensor-tuning
persist_ref: docs/fixes/kaizen-event-bus-audit-sensor-tuning
execution_id: "f9830175-0405-42fd-9e0c-e6de1c26201d"
items:
  - S1-non-ecst-sink
  - S2-needs-kaizen-actionable
  - S3-template-polimorfica
  - S4-clase-emisores
  - tests
---

# Implementation — kaizen-event-bus-audit-sensor-tuning

| Archivo | Cambio |
|---------|--------|
| `SddIA/tools/event-bus-audit/src/main.rs` | `non_ecst_sink`; `needs_kaizen = circuit_alert \|\| actionable_stale`; implicated filtrados; tests EBA-CA1–CA3 |
| `SddIA/tools/event-bus-audit.md` | v1.2.0 semántica disparo |
| `SddIA/process/event-bus-audit.md` | v1.0.2; Kaizen no ligado a DL histórico |
| `SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs` | plantilla por `alert_kind`; detector de TODO abierto DIA o bus |
| `SddIA/actions/materialize-kaizen-alert-doc.md` | v1.1.0 |
| `SddIA/events/domain/kaizen-alert-required.md` | v1.1.0; emisor `event-bus-audit` |
| índices `tools/` `process/` `actions/` `events/domain/` | SemVer alineado |
