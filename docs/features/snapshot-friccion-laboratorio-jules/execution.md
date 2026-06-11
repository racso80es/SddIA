---
feature_name: snapshot-friccion-laboratorio-jules
process: feature
created: "2026-06-11"
items_applied:
  - workspace-init
  - git-failsoft
  - da-4-norm
  - entity-manager-intent-transpiler
  - pbi-archive
---

# Ejecución — snapshot-friccion-laboratorio-jules

## Comandos

```bash
# Inicialización feature
python3 SddIA/scripts/qa/execute-process.py --process feature --inputs @docs/features/snapshot-friccion-laboratorio-jules/_init.json

# Forja skill
python3 SddIA/scripts/qa/execute-process.py --process entity-manager --inputs '{"entity_class":"skill","entity_name":"intent-transpiler",...}'

# Integridad
python3 scripts/qa/verify-process-integrity.py
```

## Smokes

| ID | Resultado |
|----|-----------|
| Offline marker detection | ✅ `_is_offline_git_failure` con mensaje Jules |
| `git-manager fetch` (online) | ✅ exit 0 |
| `verify-process-integrity.py` | ✅ OK |
| `entity-manager` → `Domain_Entity_Created` | ✅ event `ba6607db-91a9-4e58-b0fa-84dee5a082a6` |
