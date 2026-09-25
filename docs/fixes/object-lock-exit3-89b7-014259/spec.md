---
feature_name: object-lock-exit3-89b7-014259
created: "2026-09-25"
process: bug-fix
base: main
scope: dlt-object-lock-and-prosthetic-exit3
branch_name: fix/object-lock-exit3-89b7-014259
persist_ref: docs/fixes/object-lock-exit3-89b7-014259
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (89b7c8b105ec).md
sibling_pbi_ref: docs/todos/pending/[FIX] kalma2-bridge — fractura sistémica (0142599491fb).md
document_id: PBI-FIX-FRACTURE-89b7c8b105ec
execution_id: "2b45bcaf-60ea-46bc-ae41-cb2102d92925"
---

# Especificación — sellos `89b7c8b105ec` y `0142599491fb`

## Problema

Dos fracturas. Causas de runtime distintas. Clase común: un fallo operativo ya tipado se emite como `System_Fracture_Detected`, y la sección Mayeuta del stub no describe la traza.

### `89b7c8b105ec` — reserva de objeto IOTA

Traza: `iota-relay-publish-error` status 500, quórum, `reserved for another transaction`, locker `7R6vWf14dsfmfGtQKJXAcjagG5G5phFMYwTKtvUCKfTX` (stake 35.14).

| ID | Defecto |
|----|---------|
| F1 | `dlt_transient_gas_version_trace` exige `is not available for consumption` ∧ `current version:`. Esta firma no entra. `emit_dlt_batch_fracture` emite. |
| F2 | Stub Mayeuta `prompt_adjustment` («Bloqueo operativo…»). Fósil: el catch-all ya no incluye `failed`, y la traza no contiene `block`. |
| F3 | Re-enrich con la fuente actual cae en el cubo DLT opaco («inputs permanentes»). Inexacto para un lock transitorio. |

`stamp_batch_anchor_error` encola `dlt_reanchor` antes de emitir. La cola `createSerialQueue` no se toca.

### `0142599491fb` — exit 3 de la prótesis

Traza: `mayeuta-llm/prótesis exit 3`. `handle_chat` la forma con cualquier `child.wait()` no exitoso.

| ID | Defecto |
|----|---------|
| F4 | Exit 3 es el fail-closed de `stream_infer_tokens` bajo `SDDIA_LLM_REQUIRE_INFER`. El puente lo trata como `prosthetic_collapse`. |
| F5 | Mayeuta escribe la plantilla unclassified («Auditar proceso»). No hay cubo para la traza. |

La traza no distingue cuál de los cuatro `SystemExit(3)`. No es ELF ausente (`64f37c7f7b34`).

## Cambio requerido

### Core — predicado de reserva

`dlt_transient_object_lock_trace`: la causa, en minúsculas, contiene `reserved for another transaction`. `dlt_transient_error_trace` es red ∨ gas ∨ reserva. `emit_dlt_batch_fracture` retorna sin escribir pending.

Siguen emitiendo: `config-missing`; `issues with transaction inputs` sin esta firma; relay unreachable.

### Mayeuta nativo

Dentro del cubo `iota-relay-publish-error`, antes del opaco:

- firma de reserva → `process_fix`; texto de objeto reservado por otra transacción; `dlt_reanchor` absorbe; prohibido transporte, `prompt_adjustment` e «inputs permanentes».

Cubo aparte, solo sobre `error_trace`:

- literal `mayeuta-llm/prótesis exit 3` → `process_fix`; nombra infer y `SDDIA_LLM_REQUIRE_INFER`; no es ELF ausente; no emitir fractura. Exit distinto no entra en el cubo.

Cero `llm:interact`. Bump de `enrich-fracture-pbi-kaizen.md` vía `entity-manager`.

### Puente Kalma2

Tras `child.wait()` en `handle_chat`: exit 3 no llama a `emit_system_fracture`. Cualquier otro exit ≠ 0 sí. Función pura testeable; el match solo la consulta.

## Criterios de aceptación

PBI `89b7c8b105ec`: DLT-LOCK-CA1, DLT-LOCK-CA2, MAYEUTA-LOCK-CA3.
PBI `0142599491fb`: KALMA-EXIT3-CA1, MAYEUTA-EXIT3-CA2.

## Fuera de alcance

- Mutex del relay, predicado de red, predicado de gas, taxonomía `b3a715381787`, cause-propagation.
- Exits de la prótesis, `SDDIA_LLM_REQUIRE_INFER`, watchdog SSE, `aiua_interact`, ELF ausente.
- `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION`.
- Retry/sleep. Simular IOTA como Done. E2E de publish real (no hay hueco de cola que cerrar).
