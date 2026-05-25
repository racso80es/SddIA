---
feature_name: kaizen-alert-required-eda-v2
created: "2026-05-25"
process: feature
purpose: EDA v2 — evento Kaizen_Alert_Required, poda puente síncrono DIA y despertar reactivo Cúmulo
---

# Clarificación — Kaizen_Alert_Required (EDA v2)

Transcript de decisiones (2026-05-25).

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 |
| Rama | `feat/kaizen-alert-required-eda-v2` |
| `persist_ref` | `docs/features/kaizen-alert-required-eda-v2` |
| PBI | `docs/todos/pending/kaizen-alert-required-eda-v2.md` (`PBI-KAIZEN-ALERT-REQUIRED-EDA-V2`) |
| Upstream | PR #46 `norma-paridad-documental` (mergeado 2026-05-25) — sensor DIA + puente lab v1 |
| Dependencia dura | ✅ Satisfecha — `audit-doc-parity.py` y cápsulas triaje en `main` |

---

## D2 — Triaje de bloqueos (pre-planificación)

| Área | Estado | Veredicto |
|------|--------|-----------|
| Sensor `audit-doc-parity.py` | ✅ En `main` (PR #46) | **Sin bloqueo** — sin cambios en sensor |
| Puente síncrono v1 (`_dia_audit_hash`, `capsule_pr_review_kaizen` DIA) | ✅ Presente en `execute_process_capsules.py` | **Objetivo de poda** — deuda explícita a extirpar |
| Evento `Kaizen_Alert_Required` | ❌ No existe | **Alcance H1** — forjar ECST |
| Suscripción en `event-subscriptions.json` | ❌ Ausente | **Alcance H2** |
| Handler `write_pending_event` / patrón emit PR | ✅ En cápsulas y `execute-action.py` | **Sin bloqueo** — reutilizar patrón deposit |
| `route-domain-event` + `event-watcher` | ✅ Operativos (Ola C v3) | **Sin bloqueo** — extender despacho |
| `materialize-fracture-pbi` (Cúmulo) | ✅ Referencia idempotencia | **Patrón M4** — nueva acción o extensión |
| `pull-request-review.md` § DIA-3 | ⚠️ Menciona deuda EDA v2 | **Actualizar** en H4/H5 |
| `Argos_Eda_Emision` stub | ⚠️ Deuda separada | **Fuera de alcance** — solo referencia §9 PBI |

**Conclusión:** no hay bloqueos estructurales. PR #46 mergeado desbloquea implementación inmediata.

---

## D3 — Principio rector (ceguera espacial)

| Actor | Mandato |
|-------|---------|
| **Aduana** (`pull-request-review`) | Invoca sensor → si `alert_required`, deposita envelope ECST en `eda_bus.pending` → **desentendimiento total** |
| **Cúmulo** (suscriptor único) | Consume `Kaizen_Alert_Required` → materializa `PENDING_AUDIT_DOC_{hash8}.md` en `docs/todos/pending/` |

Ruta bandeja runtime: `eda_bus.pending` → `./.events/pending/` (`SddIA/core/cumulo.paths.json`).

> `.SddIA/events/` alberga customización ECST local; **no** sustituye la bandeja del bus.

---

## D4 — Contrato Chispazo ECST (H1)

| Pregunta | Decisión |
|----------|----------|
| `event_type` | `Kaizen_Alert_Required` |
| Archivo ECST | `SddIA/events/kaizen-alert-required.md` + fila en `events/index.md` |
| Payload REQUIRED | `review_id`, `alert_justification`, `implicated_files` |
| Payload OPTIONAL | `persist_ref`, `pr_branch`, `alert_kind` (default `doc_parity`), `impacts_doc` |
| Payload FORBIDDEN | Rutas `.tmp/audit-doc-parity-*.json`, invocaciones anidadas a agentes, diff completo |
| Emisores autorizados | Proceso `pull-request-review` (post-sensor); acción `emit-kaizen-alert-required-event` (opcional) |
| Fan-out | **Un solo suscriptor:** `agent:cumulo`. Prohibido Mayeuta/Argos en v1 |

### Hash idempotente (M4)

```text
hash8 = SHA256(review_id + sorted(implicated_files))[:8]
```

Coherente con `materialize-fracture-pbi`; distinto del hash v1 `(persist_ref + monitored_hits)`.

---

## D5 — Poda puente síncrono (H3 + H4)

| Bloque | Acción |
|--------|--------|
| `_dia_audit_hash` | Eliminar |
| `_invoke_dia_audit` — append `kaizen_items` / `state["dia_audit"]` | Sustituir por emisión ECST |
| `capsule_pr_review_kaizen` — rama DIA / `PENDING_AUDIT_DOC_*` | Eliminar escritura directa |
| Fase Cosecha Kaizen — side-effect DIA | Retirar; mantener Kaizen genérico no documental si aplica |

Flujo Aduana post-poda:

1. Invocar `audit-doc-parity.py` (sin cambios).
2. Si `alert_required: true`, forjar envelope y escribir en `eda_bus.pending`.
3. **Fin** — no invocar Cúmulo, no escribir `docs/todos/`, no propagar DIA a Cosecha Kaizen.

---

## D6 — Despertar ontológico Cúmulo (H5 + H6)

| Mandato | Detalle |
|---------|---------|
| M1 Materializar | TODO en `docs/todos/pending/` |
| M2 Nomenclatura | `PENDING_AUDIT_DOC_{hash8}.md` |
| M3 Contenido | Tabla `review_id`, `alert_justification`, `implicated_files`, `persist_ref`, checklist DIA |
| M4 Idempotencia | No duplicar si mismo hash |
| M5 No bloqueo | Materialización **no** altera `delivery_state` aduana |

Artefactos:

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/agents/cumulo.md` | § reactivo EDA — suscripción `Kaizen_Alert_Required` |
| `SddIA/agents/cumulo.instructions.json` | Regla táctica machine-readable (si SSOT existe) |
| Handler lab / `route-domain-event` | Despacho Cúmulo (nueva acción o extensión patrón fracture) |

---

## D7 — Relación `Argos_Eda_Emision`

| Pregunta | Laudo |
|----------|-------|
| ¿Argos emite `Kaizen_Alert_Required`? | **No** — emisor es Aduana post-sensor DIA |
| ¿Cierra `pending_argos_eda_emission`? | **No** — Kaizen separado (DLT merge) |
| Acción en este PBI | Referencia en `related` únicamente |

---

## D8 — Validación empírica (preview)

| Paso | Acción |
|------|--------|
| 1 | Simular PR con diff monitorizado e `impacts_doc: false` |
| 2 | Aduana lab → JSON en `.events/pending/` con `event_type: Kaizen_Alert_Required` |
| 3 | `event-watcher.py --once` → Cúmulo materializa `PENDING_AUDIT_DOC_*.md` |
| 4 | `verdict: aprobado`, `delivery_state: success` **sin** escritura síncrona previa en `docs/todos/` |
| 5 | Grep: cero lógica DIA en `capsule_pr_review_kaizen` |
| 6 | `verify-process-integrity` sin regresión |

---

## D9 — Orquestación implementación (preview plan)

| Hito | Entrega |
|------|---------|
| H1 | ECST `kaizen-alert-required.md` + índice events |
| H2 | Suscripción única Cúmulo en `event-subscriptions.json` |
| H3 | Emisión desde `_invoke_dia_audit` / cápsula triaje técnico |
| H4 | Poda puente síncrono + genoma `pull-request-review.md` |
| H5 | Genoma Cúmulo + handler materialización |
| H6 | Smoke E2E en `execution.md` |
| H7 | `validacion.md` APTO + PBI en `done/` (un PR) |
