---


feature_name: kaizen-pr-aduana-friction-20261012
created: "2026-10-13"
updated: "2026-10-13"
process: bug-fix
branch_name: fix/kaizen-pr-aduana-friction-20261012
persist_ref: docs/fixes/kaizen-pr-aduana-friction-20261012
pbi_ref: docs/todos/pending/[KAIZEN] Aduana PR — corte LLM, dedupe Presented, persist_ref docs y sellos post-merge.md
document_id: PBI-KAIZEN-PR-ADUANA-FRICTION-20261012
uuid: "b2e8d4f1-6a3c-4f9e-8d7b-1c0a5e9f3b62"
design_status: sealed
blueprint_process: false
phases:
  - l3-persist-ref-docs
  - l4-scan-presented
  - l5-hygiene-idempotent
  - l1-verdict-barrier
  - l6-pec-telegram
  - l2-ppr-dedupe
  - tests-unit
correlation_id: "d96571be-c2e1-4a07-96be-efb0d18b81d8"
agent: dedalo
phase: Diseño del fix
execution_id: "5dcc8ba9-e1cc-4a09-a88f-b42de0a88b07"
---
# Plan — kaizen-pr-aduana-friction-20261012

Blueprint de **ejecución** (no proceso nuevo: `blueprint_process: false`). Orden: L3 → L4 → L5 → L1 → L6 → L2 → batería unitaria. Genoma solo si L2 exigiera clase ECST (diseño sellado: no).

## Fase L3 — `infer_persist_ref_from_branch`

Archivo: `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs`.

1. Rama `docs/{slug}`: candidatos features luego fixes; `is_dir`.
2. Tests CA5/CA6.
3. No tocar prompts Argos.

## Fase L4 — Escaneo Presented

Archivo: `accept_pr.rs` — `scan_presented_for_branch`.

1. Iterar pending / processed / dead-letter.
2. Tests tempdir + JSON Presented mínimo.

## Fase L5 — Higiene

Archivo: `accept_pr.rs` — `is_remote_ref_absent_error` / `record_hygiene_remote`.

1. Ref ausente → `ok: true` (`idempotent_absent`).
2. `closed_branch` solo local ∧ remoto OK.
3. Alinear `t_a2_local_ok_closed_branch`; tests CA8/CA9.

## Fase L1 — Barrier Veredicto PPR

Archivo: `residual_runner.rs` — `ppr_verdict_skip_llm` / `ppr_documental_triage_blocked`.

1. Tras Triaje documental blocked, Veredicto sin `invoke_agent_phase`.
2. Tests CA1/CA2.
3. No redirigir PPR al executor genérico.

## Fase L6 — PEC Telegram

Archivos: `thermodynamic.rs` (`pec_notice_from_context` / `project_pec_operator_fields`), `route_domain_core.rs` (`build_telegram_message_from_event`).

1. Payload: branch, pr_url, notice (prioridad §3.6 PBI).
2. Humanizador: contrato CA11–CA13.
3. Mantener `telegram_message_for_pec_includes_correlation`; caso sin notice.

## Fase L2 — Dedupe despacho

Archivo: `route_domain_core.rs` — `try_acquire_ppr_dedupe` / proof `ppr-presented-dedupe/`.

1. Clave `(branch, head_commit)`; fail-open si SHA irresoluble o error FS no-AlreadyExists.
2. Sin clase ECST nueva.
3. Tests CA3/CA4.

## Fase tests

```text
cargo test -p execute-process --lib adu_ca -- --test-threads=4
cargo test -p execute-process --lib telegram_message_for_pec -- --test-threads=4
cargo test -p execute-process --lib t_adu_ -- --test-threads=4
```

## Cierre (fuera de Diseño)

Tekton: evidencia tests + rama vía `skill:git-manager` → Argos `validacion.md` → PBI a `done/` → `delivery-close-cycle`.
