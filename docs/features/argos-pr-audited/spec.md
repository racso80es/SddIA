---
feature_name: argos-pr-audited
process: feature
created: 2026-05-31T08:00:00Z
---

# Specification

- Target File 1: `SddIA/events/domain/pull-request-audited.md`
  - Required payload schema defining 4 fields strictly.
- Target File 2: `SddIA/agents/argos.md`
  - Needs its `outputs` refactored to use the newly defined 4 fields instead of the previous 3 fields.
- Target Script: `.dev/test-argos-emission.sh`
  - Automatically emit the PR audited domain mutation action based on a mock argos JSON payload `mock-argos-output.json`.
