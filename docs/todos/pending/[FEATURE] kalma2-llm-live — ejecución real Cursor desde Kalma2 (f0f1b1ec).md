---
document_id: PBI-KALMA2-LLM-LIVE-V2
uuid: f0f1b1ec-4b79-47c6-85e2-a0ac2ca3164b
title: "[FEATURE] kalma2-llm-live — Interacción S+ Grade y Prótesis de Ejecución"
format: markdown
version: "2.3.0"
created: "2026-07-20"
refined: "2026-07-20"
status: "especificacion-blindada"
priority: alta
process: feature
suggested_feature_name: kalma2-llm-live
suggested_branch: feat/kalma2-llm-live
depends_on:
  - docs/features/kalma2-full-cycle
baseline_delivered:
  - docs/features/kalma2-event-bus-integration
  - docs/features/kalma2-mayeuta-llm-router
  - docs/features/kalma2-process-dispatch
related:
  - SddIA/skills/mayeuta-llm/
  - SddIA/interfaces/kalma2-bridge/
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.sh
  - interfaces/kalma2/
  - .dev/.env.example
  - docs/features/kalma2-llm-live/
inherited_from: docs/todos/done/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
supersedes: "v2.2.0 — amplía backlog de tramos aún smoke/soft (inferencia, agent live, SQLite→IDE, AC2, cierre)"
evidence:
  - "2026-07-20 feat/kalma2-llm-live: /api/chat SSE + /api/execute + STREAM + prótesis SQLite (lab DB temp) + UI dual"
  - "Host: cursor-agent ausente → chat sqlite-ack; fases agent awaiting_agents"
  - "Insert SQLite ≠ disparo del agente IDE Cursor"
---

# [FEATURE] kalma2-llm-live — Interacción S+ Grade y Prótesis de Ejecución (v2.3)

## Estado

**v2.3.0 — Especificación Blindada + backlog de cierre live.** Depende de `kalma2-full-cycle` (A+B+C APTO). Rama activa: `feat/kalma2-llm-live`.

### Ya forjado (no reabrir como diseño)

| Tramo | Evidencia |
|-------|-----------|
| `/api/chat` SSE + watchdog + `System_Fracture_Detected` | bridge + smoke |
| `/api/execute` → `Kalma2_Process_Requested` | bridge + kalma2 `mode=execute` |
| `mayeuta-llm` STREAM → subproceso Python | skill + smoke |
| Dual-mode `.py` (`CHAT_STREAM` + `AGENT_PHASE`) | smoke MOCK + DB temp |
| UI Chat / Forjar Proceso | `interfaces/kalma2/` |
| Prótesis SQLite (insert `composerData` + bubbles + headers) | lab en DB temporal |

## 1. Clarificación Estratégica (Filtro B — Táctica del Refugio)

- **Aislamiento paramétrico (Foso Biológico):** SddIA desconoce Cursor. SQLite / CLI / SDK viven solo en la prótesis Python. Core Rust puro.
- **Jurisdicción de enrutamiento:** Enrutamiento determinista = aduana `kalma2-bridge` + UI (sin `CLASSIFY_INTENT` como aduana).

### 1.1 Laudos (compactos + ampliación v2.3)

| ID | Norma |
|----|--------|
| **L-EP** | `/api/chat` SSE + `/api/execute`; `/api/interact` compat/deprecado. |
| **L-CI** | Sin `CLASSIFY_INTENT` como aduana. |
| **L-SK** | Evolucionar `mayeuta-llm` (STREAM ya forjado). |
| **L-FILE** | Dual-mode chat-stream + AGENT_PHASE. |
| **L-SF** | Fractura SSE/prótesis vía `System_Fracture_Detected`. |
| **L-STOP** | UI no mezcla Chat y Forjar en la misma petición. |
| **L-INF** | Inferencia de tokens de chat = `SDDIA_LLM_INFER_COMMAND` \| `SDDIA_AGENT_RUNTIME_CLI` (nunca reentrar en el `.py` prótesis). Sin binario → **prohibido** declarar chat live; solo `sqlite-ack` / mock explícito. |
| **L-IDE** | Insertar en `state.vscdb` **no implica** disparar el agente del IDE. Disparo live = CLI/SDK (o mecanismo documentado de instancia). SQLite = persistencia/continuabilidad, no oráculo. |
| **L-WAL** | Escritura en DB live con Cursor abierto es contención WAL; lab debe usar copia (`SDDIA_CURSOR_VSCDB`) o Cursor cerrado. Smoke en DB temp ≠ validación live. |

## 2. Objetivos S+ Grade

1. Streaming SSE con watchdog.
2. Abstracción Rust → Python (coste cero ante LLM soberano).
3. Enrutamiento físico Chat vs Proceso.
4. **(v2.3)** Circuito **live** Kalma2↔Cursor: inferencia real, fases agent ejecutadas, SQLite validada fuera de mock, AC2 y cierre documental.

## 3. Huecos smoke / soft / incompleto (backlog obligatorio)

Estos puntos **bloquean** declarar Done / `validacion.md` APTO global salvo laudo Racso de alcance reducido.

| ID | Hueco | Síntoma actual | Acción | Criterio de salida |
|----|-------|----------------|--------|-------------------|
| **S1** | Inferencia LLM en chat | Sin `cursor-agent` / `SDDIA_LLM_INFER_COMMAND` → backend `sqlite-ack` | Instalar CLI o fijar `SDDIA_LLM_INFER_COMMAND` / `SDDIA_AGENT_RUNTIME_CLI` en bóveda; smoke chat sin MOCK | Tokens SSE **no** son acuse determinista; `backend≠sqlite-ack` (o telemetría equivalente) |
| **S2** | `*_MOCK=1` en chat | Eco de palabras; cero SQLite | Lab/CI puede mockear; **producción/demo live** exige `SDDIA_LLM_CHAT_MOCK` y `SDDIA_AGENT_RUNTIME_MOCK` unset | Chat live documentado sin flags mock |
| **S3** | Fases agent soft | `cursor-agent` ausente → `awaiting_agents` | Misma bóveda que S1; reiniciar runtime/bridge; smoke `bug-fix`/`feature` desde `/api/execute` hasta fases `executed` o handoff con `backend: cli\|sdk` | PEC/`cycle_phase` honestos; no solo `awaiting_agents` por CLI missing |
| **S4** | SQLite ≠ disparo IDE | Insert deja conversación; el IDE no responde solo | Documentar L-IDE; opcional: runbook «abrir composer en Cursor»; **no** vender insert como chat autónomo del IDE | Spec/runbook + AC explícito: persistencia verificada en `state.vscdb` (copia o live bajo L-WAL) con keys legibles |
| **S5** | AC2 formal | Kill prótesis no E2E automatizado | Script/lab: `kill -9` hijo durante `/api/chat` → canal cerrado + evento en `.events/pending` | AC2 verificado con evidencia en `execution.md` / `validacion.md` |
| **S6** | Cierre documental/PR | Sin `validacion.md` APTO ni PBI en `done/` | Completar Argos + mover PBI en la **misma** rama; `delivery-close-cycle` / PR | Done = un PR + `pbi_archived: true` |

## 4. Especificación Técnica y Hitos

### Entregados (Hitos 1–4 base)

Hitos 1–4 de v2.2: puente SSE, wrapper STREAM, prótesis dual-mode + SQLite lab, aduana UI — **forjados**; quedan S1–S6.

### Hito 5 — Inferencia live (cierra S1/S2)

- Cablear bóveda: `SDDIA_LLM_INFER_COMMAND` o CLI agent.
- Prohibir mock en checklist de aceptación live.
- Telemetría de backend en cola SSE o log de prótesis (`cli` \| `sdk` \| `sqlite-ack`).

### Hito 6 — Agent runtime live (cierra S3)

- Verificar `SDDIA_AGENT_RUNTIME_COMMAND` → `.py` AGENT_PHASE con CLI/SDK real.
- E2E: Forjar Proceso desde Kalma2 → evento → TQM → hijo → al menos una fase `executed` (o handoff auditable no-soft por ausencia de binario).

### Hito 7 — SQLite live bajo L-WAL/L-IDE (cierra S4)

- Smoke contra copia de `state.vscdb` **o** DB live con Cursor cerrado.
- Verificar `composerData:` + `bubbleId:` + entrada en `composer.composerHeaders`.
- Runbook: límites (no auto-disparo IDE).

### Hito 8 — AC2 + cierre (cierra S5/S6)

- Prueba kill formal.
- `validacion.md` APTO, PBI → `docs/todos/done/`, PR único.

## 5. Plan de Acción (línea de montaje)

- [x] **Fase 1–4 (base):** bridge SSE/execute, mayeuta STREAM, prótesis dual+SQLite lab, UI dual — en `feat/kalma2-llm-live`.
- [ ] **Fase 5:** S1+S2 — inferencia live; bóveda sin mock en demo.
- [ ] **Fase 6:** S3 — agent phases live desde Kalma2 execute.
- [ ] **Fase 7:** S4 — validación SQLite (copia/live) + runbook L-IDE/L-WAL.
- [ ] **Fase 8:** S5+S6 — AC2 E2E + Argos + cierre documental/PR.

## 6. Criterios de Aceptación (Validación Rúnica)

| ID | Criterio |
|----|----------|
| **AC1** | SSE tokens desde stdout Python (lab o live). |
| **AC2** | `kill -9` prótesis → cierre limpio + `System_Fracture_Detected` (evidencia formal). |
| **AC3** | Proceso UI → `/api/execute` → orquestación async (no texto libre). |
| **AC4** | Purga del `.py` → `cargo build --release` Core OK. |
| **AC5** | AGENT_PHASE JSON válido post-CHAT_STREAM (no regresión B). |
| **AC6** | Chat live: inferencia ≠ `sqlite-ack` / mock (S1/S2). |
| **AC7** | Execute live: al menos una fase agent no-soft por CLI missing (S3). |
| **AC8** | SQLite: keys `composerData`/`bubbleId` verificadas en DB de prueba o live bajo L-WAL; runbook L-IDE presente (S4). |
| **AC9** | `validacion.md` global APTO + PBI archivado en el mismo PR (S6). |

## 7. Fuera de alcance

- Re-forjar ECST/TQM / event-bus.
- Hacer que el insert SQLite **dispare** autónomamente el agente UI de Cursor (salvo nuevo laudo; contradice L-IDE).
- LLM local soberano (solo contrato a coste cero).
- Versionar secretos / `.dev/.env`.

## 8. Mandato

Cerrar S1–S6 en `feat/kalma2-llm-live` bajo L-EP…L-WAL. Done = un PR mergeado en main + `validacion.md` APTO (`pbi_archived: true`) + PBI en `docs/todos/done/`.
