---





feature_name: linear-hu-b-04-markdown-apply
document_id: PBI-LINEAR-B-04-MARKDOWN-APPLY
branch: feat/linear-hu-b-04-markdown-apply
execution_id: "419510ba-da0e-4792-a878-3f257d8a5ca2"
---
# Implementation — HU-B 04

- `SddIA/process/tracker-markdown-apply.md` + `handlers/tracker_markdown_apply.rs`.
- `SddIA/core/event-domain-subscriptions.json` → `tracker-markdown-apply`.
- `PBI_Cancelled` emisor `tracker-markdown-apply` en evento domain.
- Tests: `cargo test -p execute-process --lib tracker_markdown_apply`.
