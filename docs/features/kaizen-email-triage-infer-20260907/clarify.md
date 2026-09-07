---
feature_name: kaizen-email-triage-infer-20260907
created: "2026-09-07"
process: feature
purpose: Estabilización Filtro A PBI v1.2.0; Slice 1 peaje/deglución
version_clarify: "1.0.0"
execution_id: "a55a6cc1-b980-4205-abe5-1a4702765c96"
pbi_ref: docs/todos/pending/[KAIZEN] Triaje de correo — batería 20260907 inferencia nula.md
document_id: PBI-KAIZEN-EMAIL-TRIAGE-BATCH-20260907
pbi_uuid: "b4b41c88-d90b-47fc-bf18-94efd37f2298"
pbi_version: "1.2.0"
slice: 1
---

# Clarificación — kaizen-email-triage-infer-20260907

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `a55a6cc1-b980-4205-abe5-1a4702765c96`.

## Decisiones

| ID | Laudo |
|----|-------|
| L-SLICE | Solo Slice 1 (consumidor `classify_llm`). CA-5 (LLM vivo end-to-end) no es gate de merge: exige CLI de instancia. Tests cubren envelope + fail path. |
| L-HANDLER | Mutar `email_triage.rs`. Cero cápsula nueva. Cero bump de proceso/evento/matriz. |
| L-RECEIPT | Usar `telemetry_receipt::extract_from_capsule_body(&body, "skill:mayeuta-llm")`. Fallback raíz `tokens_in`/`tokens_out` por si un mock de test aún usa ese shape. |
| L-FAIL | `invoke_capsule_json` `Ok` no implica éxito de negocio. Gate: `exit_code == 0` ∧ `success == true`. `Err(spawn)` también `classification_error`. |
| L-D3 | Prohibido tocar `extract_actionable_from_subject` keywords. |
| L-C4 | C-LIST ya implementado; test de regresión, no reescritura del muro. |
| L-IMAP | Aserción negativa STORE en fuente (test existente / grep). |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo entonces. |

## Filtro A (no reintroducir)

- UUIDs v1.1.0 de proofs 5816–5818 y G5 `6e552199-65b1-…` eran alucinación.
- `classification-degraded` no es incondicional (L-GUARD lo omite).
- Asiento = `persist_agenda` inline, no cápsula `agenda-manager`.
