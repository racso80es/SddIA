---
document_id: PBI-FIX-FRACTURE-4f209670a96f
title: "[FIX] github-bridge-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-09-04"
status: "abierto"
priority: alta
process: bug-fix
fracture_hash: 4f209670a96f
fracture_process: github-bridge-watcher
incident_ref: "System_Fracture_Detected — 4f209670a96f"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
---

# [FIX] github-bridge-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `github-bridge-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela github-bridge-watcher lock huérfano: PID 7103 muerto. last_heartbeat=2026-09-01T14:30:15Z
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
