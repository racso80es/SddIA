---
document_id: PBI-FIX-FRACTURE-28c5228720ea
title: "[FIX] event-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-14"
status: cerrado
closed: "2026-08-16"
fix_ref: docs/fixes/centinelas-fracture-ola-20260812
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — 28c5228720ea"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
---

# [FIX] event-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `event-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela event-watcher omitió 3070 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-13T06:59:11Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Cierre (ola 20260812)

Laudo **(B) deuda documental**: runtime sano al 2026-08-16 (`missed_cycles=0`). Circuito A+B+C+D (PR #168) y panic Kaizen (PR #175) ya en `main`. Archivado bajo `docs/fixes/centinelas-fracture-ola-20260812`. EV-AUD-003 segregado.
