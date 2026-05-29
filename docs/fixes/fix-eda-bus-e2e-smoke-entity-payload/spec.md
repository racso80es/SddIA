---
feature_name: fix-eda-bus-e2e-smoke-entity-payload
created: "2026-05-29"
process: bug-fix
---

# Especificación — Payload routing en emit-domain-mutation

## Cambio

En `SddIA/scripts/qa/execute-action.py`, función `_run_emit_domain_mutation`, bloque `payload` del evento:

```python
"entity_type": entity_class,
"entity_id": entity_uuid,
```

Regla (paridad `execute_process_capsules.emit_domain_mutation` y `adecuar-ed-telemetry` § Paso 4):

- `entity_type` = valor de `entity_class` del input.
- `entity_id` = `entity_uuid` del handoff (obligatorio en create/update).

## Validación

`validate_domain_mutation_event` contra `Domain_Entity_Created` exige ambos campos REQUIRED desde merge de telemetría agnóstica.

## Smoke

```powershell
$env:SDDIA_LAB_SIMULATE_IOTA = "1"
$env:SDDIA_LAB_SIMULATE_SYNC_INDEX = "1"
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
```

Salida esperada: JSON con `"success": true` (o equivalente en raíz del último execute-process).
