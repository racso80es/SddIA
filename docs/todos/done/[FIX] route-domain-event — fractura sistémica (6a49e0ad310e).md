---

document_id: PBI-FIX-FRACTURE-6a49e0ad310e
uuid: "6a49e0ad310e-0000-4000-8000-000000000001"
title: "[FIX] route-domain-event — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-27"
updated: "2026-08-28"
status: cerrado
closed: "2026-08-28"
resolution_ref: docs/fixes/route-domain-event-fracture-6a49e0ad/
priority: alta
process: bug-fix
fracture_process: route-domain-event
fracture_hash: 6a49e0ad310e
incident_ref: "System_Fracture_Detected — 6a49e0ad310e"
resolution_ref: docs/todos/done/[KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch.md
merge_commit: 464ea3bd3718dc545446c7d1af36be2822e7c5fb
merged_pr: 210
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/features/kaizen-aduana-dlt-relay-supervisado/
  - docs/fixes/route-domain-event-fracture-6a49e0ad/
  - .cursor/rules/kintsugi-fracture-protocol.mdc


# [FIX] route-domain-event — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `route-domain-event` |
| Emisor | `execute-process` |
| Acción intentada | `merkle-batch-preseal` |

## Traza de error

```
F-DLT-RELAY-SIN-SUPERVISOR: merkle-batch-preseal failed: Campo obligatorio ausente o inválido: payload
```

La causa física: relay IOTA sin centinela supervisado → `iota-immutable-publisher` sin payload válido (relay caído).

## Cierre (Kaizen DLT #208)

| Campo | Valor |
|-------|--------|
| Remediación | Centinela `iota-publish-relay` + L-REQUIRED `/health` + rescate Merkle |
| Merge | `ecd84387db7408e46de6a153de799b5505f32b06` en `main` |
| PBI padre | `PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO` archivado |

La fractura `6a49e0ad310e` es **histórica** (pre-relay). Con relay activo y publisher alimentado, el pre-sellado no reproduce el error.

### Remediación operativa (prompt_adjustment)

Veredicto Mayeuta: touchpoint `.cursor/rules/kintsugi-fracture-protocol.mdc` (`alwaysApply: true`) — detener entrega ante colapso, escalado Kintsugi, notificación al Vértice Biológico. Fix documental: `docs/fixes/route-domain-event-fracture-6a49e0ad/`.

## Criterio de cierre

- [x] Causa raíz resuelta (relay supervisado — Kaizen DLT)
- [x] `prompt_adjustment` materializado (regla operador IA)
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/` (sin copia stale en `pending/`)

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

- Bloqueo operativo sin escalado Kintsugi previo al intento de recuperación manual.

### Veredicto evolutivo

**Ajuste de prompt o regla operador IA** (`prompt_adjustment`)

### Propuestas

- **Ajuste de prompt o regla operador IA:** Ajustar instrucción operador IA: detener, emitir `System_Fracture_Detected`, notificar al Vértice Biológico — no continuar entrega.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.
