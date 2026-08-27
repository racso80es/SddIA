---
feature_name: accept-pr-revoked-registry-rehab-ppr200
created: "2026-08-27"
purpose: Estabilización Mayeuta — PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY (rehab accept-pr post-re-revocación #194 + fail_soft sello PullRequest_Merged post-merge)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr200
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
document_id: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY
uuid: a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c
source_correlation_id: "7c215675-2ad2-436a-9749-ff635c52c8b3"
source_pr_url: https://github.com/racso80es/SddIA/pull/200
feature_ref: docs/features/accept-pr-revoked-registry-rehab-ppr194
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — accept-pr re-revoked post-rehab #194 (abrupt_success_rate_drop since 2026-08-27T11:31:15Z; rehabilitated_at 11:20:00Z; merge 6528d115… @ 11:31:11Z)"
olas:
  - A1
  - A2
---

# Clarificación — accept-pr-revoked-registry-rehab-ppr200

Transcript Mayeuta. Estabiliza el **qué** y el **por qué**. Sin diseño de cápsulas, YAML de proceso ni mutación de genoma.

## D0 — Semilla y evidencia

| Vector | Hecho |
|--------|--------|
| PBI canónico | `docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md` (`document_id: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY`; `uuid: a8f3c1e2-…`; `status: pending`) |
| Ciclo | `refactorization` · rama `refactor/accept-pr-revoked-registry-rehab-ppr200` · un `persist_ref` · un PR |
| Semilla operador | Rehabilitar `accept-pr` en Cerbero/Radamanto tras **re-revocación** post-#194 (PPR #200) **y** cortar re-muerte: **A1** Yunque + **A2** fail_soft sello `PullRequest_Merged` cuando `merge_commit_hash` ya cruzó |
| Padre | #194 done (`persist_ref`: `docs/features/accept-pr-revoked-registry-rehab-ppr194`; merge `6528d115…`) — A1 rehab + A2 payload `delete_branch` + A3 handoff truth; **no** cubrió fail_soft del sello post-merge |
| Hermano jurisprudencia | #187 DCC — adjudicación retroactiva EDA post-umbral físico (`fail_soft` antes del agregador; agregador intacto) |
| Check origen | empírico Cerbero/Radamanto FS + Cosecha Kaizen PPR #200 · F5 `REVOKED_ENTITY_ALERT_ACCEPT_PR` |
| Sighting | PPR #200 · CID `7c215675-2ad2-436a-9749-ff635c52c8b3` · `persist_ref` padre `docs/features/accept-pr-revoked-registry-rehab-ppr194` · `PullRequest_Merged` dead-letter `c24d84a7…` @ `11:31:11Z` (hash alineado `main`) · re-revocación **4s** después |
| Affirm | Cosecha PR #201 · CID `224d877d-…` @ `12:05:00Z` — dedup/affirm #200; Cerbero `accept-pr`∈revoked |
| `correlation_id` runtime de esta fase | vacío en inputs |

### Estado empírico (corte estabilización 2026-08-27 · verificado en instancia)

| Clave | Cerbero | Radamanto | Nota |
|-------|---------|-----------|------|
| `accept-pr` (raíz) | **`revoked.accept-pr`** · `entity_type: process` · `reason: abrupt_success_rate_drop` · `since: 2026-08-27T11:31:15Z` | `status: degraded` · `recovery_attempts: 1` · `degraded_at: 2026-08-27T11:31:15Z` · `rehab_laudo: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY` (residual) · `rehabilitated_at: 2026-08-27T11:20:00Z` · `structure_valid: false` · 4 samples (2 KO / 2 OK · rate 0,50) · `consecutive_success_count: 1` | **Vector activo.** Ausente de `permanent`. ≠ incidente #194 (`since` `2026-08-26T11:42:26Z`, cerrado) |
| Laterales | `revoked.refactorization`, `revoked.emit-pr-audited-event` | fuera de alcance | Prohibido rehabilitar este ciclo |

Dictamen (vinculante):

1. n=4 ≥ `abrupt_drop_min_samples: 3` y rate 0,50 < `process: 0.70` → **`abrupt_success_rate_drop`**. Umbrales 1.1.0 **intactos**.
2. Cadena causal: merge soberano #194 (`6528d115…` @ `11:31:11Z`) **cruzó** → sello `PullRequest_Merged` a dead-letter (`c24d84a7…`) → `exit_code: 1` del proceso → Radamanto degrada → Cerbero re-revoca @ `11:31:15Z` (4s).
3. #194 cerró payload `delete_branch` + veracidad handoff; **no** selló supervivencia cuando el sello EDA falla **después** de `merge_commit_hash`. Simetría producto = #187 post-umbral físico.
4. `rehab_laudo` residual #194 debe limpiarse en A1 (nuevo laudo #200).

## D1 — Misión (qué / por qué)

| Decisión | Laudo |
|----------|--------|
| Objetivo | Rehabilitar `accept-pr` en Cerbero/Radamanto **y** impedir re-muerte: A1 Yunque Rúnico + A2 fail_soft del sello `PullRequest_Merged` cuando `merge_commit_hash` ya cruzó. |
| Por qué ahora | Rehab #194 sin A2 sello reabrió el vector en el propio merge de cierre. Mientras ∈`revoked`, handoff soberano post-aduana queda peajeado otra vez. |
| Efecto observable | `accept-pr` ∉ `revoked` ni `permanent`; stats raíz `healthy` con ventana podada y laudo #200; sello KO post-merge → `fail_soft: true` + agregador `success` / `exit_code: 0`; sello KO **sin** merge hash permanece causal. |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un `persist_ref`, un PR. Prohibido despachar `bug-fix` satélite. |
| **L-WAVES** | Dos olas innegociables: **A1** saneamiento instancia, **A2** fail_soft sello post-`merge_commit_hash`. Rehab Cerbero sola = reabrir vector. |
| **L-REHAB-INST** | A1 = instancia `.SddIA/` (no genoma). Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff del PR. |
| **L-CERBERO** | Eliminar nodo `revoked.accept-pr` por completo. Verificar `permanent.accept-pr` ausente. Cerbero **no** tiene estado `healthy`. |
| **L-STATS** | Reset **solo** del bucket raíz `accept-pr`. |
| **L-RESET-ABS** | Absoluto: `status: healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY`; `rehabilitated_at` ISO de intervención A1. Limpiar residuales #194 (`rehab_laudo`/`rehabilitated_at` obsoletos). |
| **L-SAMPLES** | Poda termodinámica: vaciar `samples` **o** conservar solo ≤3 últimos OK runtime (`exit_code: 0`). Eliminar KO de la ventana actual. Sin poda, un fallo futuro re-dispara `abrupt_success_rate_drop`. |
| **L-ONTOLOGY** | Conservar `entity_type: process`. No regresionar a `tool`. |
| **L-FAILSOFT-SEAL** | Cuando `merge_commit_hash` no vacío está en el estado del proceso **y** la fase «Sello Criptográfico de Fusión» queda `failed`/`blocked` (p. ej. dead-letter `PullRequest_Merged`), inyectar `fail_soft: true` en ese report **antes** de `aggregate_execution_terminal` (adjudicación en el momento del fallo y/o post-pass retroactivo, simetría #187). |
| **L-PHYSICAL** | Umbral físico accept-pr = presencia de `merge_commit_hash` (fusión soberana materializada). Sin hash → sello KO permanece causal (`exit_code: 1`). |
| **L-AGGREGATOR** | `aggregate_execution_terminal` **intacto**. El `fail_soft` se escribe en el report **antes** del agregador. |
| **L-SIGNAL** | Prohibido silenciar el fallo del sello: el report sigue `failed`/`blocked` + error auditable; solo la supervivencia global se preserva vía `fail_soft`. |
| **L-NO-REOPEN-194** | Prohibido reabrir A2 payload `delete_branch` / A3 handoff truth de #194 salvo regresión empírica nueva demostrada. Este ciclo **no** reescribe aquel alcance. |
| **L-THRESH** | `radamanto.thresholds.json` v1.1.0 **intacto**. No reabrir `success_rate_min` ni `abrupt_drop_min_samples`. |
| **L-YAML** | No mutar YAML `{name}.md` del proceso para “inyectar” `fail_soft` estático: `fail_soft` es runtime en el JSON del `phase_report` (motor). |
| **L-OUT** | Fuera: rehab `refactorization` / `emit-pr-audited-event` / `bug-fix`; bypass `gh`/`git` crudo; mutar umbrales; versionar instancia en el PR; reabrir silencio #37 / payload #194 sin evidencia; escribir TODOs bajo `docs/todos/` (jurisdicción Cúmulo/Kaizen). |
| **L-DOC** | Cascada `features-documentation-pattern` + `validacion.md` APTO + `pbi_archived: true` + PBI canónico en `docs/todos/done/` en la rama del PR. |

### Ajustes anti-alucinación (órdenes crudas → laudo)

| Orden cruda | Laudo |
|-------------|-------|
| «pasar accept-pr a healthy en Cerbero» | Rehab = **borrar** `revoked.accept-pr`. `healthy` solo en `stats.json`. |
| «solo A1 sin fail_soft sello» | Insuficiente: el merge #194 reabrió el vector en 4s. |
| «fail_soft = silenciar dead-letter» | No. Report KO auditable + `fail_soft`; agregador no tumba survival. |
| «reabrir delete_branch / handoff #194» | No. **L-NO-REOPEN-194**. |
| «mutar agregador para tolerar sello KO» | Prohibido. Marcar `fail_soft` antes. |
| «A3 motor obligatorio por simetría #194» | No. Semilla fija A1+A2; sin vector empírica nueva de handoff/payload. |

## D3 — Matriz de aceptación (producto)

| AC | Enunciado |
|----|-----------|
| **AC-A1** | `accept-pr` ∉ `revoked` ni `permanent`; stats raíz `healthy`; `recovery_attempts: 0`; `rehab_laudo: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY`; `rehabilitated_at`; `samples` podados; evidencia en `execution.md` (no en diff de instancia). |
| **AC-GIT-CLEAN** | `.SddIA/cerbero/` y `.SddIA/radamanto/` **no** aparecen en el diff del PR. |
| **AC-ONTO** | `entity_type: process` conservado. |
| **AC-A2** | Con `merge_commit_hash` presente + sello `failed`/`blocked` → report con `fail_soft: true` + agregador `success` / `exit_code: 0`; sin hash → causal (`exit_code: 1`); error de sello sigue visible; agregador intacto. |
| **AC-TESTS** | Unit/integración: sello KO + hash → success; sello KO sin hash → fail; idempotencia de adjudicación; regresiones #194 (payload/handoff) intactas si se toca motor. |
| **AC-THRESH** | Umbrales 1.1.0 intactos. |
| **AC-DOC** | Cascada bajo `persist_ref`; PBI en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## D4 — Handoff Dedalo

1. Consumir este transcript + cuerpo de `objectives.md` como `refined_requirements`.
2. `spec.md`: procedimiento A1 instancia (borrar `revoked.accept-pr`; reset stats con L-RESET-ABS + L-SAMPLES + laudo #200); A2 touchpoints motor para L-FAILSOFT-SEAL / L-PHYSICAL / L-AGGREGATOR (simetría #187). **Verificar** cobertura de punta a punta (fallo en fase + post-pass) frente al dead-letter empírico; no asumir cierre por presencia parcial de helpers.
3. `plan.md`: un PR motor (A2 si incompleto) + procedimiento A1 instancia evidenciado en `execution.md` (git-clean instancia).
4. Tests de producto (qué, no cómo): sello KO + `merge_commit_hash` → survival; sello KO sin hash → causal; agregador no mutado; laterales intactos.
5. Prohibido rehab laterales, reabrir #194 sin evidencia, o castrar señal de dead-letter.
