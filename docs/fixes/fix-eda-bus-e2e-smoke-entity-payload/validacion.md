---
feature_name: fix-eda-bus-e2e-smoke-entity-payload
created: "2026-05-29"
process: bug-fix
branch: fix/eda-bus-e2e-smoke-entity-payload
global: APTO
pbi_archived: false
checks:
  O1-payload-routing: pass
  O2-run-eda-e2e-lab: pass
  audit-execute-action-direct: pass
  audit-entity-manager-skill-event: pass
  test_eda_bus_v3plus: pass
git_changes:
  - SddIA/scripts/qa/execute-action.py
  - docs/fixes/fix-eda-bus-e2e-smoke-entity-payload/
---

# Validación

**Veredicto global: APTO**

## O1 — Payload

`_run_emit_domain_mutation` emite `entity_type` y `entity_id` en payload `Domain_Entity_*`.

## O2 — Smoke local

`run-eda-e2e-lab.py --entity-class tool --json` → `"success": true`, `cleanup.cleaned: true`.

## O3 — CI

Pendiente de verificación en PR (job `eda-bus-e2e-smoke`).
