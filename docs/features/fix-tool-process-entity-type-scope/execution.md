---
feature_name: fix-tool-process-entity-type-scope
created: "2026-06-01"
process: feature
branch_name: feat/fix-tool-process-entity-type-scope
persist_ref: docs/features/fix-tool-process-entity-type-scope
---

# Ejecución — fix-tool-process-entity-type-scope

## Tareas aplicadas

1. `SddIA/scripts/qa/fix_tool_process_core.py`: Se eliminó compatibilidad para `entity_type == "skill"`.
2. `SddIA/process/fix-tool-process.md`: Se documentó de forma explícita el gate estricto para `tool`.
3. `SddIA/scripts/qa/test_radamanto_self_healing.py`: Se actualizaron fixtures y validaciones para usar prefijo `tool:`.

## Validación ejecutada

```text
cd SddIA/scripts/qa && python -m unittest test_radamanto_self_healing
→ OK (4 tests)
```

## Notas

- **Cierre de Deuda Técnica:** Se da por cerrada la deuda técnica documentada en `docs/features/adecuar-ed-telemetry/execution.md` §Notas referente a la aceptación transitoria de `entity_type == "skill"`. Se aplicó hard-override manteniendo inmutables los registros históricos.
