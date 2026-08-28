---

document_id: PBI-FIX-FRACTURE-131fa2c33271
title: "[FIX] telegram-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-07-24"
status: cerrado
fix_ref: docs/fixes/centinelas-fracture-ola-20260723
priority: alta
process: bug-fix
fracture_process: telegram-watcher
fracture_hash: 131fa2c33271
incident_ref: "System_Fracture_Detected — 131fa2c33271"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md


# [FIX] telegram-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `telegram-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela telegram-watcher omitió 18 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-24T05:44:49Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Cierre (ola 20260723)

Laudo **(B) deuda documental**: runtime sano al 2026-08-11 (`missed_cycles=0`); mitigaciones ya en `main` (olas 0716/0722 + PR #155). Archivado bajo `docs/fixes/centinelas-fracture-ola-20260723`.
