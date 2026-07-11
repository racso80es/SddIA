---
uuid: 8d577a50-055a-40b9-b7e2-93e2d2415796
entity_ref: PBI-EVENT-BUS-AUDIT
type: feature
version: "1.0.0"
created: "2026-07-11"
related_entities:
  - uuid: 31fce110-1622-489c-a816-112849e22adb
    kind: tool
    name: event-bus-audit
  - uuid: 8d577a50-055a-40b9-b7e2-93e2d2415796
    kind: process
    name: event-bus-audit
---

# Evolución — Proceso event-bus-audit

Proceso on-demand de auditoría empírica del bus EDA. Cápsula Rust `SddIA/tools/event-bus-audit/` escanea `./.events/` (DLT + fractal), valida ECST, detecta staleness/huérfanos, genera `audit-report.md` en workspace y emite `Kaizen_Alert_Required` cuando procede.

**persist_ref:** `docs/features/event-bus-audit/`

**Verificación:** `./sddia-run.sh --process event-bus-audit --inputs '{}'` → fase `executed`, handler `capsule-tool-event-bus-audit`, informe en workspace, Kaizen en pending.
