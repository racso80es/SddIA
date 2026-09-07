---
feature_name: kaizen-email-triage-infer-20260907
created: "2026-09-07"
process: feature
base: main
scope: email-triage-classify-llm-telemetry
version_spec: "1.0.0"
document_id: PBI-KAIZEN-EMAIL-TRIAGE-BATCH-20260907
uuid: "b4b41c88-d90b-47fc-bf18-94efd37f2298"
persist_ref: docs/features/kaizen-email-triage-infer-20260907
branch_name: feat/kaizen-email-triage-infer-20260907
execution_id: "a55a6cc1-b980-4205-abe5-1a4702765c96"
---

# Especificación — kaizen-email-triage-infer-20260907

## 1. Contrato de entrada/salida de Clasificacion

`classify_llm` invoca skill `mayeuta-llm` (`operation: SYNTHESIZE`). Envelope de cápsula (hechos):

```json
{"success": true, "data": {"text": "...", "telemetry_receipt": {"prompt_tokens": N, "completion_tokens": M}}, "error": null}
```

Fallo: `success: false`, `exit_code != 0`, `data` null, `error` string. `invoke_capsule_json` envuelve esto en `Ok(CapsuleInvokeResult { body, exit_code })` salvo error de resolución/spawn (`Err`).

## 2. Algoritmo post-invoke

1. `Err(e)` → extras `classification_error`, L-GUARD sobre asunto, coste 0, degradado si REQUIRE_INFER ∧ ¬elevado.
2. `Ok` con fallo de negocio (`exit_code != 0` ∨ `success != true`) → igual; `classification_error` desde `body.error` / `body.message`.
3. `Ok` éxito → `llm_output_blob` + parse + trampas comerciales + L-GUARD. Tokens: `extract_from_capsule_body`; si None, raíz `tokens_in`/`tokens_out`. `mark_classification_degraded` con esos tokens.

`decision_path` sigue siendo `llm` cuando la fase Clasificacion corre (aunque degrade). Coherente con matriz §5 (quien cerró el camino de clasificación, no «LLM infirió»).

## 3. No tocar

- `triaje_c` / `extract_actionable_from_subject` keywords.
- `persist_agenda` (F-TRIAGE-06).
- `kalma2-bridge` inbox filter (F-TRIAGE-03).
- `mayeuta-llm` `resolve_cli_raw` (deuda: no lee `SDDIA_LLM_INFER_COMMAND`).

## 4. Tests

| Test | Oráculo |
|------|---------|
| `tokens_from_nested_telemetry_receipt` | body `data.telemetry_receipt` prompt 10 / completion 5 → (10, 5) |
| `failed_capsule_sets_classification_error` | success false → extras error; veredicto L-GUARD o passive |
| L-GUARD + fail | asunto reunión+fecha → actionable, sin degradado si elevated |
| C-LIST | list_headers List-Id → noise deterministic (regresión) |
