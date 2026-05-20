---
feature_name: eda-domain-entities-splus
created: "2026-05-20"
process: feature
items_applied:
  - Fase 0 código y norma
  - Fase A forges + entity-manager lab
  - audit-entity-eda-coverage.py esqueleto operativo
---

# Ejecución — EDA Domain Entities S+

## Comandos de laboratorio

```bash
# Smoke entity-manager + tool
python SddIA/scripts/qa/execute-process.py --process entity-manager --inputs '{"entity_class":"tool","entity_name":"eda-lab-smoke-tool","lifecycle_operation":"create","semantic_seed":{"tool_name":"eda-lab-smoke-tool","execution_logic":"Smoke EDA"}}'

# Auditoría huérfanas
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json

# Backfill (Fase C — no ejecutado en este ciclo)
python SddIA/scripts/qa/audit-entity-eda-coverage.py --emit --skip-dlt --correlation-id eda-backfill-001
python SddIA/scripts/qa/audit-entity-eda-coverage.py --anchor-merkle docs/features/eda-domain-entities-splus/backfill-manifest.json
```

## Rama

`feat/eda-domain-entities-splus`
