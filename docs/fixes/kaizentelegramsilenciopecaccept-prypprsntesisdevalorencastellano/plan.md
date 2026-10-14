---

feature_name: kaizentelegramsilenciopecaccept-prypprsntesisdevalorencastellano
created: "2026-10-14"
updated: "2026-10-14T16:27:00Z"
process: bug-fix
branch_name: fix/kaizentelegramsilenciopecaccept-prypprsntesisdevalorencastellano
persist_ref: docs/fixes/kaizentelegramsilenciopecaccept-prypprsntesisdevalorencastellano
pbi_ref: docs/todos/done/[KAIZEN] Telegram — silencio PEC accept-pr y PPR, síntesis de valor en castellano.md
document_id: PBI-KAIZEN-TELEGRAM-PEC-SILENCE-VALUE-ES
uuid: "b2e8d1f4-6a3c-4e9b-8f0d-1a7c5e9b2d84"
design_status: sealed
blueprint_process: false
phases:
  - l1-pec-silence
  - l2-synth-es-omit
  - tests-unit
  - genome-doc-align-optional
correlation_id: "09a61ea3-b2d9-43c3-b65d-b815385e9a22"
agent: dedalo
phase: Diseño del fix
tg_ca7_language_input: discarded
design_pass: "reaffirm-dedalo-7c65bbcf"
prior_persist_ref: docs/fixes/kaizen-telegram-pec-silence-value-es
execution_id: "7c65bbcf-c809-4eb2-9fcd-3bec42aba37c"
---
# Plan — kaizentelegramsilenciopecaccept-prypprsntesisdevalorencastellano

Blueprint de **ejecución** (no proceso nuevo: `blueprint_process: false`). Orden: L1 → L2 → batería unitaria → alineación documental opcional. Sin forge de contrato `gemini-http-infer` (TG-CA7 descartado).

Estado pasada Dedalo `7c65bbcf`: diseño sellado bajo este `persist_ref`; código L1/L2 ya materializado y coherente con spec (y con `prior_persist_ref` / PR #346). Pendiente fuera de Dedalo: Tekton (`implementation.md`/`execution.md`) y Argos/`validacion.md` si el orquestador exige cascada local; git solo vía `skill:git-manager`.

## Fase L1 — Silencio PEC

Archivo: `SddIA/engine/execute-process/src/engine/route_domain_core.rs` — `build_telegram_message_from_event`.

1. Predicado `pec_telegram_silent(process_name, status)` al inicio de la rama `Process_Execution_Completed`.
2. Pares silenciados: `(accept-pr, success)`, `(pull-request-review, failed)` → `None`.
3. Resto: ensamblaje ADU-CA11–CA14 intacto.
4. Tests unitarios: expect `None` en CA1/CA2; no-silencio CA3 (+ regresión ADU).
5. No tocar `event-orchestration-subscriptions.json`.

## Fase L2 — Síntesis ES + omisión explícita

Archivo: `SddIA/engine/execute-process/src/engine/notify_humanized_pr_merged.rs`.

1. Reescribir instrucción de `build_synthesis_prompt_with_git` a castellano (conservar `PROMPT_KERNEL`).
2. Introducir `SynthesisOutcome` y cablear `try_infer_synthesis` → motivos tipados.
3. `assemble_humanized_message`: Ready | `(omitida — inferencia no disponible)` | `(omitida — texto vacío)`.
4. Tests: prompt ES; ensamblaje CA4/CA5.

## Fase tests

- Humanizador: `tg_ca1_*`, `tg_ca2_*`, `tg_ca3_*` (+ no romper ADU-CA11–14).
- Notify: prompt castellano, outcome omit, assemble Ready.
- `cargo test` acotado al crate `execute-process` — **Argos / operador** con evidencia vía runtime / `git-manager` (no Shell IDE como SSOT).

## Fase opcional — Genoma documental

Solo si se sincroniza el bloque de prompt en `SddIA/actions/notify-humanized-pr-merged.md`:

1. `entity-manager` (bump versión / cuerpo; **sin** input `language`).
2. Prohibido editar a mano `SddIA/actions/`, `SddIA/tools/gemini-http-infer.md`.

## Handoff post-diseño

- Tekton: `implementation.md` + `execution.md` (reafirmación si código ya en árbol; sin reescribir L1/L2 salvo regresión).
- Argos: `validacion.md` tras evidencia unitaria de esta ejecución si el runtime la exige.
- Git: checkout/create `fix/kaizentelegramsilenciopecaccept-prypprsntesisdevalorencastellano` solo vía `skill:git-manager` (`./sddia-run.sh --tool git-manager`).
- Sin mutaciones en `docs/todos/` desde Dedalo/Tekton/Argos (Cúmulo / Kaizen_Alert_Required). PBI ya en `done/`.
