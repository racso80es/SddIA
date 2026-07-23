---
document_id: PBI-FIX-FRACTURE-a69be9535f82
title: "[FIX] github-bridge-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-07-23"
status: "abierto"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — a69be9535f82"
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
Centinela github-bridge-watcher omitió 234 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-23T06:10:31Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

_Pendiente de síntesis Mayeuta (Kintsugi async)._

## Criterio de cierre

- [ ] Causa raíz resuelta
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO movido a `docs/todos/done/`
