---
feature_name: kalma2-pasarela-asincrona-eda
created: "2026-07-22"
process: feature
purpose: Estabilización Mayeuta — desacople HTTP kalma2-bridge sin romper ceguera espacial
branch_name: feat/kalma2-pasarela-asincrona-eda
persist_ref: docs/features/kalma2-pasarela-asincrona-eda
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
document_id: PBI-044-KALMA2-PASARELA-ASINCRONA-EDA
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
---

# Clarificación — kalma2-pasarela-asincrona-eda

Transcript Mayeuta (2026-07-22). Semilla PBI-044 v1.1.0 + orden Raw Kernel fase Estabilización.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` (fase Estabilización → handoff Dedalo) |
| `feature_name` | `kalma2-pasarela-asincrona-eda` |
| Rama | `feat/kalma2-pasarela-asincrona-eda` |
| `persist_ref` | `docs/features/kalma2-pasarela-asincrona-eda` |
| `document_id` | `PBI-044-KALMA2-PASARELA-ASINCRONA-EDA` |
| Numeración | **PBI-044** (no 043: colisión con DI residual H7+) |
| Alcance ciclo | **H1+H2** = Done mínimo; **H3/R6** opcional |
| Fase | Estabilización Mayeuta (esta sesión) → Dedalo diseño desacople HTTP |

---

## D1 — Diagnóstico de anomalía (qué falla / qué no)

| Afirmación | Veredicto Mayeuta |
|------------|-------------------|
| Socket HTTP bloqueado en `mode=execute` hasta fin/timeout de `execute-process kalma2-interact` | **Sí** — brecha de este PBI |
| Timeout default | `SDDIA_CLIENT_TIMEOUT_SECONDS` = **120 s** (no «10 min» del borrador v0) |
| Camino EDA post-emisión (`Kalma2_Process_Requested` → TQM → ciclo) | **Ya existe** — no reabrir |
| Mensaje Cursor Agent «waiting … for shell» | **Ortogonal** — runtime IDE ≠ pasarela; fuera (Q5) |
| Bridge escribe `.events/**` | **Prohibido** — viola ceguera espacial lauda |

**Toll:** la pasarela es síncrona respecto al orquestador; el bus ya sabe despachar en background. Objetivo = desacoplar **socket HTTP** de **runtime del ciclo**.

---

## D2 — Reutilización vs invención (entropía rechazada)

| Tentación (borrador v0 / ruido) | Laudo |
|---------------------------------|-------|
| Evento nuevo `Kalma2_Interaction_Requested` | **Veto** — reutilizar `Kalma2_Process_Requested` (R3) |
| Bridge emisor EDA (write pending) | **Veto** salvo laudo Racso (Q1) |
| Reescribir allowlist / emisión | **Fuera** — ya APTO en precedentes |
| Reabrir process-dispatch | **Fuera** salvo bugs de regresión |
| Chat SSE / `mode=chat` en Done mínimo | **Fuera** (AC-R5) |

---

## D3 — Vectores soberanos estabilizados (R1–R6)

| ID | Qué (requisito estable) | Piso Done |
|----|-------------------------|-----------|
| **R1** | Acuse HTTP no bloqueante: p99 &lt; 50 ms; `success:true`, `status:accepted`, `correlation_id`/`event_id`; HTTP **202** preferente | H1+H2 |
| **R2** | Bridge **no** escribe `.events/**`; chispa = spawn/detach `kalma2-interact` (o cápsula ingest ciega en genoma) | H1+H2 |
| **R3** | ECST = `Kalma2_Process_Requested` existente | H1+H2 |
| **R4** | Consumo nervioso intacto: watcher → route → TQM | H1+H2 |
| **R5** | Terminal vía `Process_Execution_Completed` en `eda_fractal.orchestration` + `GET /api/status` | H1+H2 |
| **R6** | Telegram ante PEC correlacionado | **H3 opcional** — no bloquea Done |

---

## D4 — Preguntas abiertas cerradas (laudos Q1–Q5)

Fuente: PBI-044 §8 + semilla operador (2026-07-22). Sin contradicción residual.

| # | Pregunta | Laudo |
|---|----------|-------|
| **Q1** | Fire-and-forget sin write al bus | **Spawn/detach** de `execute-process kalma2-interact` (o cápsula ingest ciega). Bridge-write-EDA = **veto** |
| **Q2** | 202 vs 200 | Preferir **202 Accepted**; UI adapta si hace falta |
| **Q3** | Origen de `correlation_id` | Bridge **preasigna UUID** → inputs → emisión reutiliza (`correlation_id ≡ event_id`) |
| **Q4** | ¿R6 en Done global? | **No** — H3 defer |
| **Q5** | ¿PBI hermano Shell Cursor? | **No** en este ciclo; solo si Racso confirma dolor Tekton aparte |

---

## D5 — Criterios de aceptación producto (mapeo AC)

| AC | Liga | Nota |
|----|------|------|
| AC-R1 | R1 | Smoke timing; cliente no espera Argos/cascada en el mismo request |
| AC-R2 | R2+R3 | Rastro `Kalma2_Process_Requested` o spawn correlacionado; audit cero writes EDA desde bridge |
| AC-R3 | R4 | Sin regresión process-dispatch / orphan baseline |
| AC-R4 | R5 | Poll `GET /api/status` hasta terminal con el `correlation_id` del acuse |
| AC-R5 | — | Chat SSE **fuera** Done mínimo |
| AC-R6 | R6 | Solo si H3 activado |
| AC-DONE-PBI | cierre | `validacion.md` APTO + PBI en `done/` + `pbi_archived: true` mismo PR |

---

## D6 — Invariantes innegociables (handoff Dedalo)

1. Paths solo vía `SddIA/core/cumulo.paths.json`.
2. Bridge = aduana inerte (valida + acusa + detach); emisión = genoma.
3. `correlation_id ≡ event_id` (UUID preasignado por bridge en inputs).
4. No inventar segundo evento de intención.
5. Git solo `skill:git-manager`; KM/TODOs solo Cumulo/Kaizen_Alert (no Tekton/Argos).

---

## D7 — Fuera de alcance (lista de exclusión)

waiting-for-shell Cursor Agent · IOTA DLT · systemd daemons · reescritura emisión/allowlist · chat SSE · PBI-043 DI residual · PPR#136 F3.

Precedentes done a preservar: `kalma2-event-bus-integration`, `kalma2-process-dispatch`, `kalma2-full-cycle`, Kaizen `feature-cycle-observability`.

---

## D8 — Veredicto Mayeuta

**ok** — requisitos termodinámicamente estables (R1–R5 + H1+H2). Handoff a Dedalo: diseñar desacople HTTP (spawn/detach + contrato acuse 202) **sin** romper ceguera espacial ni reabrir despacho EDA.

Pendiente Dedalo (cómo, no qué): detalle de spawn (PID/detach/stdio), contrato JSON del acuse UI, smokes timing/audit no-write-bus, regresión execute Kalma2.
