---
feature_name: linear-hu-b-01-outbound
document_id: PBI-LINEAR-B-01-OUTBOUND
branch: feat/linear-hu-b-01-outbound
---
# Spec — HU-B 01

Tras `create_issue` / `update_issue_state` / `create_comment` exitosos: línea JSONL con `event_id`, `operation`, `issue_ref`, `occurred_at`, `source_process` en `.SddIA/state/tracker-outbound/{issue_ref}.jsonl`. Fallo de escritura → `warn` (fail-soft).
