---
feature_name: tracker-lifecycle-provider
created: "2026-10-22"
process: feature
items_applied:
  - provides
  - outbound-resolved
  - recorded-description
branch_name: feat/tracker-lifecycle-provider
persist_ref: docs/features/tracker-lifecycle-provider
execution_id: "7a0b47aa-3afa-424e-afc9-a9cb3c67a038"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
---

# Ejecución — tracker-lifecycle-provider

`CARGO_TARGET_DIR=target cargo test -p execute-process --lib tracker_outbound -- --test-threads=1`

Siete tests, todos `ok`. Incluyen `ac2_update_issue_description_writes_jsonl`, `records_only_the_resolved_provider` y `provides_matches_binding_row`.

El `match` de operaciones del crate no cambió: `fetch_issue`, `list_issues`, `update_issue_state`, `create_issue`, `create_comment`, `update_issue_description`.

## Sello

`entity-manager` `hash_refresh_only`. `handoff_hash_signature_new` `sha256:455541dc257756b2ec330b27d84e87c6f92c7754549729efa25450e7890eb0eb`. `idempotent: true` sobre `036e080a-3fe9-44b1-8155-9f59873c960e`. Cobertura `correlation_timestamp` `2026-10-22T18:01:33.992Z`.
