---
feature_name: bug-fix-revoked-registry-rehab-ppr194
created: "2026-08-27"
purpose: Estabilización Mayeuta — PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY (rehab bug-fix + ontología tool→process + cascada documental)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/bug-fix-revoked-registry-rehab-ppr194
persist_ref: docs/features/bug-fix-revoked-registry-rehab-ppr194
pbi_ref: docs/todos/pending/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194).md
document_id: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY
uuid: 8a4b0d3f-5c2e-4f9b-8d6a-7e8f9a0b1c2d
source_correlation_id: "59606407-eed3-4da8-ac13-3cf6205b2147"
source_pr_url: https://github.com/racso80es/SddIA/pull/194
feature_ref: docs/fixes/bundle-consumer-telegram-gateway
incident_ref: "REVOKED_ENTITY_ALERT_BUG_FIX — bug-fix ∈ revoked as tool (abrupt_success_rate_drop since 2026-08-16T16:09:32Z); alerta F4/F5 PPR #194 sin PBI canónico previo"
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
olas:
  - A1
---

# Clarificación — bug-fix-revoked-registry-rehab-ppr194

Transcript Mayeuta. Estabiliza el **qué** y el **por qué**. Sin diseño de cápsulas, YAML de proceso ni mutación de genoma.

## D0 — Semilla y evidencia

| Vector | Hecho |
|--------|--------|
| PBI canónico | `docs/todos/pending/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194).md` (`document_id: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY`; `uuid: 8a4b0d3f-…`; `status: pending`) |
| Ciclo | `refactorization` · rama `refactor/bug-fix-revoked-registry-rehab-ppr194` · un `persist_ref` · un PR |
| Semilla operador | Rehabilitar `bug-fix` en Cerbero/Radamanto (PPR #194); corregir `entity_type` `tool`→`process` (jurisprudencia #174); cascada documental |
| Padre | #174+#177 (`persist_ref`: `docs/features/radamanto-process-threshold-rehab`) — umbral `process: 0.70` + `L-TYPE-RESOLVE` + ontología `process`≠`tool`; **prohibió** rehab de `bug-fix` (`L-SCOPE-HARD`) |
| Hermano lifecycle | #185 A3 poda hollow ya extensible a `bug-fix` **sin** rehabilitarlo; #186 rehab `refactorization` (re-revocado lateral hoy) |
| Check origen | `REVOKED_ENTITY_ALERT_BUG_FIX` (F4/F5 Cerbero/Argos · alerta no bloqueante) + FS Cosecha #194 |
| Sighting | PPR #194 · CID `59606407-eed3-4da8-ac13-3cf6205b2147` · `persist_ref` `docs/fixes/bundle-consumer-telegram-gateway` · emisor ECST `delivery-close-cycle` ∉ revoked |
| SSOT proceso | `SddIA/library/codexes/codex-software-engineering/process/bug-fix.md` — entidad **process** (no tool) |
| `correlation_id` runtime de esta fase | vacío en inputs |

### Estado empírico (corte estabilización 2026-08-27 · verificado en instancia)

| Clave | Cerbero | Radamanto | Nota |
|-------|---------|-----------|------|
| `bug-fix` (raíz) | **`revoked.bug-fix`** · `entity_type: tool` · `reason: abrupt_success_rate_drop` · `since: 2026-08-16T16:09:32Z` | **Clave ausente** en `stats.json` (sin bucket raíz) | **Vector activo.** Ausente de `permanent`. Ontología Cerbero **errada** (process etiquetado tool). |
| Laterales | `revoked.accept-pr` (re-revocado @ `2026-08-27T11:31:15Z`), `revoked.refactorization`, `revoked.emit-pr-audited-event` | fuera de alcance | Prohibido rehabilitar este ciclo |

Dictamen (vinculante):

1. `bug-fix` es proceso multi-fase en catálogo SSOT; `entity_type: tool` en Cerbero es **entropía ontológica** pre/post #174 (misma fractura que #174+#177).
2. Umbrales 1.1.0 vigentes: `process: 0.70` / `tool: 0.85`. Etiqueta `tool` fuerza peaje atómico sobre macro-proceso → vector de re-muerte por tipología.
3. Bucket Radamanto ausente: no hay ventana `samples` que podar; A1 debe **materializar** stats raíz sanos con ontología correcta (no inventar historial KO).
4. Semilla **no** aporta vector motor nuevo (payload, handoff, EDA). A3 hollow #185 ya cubre peaje lifecycle de `bug-fix`. Prohibido inventar olas A2/A3 sin evidencia.

## D1 — Misión (qué / por qué)

| Decisión | Laudo |
|----------|--------|
| Objetivo | Rehabilitar `bug-fix` en Cerbero/Radamanto **y** sellar ontología `entity_type: process` (corrección `tool`→`process`) bajo jurisprudencia #174, con cascada documental single-PR. |
| Por qué ahora | Deuda lateral PPR #194 sin PBI canónico hasta Cosecha; rehab previa de hermanos dejó `bug-fix` explícitamente fuera (`L-SCOPE-HARD` #174; #185). Mientras ∈`revoked`, despacho soberano de fixes queda peajeado. |
| Efecto observable | `bug-fix` ∉ `revoked` ni `permanent`; stats raíz presentes y `healthy` con `entity_type: process`, `recovery_attempts: 0`, laudo+timestamp; cascada bajo `persist_ref` + PBI en `done/`. |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un `persist_ref`, un PR. Prohibido despachar `bug-fix` satélite (meta: este ciclo **rehab** el proceso, no lo usa como carrier de otro FIX). |
| **L-WAVES** | Una ola innegociable: **A1** Yunque Rúnico (Cerbero + Radamanto + ontología). Semilla no justifica A2/A3 motor nuevos; hollow #185 y umbrales #174 se **reutilizan**, no se reabren. |
| **L-REHAB-INST** | A1 = instancia `.SddIA/` (no genoma). Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff del PR. |
| **L-CERBERO** | Eliminar nodo `revoked.bug-fix` por completo. Verificar `permanent.bug-fix` ausente. Cerbero **no** tiene estado `healthy`. |
| **L-STATS** | Materializar/reset **solo** del bucket raíz `bug-fix`. No inventar fósiles `entities.bug-fix` / `process:bug-fix` si no existen. |
| **L-RESET-ABS** | Absoluto: `status: healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY`; `rehabilitated_at` ISO de intervención A1; `samples: []` (o ≤3 OK runtime reales si aparecieran antes del reset). |
| **L-ONTOLOGY** | **Corregir** a `entity_type: process` en stats rehab (y en cualquier reescritura Cerbero si Dedalo exige sello temporal). Prohibido conservar o reintroducir `tool`. Jurisprudencia #174 `L-ONTOLOGY` / `L-TYPE-RESOLVE`. |
| **L-TYPE-VERIFY** | Dedalo verifica (no Mayeuta diseña): `entity_type_from_id` / `L-TYPE-RESOLVE` sigue resolviendo `bug-fix`→`process` vía `resolve_process_path`. Si el motor aún estampa `tool`, **escalar** a ola A2 motor en Diseño — no asumir regresión sin evidencia. |
| **L-THRESH** | `radamanto.thresholds.json` v1.1.0 **intacto**. No reabrir `success_rate_min` ni `abrupt_drop_min_samples`. |
| **L-NO-INVENT-A2** | Prohibido inventar fail-soft / payload / handoff / hollow «por simetría» sin vector empírico en semilla o instancia. |
| **L-OUT** | Fuera: rehab `accept-pr` / `refactorization` / `emit-pr-audited-event`; reabrir umbrales; versionar instancia en el PR; mutar genoma sin DA-4; escribir TODOs bajo `docs/todos/` (jurisdicción Cúmulo/Kaizen). |
| **L-DOC** | Cascada `features-documentation-pattern` + `validacion.md` APTO + `pbi_archived: true` + PBI canónico en `docs/todos/done/` en la rama del PR. |

### Ajustes anti-alucinación (órdenes crudas → laudo)

| Orden cruda | Laudo |
|-------------|-------|
| «pasar bug-fix a healthy en Cerbero» | Rehab = **borrar** `revoked.bug-fix`. `healthy` solo en `stats.json`. |
| «conservar entity_type tool» | **Falso.** Debe quedar `process` (L-ONTOLOGY). |
| «A2/A3 motor obligatorios como accept-pr #194» | No. Semilla distinta; sin vector payload/handoff. |
| «podar samples KO» | Bucket ausente → materializar `samples: []`; no inventar historial. |
| «rehab laterales del mismo PPR #194» | No. Un entidad, un ciclo. |

## D3 — Matriz de aceptación (producto)

| AC | Enunciado |
|----|-----------|
| **AC-A1** | `bug-fix` ∉ `revoked` ni `permanent`; stats raíz presentes y `healthy`; `recovery_attempts: 0`; `rehab_laudo: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY`; `rehabilitated_at`; `samples` vacíos o solo OK runtime; evidencia en `execution.md` (no en diff de instancia). |
| **AC-GIT-CLEAN** | `.SddIA/cerbero/` y `.SddIA/radamanto/` **no** aparecen en el diff del PR. |
| **AC-ONTO** | `entity_type: process` en stats rehab; cero `tool` residual para esta entidad post-A1. |
| **AC-TYPE-VERIFY** | Evidencia Dedalo/Argos: resolución tipológica vigente mapea `bug-fix`→`process` (o A2 motor si falla). |
| **AC-THRESH** | Umbrales 1.1.0 intactos. |
| **AC-DOC** | Cascada bajo `persist_ref`; PBI en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## D4 — Handoff Dedalo

1. Consumir este transcript + cuerpo de `objectives.md` como `refined_requirements`.
2. `spec.md`: procedimiento A1 instancia (borrar `revoked.bug-fix`; materializar stats raíz con L-RESET-ABS + L-ONTOLOGY); verificación L-TYPE-VERIFY; **no** inventar touchpoints motor salvo fallo tipológico demostrado.
3. `plan.md`: un PR (doc + evidencia A1) + git-clean instancia; cierre documental single-PR.
4. Tests/asserts de producto (qué, no cómo): entidad ausente de `revoked`/`permanent`; stats `healthy`+`process`; laterales intactos; umbrales intactos.
5. Prohibido rehab laterales, reabrir umbrales, o castrar peaje hollow #185 ajeno a este alcance.
