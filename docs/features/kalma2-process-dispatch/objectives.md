---
feature_name: kalma2-process-dispatch
created: "2026-07-20"
process: feature
branch_name: feat/kalma2-process-dispatch
persist_ref: docs/features/kalma2-process-dispatch
document_id: PBI-KALMA2-PROCESS-DISPATCH
uuid: 0f5191df-927b-4da0-acf3-bb715766b5fa
status: validacion_apto
pbi_archived: true
pbi_ref: docs/todos/done/[FIX] interacción con front kalma2.md
depends_on:
  - docs/features/kalma2-mayeuta-llm-router
  - docs/features/kalma2-event-bus-integration
execution_id_init: 79c09578-e15d-42cd-b6fa-0c2b542247ca
evidence_event_id: a7725b42-2661-4bc5-9795-c69d8ca2ab5c
---

# Objetivos — kalma2-process-dispatch

## Misión

Cerrar el eslabón roto entre emisión válida de `Kalma2_Process_Requested` y la activación del ciclo de vida solicitado (`bug-fix` | `feature` | `refactorization`), de modo que el suscriptor `tekton.task-queue-manager` deje de empujar el sobre a dead-letter por fallo de consumo/despacho.

## Punto objetivo

> **O-DESPACHO-KALMA2:** Dado un `Kalma2_Process_Requested` con `process` en allowlist y texto/`pbi_ref` usable, el Sistema Nervioso completa el triaje soberano y el proceso hijo arranca (o queda encolado de forma auditable) con `correlation_id ≡ event_id`, sin `delivery_state.tekton.task-queue-manager = failed` por contrato de inputs.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Contrato de consumo post-dispatcher (TQM y/o rama equivalente) | Reabrir lazo UI poll/status (`kalma2-event-bus-integration`) |
| Liquidación deuda D3 (TQM genérico vs paquete Kalma2) | Convertir bridge/`app.js` en emisor EDA |
| Robustez `pbi_ref` ante paths con espacios en `raw_text` | Autenticación, Cerbero, TLS |
| Preservar C2 (async EDA) y laudo suscriptor fijo salvo deroga Dedalo | Fallo co-ocurrente IOTA (default fuera; Q1 Racso) |
| Evidencia reproducible vs dead-letter `a7725b42` | D1 timeout CLI / D5 E2E CI del router |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Diagnóstico cerrado | Confirmado: ECST emisión OK; fallo = consumo/despacho TQM (no front) |
| **O2** | Despacho no-DLT por inputs | Replay o smoke con payload tipo a7725b42 → `tekton.task-queue-manager` ≠ `failed` por inputs |
| **O3** | Ciclo hijo correlacionado | Arranque o encolado auditable de `payload.process` con `correlation_id ≡ event_id` |
| **O4** | `pbi_ref` robusto | Path con espacios en `raw_text` produce `pbi_ref` o rechazo explícito documentado (no silencio) |
| **O5** | Invariantes | Bridge sin write al bus; C2 async; paths vía Cúmulo |
| **O6** | Documentación | `clarify`/`objectives`/`spec`/`plan` bajo `persist_ref`; PBI v1.1.0 Refinado |

## No objetivos

- Mutar genoma de emisión del evento salvo el matiz `pbi_ref` (A′).
- Sustituir `kalma2-interact` / allowlist semántica.
- Unificar o “arreglar de paso” el publisher IOTA del mismo sobre (salvo laudo Q1).

## Ley aplicada

- `.cursorrules` §4 (cápsulas, JSON), §5 (agnosticismo Core)
- C2 async EDA (`kalma2-mayeuta-llm-router` O10/O14)
- Suscriptor fijo TQM (laudo P1/O14) — deroga solo vía Dedalo documentada
- Ceguera espacial `kalma2-bridge`
- `SddIA/core/cumulo.paths.json` + `eda_fractal`
- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.0
- Clarificaciones D1–D7 y Q1–Q3 en `clarify.md`
