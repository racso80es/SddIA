---
feature_name: fix-eda-bus-e2e-smoke-entity-payload
created: "2026-05-29"
process: bug-fix
branch_name: fix/eda-bus-e2e-smoke-entity-payload
persist_ref: docs/fixes/fix-eda-bus-e2e-smoke-entity-payload
bug_summary: CI eda-bus-e2e-smoke falla — emit-domain-mutation sin entity_type/entity_id en payload ECST
related:
  - docs/features/adecuar-ed-telemetry/spec.md
  - SddIA/scripts/qa/execute-action.py
  - SddIA/scripts/qa/run-eda-e2e-lab.py
---

# Objetivos — fix eda-bus-e2e-smoke

## Misión

Restaurar `eda-bus-e2e-smoke` (y `run-eda-e2e-lab.py --entity-class tool --json`) alineando `emit-domain-mutation` en `execute-action.py` con el contrato ECST `Domain_Entity_*` post-`adecuar-ed-telemetry`.

## Síntoma

```
RuntimeError: missing required payload.entity_type; missing required payload.entity_id
```

Ruta: `entity-manager` → fase Sello universal → `action:emit-domain-mutation` → `_run_emit_domain_mutation`.

## Causa raíz

`execute_process_capsules.emit_domain_mutation` ya mapea `entity_type` / `entity_id`, pero el handler real del lab (`execute-action.py`) solo emitía `entity_class` + `entity_uuid`.

## Objetivos

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | Payload canónico | `_run_emit_domain_mutation` incluye `entity_type := entity_class`, `entity_id := entity_uuid` |
| O2 | Smoke local | `run-eda-e2e-lab.py --entity-class tool --json` → `success: true` |
| O3 | CI | Job `eda-bus-e2e-smoke` SUCCESS |

## No objetivos

- Cambiar esquema ECST ni suscripciones Radamanto/Cerbero.
- Refactor completo de `entity-manager` / forja local.
