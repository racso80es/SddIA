---
feature_name: iota-dlt-pr-presented-persistence-7c7bba8c
created: "2026-07-20"
process: bug-fix
branch: fix/iota-dlt-pr-presented-persistence-7c7bba8c
global: APTO
pbi_archived: true
pbi_ref: "docs/todos/done/[FIX] iota-immutable-publisher — DLT opaco PullRequest_Presented (7c7bba8c).md"
checks:
  - id: CA1
    result: APTO
    evidence: "capsule_error_trace_prefers_error_field + envelope release con feedback=iota-publish-unavailable"
  - id: CA2
    result: APTO
    evidence: "emit_presented_rejects_missing_pr_url"
  - id: CA3
    result: APTO
    evidence: "físico relay → digest 44 chars no lab-sim (JAL5EpNTbat4T4rkfPVC99Aa…); lab-sim OK aparte"
  - id: CA4
    result: APTO
    evidence: "cargo test capsule_error_trace / emit_presented / blocking_tests OK; release build OK"
  - id: CA5
    result: APTO
    evidence: "PBI en docs/todos/done/; pbi_archived true"
git_changes:
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/sddia-io/src/lib.rs
  - .SddIA/services/iota-publish-relay/
  - docs/fixes/iota-dlt-pr-presented-persistence-7c7bba8c/
  - docs/todos/done/[FIX] iota-immutable-publisher — DLT opaco PullRequest_Presented (7c7bba8c).md
---

# Validación — iota-dlt-pr-presented-persistence-7c7bba8c

**global: APTO**

Causa raíz: route-domain ignoraba `error` del envelope → DLT opaco `iota publish failed`. Gate `pr_url` evita ECST incompleto.

**Config física:**
- `SDDIA_LAB_MOCK_IOTA_URL=` (vacío)
- `IOTA_PUBLISH_RELAY_URL=http://127.0.0.1:8787/v1/publish`
- Probe `tmp/probe-iota-relay.sh`: `success:true`, digest real (no `lab-sim-`)
