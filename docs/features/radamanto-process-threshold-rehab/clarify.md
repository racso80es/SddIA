---
feature_name: radamanto-process-threshold-rehab
created: "2026-08-16"
purpose: Estabilización PBI-PPR-174-177 — umbrales Radamanto process + rehabilitación revoked_entities (olas PPR/DCC)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/radamanto-process-threshold-rehab
persist_ref: docs/features/radamanto-process-threshold-rehab
pbi_ref: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
document_id: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
uuid: ba900e95-1a47-4185-b86c-bc7a251b4fe6
olas:
  - ola-1
  - ola-2
---

# Clarificación — radamanto-process-threshold-rehab

Transcript Mayeuta. Estabiliza el **qué** y el **por qué** antes de blueprint Dedalo. Sin diseño de cápsulas, umbrales numéricos exactos ni YAML de proceso.

## D0 — Semilla y evidencia

| Vector | Hecho |
|--------|--------|
| PBI canónico | `docs/todos/pending/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md` (`document_id: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS`; `uuid: ba900e95-…`; `status: abierto`) |
| Ciclo | `refactorization` · rama `refactor/radamanto-process-threshold-rehab` · un `persist_ref` · un PR |
| Ola 1 satélite | `PBI-PPR-174-REVOKED-REGISTRY` · entidad `pull-request-review` · seed PPR #174 · check `RBAC_PROCESS_REGISTRY` · `status: absorbed` / `dispatch: false` |
| Ola 2 satélite | `PBI-PPR-177-DCC-REVOKED-REGISTRY` · entidad `delivery-close-cycle` · seed PPR #177 · check `RBAC_EMITTER_NOT_REVOKED` · `status: absorbed` / `dispatch: false` |
| Fractura | Radamanto aplica `success_rate_min` de herramienta atómica a macro-procesos multi-fase; instancia etiqueta `entity_type: tool` a procesos → re-revocación post-rehab histórica |
| Exención vigente | `LATENCY_THRESHOLD_EXEMPT` ⊇ `pull-request-review` (solo latency). Vector actual: `success_rate_below_threshold` |
| Umbrales SSOT | `SddIA/agents/radamanto.thresholds.json` — `success_rate_min: 0.85` plano (sin tabla por tipo) |
| Precedente | #124/#125 latency PPR; #136 signer DCC (cerrados; no reabrir) |
| Fuentes auditoras | `docs/features/kalma2-phase-barrier-timeout-persist/validacion.md`; `docs/fixes/centinelas-fracture-ola-20260812/validacion.md` |
| `correlation_id` runtime | vacío en inputs de fase |

### Estado empírico (corte estabilización 2026-08-16)

| Entidad | `revoked_entities` | Radamanto (raíz stats) | Nota |
|---------|--------------------|------------------------|------|
| `pull-request-review` | **Ausente** de `revoked` | `status: healthy` · `recovery_attempts: 2` · `rehab_laudo: PBI-PPR-124+125-REVOKED-REGISTRY` | Sightings PPR #174–#178 citan re-revocación `since 2026-08-15T08:40:55Z`; corte actual **no** retiene la clave. Mandato ola 1 = verificar + anti-recurrencia, no inventar rehabilitación ya hecha. |
| `delivery-close-cycle` | **Presente** · `entity_type: tool` · `reason: success_rate_below_threshold` · `since: 2026-08-16T16:11:08Z` | `status: pending_redemption` · `recovery_attempts: 3` · `degraded_at: 2026-08-16T16:11:08Z` | ≠ incidente #136 (`abrupt_success_rate_drop` cerrado). Firmante ECST `Vertice_Biologico_Relay` liquidado en #136. |
| Laterales | `permanent.feature`; `revoked.bug-fix`; `revoked.emit-pr-audited-event` | fuera de alcance | Prohibido tocar en este ciclo |

## D1 — Misión (qué / por qué)

| Decisión | Laudo |
|----------|--------|
| Objetivo | Unificar rehabilitación / anti-recurrencia de `pull-request-review` (ola 1) y `delivery-close-cycle` (ola 2) bajo jurisprudencia única: ontología `process`, umbrales Radamanto diferenciados por tipo, fail-soft por ola. |
| Por qué ahora | Exención latency no cubre `success_rate_below_threshold`. Sin umbral `process` multi-fase, la rehab puntual reincide (evidencia histórica #124/#125/#136 + sightings #174–#178). |
| Efecto observable | Ambas entidades fuera de `revoked` (o ya ausentes y no re-entran); `entity_type: process` cuando el registro las materialice; checks aduana APTO; política umbrales versionada por tipo. |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un `persist_ref`, un PR. Satélites ola **no** despachan `bug-fix`/`feature` propios; archivan a `done/` con el canónico. |
| **L-LAUDO-CERBERO** | Rehabilitación (no permanente) para ambas olas. Ola 2: retirar `delivery-close-cycle` de `revoked`. Ola 1: si ausente, **verificar** coherencia registry/stats y dejar evidencia; si reaparece antes del merge, misma rehabilitación. |
| **L-ONTOLOGY** | `process` ≠ `tool`. Cualquier alta/rehab en Cerbero instancia debe declarar `entity_type: process`. Corregir etiquetado `tool` residual donde aplique a estas dos entidades. |
| **L-THRESHOLDS** | Política Radamanto con umbrales diferenciados por tipo: `process` multi-fase con mayor tolerancia de `success_rate` que `tool` atómico. Dedalo fija números/esquema en `spec.md`; Mayeuta fija el **requisito** anti-recurrencia del vector `success_rate_below_threshold`. No basta solo `LATENCY_THRESHOLD_EXEMPT`. |
| **L-REDEEM** | Reset/redención stats (`pending_redemption` → `healthy`) sin reabrir el mismo vector `success_rate` por misclasificación tipológica. |
| **L-FAILSOFT-OLA1** | Kintsugi PPR: fricción de sub-fase (API externa, lectura puntual) no colapsa la ejecución lineal; éxito parcial registrable. |
| **L-FAILSOFT-OLA2** | Kintsugi DCC: timeout no crítico en `telemetry_receipt` / validación de repo no impide emitir `PullRequest_Presented` firmado si commit/push cruzó umbral físico. |
| **L-SCOPE-HARD** | Prohibido rehabilitar `feature` / `bug-fix` / `emit-pr-audited-event` en este PR. |
| **L-OUT** | Fuera: residual Kalma2 Shell/`git-manager`; merge/handoff `accept-pr` de PR #174/#177 históricos; faros Kaizen (troceo EDA PPR; aislar `RBAC_EMITTER_NOT_REVOKED` en centinela EDA) — Filtro C, no implementar. |
| **L-GENOME** | `revoked_entities.json` y `radamanto/stats.json` = instancia (`.SddIA/`), no genoma indexado. Mutaciones a umbrales/core Radamanto vía cadena autorizada (`entity-manager` / forja gobernada) cuando Dedalo lo exija. |
| **L-DOC** | Cascada `features-documentation-pattern` + `validacion.md` APTO + `pbi_archived: true` + PBI canónico **y** satélites ola en `docs/todos/done/` en la rama del PR. |

## D3 — Matriz de aceptación (producto)

| AC | Enunciado |
|----|-----------|
| **AC-OLA1** | `pull-request-review` ausente de `revoked`; tipología `process` coherente; `RBAC_PROCESS_REGISTRY: APTO` en aduana PPR posterior. |
| **AC-OLA2** | `delivery-close-cycle` ausente de `revoked`; tipología `process` coherente; `RBAC_EMITTER_NOT_REVOKED: APTO` en aduana PPR posterior. |
| **AC-THRESH** | Umbrales Radamanto versionados por tipo (`process` multi-fase ≠ `tool`); anti-recurrencia del vector `success_rate_below_threshold` documentada y aplicada. |
| **AC-FAILSOFT** | Fail-soft ola 1 (PPR interno) y ola 2 (handoff DCC) materializados según L-FAILSOFT-*. |
| **AC-DOC** | Cascada completa bajo `persist_ref`; PBI canónico + satélites en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## D4 — Handoff Dedalo

1. Auditar causa `success_rate_below_threshold` en telemetría/Radamanto para ambas entidades; proponer esquema de umbrales por `entity_type` (números, locus: `radamanto.thresholds.json` y/o `radamanto_batch_core`).
2. Plan de rehab instancia: retirar DCC de `revoked`; alinear tipología `process`; redención stats sin reabrir vector.
3. Touchpoints fail-soft por ola (PPR sub-fase; DCC handoff/`telemetry_receipt`) — mínimo necesario.
4. `spec.md` + `plan.md`: pruebas de aduana (`RBAC_PROCESS_REGISTRY`, `RBAC_EMITTER_NOT_REVOKED`), sin expandir a `feature`/`bug-fix`/faros Kaizen.
5. Consumir este transcript + cuerpo de `objectives.md` como `refined_requirements`.
