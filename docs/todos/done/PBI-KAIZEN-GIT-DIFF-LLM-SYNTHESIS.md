---
document_id: PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS
uuid: "1540ab52-4354-49a6-9d4e-63135aaccde2"
title: "[KAIZEN] Diff vía git-manager y evaluación de síntesis LLM post-merge"
format: markdown
version: "1.2.0"
created: "2026-09-05"
updated: "2026-09-06"
status: cerrado
refinement_status: implemented
pr_url: https://github.com/racso80es/SddIA/pull/263
priority: media
process: feature
executor_vehicle: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-git-diff-llm-synthesis
persist_ref_suggested: docs/features/kaizen-git-diff-llm-synthesis
parent_pbi: docs/todos/done/PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED.md
parent_document_id: PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
parent_uuid: "5880d6fc-99f3-4ecf-8c9e-a4885d45f117"
friction_ids:
  - F-GIT-MANAGER-NO-COMMIT-SUMMARY
  - F-LLM-SYNTHESIS-NO-EVAL-HARNESS
tech_debt_ids:
  - DT-GIT-MANAGER-COMMIT-SUMMARY
  - DT-LLM-SYNTHESIS-GOLDEN
derived_from:
  - PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
related:
  - SddIA/norms/skill-io-git-manager-frozen.md
  - SddIA/skills/git-manager.md
  - SddIA/skills/git-manager/src/main.rs
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/engine/execute-process/src/engine/notify_humanized_pr_merged.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/tools/gemini-http-infer.md
  - SddIA/actions/notify-humanized-pr-merged.md
  - SddIA/events/domain/pull-request-merged.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/process/entity-manager.md
  - SddIA/process/norm-creator.md
  - docs/features/eda-telegram-notify-pr-merged/spec.md
---

# [KAIZEN] Diff git-manager + evaluación de síntesis LLM

Deuda diferida de ola 2 (`PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED`, spec § Fuera: ampliar `git-manager` y `diff_name_only` en el prompt). El handler `notify_humanized_pr_merged.rs` sintetiza solo con campos ECST. Este PBI gobierna (1) una operación **nueva** de resumen de commit en `git-manager` y (2) un arnés de evaluación **determinista** del prompt/ensamblaje. Inmutabilidad ECST v1.0.0. Cero entrenamiento de pesos en el Core.

Estado empírico del padre (2026-09-06): el handler, la action uuid `1cd7bd40-b72f-4114-ac44-68b912774aa6` y el fan-out `action: notify-humanized-pr-merged` existen en este workspace. El PBI padre está en `docs/todos/done/` con `pr_url` #262. `docs/features/eda-telegram-notify-pr-merged/validacion.md` permanece `global: NO_APTO` (`FAIL_F4_RBAC`). Este kaizen **no** reabre #262 ni afirma merge-a-`main` como precondición.

## 0. Filtro A — Antialucinación y Contraste Empírico

Hallazgos de v1.1.0 (y residuos) contrastados contra genoma y cápsulas. Lo marcado **corrige este PBI**.

| ID | Afirmación falsa / Asunción ingenua | Verdad empírica | Corrección en este PBI |
|----|-------------------------------------|-----------------|------------------------|
| FA-K-1 | El payload `PullRequest_Merged` debe pasar a v1.1.0 con diffs y mensajes. | Clase v1.0.0 REQUIRED: `source_branch`, `target_branch`, `merge_commit_hash`, `author`, `security_clearance`. OPTIONAL: `pr_url`, `repository_name`. FORBIDDEN: `hash_signature`. Emisores: `emit-pr-merged-event`, `accept-pr`. | Payload ECST **intacto**. El Git es enriquecimiento del suscriptor Argos. |
| FA-K-2 | El esquema congelado v1.1.0 «solo admite» `diff_name_only` y `get_last_commit`. | Enum §2: once ops (`status` … `diff_name_only`). `get_last_commit` = `rev-parse` → `commitHash` (sin subject). `diff_name_only` = `git diff --name-only <ref_spec>` **un solo argv**. | No recortar el enum. La deuda es **ausencia de subject + semántica inadecuada de `diff_name_only` para un commit histórico**, no «solo dos ops». |
| FA-K-2b | Reutilizar `diff_name_only` con `ref_spec = merge_commit_hash` alimenta el LLM con los ficheros del merge. | `git diff --name-only HASH` compara el **working tree** contra ese árbol. Post-merge en un working tree limpio la lista es vacía o es suciedad local, no el diff del commit. Un rango `HASH^..HASH` como un solo token es frágil (shallow, root, merges). | Operación nueva `commit_summary`. Prohibido abusar de `diff_name_only` como atajo de «ficheros del merge». |
| FA-K-2c | `commit_summary.subject` es «el mensaje del PR». | `git-manager` solo ejecuta el binario `git`. No hay `gh`. El título GitHub no es un campo ECST (`pr_url` es OPTIONAL y `accept-pr` no lo inyecta). | `subject` = primera línea del **commit Git** (`%s`). Nunca título de PR. |
| FA-K-3 | Descongelar la norma invocando `norm-creator` **o** `entity-manager` `entity_class: norm`. | `entity-manager` delega `norm` → `norm-creator` → `run_norm_forge` escribe **solo** `SddIA/library/norms/`. `skill-io-git-manager-frozen.md` vive en `directories.norms`. `run_skill_forge` **no tiene rama `update`**: un `lifecycle_operation: update` regeneraría uuid. | Frozen: parche bajo ciclo `feature` (DA-4) + registro `SddIA/evolution/`. Skill `.md`: **no** EM skill-create/update genérico. Action: EM `entity_class: action` `update` (`patch_action_content_update`, uuid inmutable). Cápsula `.rs` y handler: Tekton bajo la misma topología. Prohibido `norm-creator`. |
| FA-K-4 | El handler invoca `invoke_capsule_json(repo, "git-manager", &payload, false)` y hay timeout ≤ 3s. | Helper canónico: `invoke_git_manager(repo, operation_type, payload)` (arma `operation_type` + `repository_path` + `operation_payload_json`). `invoke_capsule_json` etiqueta `"skill"` y exige el JSON raíz completo. `invoke_capsule_subprocess` hace `wait_with_output()` **sin timeout**. Gemini ya tiene timeout propio (`SDDIA_GEMINI_HTTP_TIMEOUT_SECS`, default 30s en la tool). | Consumo vía `invoke_git_manager`. Fail-soft si `Err`, `success=false` o parse incompleto. **No** inventar timeout 3s en `capsules.rs`. No bloquear el fan-out IOTA (suscriptores independientes; IOTA es el primero en el JSON). |
| FA-K-5 | Calidad de síntesis = fine-tuning / Vertex / pesos locales. | Core: inferencia zero-shot vía `gemini-http-infer` (AI Studio). Lab: `SDDIA_LAB_MOCK_OUTBOUND` / `SDDIA_LAB_MOCK_GEMINI_URL`. | Prompt anti-conjetura + hechos Git + golden **offline**. Cero artefactos de entrenamiento. |
| FA-K-6 | Este trabajo vive en el PR #262 / «no descongelar aquí». | Copia fósil de ola 2. El alcance diferido **es** este PBI, rama `feat/kaizen-git-diff-llm-synthesis`. | Aquí sí se evoluciona frozen a v1.2.0 y se implementa el arnés. No se reabre #262. |
| FA-K-7 | El contrato de la action debe declarar un input VCS nuevo. | Inputs actuales: `event_type`, `payload`, `correlation_id`. El handler ya resuelve el envelope vía `run_from_event`. El Git se deriva de `payload.merge_commit_hash` + `repository_path` del repo del centinela. | Git = enriquecimiento interno. No nuevo campo ECST ni input EDA. Sí: capability `delegate-git-manager` (convención local, paridad `delegate-gemini-http-infer`; no está en `capability-taxonomy` y **no** se exige alta allí). |
| FA-K-8 | Un golden set puede asertar «inmunidad a conjetura» y «no redundancia» sobre el texto del LLM. | Hoy no hay filtro post-LLM: `truncate_synthesis` corta a 2 líneas / 400 chars y ensambla. El padre (riesgo ola 2) dejó explícito: fail-soft no filtra invención; no segundo LLM juez. `SDDIA_LAB_MOCK_OUTBOUND` devuelve `lab-mock:{model}:{prompt80}`, no prosa de negocio. | El arnés aserta **prompt y ensamblaje** (hechos inyectados, instrucción anti-conjetura, truncado, fail-soft). No oracle semántico del modelo vivo. Sin sanitizer de paths en el MVP. |
| FA-K-9 | Existe `--test notify_humanized_eval` y el test `telegram_subscriptions_pr_merged_topology`. | Tests vivos: `notify_humanized_*` en el módulo del handler (`cargo test -p execute-process --lib -- notify_humanized`). Topología: `pull_request_merged_subscription_is_humanized_action_not_telegram_tool` en `route_domain_core.rs`. `execute-process/tests/` solo tiene `memory_evolution_ingest.rs`. Crate `git-manager`: 3 tests de `.gitignore` en `add`, cero de ops. | Extender los tests **lib** existentes. Nombre de binario de integración = opcional; no es CA con nombre inventado. No-regresión de topología con el test que ya existe. |
| FA-K-10 | Evolucionar frozen implica mutar `cumulo.paths.json`. | La clave `normative_documents.skill_io_git_manager_frozen` ya apunta al fichero. | `cumulo.paths.json` **no** entra en el diff salvo que se añada una clave nueva (fuera). |

---

## 1. F-GIT-MANAGER-NO-COMMIT-SUMMARY & DT-GIT-MANAGER-COMMIT-SUMMARY

**Hecho:** `build_synthesis_prompt` inyecta solo ECST (`source_branch`, `target_branch`, `merge_commit_hash`, `author`, `auditor`, `policy`, `pr_url?`, `repository_name?`). El LLM no ve subject Git ni lista de ficheros del merge. `diff_name_only` no rellena ese hueco (FA-K-2b). El límite de la action (`notify-humanized-pr-merged.md` §3) aún dice «No ampliar `git-manager`» — texto de ola 2 a retirar en este ciclo.

**Cambio canónico (proceso `feature`, DA-4):**

### 1.1 Norma congelada `skill-io-git-manager-frozen.md` v1.2.0

uuid inmutable `b2c3d4e5-f6a7-4890-b123-456789abcdef`. Añadir al enum `commit_summary`.

`operation_payload_json` (`payload_exact`, claves exactas):

```json
{
  "ref": "string",
  "max_files": 30,
  "max_subject_chars": 200
}
```

| Clave | Tipo | Obligatorio | Regla |
|-------|------|:-----------:|-------|
| `ref` | string | sí | Token seguro (`assert_safe_token`). No vacío. Típicamente `merge_commit_hash` (40 hex). |
| `max_files` | number (entero) | sí | 1–30 inclusive. El caller envía el entero; no omitir (frozen = claves exactas). |
| `max_subject_chars` | number (entero) | sí | 1–200 inclusive. |

Salida en `data` (además de `gitStdout` / `gitStderr` / `errorSummary`, paridad con el resto de ops):

```json
{
  "commitHash": "40 hex",
  "subject": "primera línea Git, recortada",
  "files": ["rutas", "relativas"],
  "totalFilesChanged": 3,
  "truncated": false
}
```

`files.length` ≤ `max_files`. `totalFilesChanged` = recuento **antes** de truncar. `truncated: true` si se cortó.

Ejecución (sin shell; `Command::new("git")`; args en lista):

1. `rev-parse --verify <ref>` → `commitHash` (reutilizar `parse_commit_hash`).
2. `show -s --format=%s <ref>` → `subject` (trim; corte a `max_subject_chars`; vacío permitido).
3. Diff **first-parent** de ese commit, dos argv, nunca working-tree: `diff --name-only -M <ref>^ <ref>` tras validar que `<ref>^` resuelve. Si `<ref>^` no existe (root / shallow): `success: false` / exit ≠ 0 — el handler hace fail-soft.

Prohibido: `git show` de parche; `git diff` sin acotar a names-only; unificar `ref_spec` de `diff_name_only` con esta op.

Paridad: `git-manager.md` v1.2.0 (uuid `4dac18fc-4cd1-4aa4-bdc3-faeb3bf762fc` inmutable): enum de inputs, mapeo **git-read-state** incluye `commit_summary`. SemVer de la skill al alza. `hash_signature` recalculado. Índice `skills/index.md` si la fila de versión queda desfasada.

Registro: alta `modificacion` en `SddIA/evolution/` (`evolution_contract` v1.1.2).

### 1.2 Acción `notify-humanized-pr-merged.md`

EM `entity_class: action` `lifecycle_operation: update` (uuid `1cd7bd40-b72f-4114-ac44-68b912774aa6` inmutable).

- `capabilities`: añadir `"delegate-git-manager"`.
- Cuerpo: Paso 1.5 — resolver `commit_summary` con `ref = merge_commit_hash` **antes** de Gemini; si falla, prompt ECST-only.
- Prompt: líneas opcionales `SUBJECT: …` y `FILES: a, b, c` (+ nota `truncated` / `totalFilesChanged` si aplica). Misma prohibición de conjetura.
- Límites: **retirar** «No ampliar git-manager». Conservar: no versionar ECST; no segundo suscriptor Telegram; no colapsar IOTA.

No declarar el resumen Git como `inputs` EDA (FA-K-7).

### 1.3 Handler `notify_humanized_pr_merged.rs`

Antes de `try_infer_synthesis`:

```text
invoke_git_manager(repo, "commit_summary", {
  "ref": merge_commit_hash,
  "max_files": 30,
  "max_subject_chars": 200
})
```

Fail-soft → `build_synthesis_prompt` sin SUBJECT/FILES (comportamiento ola 2). Éxito → inyectar hechos. Gemini y Telegram: invariantes ola 2 (`invoke_tool_capsule_json` / `invoke_capsule_json` para esas tools; un envío).

Tests crate `git-manager`: `payload_exact`, token inseguro, `max_files` fuera de rango, subject recortado, lista truncada con `totalFilesChanged` / `truncated`. Repo fixture mínimo (no el monorepo SddIA).

---

## 2. F-LLM-SYNTHESIS-NO-EVAL-HARNESS & DT-LLM-SYNTHESIS-GOLDEN

**Hecho:** Ola 2 cubre truncado, ensamblaje fail-soft y prompt ECST. No hay fixture de escenarios Git ni aserción de que SUBJECT/FILES entren (y solo entren) en el prompt.

**Cambio canónico:**

1. **Fixture** (lib, no binario inventado): JSON de escenarios junto al módulo o bajo `SddIA/engine/execute-process/tests/fixtures/pr_merged_golden_set.json` si se añade un test de integración. Escenarios mínimos:
   - Subject claro + ≤30 ficheros.
   - Subject vacío o convencional (`Merge branch '…'`).
   - `totalFilesChanged` > `max_files` (`truncated: true`).
   - Git `Err` / exit ≠ 0 → prompt idéntico al ECST-only actual.
   - Respuesta LLM mock verborreica (>2 líneas) → `truncate_synthesis`.
2. **Aserciones deterministas** (`cargo test -p execute-process --lib -- notify_humanized`, más casos nuevos):
   - Prompt contiene `SUBJECT:` / `FILES:` **solo** con los valores del fixture; no inventa rutas.
   - Prompt conserva `PENALIZE CONJECTURE` y `Do not invent files`.
   - Truncado ≤ 2 líneas y ≤ 400 chars (ya existe; no romper).
   - Fail-soft Git: sin `SUBJECT:`/`FILES:`; `synthesized` sigue dependiendo de Gemini, no de Git.
   - Fail-soft Gemini (cualquier `Err` / success false): mensaje estático, sin bloque síntesis (ya existe).
3. **Lab:** tests sin red. No exigir `GEMINI_API_KEY`. `SDDIA_LAB_MOCK_OUTBOUND` donde se invoque la tool; no usar el string `lab-mock:` como oracle de negocio.

**Fuera del arnés:** HTTP 429/500/JSON roto **dentro** de `gemini-http-infer` (ya mapeados a fail-soft del handler vía `try_infer_synthesis` → `None`). No duplicar la matriz HTTP en `execute-process`. No segundo modelo juez. No sanitizer de paths en el texto LLM (FA-K-8).

---

## 3. Criterios de Aceptación (Protocolo de Acero)

| ID | Criterio | Verificación |
|----|----------|--------------|
| K-DIFF-CA1 | Frozen v1.2.0 declara `commit_summary` con payload/salida de §1.1; uuid frozen intacto; evolution `modificacion`; **sin** `norm-creator` y **sin** tocar `cumulo.paths.json`. | Diff de `skill-io-git-manager-frozen.md` + registro evolution; `git grep` sin alta nueva en `library/norms/`. |
| K-DIFF-CA2 | Cápsula implementa `commit_summary` (`payload_exact`, `assert_safe_token`, límites, first-parent names-only, sin parche). Tests en la crate. | `cargo test -p git-manager`. |
| K-DIFF-CA3 | `git-manager.md` v1.2.0 enum + git-read-state alineados; uuid skill intacto. | Inspección del `{name}.md` e `skills/index.md`. |
| K-DIFF-CA4 | Action declara `delegate-git-manager`; se retira el límite «No ampliar git-manager»; uuid action intacto; forja EM `update`. | `SddIA/actions/notify-humanized-pr-merged.md`. |
| K-DIFF-CA5 | Handler usa `invoke_git_manager` + fail-soft a prompt ECST-only. Sin timeout nuevo en `capsules.rs`. | Test lib: Git `Err` → prompt sin FILES/SUBJECT; Telegram no falla por Git. |
| K-LLM-CA1 | Golden + tests lib cubren inyección SUBJECT/FILES, truncado, fail-soft Git y fail-soft Gemini. | `cargo test -p execute-process --lib -- notify_humanized`. |
| K-LLM-CA2 | Cero red viva obligatoria en esos tests. | Ejecución CI/lab desconectada; sin API key real requerida. |
| K-LLM-CA3 | Ningún artefacto de pesos, datasets de entrenamiento ni scripts de fine-tuning. | Diff del PR; DA-2 / `external-ai-constraints`. |
| K-EDA-CA1 | Topología `PullRequest_Merged` intacta: IOTA primero, action humanizada segunda; cero `tool: send-telegram-notification` en esa clave. | Test existente `pull_request_merged_subscription_is_humanized_action_not_telegram_tool`. |

---

## 4. Fuera de Alcance

- Reabrir o documentar el cierre de #262 / `accept-pr` / `FAIL_F4_RBAC`.
- Mutar `send-telegram-notification` o el contrato I/O de `gemini-http-infer`.
- Versionar `PullRequest_Merged` (ECST v1.0.0).
- Parches unificados / diffs de contenido en el prompt Gemini.
- Reutilizar `diff_name_only` o `get_last_commit` como sustituto de `commit_summary`.
- `norm-creator` / `entity_class: norm` sobre normas motor.
- `entity-manager` `skill` update/create sobre `git-manager.md` (regeneraría uuid).
- Timeout nuevo en `invoke_capsule_subprocess` / `capsules.rs`.
- Timeout Git ≤ 3s.
- Sanitizer post-LLM de rutas; segundo modelo juez; oracle de copy ilustrativo de ola 2.
- Matriz HTTP 429/500 en `execute-process` (pertenece a la tool Gemini).
- Entrenamiento, fine-tuning, cuantización, self-hosting.
- `gh` / `curl` ad-hoc.
- Alta de `delegate-git-manager` en `capability-taxonomy` (la action padre ya usa `delegate-*` fuera de ese catálogo).
- Mutar `cumulo.paths.json`.
