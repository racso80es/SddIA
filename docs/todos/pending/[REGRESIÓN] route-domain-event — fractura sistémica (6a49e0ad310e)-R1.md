---
document_id: PBI-FIX-FRACTURE-6a49e0ad310e-R1
title: "[REGRESIÓN] route-domain-event — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-28"
status: "abierto"
priority: alta
process: bug-fix
fracture_hash: 6a49e0ad310e
fracture_process: route-domain-event
incident_ref: "System_Fracture_Detected — 6a49e0ad310e"
regression_of: PBI-FIX-FRACTURE-6a49e0ad310e
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
---

# [REGRESIÓN] route-domain-event — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `route-domain-event` |
| Emisor | `execute-process` |
| Acción intentada | `merkle-batch-preseal` |

## Traza de error

```
F-DLT-RELAY-SIN-SUPERVISOR: merkle-batch-preseal failed: Campo obligatorio ausente o inválido: payload
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

_Pendiente de síntesis Mayeuta (Kintsugi async)._

## Criterio de cierre

- [ ] Causa raíz resuelta
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO movido a `docs/todos/done/`
