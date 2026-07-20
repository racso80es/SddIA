---
document_id: PBI-KALMA2-PROCESS-DISPATCH
uuid: 0f5191df-927b-4da0-acf3-bb715766b5fa
title: "[FIX] interacción con front kalma2 — despacho Kalma2_Process_Requested"
format: markdown
version: "1.1.0"
created: "2026-07-19"
refined: "2026-07-20"
status: "done"
priority: alta
process: feature
feature_ref: docs/features/kalma2-process-dispatch
branch_name: feat/kalma2-process-dispatch
archived_path: docs/todos/done/[FIX] interacción con front kalma2.md
evidence_event_id: a7725b42-2661-4bc5-9795-c69d8ca2ab5c
depends_on:
  - docs/features/kalma2-mayeuta-llm-router
  - docs/features/kalma2-event-bus-integration
related:
  - SddIA/events/domain/kalma2-process-requested.md
  - SddIA/process/task-queue-manager.md
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/handlers/kalma2.rs
  - SddIA/core/event-domain-subscriptions.json
inherited_debt: D3-task-queue-manager-generico
---

# [FIX] interacción con front kalma2 — despacho `Kalma2_Process_Requested`

## Estado

**Refinado (Mayeuta v1.1.0).** Semilla v0 era prompt Raw Kernel sin frontmatter. Alcance estabilizado: cerrar el colapso del suscriptor ejecutor tras emisión correcta desde Kalma2 (no reabrir el lazo UI de `kalma2-event-bus-integration`).

## Evidencia empírica (Dead-Letter)

| Campo | Valor |
|-------|--------|
| `event_id` | `a7725b42-2661-4bc5-9795-c69d8ca2ab5c` |
| `event_type` | `Kalma2_Process_Requested` |
| `emitter_agent` | `kalma2-interact` |
| Path bus | `.events/dead-letter/a7725b42-2661-4bc5-9795-c69d8ca2ab5c.json` |
| `delivery_state` | `tekton.task-queue-manager: failed` · `cumulo.iota-immutable-publisher: failed` |

Payload observado:

```json
{
  "process": "bug-fix",
  "raw_text": "Inicia proceso fix para corregir la incidencia documentada en docs/todos/pending/[FIX] telegram-watcher — fractura sistémica (e6cbecb9032c).md"
}
```

## Diagnóstico estabilizado (qué / por qué)

1. **Emisión ECST válida.** El payload cumple REQUIRED (`process`, `raw_text`) del evento `kalma2-process-requested`. El fallo no es de forma del sobre de dominio.
2. **Suscriptor fijo TQM.** `event-domain-subscriptions.json` enruta a `tekton` → `task-queue-manager` (laudo router O14 / P1).
3. **Dispatcher ya mapea.** `route_domain_core` (rama `Kalma2_Process_Requested`) construye `process_inputs` con `correlation_id`, `process`, `task_text←raw_text`, opcional `pbi_ref`, y despacha **el proceso del suscriptor** (`task-queue-manager`), no `payload.process` directamente.
4. **Brecha real = consumo TQM.** `task-queue-manager` declara input `tasks_path` y fases Triaje→Despacho hacia `feature|bug-fix|refactorization`, pero **no hay handler nativo** que acepte `{process, task_text, pbi_ref, correlation_id}` y complete el triaje. Resultado: `tekton.task-queue-manager: failed` → dead-letter.
5. **Deuda D3 heredada.** `kalma2-event-bus-integration` la dejó como «parcial / no reescribir TQM»; el dead-letter la convierte en bloqueante del execute path Kalma2.
6. **Matiz `pbi_ref`.** El path del PBI en `raw_text` contiene espacios (`[FIX] telegram-watcher — …`). `extract_pbi_ref` tokeniza por whitespace → no captura `pbi_ref`. Emisión sin `pbi_ref` aunque el texto lo menciona.

## Fuera de alcance (salvo laudo Racso)

| Ítem | Motivo |
|------|--------|
| Reabrir lazo UI poll/status (`kalma2-event-bus-integration`) | Ya APTO; síntoma distinto |
| Reescribir `app.js` / bridge como emisor EDA | Ceguera espacial; emisión ya correcta |
| Fallo `iota-immutable-publisher` del mismo evento | Canal DLT paralelo; no es el colapso del despacho de proceso (candidata deuda aparte) |
| Liquidar D1 timeout CLI / D5 E2E CI | No bloquean este dead-letter |

## Objetivo medible

Tras un `Kalma2_Process_Requested` con `process ∈ {bug-fix, feature, refactorization}` y `raw_text`/`pbi_ref` válidos:

- `tekton.task-queue-manager` **no** marca `failed` por contrato de inputs.
- El proceso solicitado avanza (o queda encolado de forma auditable) con `correlation_id ≡ event_id`.
- El evento no termina en dead-letter por la rama Kalma2→TQM.

## Hipótesis tácticas (handoff Dedalo — no laudo Mayeuta)

| Vía | Descripción | Nota Mayeuta |
|-----|-------------|--------------|
| **A′ (emisión)** | Mejorar extracción `pbi_ref` (paths con espacios) y/o enriquecer payload | Necesaria como matiz; **insuficiente sola** (TQM ya recibe `task_text`) |
| **B′ (recepción)** | Hacer que TQM (o el dispatcher) consuma `{process, task_text, pbi_ref}` y despache el ciclo | Alineada al laudo router: suscriptor fijo + triaje soberano |
| **C (dispatcher)** | Despachar `payload.process` directo en lugar de TQM | Contradice laudo O14/P1; solo si Dedalo deroga suscriptor fijo |

Semilla v0 hablaba de «Vía A bridge/app.js» y «Vía B parser TQM»: **A está mal enmarcada** (no es mapeo UI); **B es la dirección correcta** si se interpreta como cierre del contrato de consumo TQM/despacho.

## Mandato

Estabilizar y forjar el cierre del despacho Kalma2→ciclo de vida vía proceso `feature` `kalma2-process-dispatch`. Prohibido bypass raw del bus EDA.
