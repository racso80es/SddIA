---
feature_name: kaizen-pr-aduana-friction-20261012
created: "2026-10-13"
process: bug-fix
branch_name: fix/kaizen-pr-aduana-friction-20261012
persist_ref: docs/fixes/kaizen-pr-aduana-friction-20261012
pbi_ref: docs/todos/pending/[KAIZEN] Aduana PR — corte LLM, dedupe Presented, persist_ref docs y sellos post-merge.md
document_id: PBI-KAIZEN-PR-ADUANA-FRICTION-20261012
uuid: "a7c3e91b-2f48-4d6a-9e1c-5b8d0f4a2c71"
scope: pr-aduana-friction
base: main
execution_id: "37dc1867-9940-499b-99a7-1e4d2d933a0d"
correlation_id: "834d80af-cd3f-49b5-958a-c4a7d62f6975"
---

# Spec — Aduana PR: corte LLM, dedupe Presented, persist_ref docs, sellos e higiene, PEC Telegram

## Problema

Audición PEC PR #344 (`docs/merge-thermo-hu-f2-close`): el `failed` de `pull-request-review` y el `success` de `accept-pr` cumplen contrato, pero el motor produce fricción innecesaria — doble Argos LLM tras F2, fan-out Presented, `persist_ref` nulo en ramas `docs/`, `merge_huérfano` falso, higiene que marca fallo cuando el ref remoto ya no existe, y Telegram opaco (`PEC failed/failed`).

## Causas (mapa F → capa)

| ID | Hecho verificado | Trabajo |
|----|------------------|---------|
| F-PPR-LLM-AFTER-F2-BLOCK | `residual_runner::run_generic` no corta fases; `executor::agent_phase_blocks_downstream` solo aplica a `feature`/`bug-fix`/`refactorization` | L1 — barrier selectivo Veredicto |
| F-PPR-PRESENTED-FANOUT | O1 idempotiza pre-push; no colapsa DCC+bridge Presented ni `Local_QA_Requested` solapado | L2 — dedupe despacho `(rama, commit)` |
| F-PERSIST-REF-DOCS-PREFIX | `infer_persist_ref_from_branch` solo `feat/`/`fix/`/`refactor/` | L3 — prefijo `docs/` |
| F-PRESENTED-SCAN-PENDING-ONLY | `scan_presented_for_branch` solo `.events/pending` | L4 — tres carpetas bus |
| F-HYGIENE-REMOTE-ABSENT-REF | `record_hygiene_op` trata «ref remoto no existe» como `ok: false`; `closed_branch` con solo local OK | L5 — idempotencia + alineación markdown |
| F-PEC-TELEGRAM-OPAQUE | `build_telegram_message_from_event` ignora `failed_phase*`; emisor no proyecta `branch`/`pr_url`/`notice` | L6 — payload + humanizador |

## Solución

### L1 — Corte LLM post-F2 (ADU-CA1, ADU-CA2, ADU-CA10)

**Alcance estrecho:** solo fase **«Veredicto y bloqueo»** de `pull-request-review` cuando el state ya registra bloqueo documental de Triaje.

Punto de intervención preferente: `residual_runner::execute_phase` / bucle de `run_generic`, **antes** de `agent_runtime::invoke_agent_phase` para esa fase.

**Predicado de corte (orden):**

1. `process_name == "pull-request-review"`.
2. `phase_name` es la fase Veredicto (nombre canónico del proceso: `Veredicto y bloqueo`).
3. Existe evidencia de Triaje documental `blocked` en `phase_reports` ya acumulados **o** en `state` (p. ej. `argos_verdict == "block"` ya escrito en L881–886 de `residual_runner`).

**Comportamiento al cortar:**

- No invocar runtime Argos.
- Emitir entrada de fase `status: "blocked"` (determinista), sin payload LLM.
- Dejar que Cerbero / Cosecha / Handoff sigan (baratos; handoff ya hace `skipped` / `verdict_not_aprobado`).
- Agregación terminal conserva `status_code: 1` (peaje F2 intacto).

**Prohibido:** mover PPR entero al executor de `feature`/`bug-fix` (saltaría Cerbero/Cosecha/Handoff).

**Si Triaje no está `blocked`:** Veredicto delega en Argos como hoy (CA2).

Helper compartido opcional con `executor::agent_phase_blocks_downstream` solo como predicado de status; la política de *qué* fases saltar es específica de PPR+Veredicto.

### L2 — Dedupe Presented / Local_QA (ADU-CA3, ADU-CA4)

**Lugar:** `route_domain_core::dispatch_subscriber` (o helper inmediato antes de `dispatch_process_subscriber` cuando `process_key == "pull-request-review"`). No abortar un PPR ya arrancado.

**Clave de colapso:** `(branch, head_commit)` donde:

- `branch` = payload `branch` / `pr_branch`.
- `head_commit` = SHA del tip de la rama en el momento del despacho (vía evidencia ya disponible en precheck / git-manager `get_last_commit` sobre el ref; si no resoluble → **no** deduplicar: fail-open al PPR).

**Hit (descartar):**

- Ya hay PPR en vuelo o completado reciente para la misma clave, **o**
- Segundo `PullRequest_Presented` / `Local_QA_Requested` solapado con la misma clave.

Envelope de descarte: `success`-equivalente a skip de suscriptor (`skipped` / status durable), `reason` explícito (p. ej. `ppr_dedupe_branch_commit`), ambos `event_id` si aplica.

**Rastro durable (CA3 / TQM-CA8):** proof síncrono bajo `eda_instance.proofs` (mismo patrón `tqm-single-flight`) en namespace `ppr-presented-dedupe/`. No depender de evento con `purge_after: true` sin suscriptor.

**Clase ECST:** solo si hace falta proyección bus ya suscrita; preferir proof. Genoma nuevo → `entity-manager`, no Write manual.

**CA4:** commit distinto en la misma rama → clave distinta → despacho normal (cascada F2 puede pasar aduana).

### L3 — `persist_ref` prefijo `docs/` (ADU-CA5, ADU-CA6)

Único punto: `eda_bus_topology::infer_persist_ref_from_branch`.

Tras ramas `feat/` / `refactor/` / `fix/` (comportamiento actual intacto — CA6), añadir:

```text
docs/{slug}  →  candidatos:
  1. {features_prefix}/{slug}   si is_dir
  2. {fixes_prefix}/{slug}      si is_dir
```

Sin directorio → `None` (no inventar sink). Sufijo numérico (`BRANCH_NUMERIC_SUFFIX_RE`) aplica igual que en los otros prefijos si el slug lo lleva.

No duplicar heurística en prompts Argos.

### L4 — Escaneo Presented histórico (ADU-CA7)

`accept_pr::scan_presented_for_branch`:

- Barrer `.events/pending`, `.events/processed` y `.events/dead-letter` (rutas relativas al repo; si alguna carpeta no existe, continuar).
- Mismo predicado: `event_type == PullRequest_Presented` ∧ `payload.branch == branch`.
- `presented_found: true` si hay hit en cualquiera; `orphan_merge` / `traceability_anomaly: merge_huérfano` solo si ausencia en las tres.

### L5 — Higiene remota idempotente (ADU-CA8, ADU-CA9)

En `delete_branch_hygiene` / post-`record_hygiene_op` para scope `remote`:

1. Si el `Err` (o stderr encapsulado) clasifica como **ref ausente** (`remote ref does not exist`, `ref remoto no existe`, variantes `error: unable to delete ... not found` / `remote ref does not exist`), reclasificar a `ok: true` (éxito idempotente). No tragar auth/red/ref protegida.
2. `closed_branch = Some(branch)` **solo si** `local_ok && remote_ok` (tras reclasificación). Parcial → `hygiene_failure` con `operations`, sin `closed_branch`.
3. L-HYGIENE-SOFT intacto: higiene no tumba `status_code` si merge+push salieron.

Actualizar `t_a2_local_ok_closed_branch` y añadir casos: ref ausente → remote ok; fallo auth → hygiene_failure; local ok + remote fail → sin `closed_branch`.

### L6 — PEC Telegram legible (ADU-CA11–ADU-CA14)

#### Emisor (`thermodynamic.rs` + PEC temprano TQM si conoce rama)

Al armar payload `Process_Execution_Completed`, proyectar (si disponibles en `state` / `inputs`):

- `branch` ← primer no vacío de `source_branch` | `pr_branch` | `branch`
- `pr_url` si existe
- `notice` — **una** cadena, prioridad:
  1. higiene incompleta (`survived_branch` + causa corta)
  2. `merge_huérfano`
  3. `merge ya materializado` (`merge_already_done` / equivalente)
  4. vacío → omitir campo

`failed_phase` / `failed_phase_error` ya se escriben en fallo; conservar.

#### Humanizador (`build_telegram_message_from_event` para PEC)

Líneas en orden, omitiendo vacías:

1. `PEC {status}: {process_name}` — si `cycle_phase` ∉ {`completed`, `failed`} y ≠ `status`, sufijo ` ({cycle_phase})`
2. `fase: {failed_phase}`
3. `motivo: {failed_phase_error}` ≤120 chars
4. `rama: {branch}` **o** si no hay rama `persist_ref: {persist_ref}`
5. `pr_url` línea propia
6. `aviso: {notice}` ≤120 si no vacío
7. `correlation_id={…}` o `sin-correlation_id`

Prohibido que el humanizador lea workspaces o `.events/`. `notice` no sustituye sellos `hygiene_failure` / `traceability_anomaly`.

## Tests (ADU-CA10, ADU-CA14)

| Área | Casos mínimos |
|------|----------------|
| residual / PPR barrier | Triaje blocked → Veredicto sin invoke; Triaje OK → invoke |
| `infer_persist_ref_from_branch` | `docs/slug`→features; fallback fixes; ausente→None; feat/fix/refactor regresion |
| `scan_presented_for_branch` | hit en processed; hit en dead-letter; miss en las tres |
| hygiene | ref ausente→ok; otro error→failure; closed_branch solo ambos ok |
| Telegram | CA11 failed; CA12 cycle_phase; CA13 notice/sin notice; `telegram_message_for_pec_includes_correlation` verde |

## Fuera de alcance

- Debilitar `FAIL_F2_DOC` o L-HYGIENE-SOFT.
- Argos LLM en Auditoría Genómica `merge_already_done`.
- Reescribir O1 pre-push (complementa; no sustituye).

## Criterios (mapeo)

| CA | Capa |
|----|------|
| ADU-CA1, CA2, CA10 (corte) | L1 |
| ADU-CA3, CA4 | L2 |
| ADU-CA5, CA6 | L3 |
| ADU-CA7 | L4 |
| ADU-CA8, CA9, CA10 (accept_pr/persist) | L5 + L3 |
| ADU-CA11–CA14 | L6 |
