---
feature_name: fix-eda-bus-e2e-smoke-entity-payload
created: "2026-05-29"
process: bug-fix
branch_name: fix/eda-bus-e2e-smoke-entity-payload
---

# Ejecución

## Smoke (mismo contrato que CI)

```powershell
cd c:\Proyectos\SddIA
$env:SDDIA_LAB_SIMULATE_IOTA = "1"
$env:SDDIA_LAB_SIMULATE_SYNC_INDEX = "1"
$env:SDDIA_LAB_ROUTE_SYNC = "1"
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
```

## Auditoría execute-*.py (2026-05-29)

| Script | Emisión dominio | Estado |
|--------|-----------------|--------|
| `execute-action.py` | `_run_emit_domain_mutation` | Corregido (única ruta real del lab) |
| `execute_process_capsules.py` | `emit_domain_mutation()` duplicado | No invocado; paridad documentada |
| `execute-process.py` / `execute_process_core.py` / `execute_process_forges.py` | Delegación/forja | OK |

Smokes adicionales: `entity-manager` skill/event, `execute-action` stdin, `test_eda_bus_v3plus` 14/14.

## Limpieza post-lab

Si `lab_teardown` no purga herramientas E2E, revisar `git status` bajo `.SddIA/tools/eda-e2e-tool-*.md`.
