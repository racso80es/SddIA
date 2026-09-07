---
feature_name: kaizen-email-triage-infer-20260907
created: "2026-09-07"
process: feature
branch_name: feat/kaizen-email-triage-infer-20260907
persist_ref: docs/features/kaizen-email-triage-infer-20260907
pbi_ref: docs/todos/pending/[KAIZEN] Triaje de correo — batería 20260907 inferencia nula.md
execution_id: "a55a6cc1-b980-4205-abe5-1a4702765c96"
document_id: PBI-KAIZEN-EMAIL-TRIAGE-BATCH-20260907
pbi_uuid: "b4b41c88-d90b-47fc-bf18-94efd37f2298"
pbi_version: "1.2.0"
slice: 1
---

# Objetivos — kaizen-email-triage-infer-20260907

## Misión

Cerrar F-TRIAGE-02 / `DT-TRIAGE-LLM-QUALITY` en el consumidor `classify_llm`: peaje cognitivo medible **o** fallo de cápsula etiquetado. No ampliar L-GUARD. Cero IMAP STORE.

## Alcance (Slice 1)

1. Extraer `tokens_in`/`tokens_out` vía `telemetry_receipt::extract_from_capsule_body` (`prompt_tokens`/`completion_tokens` en `data.telemetry_receipt`).
2. Si `CapsuleInvokeResult.exit_code != 0` o `success != true`: `classification_error` + `classification-degraded` (si REQUIRE_INFER y sin L-GUARD); no tratar stdout vacío como veredicto.
3. Tests CA-1, CA-2, CA-3, CA-4. L-GUARD y muro C-LIST intactos.
4. Batería de ensayo versionada en este `persist_ref`.

## Fuera de gate

F-TRIAGE-03 (WUI). F-TRIAGE-06 (`persist_agenda` fechas pasadas / dedup). Router Multi-LLM. Mutar `email-triage-matrix` / keywords comerciales. Bind `SDDIA_LLM_INFER_COMMAND` en `mayeuta-llm` (deuda de cápsula, no este slice).

## Ley aplicada

- `email-triage-matrix` v1.1.0. Laudo D3.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CI verde (`run_id`).
- DA-2: handler `email_triage.rs` no es genoma indexado. Prohibido entity-manager innecesario.
- Git vía proceso; DCC abre PR; `accept-pr` solo post-CI verde.

## Criterios

| ID | Criterio |
|----|----------|
| CA-1 | Tokens desde `telemetry_receipt`; tokens > 0 no activan `classification-degraded`. |
| CA-2 | Cápsula `success: false` / `exit_code != 0` → extras `classification_error` + degradado fail-open `passive` (salvo L-GUARD). |
| CA-3 | Asunto reunión+fecha → `actionable` con cápsula sana y degradada. |
| CA-4 | `List-Id`/`List-Unsubscribe` → `noise` `deterministic` `C-LIST`; Clasificacion skipped. |
| CA-6 | Cero keywords factura/documentación/computrabajo en L-GUARD. |
| CA-7 | Cero STORE/EXPUNGE. |
| CA-8 | F-TRIAGE-03/06 no bloquean merge Slice 1. |
