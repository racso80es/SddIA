---
feature_name: remove-route-domain-event-action
created: "2026-05-22"
process: refactorization
branch: feat/remove-route-domain-event-action
global: APTO
checks:
  CA1-artifact-removed: pass
  CA2-no-shim: pass
  CA3-process-alive: pass
  CA4-readme: pass
---

# Validación

| ID | Criterio | Resultado | Evidencia |
|----|----------|-----------|-----------|
| CA1 | Acción eliminada del genoma | ✅ | Sin `SddIA/actions/route-domain-event.md`; índice 8 filas |
| CA2 | Shim retirado | ✅ | Sin `route-domain-event` en `PHYSICAL_HANDLERS` |
| CA3 | Proceso operativo | ✅ | `execute-process --process route-domain-event` + tests `test_eda_bus_v3plus` |
| CA4 | README actualizado | ✅ | Sección pipeline V3+ |

## Comandos

```bash
python -m unittest SddIA/scripts/qa/test_eda_bus_v3plus -v
python SddIA/scripts/qa/execute-action.py --action route-domain-event --inputs '{}'  # debe fallar
rg "actions/route-domain-event" SddIA --glob "!**/evolution/**"
```

## Veredicto

**APTO** — Enrutamiento exclusivo vía proceso `route-domain-event`.
