---
document_id: PBI-FIX-FRACTURE-9967f9f38f67
title: "[FIX] github-bridge-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-06-16"
status: "cerrado"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — 9967f9f38f67"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/fixes/centinelas-heartbeat-fracture/validacion.md
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
Centinela github-bridge-watcher omitió 160 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-06-16T05:56:01Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

**Duplicado de incidente** — misma causa raíz: bloqueo síncrono de HTTP GitHub e `invoke_process_pr` sin latido intermedio.

**Resolución:** consolidada en `docs/fixes/centinelas-heartbeat-fracture/` (PR consolidado).

## Criterio de cierre

- [x] Causa raíz resuelta
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`
