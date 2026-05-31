---
feature_name: argos-pr-audited
process: feature
created: 2026-05-31T08:00:00Z
---

# Objectives

1. Create a new domain event contract `pull-request-audited` with properties `audit_event_reference`, `target_entity_id`, `resolution`, and `violated_rules`.
2. Refactor `argos` agent output schema to match these outputs exactly.
3. Provide a deterministic mock artifact `mock-argos-output.json` with a test execution CLI action in `.dev/test-argos-emission.sh`.
