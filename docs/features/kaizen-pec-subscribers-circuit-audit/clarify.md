---
feature_name: kaizen-pec-subscribers-circuit-audit
created: "2026-08-17"
process: feature
purpose: Estabilización Mayeuta — suscriptores PEC + auditoría de cobertura catálogo↔registro EDA
phase: Estabilización de Requisitos
agents: mayeuta
branch_name: feat/kaizen-pec-subscribers-circuit-audit
persist_ref: docs/features/kaizen-pec-subscribers-circuit-audit
pbi_ref: docs/todos/pending/[Kaizen] Process_Execution_Completed — suscriptores y auditoría de circuito EDA.md
document_id: PBI-KAIZEN-PEC-SUBSCRIBERS-CIRCUIT-AUDIT
uuid: fe8d3d21-ebeb-4a83-8b53-f2d7f0c19b16
source_incident: e273713c-dd91-487b-8716-1bdc8c5da741
source_pec: 9ff24776-26c7-4596-8b08-7b6fc4531641
correlation_id: ""
status: stabilized
mayeuta_verdict: ok
---

# Clarificación — kaizen-pec-subscribers-circuit-audit

Transcript Mayeuta (2026-08-17). Semilla Kaizen PBI-KAIZEN-PEC-SUBSCRIBERS-CIRCUIT-AUDIT → requisitos estabilizados para handoff Dedalo.

Fuentes: PBI adjunto; `SddIA/core/event-orchestration-subscriptions.json` (`Process_Execution_Completed: []`); `route_fractal_core.rs` (`purge_after` + rama `subscribers.is_empty`); `kalma2-bridge` `find_pec_by_correlation` / `build_status_body` (404 si domain∅ ∧ pec∅); `cumulo.paths.json` `eda_instance.proofs`; precedente dual-signal PBI-044 / `kalma2-event-bus-integration` L1–L3; `event-bus-audit` (cobertura ECST/staleness vigente — O2 añade cruce catálogo↔registro).

`correlation_id` de fase: vacío en inputs (incidente fuente `e273713c-…`).

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` |
| `feature_name` | `kaizen-pec-subscribers-circuit-audit` |
| Rama | `feat/kaizen-pec-subscribers-circuit-audit` |
| `persist_ref` | `docs/features/kaizen-pec-subscribers-circuit-audit` |
| `document_id` | `PBI-KAIZEN-PEC-SUBSCRIBERS-CIRCUIT-AUDIT` |
| Fase | Estabilización de Requisitos (Mayeuta) |
| Intención estable | Cerrar el agujero de información: PEC con fan-out real + proyección durable post-purge para `GET /api/status`; automatizar detección de circuitos ciegos en el bus |

---

## D1 — Hechos (evidencia, no hipótesis)

| Afirmación | Hecho |
|------------|-------|
| Registro orquestación PEC | `"Process_Execution_Completed": []` en SSOT Cúmulo |
| Router orquestación | `purge_after=true`; array vacío → consenso vacuoso → `safe_remove_path` |
| Dual-signal Kalma2 | Asume PEC recuperable por `correlation_id ≡ event_id` del `Kalma2_Process_Requested` (L3) |
| Status bridge | Lee domain + PEC en `eda_fractal.orchestration`; sin ambos → HTTP 404 |
| Incidente 2026-08-15 | Dominio y PEC purgados; UI timeout 120s; status 404 |
| Telegram solo | Notifica humano; **no** impide unlink del padre → **no** cierra 404 WUI |
| Radamanto ↔ PEC | Desacoplado; stats multi-fase vía telemetría atómica — Faro, no O1 |

---

## D2 — Triaje de incongruencias

| ID | Afirmación / tentación | Hecho / norma | Laudo Mayeuta |
|----|------------------------|---------------|---------------|
| **I1** | Suscribir `kalma2-bridge` a PEC | Ceguera espacial: bridge **lee**; nervio **entrega** | **Filtro C** — prohibido |
| **I2** | Inventar segundo evento de cierre | PEC ya es peaje termodinámico de cierre | Un solo cierre: `Process_Execution_Completed` |
| **I3** | Telegram = cierre O1 | Purge post-`all_ok` borra el JSON que el bridge lee | Telegram = S1 (humano). Cierre 404 = S2 **XOR** S3 |
| **I4** | Editar a mano JSON de suscripciones | SSOT Cúmulo; genoma vía `entity-manager` / forja del proceso | Mutación solo vía cadena autorizada |
| **I5** | Congelar tabla semilla §2 PBI como spec | Semilla es snapshot; O2 **automatiza** el cruce | Tabla = evidencia inicial, no contrato congelado |
| **I6** | Absorber SSE progreso / PPR #174 / reabrir PBI-044 | PBIs adyacentes vivos o cerrados | **Fuera de alcance** (D5) |

---

## D3 — Decisiones de estabilización (laudos)

| ID | Decisión |
|----|----------|
| **L-O1-S1** | Suscriptor Telegram (Argos / `send-telegram-notification`): terminal `success`/`failed` + `process_name` + `correlation_id`. Obligatorio salvo exención Dedalo documentada. |
| **L-O1-XOR** | Cierre 404 WUI: **S2** testigo durable (proyección indexada por `correlation_id`, locus candidato `eda_instance.proofs` o equivalente Cúmulo laudo Dedalo) **XOR** **S3** política `purge_after`/archivo para PEC. **No ambas.** Dedalo elige en `spec.md`. |
| **L-O1-STATUS** | Tras route+purge del padre, `GET /api/status?event_id=<cid>` proyecta `completed`\|`failed`\|`initialized`\|`awaiting_agents` — no 404 persistente ni timeout 120s ciego. |
| **L-CID** | Invariante vigente: `correlation_id ≡ event_id` del `Kalma2_Process_Requested`. |
| **L-O2** | Extender `event-bus-audit` (o wrapper laudo) con hallazgos: `EMPTY_SUBSCRIBERS`, `FAMILY_MISMATCH`, `ORPHAN_REGISTRY_KEY`, `PURGE_BLACKHOLE`. No sustituye ECST/staleness. |
| **L-O2-THRESH** | `PURGE_BLACKHOLE` o `EMPTY_SUBSCRIBERS` en orchestration → `Kaizen_Alert_Required` (no `delivery_state: failed` del audit). |
| **L-O2-H2H5** | H2–H5 (`PullRequest_Audited` vacío, `Telemetry_Compliance_Breached` vacío, `Local_QA_Requested` familia cruzada, `CapabilityDi_Requested` huérfana) = hallazgos accionables de auditoría en v1; cableado de suscriptores **opcional**, no gate O1. |
| **L-GENOME** | Mutación de `events/`, tool `event-bus-audit`, `route-orchestration` (si S3) solo vía `entity-manager` / proceso `feature`. JSON suscripciones = SSOT Cúmulo. |
| **L-DOC** | Cascada `features-documentation-pattern`; cierre = un PR + `validacion.md` APTO + PBI en `docs/todos/done/` en la misma rama. |

---

## D4 — Matriz de aceptación (producto)

| AC | Enunciado |
|----|-----------|
| **AC-O1-FANOUT** | `Process_Execution_Completed` tiene ≥1 suscriptor indexado; fan-out observable en testigos. |
| **AC-O1-STATUS** | Post route+purge, `GET /api/status?event_id=<cid>` ≠ 404 ciego; proyecta estado de ciclo/terminal. |
| **AC-O1-TG** | Telegram (o exención Dedalo) notifica cierre PEC con `process_name` + `correlation_id`. |
| **AC-O1-XOR** | Existe exactamente una de: testigo durable legible por status **o** política no-unlink/archivo PEC. |
| **AC-O2-CODES** | `event-bus-audit` (o proceso laudo) emite los cuatro códigos; `PURGE_BLACKHOLE` / empty orchestration alertan vía Kaizen. |
| **AC-DOC** | `validacion.md` APTO + `pbi_archived: true` + PBI en `done/` en el PR. |

---

## D5 — Fuera de alcance

- Canal SSE / telemetría de progreso (PBI OPERATIVO Kalma2 pendiente).
- Rehabilitación `pull-request-review` / umbrales Radamanto (PPR #174+#177).
- Reabrir PBI-044 / dual-signal (ya laudo; este Kaizen cierra residual de purge).
- Convertir bridge en emisor o suscriptor del bus.
- IOTA o Radamanto-on-PEC.
- Telegram en `Kalma2_Process_Requested` (ruido; basta cierre PEC).

---

## D6 — Handoff Dedalo

1. Elegir **S2 XOR S3** (L-O1-XOR); definir locus del testigo o cambio de política purge/archivo.
2. Cablear S1 Telegram en registro orquestación vía forja autorizada.
3. Extender `event-bus-audit` con cruce catálogo↔tres registros; umbral L-O2-THRESH.
4. Alinear `project_status` / lectura bridge al testigo si S2 (sin convertir bridge en suscriptor).
5. Consumir este transcript + cuerpo de `objectives.md` como `refined_requirements`.
6. Prohibido diseño que viole I1–I4 o D5.

---

## D7 — Veredicto

**ok** — requisitos estabilizados; incongruencias I1–I6 laudadás; handoff Dedalo con O1/O2 acotados y XOR explícito.
