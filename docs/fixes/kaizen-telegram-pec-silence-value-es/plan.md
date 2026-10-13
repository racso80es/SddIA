---





feature_name: kaizen-telegram-pec-silence-value-es
created: "2026-10-13"
updated: "2026-10-13"
process: bug-fix
branch_name: fix/kaizen-telegram-pec-silence-value-es
persist_ref: docs/fixes/kaizen-telegram-pec-silence-value-es
pbi_ref: docs/todos/done/[KAIZEN] Telegram — silencio PEC accept-pr y PPR, síntesis de valor en castellano.md
document_id: PBI-KAIZEN-TELEGRAM-PEC-SILENCE-VALUE-ES
uuid: "a9d2c7e4-5f18-4b6a-9e3d-0c8f1a2b4d67"
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
design_pass: "reaffirm-dedalo-f0d9a570"
execution_id: "19a1785f-ae5a-49af-8e21-22cacbba7b34"
---
# Plan — kaizen-telegram-pec-silence-value-es

Blueprint de **ejecución** (no proceso nuevo: `blueprint_process: false`). Orden: L1 → L2 → batería unitaria → alineación documental opcional. Sin forge de contrato `gemini-http-infer` (TG-CA7 descartado).

Estado pasada Dedalo `f0d9a570`: diseño sellado; código L1/L2 ya materializado y coherente con spec. Pendiente (fuera de Dedalo): evidencia `cargo test` + rama `fix/kaizen-telegram-pec-silence-value-es` vía `skill:git-manager` → Argos/`validacion.md`.

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
- `cargo test` acotado al crate `execute-process` (filtros por nombre de test) — **Argos / operador** con Shell aprobado.

## Fase opcional — Genoma documental

Solo si se sincroniza el bloque de prompt en `SddIA/actions/notify-humanized-pr-merged.md`:

1. `entity-manager` (bump versión / cuerpo; **sin** input `language`).
2. Prohibido editar a mano `SddIA/actions/`, `SddIA/tools/gemini-http-infer.md`.

## Handoff post-diseño

- Tekton: `implementation.md` + `execution.md` (si código incompleto) o reafirmación de evidencia.
- Argos: `validacion.md` tras batería unitaria.
- Git: checkout/create `fix/kaizen-telegram-pec-silence-value-es` solo vía `skill:git-manager` (`./sddia-run.sh --tool git-manager`).
- Sin mutaciones en `docs/todos/` desde Dedalo/Tekton/Argos (Cúmulo / Kaizen_Alert_Required).
