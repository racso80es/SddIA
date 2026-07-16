---
document_id: PBI-FIX-FRACTURE-cc7c92a22c17
title: "[FIX] github-bridge-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-07-16"
status: "cerrado"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — cc7c92a22c17"
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
Centinela github-bridge-watcher omitió 7 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-16T15:54:35Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

- Causa raíz no clasificada automáticamente para `github-bridge-watcher`; requiere laudo humano.

### Veredicto evolutivo

**Corrección de proceso oficial** (`process_fix`)

### Propuestas

- **Corrección de proceso oficial:** Auditar proceso `github-bridge-watcher`, acción `daemon-heartbeat-audit` y emisor `argos`.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.

## Resolución (ola 2026-07-16)

**Duplicado / satélite** — consolidado en `docs/fixes/centinelas-fracture-ola-20260716/` (PBI-CENTINELAS-FRACTURE-OLA-20260716).
Causa de spam: materialize-fracture-pbi hasheaba traza variable; idempotencia por `process_name` corregida en este fix.
