---
feature_name: argos-pr-audited
process: feature
created: 2026-05-31T08:00:00Z
---

# Plan

1. Generate standard documentation under `docs/features/argos-pr-audited/`.
2. Author `SddIA/events/domain/pull-request-audited.md`.
3. Refactor `outputs` in `SddIA/agents/argos.md`.
4. Create `.dev/test-argos-emission.sh` to generate `mock-argos-output.json` and pipeline it to `execute-action.py --action emit-domain-mutation`.
