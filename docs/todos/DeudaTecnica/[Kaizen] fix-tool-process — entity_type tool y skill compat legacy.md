---
document_id: PBI-KAIZEN-FIX-TOOL-PROCESS-ENTITY-TYPE-SCOPE
title: "[Kaizen] fix-tool-process — acotar entity_type a tool (eliminar compat skill)"
format: markdown
version: "1.0.0"
created: "2026-05-29"
status: pendiente
priority: media
process: refactorization
related:
  - SddIA/process/fix-tool-process.md
  - SddIA/scripts/qa/fix_tool_process_core.py
  - docs/features/adecuar-ed-telemetry/
introduced_by: docs/features/adecuar-ed-telemetry/execution.md
---

# [Kaizen] fix-tool-process — acotar `entity_type` a `tool`

## 0. Mandato

Eliminar la **compatibilidad transitoria** que acepta `entity_type=skill` en `fix-tool-process`, alineando runtime con el nombre ontológico del proceso y con `spec.md` de `adecuar-ed-telemetry` (gate estricto `entity_type == "tool"`).

Iniciar como **`refactorization`** cuando telemetría/Radamanto emitan `entity_type` coherente con la ED degradada o exista proceso de reparación dedicado por clase.

## 1. Contexto (deuda introducida)

En PR #66 (`adecuar-ed-telemetry`), `fix_tool_process_core.process_fix_tool` acepta:

```python
if entity_type not in ("tool", "skill"):
    return {"ok": True, "skipped": True, ...}
```

**Motivo:** los tests y la telemetría legacy usan identificadores `skill:lab-test`, `skill:doomed`; Radamanto infiere `entity_type` desde el prefijo del `entity_id` (`entity_type_from_id`). Sin el bypass, el bucle Self-Healing E2E habría roto porque el proceso se llama `fix-tool-process` pero el piloto histórico degradaba entidades con prefijo `skill:`.

Esto contradice parcialmente la spec (§6.2: gate solo `tool`) y mantiene acoplamiento semántico nombre-proceso ↔ alcance real.

## 2. Objetivo

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **Gate estricto** | `fix-tool-process` solo procesa `Domain_Entity_Degraded` con `entity_type=tool`; resto → no-op auditable sin sandbox |
| **O2** | **Telemetría coherente** | Radamanto emite `entity_type` alineado al activo real (tool vs skill) o documenta mapping explícito |
| **O3** | **Tests actualizados** | `test_radamanto_self_healing` usa entidades `tool:*` o mocks con `entity_type=tool` |
| **O4** | **Proceso futuro (opcional)** | Si Self-Healing debe cubrir `skill`, forjar `fix-skill-process` o `fix-entity-process` genérico con filtro en suscripción — no ampliar `fix-tool` |

## 3. Alcance

- `SddIA/scripts/qa/fix_tool_process_core.py` — retirar `"skill"` del allowlist.
- `SddIA/process/fix-tool-process.md` — documentar gate estricto.
- `SddIA/scripts/qa/test_radamanto_self_healing.py` — fixtures con `tool:` o payload explícito.
- Nota en `docs/features/adecuar-ed-telemetry/execution.md` §Notas — marcar deuda cerrada.

## 4. Fuera de alcance

- Renombrar `fix-tool-process` → `fix-entity-process` (refactor mayor; PBI separado si aplica).
- Cambiar taxonomía `Domain_Entity_Degraded` ni suscripciones agnósticas ya mergeadas.

## 5. Disparador sugerido

Tras normalizar telemetría CLI para declarar `entity_type` explícito en payload (no solo inferido desde `skill:`/`tool:` en `entity_id`), o al iniciar cobertura Self-Healing multi-ED.

## 6. Referencias

- Feature origen: `docs/features/adecuar-ed-telemetry/` (PR #66)
- Handler: `SddIA/scripts/qa/fix_tool_process_core.py` L86–93
- Inferencia Radamanto: `entity_type_from_id()` en `radamanto_batch_core.py`
