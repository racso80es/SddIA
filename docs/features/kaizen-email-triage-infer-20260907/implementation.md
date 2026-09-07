---
feature_name: kaizen-email-triage-infer-20260907
created: "2026-09-07"
process: feature
items:
  - T1-tokens-receipt
  - T2-capsule-fail-gate
  - T3-tests-slice1
  - T4-evolution
branch_name: feat/kaizen-email-triage-infer-20260907
persist_ref: docs/features/kaizen-email-triage-infer-20260907
execution_id: "a55a6cc1-b980-4205-abe5-1a4702765c96"
document_id: PBI-KAIZEN-EMAIL-TRIAGE-BATCH-20260907
---

# Implementation — kaizen-email-triage-infer-20260907

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/engine/execute-process/src/engine/handlers/email_triage.rs` | `tokens_from_capsule_body` vía `telemetry_receipt::extract_from_capsule_body`; gate `exit_code`/`success`; `classification_error` en extras; `degrade_without_llm` |
| `SddIA/evolution/29e6cbc5-6d13-45b9-bd62-f24e3fdb0c45.md` | Registro Slice 1 |
| `SddIA/evolution/Evolution_log.md` | Fila correlato |

## Handler (`classify_llm`)

1. `Err(spawn)` → `degrade_without_llm` (`classification_error` + L-GUARD).
2. `Ok` con `exit_code != 0` o `success != true` → igual; texto de `body.error` / `body.message`.
3. `Ok` éxito → parse existente; tokens desde `data.telemetry_receipt.{prompt_tokens,completion_tokens}` (fallback raíz `tokens_in`/`tokens_out`).

## Fuera de este PR

`mayeuta-llm` `SDDIA_LLM_INFER_COMMAND`. WUI F-TRIAGE-03. `persist_agenda` F-TRIAGE-06. Keywords L-GUARD. Matriz. IMAP STORE.
