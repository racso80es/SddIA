---
document_id: PBI-FIX-FRACTURE-0142599491fb
title: "[FIX] kalma2-bridge — fractura sistémica"
format: markdown
version: "1.1.1"
created: "2026-09-09"
updated: "2026-09-25"
status: "cerrado"
priority: alta
process: bug-fix
fracture_hash: 0142599491fb
fracture_process: kalma2-bridge
fracture_kind: prosthetic_collapse
incident_ref: "System_Fracture_Detected — 0142599491fb"
refined: true
suggested_branch: fix/object-lock-exit3-89b7-014259
persist_ref_suggested: docs/fixes/object-lock-exit3-89b7-014259
source_audit: "2026-09-25 Filtro A sobre stub v1.0.0. Traza literal. handle_chat espera al hijo mayeuta-llm STREAM y, si el exit no es 0, emite prosthetic_collapse con solo el código. mayeuta-llm STREAM propaga el exit del CLI. En kalma2-agent-runtime-cursor.py, SystemExit(3) solo ocurre con SDDIA_LLM_REQUIRE_INFER truthy (CLI vacío/fallido, FileNotFound, excepción, o sin CLI). La traza no dice cuál de los cuatro. No es el sello 64f37c7f7b34."
review_notes: "v1.0.0: plantilla unclassified («no clasificada» / «Auditar proceso»). v1.1.0: eso no es causa; es el fallback de analyze_fracture_kaizen cuando root_causes queda vacío. Exit 3 es fallo operativo de infer, no colapso del puente ni ELF ausente. No comparte causa con 89b7c8b105ec."
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/skills/mayeuta-llm/src/main.rs
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - docs/todos/done/[FIX] kalma2-bridge — fractura sistémica (64f37c7f7b34).md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (89b7c8b105ec).md
related_pbis:
  - id: PBI-FIX-FRACTURE-64f37c7f7b34
    rol: "Ancestro ELF ausente. Traza distinta ('mayeuta-llm no encontrado'). No reabrir."
  - id: PBI-FIX-FRACTURE-89b7c8b105ec
    rol: "Mismo ciclo. Clase compartida (sobre-escalado). Causa distinta: reserva de objeto IOTA. No es la prótesis."
deferred_pbis:
  - id: PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION
    path: "docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md"
    rol: "Fuera de alcance."
gates_this_wave:
  - KALMA-EXIT3-CA1
  - MAYEUTA-EXIT3-CA2
---

# [FIX] kalma2-bridge — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `kalma2-bridge` |
| Emisor | `kalma2-bridge` |
| Acción intentada | `sse_chat_stream` |

## Traza de error

```
mayeuta-llm/prótesis exit 3
```

`handle_chat` forma esa cadena con `status.code()` tras `child.wait()` no exitoso. No incluye stderr. No declara `friction_id`.

## Filtro A — inexactitudes del stub v1.0.0

| Afirmación v1.0.0 | Veredicto | Hecho |
|-------------------|-----------|--------|
| «Causa raíz no clasificada… requiere laudo humano» + «Auditar proceso `kalma2-bridge`» | **Plantilla, no diagnóstico.** | Texto fijo de `analyze_fracture_kaizen` cuando ningún cubo hace match. La traza no entra en heartbeat, DLT, hook ni catch-all. |
| Exit 3 = colapso del puente / ELF ausente | **Falso.** | `64f37c7f7b34` es `mayeuta-llm no encontrado` antes del spawn. Aquí el hijo arrancó y salió 3. |
| Cuál de los cuatro `SystemExit(3)` | **No consta.** | Los cuatro exigen `SDDIA_LLM_REQUIRE_INFER`. La traza no distingue CLI vacío, `FileNotFound`, excepción o CLI ausente. |
| Bloquea el despliegue consumidor | **Exceso** (ya anotado en Paciente 0). | El bundle consumidor no empaqueta `mayeuta-llm`. |
| Misma causa que `89b7c8b105ec` | **No.** | Aquel sello es publish IOTA `reserved for another transaction`. |

## Causa

`POST /api/chat` → `mayeuta-llm` operación `STREAM` → prótesis `CHAT_STREAM` → `stream_infer_tokens`. Con `SDDIA_LLM_REQUIRE_INFER` activo, un infer sin tokens, sin binario o con error termina en exit 3. `mayeuta-llm` propaga ese código. `handle_chat` trata todo exit ≠ 0 como `prosthetic_collapse` y emite `System_Fracture_Detected`.

Exit 3 es el fail-closed de infer ya definido en la prótesis. El stream SSE ya se respondió. No es muerte del puente.

## Mandato

No emitir `System_Fracture_Detected` para el exit 3 de esta prótesis. Cubo léxico Mayeuta para la traza literal, por si reaparece. Exits distintos de 3 siguen fracturando.

## Criterio de cierre

- [x] **KALMA-EXIT3-CA1** Exit 3 no llama a `emit_system_fracture`. Exit ≠ 0 y ≠ 3 sí.
- [x] **MAYEUTA-EXIT3-CA2** Traza `mayeuta-llm/prótesis exit 3` → `process_fix` que nombra infer/`SDDIA_LLM_REQUIRE_INFER`. No «requiere laudo humano». No `prompt_adjustment`.
- [x] Argos APTO en `validacion.md` tras CI verde del PR.
- [x] Este TODO en `docs/todos/done/` en la misma rama.

## Fuera de alcance

- Reabrir `64f37c7f7b34` (orden release-first / ELF ausente).
- Cambiar los exit de la prótesis o relajar `SDDIA_LLM_REQUIRE_INFER`.
- Watchdog SSE, cliente desconectado, `aiua_interact`.
- Reserva de objeto IOTA.
