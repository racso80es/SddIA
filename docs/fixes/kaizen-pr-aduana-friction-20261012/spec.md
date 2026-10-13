---
feature_name: kaizen-pr-aduana-friction-20261012
created: "2026-10-13"
updated: "2026-10-13"
process: bug-fix
branch_name: fix/kaizen-pr-aduana-friction-20261012
persist_ref: docs/fixes/kaizen-pr-aduana-friction-20261012
pbi_ref: docs/todos/pending/[KAIZEN] Aduana PR — corte LLM, dedupe Presented, persist_ref docs y sellos post-merge.md
document_id: PBI-KAIZEN-PR-ADUANA-FRICTION-20261012
uuid: "a7c3e91b-2f48-4d6a-9e1c-5b8d0f4a2c71"
scope: pr-aduana-friction
base: main
design_status: sealed
correlation_id: "d96571be-c2e1-4a07-96be-efb0d18b81d8"
execution_id: "c71cb9cb-56cb-48e3-a6b8-69b45dddc156"
agent: dedalo
phase: Diseño del fix
---
# Spec — Aduana PR: corte LLM, dedupe Presented, persist_ref docs, sellos e higiene, PEC Telegram

## Problema

Audición PEC PR #344 (`docs/merge-thermo-hu-f2-close`): el `failed` de `pull-request-review` y el `success` de `accept-pr` cumplen contrato, pero el motor produce fricción — doble Argos LLM tras F2, fan-out Presented, `persist_ref` nulo en ramas `docs/`, `merge_huérfano` falso, higiene que marca fallo cuando el ref remoto ya no existe, y Telegram opaco (`PEC failed/failed`).

Fuente: `objectives.md` + `PBI-KAIZEN-PR-ADUANA-FRICTION-20261012` (ADU-CA1–ADU-CA14).

## Causas → capas

| ID | Hecho | Capa |
|----|-------|------|
| F-PPR-LLM-AFTER-F2-BLOCK | `residual_runner::run_generic` no corta fases; `agent_phase_blocks_downstream` solo `feature`/`bug-fix`/`refactorization` | L1 |
| F-PPR-PRESENTED-FANOUT | O1 idempotiza pre-push; no colapsa DCC+bridge ni `Local_QA_Requested` solapado | L2 |
| F-PERSIST-REF-DOCS-PREFIX | `infer_persist_ref_from_branch` solo `feat/`/`fix/`/`refactor/` | L3 |
| F-PRESENTED-SCAN-PENDING-ONLY | `scan_presented_for_branch` solo `.events/pending` | L4 |
| F-HYGIENE-REMOTE-ABSENT-REF | «ref remoto no existe» → `ok: false`; `closed_branch` con solo local OK | L5 |
| F-PEC-TELEGRAM-OPAQUE | humanizador ignora `failed_phase*`; emisor no proyecta `branch`/`pr_url`/`notice` | L6 |

## Solución

### L1 — Corte LLM post-F2 (ADU-CA1, CA2, CA10)

**Alcance:** solo fase `Veredicto y bloqueo` de `pull-request-review` si el state ya registra bloqueo documental.

**Predicados (API):**

- `ppr_documental_triage_blocked(state)` — `documental_triage_blocked == true` **o** `phase_reports[]` con `phase_name == "Triaje documental"` ∧ `status == "blocked"`.
- `ppr_verdict_skip_llm(process, phase, state)` — PPR ∧ fase `Veredicto y bloqueo` ∧ predicado anterior.

**Punto:** `residual_runner::execute_phase`, antes de `agent_runtime::invoke_agent_phase`.

**Al cortar:** no invocar Argos; fase `status: "blocked"`, `handler: "ppr-verdict-barrier"`, `reason: "documental_triage_blocked"`; `argos_verdict: "block"` en state; Cerbero/Cosecha/Handoff siguen (`skipped` / `verdict_not_aprobado`); `status_code: 1` (peaje F2 intacto).

**Prohibido:** redirigir PPR entero al executor `feature`/`bug-fix`.

**CA2:** Triaje no blocked → Veredicto delega en Argos.

### L2 — Dedupe Presented / Local_QA (ADU-CA3, CA4)

**Lugar:** pre-`dispatch` cuando destino es `pull-request-review` (`route_domain_core`).

**API:** `try_acquire_ppr_dedupe` / `release_ppr_dedupe`; clave `ppr_dedupe_hex(branch, head_commit)`.

- `branch` ← `branch` | `pr_branch`.
- `head_commit` irresoluble → **fail-open** (`Ok(None)`).
- Hit: skip `reason: "ppr_dedupe_branch_commit"` + proof durable bajo proofs namespace `ppr-presented-dedupe/` (lock+json). Sin clase ECST nueva.

**CA4:** commit distinto → clave distinta → despacho normal.

### L3 — `persist_ref` prefijo `docs/` (ADU-CA5, CA6)

Único punto: `eda_bus_topology::infer_persist_ref_from_branch`.

Tras `feat/` / `refactor/` / `fix/` (CA6 intacto):

```text
docs/{slug} → candidatos in order:
  1. {features_prefix}/{slug}   si is_dir
  2. {fixes_prefix}/{slug}      si is_dir
  (+ bases sin sufijo numérico vía BRANCH_NUMERIC_SUFFIX_RE)
```

Sin directorio → `None`. No duplicar heurística en prompts Argos.

### L4 — Escaneo Presented histórico (ADU-CA7)

`accept_pr::scan_presented_for_branch`: barrer `.events/pending`, `processed`, `dead-letter`. Predicado: `PullRequest_Presented` ∧ `payload.branch == branch`. `orphan_merge` / `traceability_anomaly: merge_huérfano` solo si ausencia en las tres.

### L5 — Higiene remota idempotente (ADU-CA8, CA9)

- `is_remote_ref_absent_error`: `remote ref does not exist` / `ref remoto no existe` / (`unable to delete` ∧ not found|does not exist).
- `record_hygiene_remote`: ausente → `ok: true`, `idempotent_absent: true`.
- No tragar auth/red/ref protegida.
- `closed_branch = Some(branch)` **solo si** `local_ok && remote_ok`.
- L-HYGIENE-SOFT intacto (`status_code` 0 si merge+push OK).

### L6 — PEC Telegram legible (ADU-CA11–CA14)

**Emisor** (`thermodynamic::project_pec_operator_fields` / `pec_notice_from_context`):

- `branch` ← `source_branch` | `pr_branch` | `branch`
- `pr_url` si existe
- `notice` prioridad: higiene incompleta (`survived_branch` + causa) > `merge_huérfano` > `merge ya materializado`; omitir si vacío
- Conservar `failed_phase*`

**Humanizador** (`build_telegram_message_from_event` para `Process_Execution_Completed`) — omitir vacías:

1. `PEC {status}: {process_name}` — sufijo ` ({cycle_phase})` solo si cycle ∉ {`completed`,`failed`} y ≠ status
2. `fase: {failed_phase}`
3. `motivo: {failed_phase_error}` ≤120
4. `rama: {branch}` **o** `persist_ref: {persist_ref}`
5. `pr_url` línea propia
6. `aviso: {notice}` ≤120
7. `correlation_id=…` o `sin-correlation_id`

Prohibido que el humanizador lea workspaces o `.events/`.

## Criterios de aceptación (Argos)

| ID | Predicado verificable | Medio |
|----|----------------------|-------|
| ADU-CA1 | Triaje documental blocked → Veredicto sin `invoke_agent_phase`; blocked; handoff skipped; status_code 1 | unit `adu_ca1_*` |
| ADU-CA2 | Triaje no blocked → Veredicto invoca Argos | unit `adu_ca2_*` |
| ADU-CA3 | Dos Presented/Local_QA misma (rama, commit) → un PPR; descarte + proof | unit `adu_ca3_*` |
| ADU-CA4 | Commit distinto misma rama → no suprimido | unit `adu_ca4_*` |
| ADU-CA5 | `docs/{slug}` → features\|fixes si dir existe; else None | unit `adu_ca5_*` |
| ADU-CA6 | feat/fix/refactor sin regresión | unit `adu_ca6_*` |
| ADU-CA7 | presented_found si hit en pending\|processed\|dead-letter | unit `t_adu_ca7_*` |
| ADU-CA8 | ref remoto ausente → ok true, sin hygiene_failure | unit `t_adu_ca8_*` |
| ADU-CA9 | otro error remoto → hygiene_failure; closed_branch solo local∧remoto; L-HYGIENE-SOFT | unit `t_adu_ca9_*` + `t_a2_*` |
| ADU-CA10 | Batería CA1–CA2 y CA5–CA9 presente | inventario tests |
| ADU-CA11 | PEC failed: titular sin `failed/failed`; fase; motivo≤120; correlation | unit `adu_ca11_*` |
| ADU-CA12 | completed sin ciclo en titular; awaiting_agents entre paréntesis | unit `adu_ca12_*` |
| ADU-CA13 | Payload branch/pr_url/notice; humanizador solo payload | unit `adu_ca13_*` |
| ADU-CA14 | `telegram_message_for_pec_includes_correlation` + CA11–13 (caso sin notice) | unit + regresión |

## Fuera de alcance

- Debilitar `FAIL_F2_DOC` o L-HYGIENE-SOFT.
- Argos LLM en Auditoría Genómica `merge_already_done`.
- Reescribir O1 pre-push (complementa; no sustituye).
- Forja de genoma / proceso nuevo (no requerido por ADU-CA3).

## Mapa CA → capa

| CA | Capa |
|----|------|
| ADU-CA1, CA2, CA10 (corte) | L1 |
| ADU-CA3, CA4 | L2 |
| ADU-CA5, CA6 | L3 |
| ADU-CA7 | L4 |
| ADU-CA8, CA9, CA10 (accept_pr/persist) | L5 + L3 |
| ADU-CA11–CA14 | L6 |

## Nota de coherencia (revisión estática Dedalo)

Símbolos L1–L6 presentes en el árbol de trabajo (`residual_runner`, `route_domain_core`, `eda_bus_topology`, `accept_pr`, `thermodynamic`). Esto **no** sustituye batería unitaria ni `validacion.md` APTO (fases Tekton/Argos posteriores).
