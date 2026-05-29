---
feature_name: inmunidad-caos-fase4
created: "2026-05-29"
process: feature
branch_name: feat/inmunidad-caos-fase4
---

# Ejecución — Fase 4

## Comandos de verificación

```powershell
cd SddIA/scripts/qa
python -m unittest test_chaos_immunity_eda.py test_execute_suite.py test_chaos_audit_processes.py -v
cd ../../..
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan
```

## Smoke E2E (lab)

1. `python SddIA/scripts/qa/execute-action.py --action emit-suite-execution-requested --inputs "{\"suite_id\":\"core-full-stress\"}"`
2. `SDDIA_LAB_ROUTE_SYNC=1` + `route-domain` sobre `target_path` devuelto, **o** watcher `--once`.
3. Verificar `System_Immunity_Certified` en `.events/domain/` tras éxito de campaña.

Fixture: `_smoke-suite-execution-eda-immunity.json`.

## Resultado (2026-05-29)

| Suite | Tests | Resultado |
|-------|-------|-----------|
| `test_chaos_immunity_eda.py` | 6 | ✅ |
| `test_execute_suite.py` | 5 | ✅ regresión |
| `test_chaos_audit_processes.py` | 5 | ✅ regresión |
