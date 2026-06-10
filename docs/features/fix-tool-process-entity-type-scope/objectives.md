---
feature_name: fix-tool-process-entity-type-scope
created: "2026-06-01"
process: feature
branch_name: feat/fix-tool-process-entity-type-scope
persist_ref: docs/features/fix-tool-process-entity-type-scope
---

# Objetivos — fix-tool-process-entity-type-scope

## Misión

Acotar `fix-tool-process` al gate estricto `entity_type == "tool"` y cerrar la deuda Kaizen de compatibilidad legacy con `"skill"`.

## Alcance (manifiesto)

- `fix_tool_process_core.py` — validación central
- `test_radamanto_self_healing.py` — regresión Self-Healing
- PBI Kaizen → historial completado

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
