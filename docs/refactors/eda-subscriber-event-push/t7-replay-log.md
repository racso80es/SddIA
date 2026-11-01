---
feature_name: eda-subscriber-event-push
created: "2026-10-31"
process: refactorization
tracker_ref: AUT-21
---

# T7 — pasada operativa (2026-10-31)

Ejecución de `_lab/replay-false-success-stamps.py` sobre el UUID citado en AUT-21.

| event_id | Resultado |
|----------|-----------|
| `4581b7f6-cc3b-4486-b2b2-efa9a12491cb` | **No aplicable** — ECST ausente en `events.redb` (`sin evento`). Re-enrutado pendiente si el evento reaparece en store o se republica. |

Herramientas listas: `--reset-subscriber-delivery` + runbook `t7-reanchor-runbook.md`.
