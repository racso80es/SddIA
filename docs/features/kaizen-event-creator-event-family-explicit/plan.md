---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: feature
---

# Plan — Kaizen event_family explícito

| Hito | Entregable | Estado |
|------|------------|--------|
| H0 | Rama + objectives/clarify/spec/plan | [x] |
| H1 | `event-creator.md` v1.2.0 | [x] |
| H2 | `entity-manager.md` semilla | [x] |
| H3 | `run_event_forge` + `resolve_effective_event_family` | [x] |
| H4 | Labs + `ola-c-event-entity/execution.md` | [x] |
| H5 | Fase 1 spec/clarify/validacion | [x] |
| H6 | PBI → `docs/todos/done/` | [x] |
| H7 | `validacion.md` + PR | [x] |

## Verificación

```powershell
python -c "from pathlib import Path; import sys; sys.path.insert(0,'SddIA/scripts/qa'); from execute_process_capsules import resolve_effective_event_family; resolve_effective_event_family({'event_family':'domain'}); 
try: resolve_effective_event_family({}); raise SystemExit(1)
except ValueError: print('ok missing')"
python -m unittest SddIA.scripts.qa.test_eda_fractal_bus -v
```
