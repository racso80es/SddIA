---
feature_name: fix-tool-process-entity-type-scope
created: "2026-06-10"
process: feature
scope: fix-tool-process-entity-type-gate
version_spec: "1.0.0"
related_todo: docs/todos/done/[Kaizen] fix-tool-process — entity_type tool y skill compat legacy.md
---

# Especificación — Fix: Tool Process Entity Type Scope

## 1. Naturaleza y Propósito

Resolver deuda técnica de retrocompatibilidad y alcance en el proceso `fix-tool-process`. La validación discrimina tipos de entidad: gate estricto `entity_type == "tool"`; eliminación de compatibilidad legacy con `"skill"`.

## 2. Fronteras del Dominio

| Frontera | Regla |
|----------|-------|
| Aislamiento lógico | Mutación solo en `fix_tool_process_core.py` y `test_radamanto_self_healing.py` |
| Proceso | Nota documental en `SddIA/process/fix-tool-process.md` |
| Topología Kaizen | PBI → `docs/todos/done/` |

## 3. Criterios de Aceptación (S+ Grade)

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC1 | `entity_type != "tool"` → skip auditable sin sandbox | `process_fix_tool` L87–93 |
| AC2 | `test_radamanto_self_healing` 4/4 | unittest |
| AC3 | Integridad estructural aduana | `verify-process-integrity.py` |

## 4. Fuera de alcance

- Renombrar `fix-tool-process` → `fix-entity-process`
- Cambiar suscripciones EDA ni taxonomía `Domain_Entity_Degraded`
- Normalización Radamanto `entity_type_from_id` (PBI O2 futuro)
