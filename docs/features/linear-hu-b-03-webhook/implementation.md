---
feature_name: linear-hu-b-03-webhook
document_id: PBI-LINEAR-B-03-WEBHOOK
branch: feat/linear-hu-b-03-webhook
---
# Implementation — HU-B 03

- Norma: `SddIA/norms/execution-contexts.md` 1.4.0 §2.10.
- Runtime: `SddIA/sddia-daemon-runtime/src/linear_bridge_webhook.rs` + `source` en `linear_bridge.rs`.
- Daemon: listener opcional `SDDIA_LINEAR_WEBHOOK_PORT` / `LINEAR_WEBHOOK_SECRET` (bind local).
- Gate: `tracker_operations_gate::ac_wh_1`, `tracker_section_allows_signed_webhooks`.
- Tests: AC-13/AC-22 en `linear_bridge_webhook::tests`.
