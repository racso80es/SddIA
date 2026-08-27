---
feature_name: accept-pr-revoked-registry-rehab-ppr194
created: "2026-08-27"
purpose: Estabilización Mayeuta — PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY (rehab accept-pr + anti-recurrencia delete_branch + handoff PPR sin mentira de merge)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr194
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr194
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
document_id: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY
uuid: 7f3a9c2e-4b1d-4e8a-9c5f-6d7e8a9b0c1d
source_correlation_id: "59606407-eed3-4da8-ac13-3cf6205b2147"
source_pr_url: https://github.com/racso80es/SddIA/pull/194
feature_ref: docs/fixes/bundle-consumer-telegram-gateway
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — accept-pr ∈ revoked (abrupt_success_rate_drop since 2026-08-26T11:42:26Z); bloquea handoff soberano post-aduana (accept_pr_handoff true · merge ausente)"
satellite_fix_pbi: docs/todos/pending/[FIX] accept-pr delete_branch payload vs git-manager.md
olas:
  - A1
  - A2
  - A3
---

# Clarificación — accept-pr-revoked-registry-rehab-ppr194

Transcript Mayeuta. Estabiliza el **qué** y el **por qué**. Sin diseño de cápsulas, YAML de proceso ni mutación de genoma.

## D0 — Semilla y evidencia

| Vector | Hecho |
|--------|--------|
| PBI canónico | `docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md` (`document_id: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY`; `uuid: 7f3a9c2e-…`; `status: pending`) |
| Ciclo | `refactorization` · rama `refactor/accept-pr-revoked-registry-rehab-ppr194` · un `persist_ref` · un PR |
| Semilla operador | Rehabilitar `accept-pr` en Cerbero/Radamanto (PPR #194) **y** cortar re-muerte: payload `delete_branch` alineado a `accept-pr.md` + handoff PPR **no miente** merge ausente |
| Satélite absorbible | `docs/todos/pending/[FIX] accept-pr delete_branch payload vs git-manager.md` (`PBI-FIX-ACCEPT-PR-DELETE-BRANCH-PAYLOAD` · `uuid: 94f74fa6-…`) — empiría PR #193 `hygiene_failure` payload |
| Antecesor higiene | `docs/todos/done/[FIX] accept-pr — higiene silenciosa delete_branch tras merge.md` (PR #37 Python) — **no** reabrir; síntoma distinto (silencio vs contrato payload Rust) |
| Check origen | `REVOKED_ENTITY_ALERT_ACCEPT_PR` (Cosecha Kaizen PPR #194 · alerta no bloqueante aduana; riesgo handoff) |
| Sighting | PPR #194 · CID `59606407-eed3-4da8-ac13-3cf6205b2147` · `persist_ref` `docs/fixes/bundle-consumer-telegram-gateway` · emisor ECST `delivery-close-cycle` ∉ revoked |
| F5 heredado | `accept_pr_handoff: true` · `MERGE_ALREADY_OBSERVED: NO_APTO` (merge ausente; sin `PullRequest_Merged`) |
| `correlation_id` runtime de esta fase | vacío en inputs |

### Estado empírico (corte estabilización 2026-08-27 · verificado en instancia)

| Clave | Cerbero | Radamanto | Nota |
|-------|---------|-----------|------|
| `accept-pr` (raíz) | **`revoked.accept-pr`** · `entity_type: process` · `reason: abrupt_success_rate_drop` · `since: 2026-08-26T11:42:26Z` | `status: degraded` · `recovery_attempts: 1` · `degraded_at: 2026-08-26T11:42:26Z` · `structure_valid: false` · 5 samples (3 OK / 2 KO · rate 0,60) · `consecutive_success_count: 2` | **Vector activo.** Ausente de `permanent` |
| Laterales | `revoked.bug-fix`, `revoked.refactorization`, `revoked.emit-pr-audited-event` | fuera de alcance | Prohibido rehabilitar este ciclo |

Dictamen (vinculante): n=5 ≥ `abrupt_drop_min_samples: 3` y rate 0,60 < `process: 0.70` → **`abrupt_success_rate_drop`**. Umbrales 1.1.0 **intactos**.

Causa de re-muerte (semilla + FIX satélite + código vigente):

1. **Payload ilegítimo** en `delete_branch_hygiene` (`accept_pr.rs`): `{"branch_name", "remote": "origin"}` — viola `payload_exact(["branch_name","remote","force"])` de la cápsula; SSOT `accept-pr.md` § Fase 4 exige dos llamadas con `remote`/`force` **booleanos**.
2. **Semántica handoff**: F5/PPR marca `accept_pr_handoff: true` con merge ausente mientras `accept-pr`∈revoked → peaje «handoff listo» **sin** merge soberano ni capacidad de ejecutarlo (mentira operativa / bloqueo silencioso del cierre).

## D1 — Misión (qué / por qué)

| Decisión | Laudo |
|----------|--------|
| Objetivo | Rehabilitar `accept-pr` en Cerbero/Radamanto **y** impedir re-muerte: A1 Yunque Rúnico + A2 payload `delete_branch` canónico + A3 handoff PPR/F5 que no afirme merge ni handoff consumado cuando el merge está ausente o `accept-pr` no es ejecutable. |
| Por qué ahora | Rehab de registro **sin** A2+A3 reabre el vector (jurisprudencia #185/#187/#190). Handoff soberano post-aduana PPR #194 está bloqueado mientras la entidad permanezca revoked. |
| Efecto observable | `accept-pr` ∉ `revoked` ni `permanent`; stats raíz `healthy` con ventana podada; higiene Fase 4 emite payloads contractuales; PPR/F5 distingue merge ausente / handoff pendiente / handoff consumado / bloqueo por revoked sin inventar éxito. |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un `persist_ref`, un PR. El FIX satélite `PBI-FIX-ACCEPT-PR-DELETE-BRANCH-PAYLOAD` se **absorbe** (no despacha `bug-fix` propio); archiva a `done/` con el canónico. |
| **L-WAVES** | Tres olas innegociables: **A1** saneamiento instancia, **A2** contrato `delete_branch` (+ coherencia frozen I/O si Dedalo exige una verdad), **A3** veracidad handoff PPR/F5 ante merge ausente. Rehab Cerbero sola = reabrir vector. |
| **L-REHAB-INST** | A1 = instancia `.SddIA/` (no genoma). Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff del PR. |
| **L-CERBERO** | Eliminar nodo `revoked.accept-pr` por completo. Verificar `permanent.accept-pr` ausente. Cerbero **no** tiene estado `healthy`. |
| **L-STATS** | Reset **solo** del bucket raíz `accept-pr`. |
| **L-RESET-ABS** | Absoluto: `status: healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY`; `rehabilitated_at` ISO de intervención A1. |
| **L-SAMPLES** | Poda termodinámica: vaciar `samples` **o** conservar solo ≤3 últimos OK runtime (`exit_code: 0`). Eliminar KO `53d07f32…` y `f95e8c2f…`. Sin poda, un fallo futuro re-dispara `abrupt_success_rate_drop` (p. ej. rate ≤2/4 < 0,70 con n≥3). |
| **L-ONTOLOGY** | Conservar `entity_type: process`. No regresionar a `tool`. |
| **L-DELETE-PAYLOAD** | `delete_branch_hygiene` = **dos** invocaciones alineadas a `accept-pr.md` § Fase 4: local `{branch_name, remote: false, force: false}` luego remoto `{branch_name, remote: true, force: false}`. Prohibido `"remote": "origin"` (string) en `delete_branch`. |
| **L-HYGIENE-SOFT** | Fallo de delete (local o remoto, incl. «ref no existe» post-merge GitHub) → `hygiene_failure` auditable + `closed_branch` solo si delete **local** OK; **no** tumba `success` del proceso si merge+push ya cruzaron. Prohibido silencio. |
| **L-FROZEN-IO** | Una verdad: `skill-io-git-manager-frozen.md` declara `delete_branch` (y ops que la cápsula ya ejecuta si el laudo Dedalo lo exige) **o** la cápsula se recorta al enum. Homónimo `remote` string (push/pull/fetch) ≠ `remote` bool (`delete_branch`) — no unificar. Mutación norma vía `entity-manager`. |
| **L-HANDOFF-TRUTH** | PPR/F5 **no miente** merge ausente: si no hay `PullRequest_Merged` / `merge_commit_hash` observado, prohibido afirmar merge consumado. `accept_pr_handoff` debe distinguir al menos: pendiente (merge ausente, handoff aún no ejecutado con éxito) vs consumado (accept-pr devolvió merge soberano) vs bloqueado (`accept-pr` revoked / invoke fallido). Prohibido `accept_pr_handoff: true` como eufemismo de «todo OK» cuando el merge no existió. |
| **L-THRESH** | `radamanto.thresholds.json` v1.1.0 **intacto**. No reabrir `success_rate_min` ni `abrupt_drop_min_samples`. |
| **L-OUT** | Fuera: rehab `bug-fix` / `refactorization` / `emit-pr-audited-event`; bypass `gh`/`git` crudo; `SDDIA_SKIP_HOOKS` fuera del hijo ya acotado; `force: true` default; namespacing remotes ≠ `origin`; reabrir FIX #37 silencio Python; versionar instancia en el PR. |
| **L-DOC** | Cascada `features-documentation-pattern` + `validacion.md` APTO + `pbi_archived: true` + PBI canónico **y** satélite FIX en `docs/todos/done/` en la rama del PR. |

### Ajustes anti-alucinación (órdenes crudas → laudo)

| Orden cruda | Laudo |
|-------------|-------|
| «pasar accept-pr a healthy en Cerbero» | Rehab = **borrar** `revoked.accept-pr`. `healthy` solo en `stats.json`. |
| «`remote: "origin"` en delete_branch» | **Falso.** En `delete_branch`, `remote` es **boolean**. |
| «solo A1 sin tocar Rust» | Insuficiente: reabre `abrupt_success_rate_drop` / handoff mentiroso. |
| «despachar bug-fix satélite aparte» | No. **L-UNIFY** absorbe el FIX en este PR. |
| «fallo remoto debe abortar accept-pr» | No. Fail-soft por op post-merge+push (L-HYGIENE-SOFT). |
| «accept_pr_handoff true = merge hecho» | Mentira. true pendiente ≠ merge consumado. |

## D3 — Matriz de aceptación (producto)

| AC | Enunciado |
|----|-----------|
| **AC-A1** | `accept-pr` ∉ `revoked` ni `permanent`; stats raíz `healthy`; `recovery_attempts: 0`; `rehab_laudo: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY`; `rehabilitated_at`; `samples` podados; evidencia en `execution.md` (no en diff de instancia). |
| **AC-GIT-CLEAN** | `.SddIA/cerbero/` y `.SddIA/radamanto/` **no** aparecen en el diff del PR. |
| **AC-ONTO** | `entity_type: process` conservado. |
| **AC-A2** | Cero `"remote": "origin"` en `delete_branch`; dos `invoke_git_manager` con booleanos + `force`; remoto ausente no tumba éxito post-merge+push; `hygiene_failure` visible si falla alguna op; frozen I/O coherente (L-FROZEN-IO). |
| **AC-A3** | Con merge ausente, PPR/F5 no afirma merge consumado; handoff pendiente ≠ consumado; si `accept-pr` revoked/bloqueado, señal explícita de bloqueo (no éxito inventado). |
| **AC-SMOKE** | Smoke/lab: rama dummy local se borra; remoto ausente no fuerza `exit_code: 1` del proceso tras merge+push OK. |
| **AC-THRESH** | Umbrales 1.1.0 intactos. |
| **AC-DOC** | Cascada bajo `persist_ref`; PBI canónico + FIX satélite en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## D4 — Handoff Dedalo

1. Consumir este transcript + cuerpo de `objectives.md` como `refined_requirements`.
2. `spec.md`: touchpoints A2 en `accept_pr.rs` (`delete_branch_hygiene`); A3 en `pull_request_review.rs` / peaje F5 Argos (semántica handoff sin mentira); frozen I/O vía `entity-manager` si L-FROZEN-IO lo exige. **Prohibido** mutar umbrales Radamanto ni agregador genérico sin laudo.
3. `plan.md`: un PR motor (A2+A3) + procedimiento A1 instancia evidenciado en `execution.md` (git-clean instancia).
4. Tests de producto (qué, no cómo): payload canónico local/remoto; payload ilegítimo no se emite; remoto ausente → proceso success post-merge+push; handoff con merge ausente no reporta merge consumado; handoff con entidad revoked no inventa éxito.
5. Absorber y archivar `PBI-FIX-ACCEPT-PR-DELETE-BRANCH-PAYLOAD` en el mismo cierre documental.
6. Prohibido rehab laterales, bypass git crudo, o reabrir silencio #37.
