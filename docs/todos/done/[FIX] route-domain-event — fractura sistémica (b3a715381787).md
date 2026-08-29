---
document_id: PBI-FIX-FRACTURE-b3a715381787
uuid: "b3a71538-1787-4000-8000-000000000001"
title: "[FIX] route-domain-event — fractura sistémica"
format: markdown
version: "1.2.0"
created: "2026-08-28"
updated: "2026-08-29"
status: cerrado
closed: "2026-08-29"
priority: alta
process: bug-fix
fracture_hash: b3a715381787
fracture_process: route-domain-event
friction_id: F-DLT-PUBLISH-ERROR
incident_ref: "System_Fracture_Detected — b3a715381787"
resolution_ref: docs/fixes/route-domain-event-fracture-b3a715381787/
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/tools/iota-immutable-publisher.md
  - SddIA/daemons/iota-publish-relay.md
  - docs/fixes/route-domain-event-fracture-b3a715381787/validacion.md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (6a49e0ad310e).md
---

# [FIX] route-domain-event — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `route-domain-event` |
| Emisor | `execute-process` |
| Acción intentada | `merkle-batch-preseal` |
| Cápsula física | `iota-immutable-publisher` → `publish_via_relay` |
| Endpoint | `POST http://127.0.0.1:8787/v1/publish` |
| Clasificación original | `F-DLT-RELAY-SIN-SUPERVISOR` (imprecisa) |
| Clasificación corregida | `F-DLT-PUBLISH-ERROR` (HTTP 500 con relay vivo) |

## Traza de error

```
merkle-batch-preseal failed: iota-relay-unreachable: http://127.0.0.1:8787/v1/publish: status code 500
```

## Cierre (taxonomía DLT — rama `fix/route-domain-event-fracture-b3a715381787`)

| Campo | Valor |
|-------|--------|
| Remediación código | Prefijos `iota-relay-publish-error` + fricción `F-DLT-PUBLISH-ERROR` |
| Commits | `36e318a` (spec/plan), `e5b445a` (implementación) |
| Validación | `docs/fixes/route-domain-event-fracture-b3a715381787/validacion.md` — APTO (alcance taxonomía) |
| Deuda operativa | Publish E2E verde + causa física exacta del 500 (relay inactivo en validación) |

## Criterio de cierre

- [x] Kaizen `F-DLT-PUBLISH-ERROR` desambiguada de `F-DLT-RELAY-SIN-SUPERVISOR`
- [x] Argos APTO en `validacion.md` del fix (alcance código)
- [x] Este TODO movido a `docs/todos/done/`
- [ ] Causa raíz del HTTP 500 confirmada en runtime (diferido — operación)
- [ ] Publish real reproducible en verde (diferido — relay off en validación)
- [x] Cola `.SddIA/dlt/reanchor-queue/` sin lotes huérfanos

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

- Fallo de **publish** en relay vivo (HTTP 500), no ausencia de supervisor. La telemetría de fricción colapsaba dos modos de fallo distintos bajo un mismo `friction_id`.

### Veredicto evolutivo

**Kaizen de clasificación** entregado; **infra DLT** diferida a operación con relay activo.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.
