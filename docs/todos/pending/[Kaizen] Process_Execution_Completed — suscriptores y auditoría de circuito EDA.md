---
document_id: PBI-KAIZEN-PEC-SUBSCRIBERS-CIRCUIT-AUDIT
title: "[Kaizen] Process_Execution_Completed — suscriptores y auditoría de circuito EDA"
format: markdown
version: "1.0.0"
created: "2026-08-15"
updated: "2026-08-15T13:47:00Z"
status: abierto
priority: alta
process: feature
uuid: fe8d3d21-ebeb-4a83-8b53-f2d7f0c19b16
incident_ref: "Kalma2 Forjar Proceso 2026-08-15 — timeout UI 120s; PEC 9ff24776 y dominio e273713c enrutados y purgados; GET /api/status 404"
source_correlation_id: e273713c-dd91-487b-8716-1bdc8c5da741
related:
  - SddIA/core/event-orchestration-subscriptions.json
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/event-telemetry-subscriptions.json
  - SddIA/events/orchestration/process-execution-completed.md
  - SddIA/process/route-orchestration.md
  - SddIA/tools/event-bus-audit.md
  - docs/todos/done/[OPERATIVO] PBI: Integración Real de Kalma2 con el Motor de Eventos SddIA.md
  - docs/todos/done/[Kaizen] ciclo Kalma2-feature — correlación EDA, estados terminales y aduana PPR.md
  - docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
  - docs/todos/pending/[OPERATIVO] PBI: Canal Asíncrono de Telemetría de Progreso y Observabilidad Activa para Interfaces Externas (Kalma2).md
  - docs/todos/pending/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #174).md
---

# [Kaizen] Process_Execution_Completed — suscriptores y auditoría de circuito EDA

## 0. Mandato

Abrir como **`feature`**. El PEC es el cierre táctico del peaje termodinámico, pero el registro de orquestación está **vacío**. `route-orchestration` (`purge_after=true`) trata cero suscriptores como consenso vacuoso y **borra** el JSON. Kalma2 sondea `GET /api/status` sobre `eda_fractal.orchestration` y recibe 404 → timeout 120s.

Objetivo dual:

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Suscriptores de interés en `Process_Execution_Completed` | Registro no vacío; fan-out real; circuito humano + testigo durable de `correlation_id` |
| **O2** | Auditoría de cobertura catálogo ↔ registro | `event-bus-audit` (o proceso homólogo) detecta tipo catalogado con 0 suscriptores, familia cruzada y claves huérfanas; informe accionable |

Filtro C: **no** convertir `kalma2-bridge` en suscriptor (ceguera espacial). El bridge **lee**; el nervio **entrega**.

## 1. Incidente (fuente empírica 2026-08-15)

| Campo | Valor |
|-------|--------|
| Estímulo | Kalma2 WUI «Forjar Proceso» → `POST /api/execute` 202 `intención aceptada` |
| Síntoma | `[timeout] sin completed/failed en 120s` |
| Dominio | `e273713c-dd91-487b-8716-1bdc8c5da741` — `route-domain (purgado)` |
| PEC | `9ff24776-26c7-4596-8b08-7b6fc4531641` — `route-orchestration (purgado)` |
| Status post-facto | `GET /api/status?event_id=e273713c-…` → HTTP 404 `evento no encontrado` |
| Registro | `event-orchestration-subscriptions.json` → `"Process_Execution_Completed": []` |
| Router | `route_orchestration_event(..., purge_after=true)` + rama `subscribers.is_empty()` → `safe_remove_path` |

Causalidad: dual-signal PBI-044 / `kalma2-event-bus-integration` asume PEC durable en `eda_fractal.orchestration`. El array vacío + purge convierte el cierre en **agujero de información**.

Añadir Telegram **solo** no cierra O1 para la WUI: tras `all_ok` el padre **sigue** unlinked. Hace falta **testigo durable** (índice `correlation_id`) o laudo Dedalo sobre política de purge orquestación.

## 2. Auditoría semilla — cobertura de suscriptores

Cruce catálogo `SddIA/events/{family}/` ↔ registros Cúmulo (`eda_fractal.*_subscriptions`). Semilla 2026-08-15; la feature debe **automatizar** este cruce (O2), no congelar la tabla.

### 2.1 Orquestación — `event-orchestration-subscriptions.json`

| `event_type` | Familia clase | n suscriptores | Hueco |
|--------------|---------------|----------------|-------|
| `Process_Execution_Completed` | orchestration | **0** | **Circuito ciego.** Purge inmediato. Kalma2 404. Sin Telegram. Sin índice durable. |
| `Local_QA_Requested` | **orchestration** | 0 en este registro | Suscrito en **dominio** (`argos` → `pull-request-review`). Familia de clase ≠ registro. |

### 2.2 Dominio — `event-domain-subscriptions.json`

| `event_type` | n | Nota |
|--------------|---|------|
| `PullRequest_Presented` | 3 | Argos PPR + IOTA + Telegram |
| `PullRequest_Merged` | 1 | Solo IOTA — sin Telegram de cierre merge |
| `PullRequest_Audited` | **0** | Clase catalogada; **sin fila**. Veredicto Argos no fan-out. |
| `System_Fracture_Detected` | 3 | PBI + Kaizen enrich + Telegram |
| `Kaizen_Alert_Required` | 1 | Cúmulo `materialize-kaizen-alert-doc` (único legítimo) |
| `Kalma2_Process_Requested` | 2 | TQM + IOTA — **sin** Telegram de aceptación |
| `Domain_Entity_{Created,Updated,Deleted}` | 2 c/u | sync-index + IOTA |
| `Domain_Entity_Telemetry_Captured` | 1 | memory-evolution-ingest |
| `Domain_Entity_{Degraded,Restored,Deprecated}` | 2–3 | Cerbero + DLT Radamanto (+ Dedalo en degraded) |
| `System_Immunity_Certified` | 1 | DLT Radamanto |
| `Suite_Execution_Requested` | 1 | execute-suite |
| `Local_QA_Requested` | 1 | **en dominio** pese a clase orchestration |
| `TelegramMessage_Received` | 2 | fallback + IOTA |
| `Manual_Task_Requested` / `Kaizen_Idea_Captured` / `Thought_Persisted` / `Vector_Memory_Indexed` | 1 | Solo IOTA |
| `CapabilityDi_Requested` | 1 | **Sin clase** bajo `SddIA/events/` — clave huérfana de registro |
| `Telemetry_Compliance_Breached` | **0** | Clase catalogada; alerta de contrato **sin** consumidor |

### 2.3 Telemetría — `event-telemetry-subscriptions.json`

| `event_type` | n | Nota |
|--------------|---|------|
| `Raw_Execution_Finished` | 2 | Radamanto batch + compliance-audit |
| `Daemon_Heartbeat` | 1 | daemon-heartbeat-audit |

Radamanto **no** consume PEC. `success_rate` de procesos multi-fase (PPR #174) se alimenta de telemetría atómica, no del cierre de orquestación — Faro, no O1.

### 2.4 Patrones de mejora detectados (no todos entran en v1)

| ID | Hallazgo | Circuito que falta | Prioridad v1 |
|----|----------|-------------------|--------------|
| H1 | PEC `[]` + `purge_after` | Cierre táctico invisible | **Sí — O1** |
| H2 | `PullRequest_Audited` sin fila | Veredicto Argos no notifica ni ancla | Auditoría O2; cableado opcional |
| H3 | `Telemetry_Compliance_Breached` sin fila | Breach termodinámico sin fan-out | Auditoría O2; cableado opcional |
| H4 | `Local_QA_Requested` dominio vs orchestration | Enrutado por familia física vs registro | O2 (invariante familia) |
| H5 | `CapabilityDi_Requested` sin clase | Registro sin genoma | O2 (clave huérfana) |
| H6 | Kalma2 request sin Telegram | Operador no ve aceptación ni cierre | Cierre vía PEC+Telegram (O1); no duplicar en request |
| H7 | Radamanto ↔ PEC desacoplado | Stats de proceso vs herramienta | Faro; no PPR #174 |

## 3. Suscriptores de interés para PEC (O1 — laudo propuesto)

Candidatos. Dedalo confirma/recorta en Clarify. Prohibido inventar segundo evento de cierre.

| # | Agente | Proveedor | Intent | ¿Cierra 404 WUI? |
|---|--------|-----------|--------|------------------|
| S1 | argos | `tool: send-telegram-notification` | Notificar al Vértice Biológico terminal `success`/`failed` + `process_name` + `correlation_id` | No (purge post-ok) |
| S2 | cumulo | acción/proceso **testigo** (nombre lauda Dedalo; p. ej. índice `correlation_id` bajo `eda_instance.proofs` o equivalente Cúmulo) | Persistir proyección durable del PEC **después** del unlink del padre | **Sí** — `GET /api/status` debe resolver este testigo |
| S3 | — | Política `purge_after` orquestación | Alternativa a S2: no unlink si el tipo es PEC; o archivar en vez de borrar | Sí si Dedalo elige esta vía **en lugar de** S2 (no ambas) |

**Fuera de O1 (Filtro C):**

- `kalma2-bridge` como suscriptor.
- IOTA en PEC (orquestación táctica ≠ libro mayor).
- Radamanto-on-PEC (Faro H7).
- Telegram en `Kalma2_Process_Requested` (ruido; el cierre PEC basta).

Invariante: `correlation_id ≡ event_id` del `Kalma2_Process_Requested` (laudo L3 vigente).

## 4. Auditoría continua (O2)

Extender `event-bus-audit` (cápsula ya indexada) **o** proceso wrapper si el laudo exige fase Argos:

1. Cargar catálogo por familia (`events/{telemetry,orchestration,domain}/index.md` + clases).
2. Cargar los tres JSON de `eda_fractal.*_subscriptions`.
3. Emitir hallazgos:
   - `EMPTY_SUBSCRIBERS` — clase con array vacío o clave ausente.
   - `FAMILY_MISMATCH` — clave en registro de familia distinta a `event_family` de la clase.
   - `ORPHAN_REGISTRY_KEY` — clave en JSON sin clase catalogada.
   - `PURGE_BLACKHOLE` — `EMPTY_SUBSCRIBERS` ∧ router de esa familia con `purge_after=true`.
4. Umbral: `PURGE_BLACKHOLE` o `EMPTY_SUBSCRIBERS` en orchestration → `Kaizen_Alert_Required` (no `delivery_state: failed` del audit).

No sustituye ECST/staleness actuales del tool; **añade** cobertura de circuito.

## 5. Fuera de alcance

- Canal SSE de telemetría de progreso (PBI OPERATIVO Kalma2 pendiente — coordina, no absorbe).
- Rehabilitación `pull-request-review` / umbrales Radamanto (PPR #174).
- Reabrir PBI-044 / dual-signal (ya laudo; este Kaizen **cierra el residual** de purge).
- Convertir el bridge en emisor o suscriptor del bus.

## 6. Diseño objetivo

```text
execute-process peaje
  → PEC @ eda_fractal.orchestration (correlation_id plumbed)
  → route-orchestration
       ├─ S1 Telegram (humano)
       └─ S2 testigo durable  XOR  no-unlink PEC (laudo Dedalo)
  → GET /api/status?event_id=<cid> proyecta completed|failed|initialized|awaiting_agents
     sin depender de la ventana de vida del JSON padre
  → event-bus-audit cruza catálogo↔registros (O2) en aduana/on-demand
```

## 7. Proceso de inicio

```json
{
  "process": "feature",
  "feature_name": "kaizen-pec-subscribers-circuit-audit",
  "branch_name": "feat/kaizen-pec-subscribers-circuit-audit",
  "persist_ref": "docs/features/kaizen-pec-subscribers-circuit-audit",
  "refined_requirements": "Kaizen: suscriptores reales de Process_Execution_Completed (Telegram + testigo durable XOR política de purge); GET /api/status deja de 404 post-route; event-bus-audit detecta EMPTY_SUBSCRIBERS, FAMILY_MISMATCH, ORPHAN_REGISTRY_KEY y PURGE_BLACKHOLE.",
  "pbi_ref": "docs/todos/pending/[Kaizen] Process_Execution_Completed — suscriptores y auditoría de circuito EDA.md",
  "base_branch": "main"
}
```

Mutación de genoma (`events/`, `tools/event-bus-audit`, `process/route-orchestration` si aplica) **solo** vía `./sddia-run.sh --process feature` / `entity-manager`. El JSON de suscripciones es SSOT Cúmulo; no editar a mano en esta semilla.

## 8. Criterio de cierre del PBI

- [ ] `Process_Execution_Completed` tiene ≥1 suscriptor indexado y el fan-out se observa en testigos.
- [ ] Tras route+purge del padre, `GET /api/status?event_id=<cid>` proyecta estado terminal o de ciclo (`completed`/`failed`/`initialized`/`awaiting_agents`) — **no** 404 persistente ni timeout 120s ciego.
- [ ] Telegram (o exención Dedalo documentada) notifica cierre PEC con `process_name` + `correlation_id`.
- [ ] `event-bus-audit` (o proceso laudo) falla/alerta ante `PURGE_BLACKHOLE` y lista H2–H5 como hallazgos, no como silencio.
- [ ] `validacion.md` APTO + este PBI en `docs/todos/done/` en el **mismo** PR.

## 9. Referencias

- Clase: `SddIA/events/orchestration/process-execution-completed.md`
- Router: `SddIA/engine/execute-process/src/engine/route_fractal_core.rs` (`subscribers.is_empty` + `purge_after`)
- Status: `SddIA/interfaces/kalma2-bridge` `find_pec_by_correlation` / `project_status`
- Dual-signal: `docs/features/kalma2-event-bus-integration/spec.md` L1–L3
- Pasarela: `docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md`
