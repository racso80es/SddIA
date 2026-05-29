---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: feature
branch_name: feat/kaizen-event-creator-event-family-explicit
---

# Ejecución

## Inicialización

Rama `feat/kaizen-event-creator-event-family-explicit` (workspace-init parcial; objetivos ampliados manualmente).

## Smoke forja (post-cambio)

```powershell
# Debe fallar sin event_family
python SddIA/scripts/qa/execute-process.py --process entity-manager --inputs '{"entity_class":"event","entity_name":"smoke-no-family","lifecycle_operation":"create","semantic_seed":{"event_name":"smoke-no-family","event_type":"Smoke_No_Family","payload_required":[],"payload_optional":[],"payload_forbidden":[]}}'

# Con familia (solo laboratorio; event_type único)
python SddIA/scripts/qa/execute-process.py --process entity-manager --inputs '{"entity_class":"event","entity_name":"smoke-explicit-family","lifecycle_operation":"create","semantic_seed":{"event_name":"smoke-explicit-family","event_family":"domain","event_type":"Smoke_Explicit_Family","payload_required":[],"payload_optional":[],"payload_forbidden":[]}}'
```

## Tests

```powershell
python -m unittest SddIA.scripts.qa.test_eda_fractal_bus -v
```
