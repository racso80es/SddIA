---
feature_name: kaizen-email-triage-infer-20260907
created: "2026-09-07"
process: feature
phases:
  - handler-classify-llm
  - tests-slice1
  - docs-execution
  - dcc-pr-ci-accept
branch_name: feat/kaizen-email-triage-infer-20260907
persist_ref: docs/features/kaizen-email-triage-infer-20260907
pbi_ref: docs/todos/pending/[KAIZEN] Triaje de correo — batería 20260907 inferencia nula.md
slice: 1
document_id: PBI-KAIZEN-EMAIL-TRIAGE-BATCH-20260907
uuid: "b4b41c88-d90b-47fc-bf18-94efd37f2298"
execution_id: "a55a6cc1-b980-4205-abe5-1a4702765c96"
---

# Plan — kaizen-email-triage-infer-20260907

Corte diseño: clarify + objectives + spec + plan + batería + PBI v1.2.0. **Commit planificación** antes de mutar `email_triage.rs`.

## L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. PBI refinado Filtro A v1.2.0.

## L1 — Handler

`SddIA/engine/execute-process/src/engine/handlers/email_triage.rs`:

- `tokens_from_capsule_body`
- Gate `exit_code`/`success` en `classify_llm`
- `classification_error` en extras
- import `telemetry_receipt::extract_from_capsule_body`

Core handler: no DA-2.

## L2 — Tests

```text
cd SddIA && cargo test -p execute-process --lib -- email_triage
```

## L3 — Docs de ejecución + DCC

`implementation.md` / `execution.md` / `validacion.md` (CI `PENDIENTE` hasta verde). `./sddia-run.sh --process delivery-close-cycle`. `accept-pr` solo con checks verdes.

## Fuera

`mayeuta-llm` env INFER. WUI. `persist_agenda`. Entity-manager.
