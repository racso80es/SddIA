---
feature_name: kaizen-event-bus-audit-sensor-tuning
created: "2026-09-05"
process: bug-fix
base: main
scope: event-bus-audit-needs-kaizen-and-kaizen-template
branch_name: fix/kaizen-event-bus-audit-sensor-tuning
persist_ref: docs/fixes/kaizen-event-bus-audit-sensor-tuning
pbi_ref: docs/todos/pending/PENDING_AUDIT_DOC_31867981.md
document_id: PBI-KAIZEN-EVENT-BUS-AUDIT-31867981
uuid: 46dde226-6672-420f-8d2a-a5f3b49cdea8
execution_id: "f9830175-0405-42fd-9e0c-e6de1c26201d"
---

# Especificación — tuning sensor `event-bus-audit` y plantilla Kaizen

## Problema

`tool:event-bus-audit` evalúa `needs_kaizen` con OR acumulado sobre el censo histórico del bus (`dead_letter_count > 0`, estructurales, huérfanos, stale). En instancia con DL terminal (~12.9k cabeceras) el predicado es perenne. `take(50)` sobre `anomalies` cambia de conjunto → nuevo `PENDING_AUDIT_DOC_{hash8}`.

La acción `materialize-kaizen-alert-doc` emite plantilla DIA (`sensor DIA`, «fuga documental») para cualquier `alert_kind`, incluido `event-bus-audit`.

Censo 2026-09-05 (PBI v1.2.0): el 41,7 % de cabeceras DL son dumps `github-bridge-*` no-ECST (cuentan como `structural`); el 50 % son `Email_Received` crónicos; los pending stale son `System_Fracture_Detected` ya materializados.

## Fuera de alcance

- Purga o mutación de `.events/dead-letter/`.
- Recablear IOTA, Telegram, `email-triage-gateway` o fracturas sistémicas.
- Flag nuevo `alert_on_dead_letter` (ya existe `emit_kaizen_alert`).
- El snippet v1.1.0 (`stale_pending \|\| orphan \|\| circuit_alert`) — seguiría en `true`.

## Cambio

### S1 — Clasificación `non_ecst_sink`

En `validate_ecst_event`, si faltan a la vez `event_id`, `event_type` y `emitter_agent`, la anomalía es `kind: non_ecst_sink` (no `structural`). Cubre dumps `github-bridge-watcher` / `FALLBACK_LOCAL_SIGNATURE`.

`structural_error_count` no incluye `non_ecst_sink`. El informe lista ambos.

### S2 — `needs_kaizen` accionable

Predicado único:

```text
needs_kaizen = circuit_alert || actionable_stale_pending_count > 0
```

| Señal | Entra en Kaizen | Motivo |
|-------|-----------------|--------|
| `circuit_alert` (`PURGE_BLACKHOLE` / `EMPTY_SUBSCRIBERS` orchestration) | Sí | Defecto de catálogo |
| `stale_pending` con `event_type != System_Fracture_Detected` | Sí | Cola dominio atascada no cubierta por PBI de fractura |
| `dead_letter_count` / testigos DL / `non_ecst_sink` / estructurales de sink | No | Acumulación histórica |
| `orphan_witness` | No | Testigos de DL terminal |
| `stale_pending` de `System_Fracture_Detected` | No | Ya hay PBI de fractura |

`emit_kaizen_alert` (default `true`) no cambia. El proceso `event-bus-audit.md` deja de afirmar emisión por «haber dead-letters».

### S3 — Plantilla polimórfica

`build_todo_body` en `materialize_kaizen_alert_doc.rs`:

- `alert_kind == "doc_parity"` (default): plantilla DIA vigente.
- `alert_kind == "event-bus-audit"` u otro técnico: encabezado de infraestructura; checklist sin «fuga documental»; conserva tabla `implicated_files`.

`find_open_kaizen_audit_doc` reconoce TODO abierto si hay casilla DIA **o** casilla de bus sin marcar.

### S4 — Contrato de Clase

`SddIA/events/domain/kaizen-alert-required.md`: añadir emisor autorizado `tool:event-bus-audit` / proceso `event-bus-audit`. El propósito de la Clase deja de afirmar «solo DIA».

## Laudos

| ID | Decisión |
|----|----------|
| L-NO-PURGE | Cero escritura en `.events/dead-letter/`. |
| L-NO-FALSE-FIX | Prohibido un `needs_kaizen` verdadero ante solo DL histórico, dumps github-bridge o stale de fractura. |
| L-EMIT-FLAG | No duplicar `emit_kaizen_alert`. |
| L-ENGINE-OK | Handler `materialize_kaizen_alert_doc.rs` no es DA-2; `SddIA/tools/` y `SddIA/events/` sí — mutación en este ciclo `bug-fix` (DA-4). |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| EBA-CA1 | Dump sin `event_id`/`event_type`/`emitter_agent` → `non_ecst_sink`, no incrementa `structural_error_count`. |
| EBA-CA2 | Fixture con DL histórico + stale solo `System_Fracture_Detected` + sin `circuit_alert` → `needs_kaizen: false`. |
| EBA-CA3 | `circuit_alert` o stale pending no-fractura → `needs_kaizen: true`. |
| EBA-CA4 | `alert_kind: event-bus-audit` no contiene «fuga de conocimiento documental» ni «sensor DIA». |
| EBA-CA5 | Dedupe por huella sigue activo para plantilla bus (casilla abierta). |
| EBA-CA6 | Tests crate `event-bus-audit` + `materialize_kaizen_*` OK. |
| EBA-CA7 | PBI archivado en `docs/todos/done/` + `validacion.md` `global: APTO` `pbi_archived: true` en este PR. |
| EBA-CA8 | Cero diffs bajo `.events/`. |

## Impacto en Documentación

- `SddIA/tools/event-bus-audit.md` — semántica `needs_kaizen` v1.2.0.
- `SddIA/process/event-bus-audit.md` — dejar de ligar emisión Kaizen a «hay dead-letters».
- `SddIA/actions/materialize-kaizen-alert-doc.md` — plantilla por `alert_kind`.
- `SddIA/events/domain/kaizen-alert-required.md` — emisor `event-bus-audit`.
- Este `persist_ref` (spec / plan / implementation / execution / validacion).
