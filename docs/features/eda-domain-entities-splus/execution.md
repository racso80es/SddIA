---
feature_name: eda-domain-entities-splus
created: "2026-05-20"
process: feature
items_applied:
  - Fase 0–B código, norma y validación
  - Fase C backfill 40 entidades + acta Merkle
---

# Ejecución — EDA Domain Entities S+

## Comandos de laboratorio

```bash
# Smoke entity-manager + tool
python SddIA/scripts/qa/execute-process.py --process entity-manager --inputs '{"entity_class":"tool","entity_name":"eda-lab-smoke-tool","lifecycle_operation":"create","semantic_seed":{"tool_name":"eda-lab-smoke-tool","execution_logic":"Smoke EDA"}}'

# Auditoría huérfanas
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json

# Fase C — Backfill (2026-05-20)
python SddIA/scripts/qa/audit-entity-eda-coverage.py --emit --skip-dlt --correlation-id eda-backfill-fase-c-20260520 --json
# PowerShell: $env:SDDIA_LAB_SIMULATE_IOTA="1"
python SddIA/scripts/qa/audit-entity-eda-coverage.py --anchor-merkle docs/features/eda-domain-entities-splus/backfill-manifest.json

# Fase B — E2E watcher
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json

# Fase B — Aduana Argos (debe block con huérfanas)
python SddIA/scripts/qa/execute-process.py --process delivery-close-cycle --inputs-file docs/features/eda-domain-entities-splus/_smoke-close-cycle.json
```

## Rama

`feat/eda-domain-entities-splus`
