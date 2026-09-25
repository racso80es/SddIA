---
document_id: PBI-FIX-FRACTURE-89b7c8b105ec
title: "[FIX] route-domain-event — fractura sistémica"
format: markdown
version: "1.1.1"
created: "2026-09-09"
updated: "2026-09-25"
status: "cerrado"
priority: alta
process: bug-fix
fracture_hash: 89b7c8b105ec
fracture_process: route-domain-event
friction_id: F-DLT-PUBLISH-ERROR
incident_ref: "System_Fracture_Detected — 89b7c8b105ec"
refined: true
suggested_branch: fix/object-lock-exit3-89b7-014259
persist_ref_suggested: docs/fixes/object-lock-exit3-89b7-014259
source_audit: "2026-09-25 Filtro A sobre stub v1.0.0. Traza literal del PBI. JSON del evento ausente en .events/. Sin RPC del digest 7R6vWf14…. Fuente actual: is_dlt_publish_error_trace cubre iota-relay-publish-error; dlt_transient_gas_version_trace exige 'is not available for consumption' y 'current version:'; esta traza no cumple esa firma. Cola serial publish-queue.mjs ya existe (60db1db67e49). No se afirma que la cola fallara."
review_notes: "v1.0.0 stub Cúmulo + sección Mayeuta prompt_adjustment. v1.1.0: ese veredicto es fósil (catch-all failed, retirado en 60db1db67e49). Re-enrich con la fuente actual caería en el cubo DLT opaco (config-missing / Move / inputs permanentes), también inexacto: la traza es reserva de objeto, transitoria, hermana de la colisión de gas. No comparte causa de runtime con 0142599491fb."
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - .SddIA/services/iota-publish-relay/publish-queue.mjs
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (60db1db67e49).md
  - docs/todos/done/[FIX] kalma2-bridge — fractura sistémica (0142599491fb).md
related_pbis:
  - id: PBI-FIX-FRACTURE-60db1db67e49
    rol: "Ancestro de carrera de objeto propio. Predicado de gas y cola serial intactos. No reabrir."
  - id: PBI-FIX-FRACTURE-41717b4bb229
    rol: "Predicado de red transitoria intacto. No reabrir."
  - id: PBI-FIX-FRACTURE-0142599491fb
    rol: "Mismo ciclo de entrega. Clase compartida (sobre-escalado a Kintsugi). Causa distinta: exit 3 de prótesis. No es IOTA."
deferred_pbis:
  - id: PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION
    path: "docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md"
    rol: "Fuera de alcance. L-ENRICH-KINTSUGI-DETERMINISTA intacto."
gates_this_wave:
  - DLT-LOCK-CA1
  - DLT-LOCK-CA2
  - MAYEUTA-LOCK-CA3
---

# [FIX] route-domain-event — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `route-domain-event` |
| Emisor | `execute-process` |
| Acción intentada | `merkle-batch-preseal` |

## Traza de error

```
merkle-batch-preseal failed: iota-relay-publish-error: status=500 Failed to sign transaction by a quorum of validators because one or more of its objects is reserved for another transaction. Other transactions locking these objects:
- 7R6vWf14dsfmfGtQKJXAcjagG5G5phFMYwTKtvUCKfTX (stake 35.14)
```

El prefijo `iota-relay-publish-error` es el cubo `F-DLT-PUBLISH-ERROR` de la fuente actual (`classify_batch_anchor_friction`). El digest y el stake están en la traza; no hay consulta RPC en este refinamiento.

## Filtro A — inexactitudes del stub v1.0.0

| Afirmación v1.0.0 | Veredicto | Hecho |
|-------------------|-----------|--------|
| Causa = «Bloqueo operativo sin escalado Kintsugi» / `prompt_adjustment` | **Fósil.** | Ese párrafo es el catch-all (`timeout`/`block`/`abort`/`colaps`, antes también `failed`). La traza no contiene `block`. Contiene `failed`. El ELF que enriqueció el stub aún trataba `failed` como prompt de operador. `60db1db67e49` retiró ese token. |
| Re-enrich hoy diría lo mismo | **Falso.** | `is_dlt_publish_error_trace` es verdadero. El catch-all exige `!dlt_publish`. El cubo DLT opaco diría «config-missing / Move / inputs permanentes». Esta traza no es eso. |
| Operador continuó la entrega en raw | **No está en la traza.** | Emisor `execute-process`, acción `merkle-batch-preseal`. La fractura la emite `emit_dlt_batch_fracture`. |
| Misma causa que `0142599491fb` | **No.** | Aquel sello es `mayeuta-llm/prótesis exit 3` en `sse_chat_stream`. Cero IOTA. |

## Causa

Reserva de objeto IOTA: el quórum rechazó la firma porque un objeto estaba bloqueado por otra transacción (`7R6vWf14…`). Es la familia de la colisión de gas (`60db1db67e49`), con otra cadena de error.

`dlt_transient_error_trace` absorbe red (`ENETUNREACH` / `ETIMEDOUT` / `ENOTFOUND`) y gas (`is not available for consumption` ∧ `current version:`). Esta firma no entra. `emit_dlt_batch_fracture` sigue materializando `System_Fracture_Detected`. `stamp_batch_anchor_error` ya encola `dlt_reanchor` antes de emitir.

La cola `createSerialQueue` serializa `publishImmutableData` dentro del proceso del relay. No se reabre. No se afirma que haya fallado: un locker externo (la traza nombra otra transacción) queda fuera de esa cola. Palanca de este sello: no escalar esta firma a Kintsugi.

## Mandato

Suprimir `System_Fracture_Detected` para esta firma. Subtipar el cubo DLT de Mayeuta (léxico, sin LLM). `dlt_reanchor` absorbe.

## Criterio de cierre

- [x] **DLT-LOCK-CA1** Traza con `reserved for another transaction` no escribe `System_Fracture_Detected`. El stamp/`dlt_reanchor` de `stamp_batch_anchor_error` sigue. `config-missing` y «issues with transaction inputs» sin esta firma siguen emitiendo.
- [x] **DLT-LOCK-CA2** Predicados de red y de gas de `41717` / `60db1db67e49` intactos.
- [x] **MAYEUTA-LOCK-CA3** La misma traza → `process_fix`, texto de reserva de objeto. Sin `prompt_adjustment`, sin «Causa de transporte», sin «inputs permanentes».
- [x] Argos APTO en `validacion.md` tras CI verde del PR.
- [x] Este TODO en `docs/todos/done/` en la misma rama.

## Fuera de alcance

- Reabrir taxonomía `b3a715381787`, cause-propagation `a90fad3fa8fa`, predicado de red, mutex del relay.
- Retry, sleep, backoff (DA-5). Simular IOTA como cierre.
- `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION`.
- Tratar el exit 3 de Kalma2 como defecto IOTA.
