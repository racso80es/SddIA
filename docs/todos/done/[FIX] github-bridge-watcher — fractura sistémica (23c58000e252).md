---

fix_ref: "docs/features/heartbeat-circuit-regimen-20260811"
document_id: PBI-FIX-FRACTURE-23c58000e252
title: "[FIX] github-bridge-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-11"
status: cerrado
closed: "2026-08-11"
priority: alta
process: bug-fix
fracture_process: github-bridge-watcher
fracture_hash: 23c58000e252
incident_ref: "System_Fracture_Detected — 23c58000e252"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md


# [FIX] github-bridge-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `github-bridge-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela github-bridge-watcher omitió 13 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-11T07:32:02Z
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

## Resolución

Causa raíz: inanición del circuito Daemon_Heartbeat (fan-out), no muerte del centinela.

Remediado por PBI-REFACTOR-HEARTBEAT-CIRCUIT-20260811 (`docs/features/heartbeat-circuit-regimen-20260811`): vías A+B+C+D.
Archivado en ola de cierre del refactor (2026-08-11).
