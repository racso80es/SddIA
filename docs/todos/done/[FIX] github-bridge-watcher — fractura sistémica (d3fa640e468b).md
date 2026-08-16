---
document_id: PBI-FIX-FRACTURE-d3fa640e468b
title: "[FIX] github-bridge-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-13"
status: cerrado
closed: "2026-08-16"
fix_ref: docs/fixes/centinelas-fracture-ola-20260812
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — d3fa640e468b"
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
Centinela github-bridge-watcher omitió 790 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-12T16:19:14Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Cierre (ola 20260812)

Laudo **(B) deuda documental**: runtime sano al 2026-08-16 (`missed_cycles=0`). Circuito A+B+C+D (PR #168) y panic Kaizen (PR #175) ya en `main`. Archivado bajo `docs/fixes/centinelas-fracture-ola-20260812`. EV-AUD-003 segregado.
