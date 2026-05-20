---
feature_name: eda-domain-entities-splus
created: "2026-05-20"
process: feature
purpose: Estabilización de requisitos — EDA universal + Protocolo de Acero S+
---

# Clarificación — EDA Domain Entities S+

Transcript de decisiones (2026-05-20) para cerrar ambigüedades antes de implementación.

---

## D1 — Inicio de feature

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.2.0 |
| Rama | `feat/eda-domain-entities-splus` |
| `persist_ref` | `docs/features/eda-domain-entities-splus` |
| Fuente objetivos | TODO-EDA-DOMAIN-ENTITIES v3.0.0 (Protocolo de Acero) |
| Detener tras | Planificación formalizada (`objectives`, `clarify`, `spec`, `plan`) |

---

## D2 — Pausa táctica (Yunque Rúnico)

| Pregunta | Decisión |
|----------|----------|
| ¿Implementar forges antes de norma S+? | **No.** Fase 0 (contrato ECST + routing + DLT + idempotencia) precede Fase A (6 clases). |
| Motivo | Cerrar solo el hueco create/update deja flancos: propagación local→core, DLT ambiguo, duplicados, atajos manuales recurrentes. |

---

## D3 — Topología fractal (`origin_topology`)

| Pregunta | Decisión |
|----------|----------|
| Campo payload | `origin_topology: "core" \| "local"` — REQUIRED en eventos nuevos |
| Legacy | Default `core` si ausente en instancias pre-existentes |
| Resolución | `semantic_seed.scope` (tool) → `origin_topology`; resto de clases → `core` |
| Fan-out `core` | `sync-entity-index` + DLT + sync laboratorios |
| Fan-out `local` | Solo `.SddIA/**`; **prohibido** mutar índices canónicos `SddIA/` |
| Alcance v1 | Bifurcación física solo en `tool-creator`; otras clases documentadas como core-only hasta extensión futura |

---

## D4 — Mandato DLT (IOTA Rebased)

| Pregunta | Decisión |
|----------|----------|
| ¿Política backfill abierta? | **Revocada.** Mandato normativo, no decisión operativa ad hoc |
| ¿Cuándo anclar? | `Domain_Entity_Created` + `origin_topology=core` + umbral de validación |
| Umbral | UUID v4 válido; `hash_signature_new` con prefijo `sha256:`; sin placeholders (`pending-forge`) |
| Local | Sin anclaje DLT en genoma canónico (default: skip con causa auditable) |
| Excepciones | Solo acta Argos por entidad |
| Suscripción | Ya existe en `event-subscriptions.json`; falta guarda de umbral en runtime |

---

## D5 — Idempotencia (Táctica de Inmunidad)

| Pregunta | Decisión |
|----------|----------|
| Puntos de guarda | Pre-forja, pre-sello, pre-backfill, pre-index-upsert |
| Respuesta idempotente | `success: true`, `idempotent: true`, sin side-effects |
| Gap crítico | Fallo entre forja y sello — pre-sello consulta bus por `entity_uuid` |
| Create duplicado | Mismo nombre + hash distinto → error controlado, no overwrite |

---

## D6 — Aduana Argos permanente

| Pregunta | Decisión |
|----------|----------|
| ¿Script efímero Fase C? | Evoluciona a `audit-entity-eda-coverage.py` permanente |
| Integración | Fase **Aduana EDA genómica** en `delivery-close-cycle` (ampliación de «Impacto SddIA condicional») |
| Veredicto | `pass` → continuar; `block` → PR bloqueado hasta remediación o acta |
| Correlación | Por `entity_uuid`, no solo nombre |
| CI | Refuerzo opcional pre-merge; gate primario en close-cycle |

---

## D7 — Ruido de Sistema

| Pregunta | Decisión |
|----------|----------|
| Definición | Forja directa `.md` + fila `index.md` sin `entity-manager` |
| Backfill Fase C | Liquida pasivo; **no** normaliza el atajo como vía válida |
| Ejemplo | `markdown-table-editor` (Hito 2 PBI-005) |

---

## D8 — Git y commits (ejecución futura)

| Pregunta | Decisión |
|----------|----------|
| Canal Git | **`git-manager`** exclusivamente |
| Commits | Atómicos por fase del plan (F0 → A → B → C) |
| Commit planificación | `docs(feature): planificación EDA domain entities S+` |

---

## D9 — Handoff de creators

| Pregunta | Decisión |
|----------|----------|
| Deuda | Seis creators sin outputs `handoff_*` explícitos |
| Resolución preferida | Ampliar contratos `*-creator.md` (patrón `skill-creator`) antes de Fase A |
| Alternativa temporal | Nota normativa en `entity-manager.md` + garantía lab en forges |

---

## D10 — Orden de ejecución Tekton

```text
Fase 0 (S+ norma/contrato) → Fase A (piloto 6 clases) → Fase B (E2E + Argos) → Fase C (backfill DLT)
```

Piloto incremental en Fase A: **`tool`** primero (cierra síntoma `markdown-table-editor`).

---

## D11 — Suscripciones declarativas por topología (H0.1.6)

| Pregunta | Decisión |
|----------|----------|
| ¿Dónde vive el filtro fan-out? | **Doble capa:** (1) SSOT declarativo en `event-subscriptions.json`; (2) enforcement en watcher / `route-domain-event` |
| Campo suscriptor | `applies_to_origin_topology`: array de `"core"` \| `"local"` |
| Default si ausente | `["core"]` — compatibilidad con entradas legacy sin campo |
| `sync-entity-index` | `["core"]` explícito — nunca muta índices bajo `SddIA/` ante eventos `local` |
| `iota-immutable-publisher` | `["core"]` explícito — sin anclaje DLT para topología local |
| Eventos sin `origin_topology` | Tratar como `core` (legacy) antes de evaluar suscriptores |
| Eventos `PullRequest_*` | Campo ignorado (no aplican); suscriptores actuales sin cambio semántico |
| Índice local v1 | Sin suscriptor `sync-entity-index` para `.SddIA/` en v1; eventos `local` → `delivery_state.cumulo = "skipped-local-index-v1"` si no hay handler |

**Regla de matching:** un suscriptor se despacha solo si `payload.origin_topology` (o default `core`) ∈ `applies_to_origin_topology`.

---

## D12 — Backfill histórico y saturación DLT (H0.2.4 / Fase C)

| Pregunta | Decisión |
|----------|----------|
| ¿Mandato DLT por entidad en backfill masivo? | **No** — riesgo de saturación termodinámica / coste red |
| Circuito operativo (post Fase A) | Mandato DLT **innegociable** por `Domain_Entity_Created` core válido en tiempo real |
| Circuito backfill Fase C | **Separado** del mandato operativo |
| Modo `--emit` retroactivo | **`--skip-dlt` por defecto** — eventos al bus; watcher omite IOTA; `delivery_state.dlt = "skipped-backfill-v1"` |
| Anclaje batch (Propuesta de Acero) | **`--anchor-merkle` obligatorio** para cierre Fase C: un único payload IOTA con Merkle root + manifiesto `{correlation_id, entity_uuid[], hashes[]}` |
| Orden Fase C | C.2a `--emit --skip-dlt` → C.2b **`--anchor-merkle`** (bloqueante; sin digest no hay cierre) |
| `--emit` sin flags | Equivalente a `--skip-dlt` en contexto backfill (`emitter_agent=cumulo-eda-backfill`) |
| `--emit-with-dlt` | Explícito; solo entidades sueltas bajo acta Argos — prohibido en lote masivo |
| Cierre Fase C | Requiere **ambos:** eventos en bus (C.2a) + `transaction_digest` Merkle (C.2b) |
| Evidencia | Acta JSON del lote en `persist_ref` + tx digest Merkle **obligatorio** |

---

## D13 — Cierre Fase C (confirmación operador)

| Pregunta | Decisión |
|----------|----------|
| ¿Merkle opcional u obligatorio al cierre? | **Obligatorio** — la Fase C no se da por cerrada sin `--anchor-merkle` y `transaction_digest` registrado |
| Secuencia | C.2a (bus) → C.2b (IOTA agregado) → evidencia en `validacion.md` |
