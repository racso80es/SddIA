---
document_id: PBI-FIX-FRACTURE-3b71cec5263b
title: "[FIX] email-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-09-06"
status: "invalidado_por_falso_positivo"
priority: alta
process: bug-fix
fracture_hash: 3b71cec5263b
fracture_process: email-watcher
incident_ref: "System_Fracture_Detected — 3b71cec5263b"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
subsumed_by: PBI-FIX-THERMAL-RESILIENCE-HEARTBEAT-MAYEUTA
fix_ref: docs/fixes/thermal-resilience-heartbeat-mayeuta
---

# [FIX] email-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `email-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela email-watcher lock huérfano: PID 13215 muerto. last_heartbeat=2026-09-06T06:12:04Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

- Entidad genómica indexada sin correlato `Domain_Entity_Created` en bus EDA.

### Veredicto evolutivo

**Refactor de herramienta / cápsula / handler lab** (`refactor_tool`)

### Propuestas

- **Refactor de herramienta / cápsula / handler lab:** Ejecutar backfill Fase C (`audit-entity-eda-coverage --emit`) o integrar sello en `entity-manager` create.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.
## Criterio de cierre

- [ ] Causa raíz resuelta
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO movido a `docs/todos/done/`
