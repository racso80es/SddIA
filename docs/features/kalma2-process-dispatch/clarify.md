---
feature_name: kalma2-process-dispatch
created: "2026-07-20"
process: feature
purpose: Estabilización Mayeuta del PBI de despacho Kalma2_Process_Requested (dead-letter TQM)
---

# Clarificación — kalma2-process-dispatch

Transcript Mayeuta (2026-07-20). Semilla v0 «Por Refinar» (prompt Raw Kernel sin frontmatter) → PBI v1.1.0 **Refinado**.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.0 (orden Racso; filename `[FIX]` se conserva como etiqueta de fricción) |
| `feature_name` | `kalma2-process-dispatch` |
| Rama | `feat/kalma2-process-dispatch` |
| `persist_ref` | `docs/features/kalma2-process-dispatch` |
| `document_id` | `PBI-KALMA2-PROCESS-DISPATCH` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` → `execution_id` `79c09578-e15d-42cd-b6fa-0c2b542247ca` |
| Fase actual | Estabilización Mayeuta (esta sesión) |
| Dependencias | `kalma2-mayeuta-llm-router` (APTO) · `kalma2-event-bus-integration` (APTO) |
| Evidencia | `.events/dead-letter/a7725b42-2661-4bc5-9795-c69d8ca2ab5c.json` |

---

## D1 — Título vs síntoma (incongruencia corregida)

| Borrador v0 | Hecho / evidencia | Decisión |
|-------------|-------------------|----------|
| «interacción con front kalma2» | UI lazo ya cerrado en `kalma2-event-bus-integration` | **No** reabrir poll/status/degraded |
| Prompt «parche físico ONLY CODE» | Falta contrato de consumo post-emisión | Feature de cierre de despacho, no bisturí ciego |
| Culpa implícita del front/bridge | Payload ECST válido; bridge no escribe bus | Front **fuera de sospecha primaria** |

**Toll:** el síntoma operativo es dead-letter del suscriptor ejecutor, no interacción HTTP del cliente.

---

## D2 — Topología ya entregada (no re-forjar)

| Capacidad | Estado | Implicación |
|-----------|--------|-------------|
| Emisión `Kalma2_Process_Requested` + allowlist | ✅ router | ECST `{process, raw_text}` OK |
| Suscriptor fijo `task-queue-manager` | ✅ subscriptions | No sustituir por `{payload.process}` sin derogar laudo |
| Rama dispatcher Kalma2 (`task_text←raw_text`) | ✅ `route_domain_core` | Emisión→inputs parciales ya existen |
| Lazo UI status/PEC | ✅ event-bus-integration | Fuera de alcance |
| Consumo TQM de `{process, task_text, pbi_ref}` | ❌ | **Brecha real de este PBI** (D3) |

---

## D3 — Contrato TQM vs inputs Kalma2 (núcleo)

| Actor | Espera / produce | Gap |
|-------|------------------|-----|
| Evento dominio | REQUIRED `process`, `raw_text`; OPTIONAL `pbi_ref` | Cumplido en a7725b42 (sin `pbi_ref`) |
| `dispatch_subscriber` Kalma2 | → TQM con `correlation_id`, `process`, `task_text`, `pbi_ref?` | Implementado |
| `task-queue-manager.md` | Input `tasks_path`; fases Triaje→Despacho a feature/bug-fix/refactorization | **Sin handler nativo** que lea el paquete Kalma2 |
| Resultado a7725b42 | `tekton.task-queue-manager: failed` | Colapso empírico |

**Laudo Mayeuta (qué):** el sistema nervioso acepta el evento y falla al **activar** el ciclo solicitado. El requisito estable es cerrar ese eslabón.

---

## D4 — Reencuadre Vía A / Vía B (semilla v0)

| Semilla v0 | Reencuadre |
|------------|------------|
| Vía A: mapear en bridge/app.js `raw_text`/`process` | **Mal enmarcada.** Emisión ya mapea. Queda solo matiz A′: `pbi_ref` con paths que contienen espacios |
| Vía B: parser TQM si `event_type == Kalma2_Process_Requested` | **Dirección correcta** si = contrato de consumo / despacho del ciclo (no «parser de event_type» en TQM: el event_type ya lo filtra el dispatcher) |
| (nueva) Vía C: despachar `payload.process` directo | Requiere **derogar** laudo suscriptor fijo TQM (O14/P1) |

Handoff Dedalo: elegir B′ vs C; A′ es complemento, no sustituto.

---

## D5 — Matiz extracción `pbi_ref`

Prompt evidencia:

`… docs/todos/pending/[FIX] telegram-watcher — fractura sistémica (e6cbecb9032c).md`

`extract_pbi_ref` usa `split_whitespace` + token que `ends_with(".md")` → el path fragmentado **nunca** cuadra. Por eso el dead-letter carece de `pbi_ref` pese a mencionarlo.

Incluir en alcance como **matiz de robustez** (no causa raíz única del `failed`).

---

## D6 — IOTA fallido en el mismo sobre

`cumulo.iota-immutable-publisher: failed` co-ocurre. Canal DLT ≠ despacho de proceso.

| Opción | Efecto |
|--------|--------|
| **Fuera** (default Mayeuta) | Este PBI solo cierra TQM/despacho |
| Dentro | Amplía a anclaje IOTA; riesgo de mezclar dos fracturas |

**Pendiente laudo Racso** si debe entrar en el mismo PR.

---

## D7 — Higiene documental

| Ítem | Acción |
|------|--------|
| Frontmatter PBI | Añadido (`document_id`, `uuid`, `feature_ref`, `evidence_event_id`) |
| Versión | `1.0.0` implícita semilla → `1.1.0` Refinado |
| Prompt Raw Kernel | Sustituido por PBI operativo estructurado |
| Nombre archivo | Conservado (`[FIX] interacción con front kalma2.md`) para no romper refs; título interno aclara despacho |

---

## Preguntas abiertas (Racso) — **cerradas 2026-07-20**

| # | Pregunta | Laudo Racso |
|---|----------|-------------|
| Q1 | ¿Incluir fallo IOTA del mismo `event_id`? | **No** — deuda aparte |
| Q2 | ¿Renombrar archivo PBI? | **No** en esta fase |
| Q3 | B′ vs C | **B′** — TQM consume y despacha el ciclo hijo |

---

## Handoff Dedalo

Consumir cuerpo de `objectives.md` como `refined_requirements`. Laudo Q3 = B′; diseñar handler nativo TQM + matiz A′ `pbi_ref` sin violar ceguera espacial del bridge ni C2.
