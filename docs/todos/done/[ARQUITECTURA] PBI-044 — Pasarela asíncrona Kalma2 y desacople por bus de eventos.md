---
document_id: PBI-044-KALMA2-PASARELA-ASINCRONA-EDA
title: "[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos"
format: markdown
version: "1.2.0"
closed: "2026-07-22"
created: "2026-07-22"
uuid: 8c71b50f-7067-472a-a149-40041920b054
status: done
priority: alta
process: feature
feature_ref: docs/features/kalma2-pasarela-asincrona-eda
validacion_ref: docs/features/kalma2-pasarela-asincrona-eda/validacion.md
branch_name: feat/kalma2-pasarela-asincrona-eda
persist_ref: docs/features/kalma2-pasarela-asincrona-eda
supersedes_draft: docs/todos/pending/[ARQUITECTURA] PBI-043 — Pasarela asíncrona de Cursor y desacople por bus de eventos.md
numbering_note: "Borrador v0 usaba PBI-043; ese número ya está ocupado por PBI-043-DI-CATALOGO-RESIDUAL-H7"
related:
  - docs/todos/done/[OPERATIVO] PBI: Integración Real de Kalma2 con el Motor de Eventos SddIA.md
  - docs/todos/done/[Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR.md
  - docs/todos/done/[FIX] interacción con front kalma2.md
  - docs/todos/done/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
  - docs/todos/pending/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/core/cumulo.paths.json
  - SddIA/core/event-domain-subscriptions.json
---

# [ARQUITECTURA] PBI-044: Pasarela asíncrona Kalma2 y desacople por bus de eventos

## 0. Corrección de numeración y estado del borrador

| Hallazgo | Corrección |
|----------|------------|
| Colisión `PBI-043` con DI residual H7+ | Este ítem = **PBI-044** (`PBI-044-KALMA2-PASARELA-ASINCRONA-EDA`) |
| Frontmatter/cuerpo fusionados (entropía documental) | Documento reescrito con YAML + secciones |
| `persist_ref` concatenado al título | `docs/features/kalma2-pasarela-asincrona-eda` |

---

## 1. Historia de Usuario

* **Como:** Operador (Vértice Biológico) vía terminal sensorial Kalma2 / cliente HTTP local.
* **Quiero:** Que la aduana HTTP acepte la intención y libere el socket **sin** esperar el fin del ciclo `execute-process` (cascada documental, Argos, hijos).
* **Para:** Recuperar el control del UI de inmediato y seguir el veredicto por correlación EDA (`GET /api/status` y/o notificación), bajo patrón fire-and-forget.

**No confundir con:** el mensaje de Cursor Agent *«waiting up to … for shell»* (tool Shell del IDE). Ese bloqueo pertenece al runtime de agentes Cursor, **no** al socket de `kalma2-bridge`. Ver §7 Fuera de alcance y §8 Q5.

---

## 2. Anomalía real (estado de partida)

### 2.1 Hechos verificables

| Vector | Estado actual |
|--------|---------------|
| `POST /api/interact` · `mode=execute` | Bridge invoca `execute-process --process kalma2-interact` y **bloquea** el HTTP hasta stdout/timeout (`SDDIA_CLIENT_TIMEOUT_SECONDS`, default **120 s**, no 10 min) |
| Emisión EDA | Ya existe: `kalma2-interact` emite `Kalma2_Process_Requested` (genoma). Suscriptor TQM → ciclo hijo. Features done: router, event-bus-integration, process-dispatch, full-cycle |
| Ceguera espacial del bridge | **Laudo vigente:** `kalma2-bridge` **no interpreta ni emite** eventos al bus (PBI Kalma2 event-bus-integration D2; O5 process-dispatch) |
| Feedback UI | `GET /api/status?event_id=` correlaciona dominio + `Process_Execution_Completed` en `eda_fractal.orchestration` |
| Bus SSOT | `eda_bus.pending` = `./.events/pending`; terminal PEC = `eda_fractal.orchestration` = `./.events/orchestration` (vía `cumulo.paths.json`) |

### 2.2 Brecha de este PBI

El camino **execute** ya es EDA *después* de que `kalma2-interact` emite; la aduana HTTP sigue siendo **síncrona** respecto al subproceso orquestador. El operador percibe “pasarela bloqueada” aunque el bus ya sepa despachar en background.

**Objetivo:** desacoplar el **socket HTTP** del **runtime del ciclo**, sin reabrir arquitectura ya sellada ni inventar un segundo evento de intención.

---

## 3. Vectores soberanos (requerimientos)

| ID | Requerimiento | Estado |
|----|---------------|--------|
| **R1** | **Ingesta no bloqueante (HTTP):** el endpoint de ejecución (mínimo `mode=execute` / `POST /api/execute`) responde en **p99 &lt; 50 ms** tras validar input mínimo, con sobre de aceptación (`success: true`, `status: "accepted"`, `correlation_id`/`event_id`). Código HTTP preferente **202 Accepted** (coherente con async); 200 solo si Dedalo lauda compat UI. | Abierto |
| **R2** | **Preservar ceguera espacial:** el bridge **no escribe** JSON en `.events/**`. La chispa EDA sigue saliendo del genoma (`kalma2-interact` u homólogo). Mecanismo admisible por defecto: spawn/detach del orquestador (o cápsula de ingesta ciega) **sin** `wait` del ciclo completo. Override solo con laudo Racso (§8 Q1). | Abierto |
| **R3** | **Reutilizar ECST existente:** evento de intención = **`Kalma2_Process_Requested`** (prohibido inventar `Kalma2_Interaction_Requested` sin alta formal + suscripciones). Payload ECST = Event-Carried State Transfer ya forjado (`process`, `raw_text`, `correlation_id ≡ event_id`, extras allowlist). | Abierto |
| **R4** | **Consumo por Sistema Nervioso:** `event-watcher` → `route-domain-event` → suscriptores actuales (TQM / ciclos). Sin reescritura del despacho salvo bugs. | Abierto |
| **R5** | **Terminal inmutable:** cierre observable vía `Process_Execution_Completed` en **`eda_fractal.orchestration`** (no confundir con `eda_bus.processed`). Indexación/consulta alineada a `GET /api/status` existente. | Abierto |
| **R6** *(opcional / soft)* | **Puente sensorial Telegram:** suscriptor o proceso que, ante PEC correlacionado al ciclo Kalma2, invoque `send-telegram-notification` con UUID + veredicto. No bloquea Done de R1–R5 salvo laudo. | Abierto |

---

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-R1** | Smoke HTTP: disparo execute → respuesta `accepted` + `correlation_id` en &lt; 50 ms p99 (lab local); el cliente **no** espera Argos ni fin de cascada en el mismo request. |
| **AC-R2** | Tras el acuse, existe rastro durable: `Kalma2_Process_Requested` bajo topología Cúmulo **o** evidencia de spawn del emisor genómico correlacionado; **cero** writes EDA desde `kalma2-bridge` (grep/audit). |
| **AC-R3** | Ciclo hijo / TQM progresa como hoy (no regresión process-dispatch). `orphan`/DL por inputs no empeora baseline. |
| **AC-R4** | `GET /api/status` con el `correlation_id` del acuse proyecta estados hasta terminal (`completed`/`failed`/equivalente) sin depender del HTTP original. |
| **AC-R5** | Chat SSE / `mode=chat` **fuera** del Done mínimo salvo laudo explícito (ver §7). |
| **AC-R6** | Si R6 activado: notificación Telegram con `correlation_id` + veredicto; si no, documentado como defer. |
| **AC-DONE-PBI** | Feature bajo `persist_ref` con `validacion.md` APTO + este PBI en `docs/todos/done/` + `pbi_archived: true` en el **mismo** PR (`task-closure-documental`). |

---

## 5. Flujo ontológico objetivo

```text
Cliente Kalma2 / HTTP
  → POST execute (intención)
  → kalma2-bridge: valida mínimo + libera socket (202 + correlation_id)
  → [fondo] execute-process kalma2-interact (genoma)
       → escribe Kalma2_Process_Requested en eda_bus / fractal según contrato vigente
  → event-watcher → route-domain-event → TQM / ciclo hijo
  → Process_Execution_Completed (eda_fractal.orchestration)
  → Cliente: GET /api/status (y opcionalmente Telegram R6)
```

**Invariantes:** paths solo vía `SddIA/core/cumulo.paths.json`. Bridge = aduana inerte. Emisión = genoma.

---

## 6. Plan de olas (propuesta Dedalo)

| Hito | Contenido | Piso |
|------|-----------|------|
| **H1** | Desacople HTTP execute (R1–R2) + smoke timing + audit no-write-bus | Obligatorio |
| **H2** | Contrato acuse UI (`accepted`, poll status) + regresión Kalma2 execute | Obligatorio |
| **H3** | R6 Telegram / endurecer PEC→notificación | Opcional |

---

## 7. Fuera de alcance

| Ítem | Motivo / destino |
|------|------------------|
| Mensaje Cursor *«waiting … for shell»* / `block_until` del Agent Shell | Runtime IDE ≠ pasarela Kalma2; PBI distinto o laudo Q5 |
| Mutación protocolo firma DLT IOTA | Fuera (heredado del borrador) |
| Migración daemons a systemd | Fuera; siguen bajo `start-sddia` / entorno lab |
| Reescribir emisión `Kalma2_Process_Requested` / allowlist | Ya APTO; no reabrir |
| Convertir bridge en emisor EDA | Prohibido sin laudo Racso (rompe ceguera) |
| Chat SSE síncrono (`/api/chat`, mayeuta-llm stream) | Soft-dep / PBI aparte; timeout distinto (`SDDIA_LLM_SSE_TIMEOUT_SECS`) |
| Residual DI H7+ / PBI-043 | Ortogonal |
| F3 git-manager KM / PPR #136 | Soft-dep operativo, no absorber |

---

## 8. Preguntas abiertas (Dedalo / laudo)

| # | Pregunta | Default provisional |
|---|----------|---------------------|
| **Q1** | ¿Cómo materializa el bridge el fire-and-forget sin escribir al bus? | **Spawn/detach** de `execute-process kalma2-interact` (o cápsula ingest ciega en genoma). Bridge-write-EDA = **veto** salvo laudo. |
| **Q2** | ¿HTTP 202 vs 200 en el acuse? | Preferir **202**; adaptar UI Kalma2 si hace falta. |
| **Q3** | ¿El `correlation_id` del acuse HTTP es preasignado por bridge o solo el `event_id` emitido por genoma? | Preferir: bridge genera UUID de correlación → lo pasa en inputs → emisión usa el mismo id (`correlation_id ≡ event_id`). |
| **Q4** | ¿R6 Telegram entra en Done global? | **No** por defecto; H3 opcional. |
| **Q5** | ¿Abrir PBI hermano para Shell async del Agent Cursor? | Solo si Racso confirma dolor real en sesiones Tekton; no mezclar scopes. |

---

## 9. Incoherencias del borrador v0 (auditoría)

| # | Borrador | Verdad / corrección |
|---|----------|---------------------|
| 1 | Numeración PBI-043 | Colisión con DI residual → **PBI-044** |
| 2 | «waiting for shell» = síntoma de kalma2-bridge | Falso: es UI del tool Shell de Cursor Agent |
| 3 | Timeout «hasta 10 minutos» | Default bridge orquestador = **120 s** (`SDDIA_CLIENT_TIMEOUT_SECONDS`) |
| 4 | Bridge escribe `.events/pending/{uuid}.json` | Viola ceguera espacial lauda |
| 5 | Evento nuevo `Kalma2_Interaction_Requested` | Ya existe **`Kalma2_Process_Requested`** + suscripciones |
| 6 | PEC en «topología de procesados» | PEC vive en **`eda_fractal.orchestration`**; `processed` es otra etapa del pipeline `eda_bus` |
| 7 | R1 dice `success/accepted` y §flujo dice `202` sin unificar | Unificado en R1/Q2 |
| 8 | AC mezcla Argos + Cerbero RBAC como requisito de pasarela | Son fases/gobernanza **dentro** del runtime; el AC correcto es «no bloquean el HTTP» (efecto de R1), no redefinir Argos/Cerbero |
| 9 | Telegram como AC-DONE obligatorio | Baja a R6/H3 opcional |
| 10 | Documento sin `uuid` / cuerpo colapsado | Entropía documental → este v1.1.0 |

---

## 10. Definición de Done

```text
Done = un PR mergeado en main
 + validacion.md APTO en el diff (pbi_archived: true)
 + este PBI en docs/todos/done/ en la misma rama
 + H1+H2 APTO (H3/R6 no obligatorio)
```

---

## 11. Referencias

- Integración UI↔EDA: `docs/todos/done/[OPERATIVO] PBI: Integración Real de Kalma2 con el Motor de Eventos SddIA.md`
- Despacho: `docs/features/kalma2-process-dispatch/`
- Observabilidad ciclo: `docs/todos/done/[Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR.md`
- Cúmulo: `SddIA/core/cumulo.paths.json`
- Bridge: `SddIA/interfaces/kalma2-bridge/`
- Cierre documental: `features-documentation-pattern` v1.2.0 + `task-closure-documental`
