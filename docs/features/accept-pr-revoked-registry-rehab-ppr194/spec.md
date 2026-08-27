---
feature_name: accept-pr-revoked-registry-rehab-ppr194
created: "2026-08-27"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-accept-pr-cerbero + delete-branch-payload + handoff-truth-f5
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr194
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr194
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
document_id: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY
uuid: 7f3a9c2e-4b1d-4e8a-9c5f-6d7e8a9b0c1d
version_spec: "1.0.0"
status: dedalo_locked
olas:
  - A1
  - A2
  - A3
source_correlation_id: "59606407-eed3-4da8-ac13-3cf6205b2147"
source_pr_url: https://github.com/racso80es/SddIA/pull/194
satellite_fix_pbi: docs/todos/pending/[FIX] accept-pr delete_branch payload vs git-manager.md
satellite_document_id: PBI-FIX-ACCEPT-PR-DELETE-BRANCH-PAYLOAD
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — abrupt_success_rate_drop since 2026-08-26T11:42:26Z"
---

# Spec — accept-pr-revoked-registry-rehab-ppr194

## 1. Misión técnica

Rehabilitar `accept-pr` en Cerbero/Radamanto (revocación `abrupt_success_rate_drop` since `2026-08-26T11:42:26Z`, PPR #194) **y** cortar re-muerte: A1 Yunque Rúnico + A2 payload `delete_branch` canónico (absorbe FIX satélite) + A3 veracidad handoff PPR/F5 sin mentir merge ausente. Un `persist_ref`, un PR.

Consumir `objectives.md` + `clarify.md` como `refined_requirements` (Mayeuta).

## 2. Diagnóstico (evidencia código + instancia)

| Vector | Hecho |
|--------|--------|
| Cerbero instancia | `revoked.accept-pr` · `entity_type: process` · `reason: abrupt_success_rate_drop` · `since: 2026-08-26T11:42:26Z`. Ausente de `permanent`. |
| Radamanto bucket raíz | `degraded` · `recovery_attempts: 1` · 5 samples (3 OK / 2 KO · rate 0,60 < `process: 0.70`) · n≥`abrupt_drop_min_samples: 3` → dictamen abrupto válido. |
| Cuello A2 | `accept_pr.rs` `delete_branch_hygiene` emite `{"branch_name", "remote": "origin"}` — viola `payload_exact(["branch_name","remote","force"])`; `remote` debe ser **bool**. Empiría PR #193 `hygiene_failure`. |
| SSOT proceso | `accept-pr.md` § Fase 4 ya exige dos llamadas: local `remote: false` luego remoto `remote: true`, ambas `force: false`. |
| Cápsula | `skills/git-manager/src/main.rs` implementa `delete_branch` con bools; enum físico incluye `merge` / `get_last_commit` / `diff_name_only`. |
| Frozen I/O | `skill-io-git-manager-frozen.md` v1.0.0 enum §2 **omite** esas ops → dualidad norma↔cápsula (**F-GIT-FROZEN-IO-GAP**). |
| Cuello A3 | F5/Argos escribe `accept_pr_handoff: true` con `MERGE_ALREADY_OBSERVED: NO_APTO` mientras `accept-pr`∈revoked → peaje «handoff listo» sin merge soberano ni capacidad de ejecutarlo. Runtime `handoff_accept_pr` solo marca `true` post-`invoke_process` Ok; no expone estados `pending`/`blocked`. |
| Homónimo | `remote` string (`push`/`pull`/`fetch`) ≠ `remote` bool (`delete_branch`) — no unificar. |
| Umbrales | `radamanto.thresholds.json` **1.1.0** intactos. |
| Laterales | `revoked.bug-fix`, `revoked.refactorization`, `revoked.emit-pr-audited-event` — fuera. |
| Antecesor | FIX #37 (silencio Python) — **no** reabrir. |

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un PR. FIX satélite `PBI-FIX-ACCEPT-PR-DELETE-BRANCH-PAYLOAD` absorbido; archiva a `done/` con el canónico. Prohibido `bug-fix` satélite. |
| **L-WAVES** | A1 + A2 + A3 innegociables. Rehab Cerbero sola = reabrir vector. |
| **L-REHAB-INST** | A1 = instancia `.SddIA/` (Cerbero/Radamanto). Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff del PR. |
| **L-CERBERO** | Eliminar nodo `revoked.accept-pr`. Assert `permanent.accept-pr` ausente. Cerbero **no** tiene estado `healthy`. |
| **L-STATS** | Reset **solo** bucket raíz `accept-pr`. |
| **L-RESET-ABS** | `status: healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY`; `rehabilitated_at` ISO UTC de A1. |
| **L-SAMPLES** | Vaciar `samples` **o** ≤3 últimos OK runtime (`exit_code: 0`). Eliminar KO `53d07f32…` y `f95e8c2f…`. |
| **L-ONTOLOGY** | Conservar `entity_type: process`. |
| **L-DELETE-PAYLOAD** | `delete_branch_hygiene` = **dos** `invoke_git_manager("delete_branch", …)`: (1) `{branch_name, remote: false, force: false}` (2) `{branch_name, remote: true, force: false}`. Cero `"remote": "origin"` string en `delete_branch`. |
| **L-HYGIENE-SOFT** | Fallo por op (local o remoto, incl. ref remoto ausente post-merge GitHub) → acumular en `hygiene_failure.operations[]` auditable; `closed_branch` **solo** si delete **local** Ok; **no** tumba `success` del proceso si merge+push ya cruzaron. Prohibido silencio. Push de `main` sigue siendo causal (si push falla, abortar sin delete). |
| **L-FROZEN-ALIGN** | **Una verdad = declarar en frozen**, no recortar cápsula. Evolucionar `skill-io-git-manager-frozen.md` (SemVer → `1.1.0`) para incluir al menos `delete_branch` (+ `merge`, `get_last_commit`, `diff_name_only` que la cápsula ya ejecuta) con schemas alineados al `.rs`. Actualizar `skills/git-manager.md` inputs enum. Mutación vía `action:execute-process` → `entity-manager` (forja gobernada). Homónimo `remote` documentado. |
| **L-HANDOFF-STATUS** | Introducir `accept_pr_handoff_status` ∈ {`pending`, `consumed`, `blocked`, `skipped`}. Semántica boolean `accept_pr_handoff`: **`true` solo si `status == consumed`** (accept-pr devolvió fusión soberana / handoff runtime Ok con merge path). Merge ausente sin accept-pr exitoso → `false` + `pending`. Invoke fallido / Cerbero revoked → `false` + `blocked` + `block_reason`. Lab skip → `false` + `skipped`. Prohibido `true` como eufemismo de «todo OK» o «handoff pendiente». |
| **L-HANDOFF-RUNTIME** | Touchpoint `pull_request_review.rs` `handoff_accept_pr`: (a) tras Ok de `invoke_process("accept-pr")` → `consumed` + `accept_pr_handoff: true`; (b) Err por revoked/forbidden/cerbero → Ok fase con `blocked` (no inventar éxito; no afirmar merge); (c) otros Err → propagar o `blocked` con reason (preferir señal explícita sin mentir). No mutar agregador genérico. |
| **L-HANDOFF-F5** | Peaje F5/Argos/`validacion.md`: si `MERGE_ALREADY_OBSERVED: NO_APTO`, **prohibido** `accept_pr_handoff: true`. Usar `accept_pr_handoff: false` + `accept_pr_handoff_status: pending` (o `blocked` si accept-pr∈revoked observable). Prohibido afirmar `merge_commit` / merge consumado sin evidencia. Actualizar outputs en genoma `pull-request-review.md` vía `entity-manager`. |
| **L-PROCESS-YAML-ACCEPT** | `accept-pr.md` § Fase 4 ya correcto — **no** mutar salvo nota mínima si Argos lo exige. |
| **L-THRESH** | Umbrales 1.1.0 bit-idénticos. Prohibido mutar agregador / `radamanto_batch_core` / hollow bajo pretexto. |
| **L-NO-37** | Prohibido reabrir silencio Python FIX #37. |
| **L-DOC** | Cascada patrón + PBI canónico **y** satélite FIX → `docs/todos/done/` + `validacion.md` APTO `pbi_archived: true` en la misma rama. |

## 4. Touchpoints

| Locus | Mutación |
|-------|----------|
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | Reescribir `delete_branch_hygiene` per **L-DELETE-PAYLOAD** / **L-HYGIENE-SOFT**; tests unitarios §7. |
| `SddIA/engine/execute-process/src/engine/pull_request_review.rs` | Extender `handoff_accept_pr` per **L-HANDOFF-STATUS** / **L-HANDOFF-RUNTIME**; tests. |
| `normative_documents.skill_io_git_manager_frozen` (`SddIA/norms/skill-io-git-manager-frozen.md`) | SemVer 1.1.0: declarar ops faltantes + payload `delete_branch` bools. Vía `entity-manager`. |
| `directories.skills` → `git-manager.md` | Alinear enum inputs al frozen 1.1.0. Vía `entity-manager`. |
| `process_domain_roots` → `pull-request-review.md` | Outputs: documentar `accept_pr_handoff` + `accept_pr_handoff_status`. Vía `entity-manager`. |
| `accept-pr.md` | Preferible **intacto**. |
| Instancia `revoked_entities` / `stats` | A1 solo; evidencia `execution.md`; fuera del diff. |
| `directories.agents` → `radamanto.thresholds.json` | **Prohibido.** |
| `phase_terminal.rs` / `radamanto_batch_core.rs` | **Prohibido.** |
| `directories.evolution` | Entrada UUID ciclo `7f3a9c2e-4b1d-4e8a-9c5f-6d7e8a9b0c1d`. |
| `persist_ref` | Cascada + archive ambos PBI. |

## 5. Contratos de comportamiento

### 5.1 Higiene delete_branch (A2)

```text
push(main) Ok?
  NO → abort fase (causal); no delete
  YES →
    local  = delete_branch{branch_name, remote:false, force:false}
    remote = delete_branch{branch_name, remote:true,  force:false}
    closed_branch = branch iff local Ok
    hygiene_failure = { operations: [ {scope, ok, error?}... ] } if any !ok
    process success = true (post merge+push); hygiene auditable
```

### 5.2 Handoff status (A3)

```text
accept_pr_handoff          <=> status == "consumed"
accept_pr_handoff_status   ∈ { pending, consumed, blocked, skipped }

F5 / MERGE ausente / sin accept-pr Ok  → false + pending
accept-pr∈revoked / invoke fail Cerbero → false + blocked (+ block_reason)
accept-pr Ok (merge soberano)          → true  + consumed
SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF       → false + skipped
verdict ≠ aprobado                     → false + skipped (reason verdict_not_aprobado)
```

### 5.3 Frozen `delete_branch` (schema 1.1.0)

```json
{
  "branch_name": "string",
  "remote": false,
  "force": false
}
```

| Clave | Tipo | Notas |
|-------|------|-------|
| `branch_name` | string | obligatorio |
| `remote` | **boolean** | `false` → `git branch -d/-D`; `true` → `git push origin --delete` |
| `force` | boolean | solo afecta delete local (`-D` vs `-d`) |

## 6. Criterios de aceptación (producto)

| AC | Verificación |
|----|--------------|
| **AC-A1** | `accept-pr` ∉ revoked/permanent; stats raíz healthy; `recovery_attempts: 0`; laudo + `rehabilitated_at`; samples podados; evidencia en `execution.md`. |
| **AC-GIT-CLEAN** | Diff PR sin `.SddIA/cerbero/` ni `.SddIA/radamanto/`. |
| **AC-ONTO** | `entity_type: process` conservado. |
| **AC-A2** | Cero `"remote": "origin"` en payloads `delete_branch`; dos invokes bool+force; remoto ausente no tumba éxito post-merge+push; `hygiene_failure` visible; frozen 1.1.0 declara `delete_branch`. |
| **AC-A3** | Merge ausente ≠ merge afirmado; `accept_pr_handoff: true` solo con `consumed`; pending/blocked explícitos; revoked → `blocked` sin inventar éxito. |
| **AC-SMOKE** | Lab: delete local OK; remoto ausente no fuerza `exit_code: 1` post-merge+push. |
| **AC-THRESH** | Umbrales 1.1.0 intactos. |
| **AC-DOC** | Cascada; PBI canónico + FIX satélite en `done/`; `validacion.md` `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## 7. Tests de producto (qué, no cómo)

| ID | Caso |
|----|------|
| T-A2-CANON | Helper emite exactamente dos payloads canónicos (local luego remoto). |
| T-A2-ILLEGIT | Payload con `"remote": "origin"` **no** se construye en el helper post-fix. |
| T-A2-REMOTE-MISS | Simular fallo delete remoto → `hygiene_failure` presente; fase sync no retorna Err si push Ok. |
| T-A2-LOCAL-OK | Delete local Ok → `closed_branch` = branch. |
| T-A3-CONSUMED | invoke accept-pr Ok → `accept_pr_handoff: true` + status `consumed`. |
| T-A3-BLOCKED | invoke Err revoked/forbidden → `accept_pr_handoff: false` + status `blocked`; sin merge inventado. |
| T-A3-SKIP-VERDICT | sin verdict aprobado → skipped + false. |
| T-A3-NO-TRUE-PENDING | Convención F5 documentada: MERGE ausente ⇒ no `true`. |

## 8. Límites / fuera de alcance

- Rehab laterales Cerbero.
- Bypass `gh` / `git` crudo; `force: true` default; remotes ≠ `origin`.
- Mutar umbrales / agregador / hollow / `phase_terminal`.
- Recortar cápsula git-manager al enum 1.0.0 (contrario a **L-FROZEN-ALIGN**).
- Despachar `bug-fix` satélite aparte.
- Versionar mutaciones de instancia en el PR.

## 9. Viabilidad RBAC (Dedalo)

`target_executor_rbac` del proceso `refactorization`: `ecosystem-evolution`, `filesystem-ops`, `source-control`.

| Delegación | Contexto cápsula | Cruce |
|------------|------------------|-------|
| Motor `execute-process` (Tekton FS) | filesystem-ops / ecosystem-evolution | OK |
| `skill:git-manager` | source-control | OK |
| `action:execute-process` → `entity-manager` (frozen/skill/process YAML) | ecosystem-evolution | OK |
| `action:execute-process` → `delivery-close-cycle` | cierre | OK (fase T5) |

Ninguna fase del blueprint exige política fuera del pack. Genoma `{name}.md` de proceso/skill/norma **solo** vía `entity-manager`.
