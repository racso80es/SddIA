---








feature_name: linear-hu-b-04-markdown-apply
document_id: PBI-LINEAR-B-04-MARKDOWN-APPLY
branch: feat/linear-hu-b-04-markdown-apply
execution_id: "d7d86b90-a53c-4ff1-980a-2233b8d2fa3f"
---
# Implementation — HU-B 04

- `SddIA/process/tracker-markdown-apply.md` + `handlers/tracker_markdown_apply.rs`.
- `SddIA/core/event-domain-subscriptions.json` → `tracker-markdown-apply`.
- `PBI_Cancelled` emisor `tracker-markdown-apply` en evento domain.
- Tests: `cargo test -p execute-process --lib tracker_markdown_apply`.
