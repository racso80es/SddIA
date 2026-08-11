---
fix_ref: "docs/features/heartbeat-circuit-regimen-20260811"
document_id: PBI-FIX-FRACTURE-b8e3c0e97eb4
title: "[FIX] event-sweeper — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-11"
status: cerrado
closed: "2026-08-11"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — b8e3c0e97eb4"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
---

# [FIX] event-sweeper — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `event-sweeper` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela event-sweeper omitió 27 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-11T07:32:02Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

- Causa raíz no clasificada automáticamente para `event-sweeper`; requiere laudo humano.

### Veredicto evolutivo

**Corrección de proceso oficial** (`process_fix`)

### Propuestas

- **Corrección de proceso oficial:** Auditar proceso `event-sweeper`, acción `daemon-heartbeat-audit` y emisor `argos`.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.

## Resolución

Causa raíz: inanición del circuito Daemon_Heartbeat (fan-out), no muerte del centinela.

Remediado por PBI-REFACTOR-HEARTBEAT-CIRCUIT-20260811 (`docs/features/heartbeat-circuit-regimen-20260811`): vías A+B+C+D.
Archivado en ola de cierre del refactor (2026-08-11).
