---

feature_name: kaizen-pr-aduana-friction-20261012
created: "2026-10-13"
process: bug-fix
branch_name: fix/kaizen-pr-aduana-friction-20261012
persist_ref: docs/fixes/kaizen-pr-aduana-friction-20261012
pbi_ref: docs/todos/pending/[KAIZEN] Aduana PR — corte LLM, dedupe Presented, persist_ref docs y sellos post-merge.md
document_id: PBI-KAIZEN-PR-ADUANA-FRICTION-20261012
uuid: "b2e8d4f1-6a3c-4f9e-8d7b-1c0a5e9f3b62"
phases:
  - l3-persist-ref-docs
  - l4-scan-presented
  - l5-hygiene-idempotent
  - l1-verdict-barrier
  - l6-pec-telegram
  - l2-ppr-dedupe
  - tests-unit
execution_id: "37dc1867-9940-499b-99a7-1e4d2d933a0d"
---
# Plan — kaizen-pr-aduana-friction-20261012

Orden: L3 → L4 → L5 (cambios locales acotados + tests) → L1 → L6 → L2 (despacho + proof) → batería unitaria cruzada. Sin blueprint de proceso nuevo; genoma solo si L2 exige clase ECST.

## Fase L3 — `infer_persist_ref_from_branch`

Archivo: `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs`.

1. Rama `docs/{slug}`: candidatos features luego fixes; `is_dir` en repo.
2. Tests unitarios en el mismo módulo (o `route_domain_core` si ya hay fixtures de paths).
3. No tocar prompts Argos.

## Fase L4 — Escaneo Presented

Archivo: `accept_pr.rs` — `scan_presented_for_branch`.

1. Iterar `pending`, `processed`, `dead-letter`.
2. Tests con tempdir + JSON mínimo Presented.

## Fase L5 — Higiene

Archivo: `accept_pr.rs` — `delete_branch_hygiene` / helper de clasificación stderr.

1. Clasificador «ref ausente» → `ok: true`.
2. `closed_branch` solo si local ∧ remoto OK.
3. Actualizar `t_a2_local_ok_closed_branch`; nuevos tests CA8/CA9.

## Fase L1 — Barrier Veredicto PPR

Archivo: `residual_runner.rs` (+ helper status si hace falta).

1. Tras Triaje documental `blocked`, fase Veredicto sin `invoke_agent_phase`.
2. Test: mock/state con phase_reports — Veredicto `blocked` determinista; camino feliz sigue invocando.
3. No redirigir PPR al executor genérico.

## Fase L6 — PEC Telegram

Archivos: `thermodynamic.rs` (proyección), `route_domain_core.rs` (`build_telegram_message_from_event`).

1. Payload: `branch`, `pr_url`, `notice` (prioridad §3.6 PBI).
2. Humanizador: contrato de líneas CA11–CA13.
3. Mantener `telegram_message_for_pec_includes_correlation`; tests nuevos sin `notice`.

## Fase L2 — Dedupe despacho

Archivo: `route_domain_core.rs` (pre-`dispatch_process_subscriber` para PPR).

1. Clave `(branch, head_commit)`; fail-open si SHA irresoluble.
2. Proof durable bajo proofs `ppr-presented-dedupe/`.
3. Clase ECST solo si CA3 exige bus suscrito → `entity-manager`.
4. Test: dos Presented misma clave → un dispatch; commit distinto → dos.

## Fase tests

```text
cargo test -p execute-process infer_persist_ref
cargo test -p execute-process accept_pr
cargo test -p execute-process telegram_message_for_pec
cargo test -p execute-process # filtros L1/L2 según nombres elegidos
```

## Cierre (fuera de Diseño)

Tekton: `implementation.md` + `execution.md` → Argos `validacion.md` → PBI a `done/` → `delivery-close-cycle`.
