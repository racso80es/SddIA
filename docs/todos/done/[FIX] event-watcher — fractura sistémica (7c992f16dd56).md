---

document_id: PBI-FIX-FRACTURE-7c992f16dd56
title: "[FIX] event-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-07-16"
status: "cerrado"
priority: alta
process: bug-fix
fracture_process: event-watcher
fracture_hash: 7c992f16dd56
incident_ref: "System_Fracture_Detected — 7c992f16dd56"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md


# [FIX] event-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `event-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela event-watcher omitió 13 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-16T15:55:03Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

- Causa raíz no clasificada automáticamente para `event-watcher`; requiere laudo humano.

### Veredicto evolutivo

**Corrección de proceso oficial** (`process_fix`)

### Propuestas

- **Corrección de proceso oficial:** Auditar proceso `event-watcher`, acción `daemon-heartbeat-audit` y emisor `argos`.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.

## Resolución (ola 2026-07-16)

**Duplicado / satélite** — consolidado en `docs/fixes/centinelas-fracture-ola-20260716/` (PBI-CENTINELAS-FRACTURE-OLA-20260716).
Causa de spam: materialize-fracture-pbi hasheaba traza variable; idempotencia por `process_name` corregida en este fix.
