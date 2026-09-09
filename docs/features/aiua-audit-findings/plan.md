---
feature_name: aiua-audit-findings
created: "2026-09-09"
process: feature
phases:
  - docs-design
  - crate-thinking-level
  - forge-actions-em
  - handler-preface
  - tests-lab-mock
  - docs-execution-dcc-ci-accept
branch_name: feat/aiua-audit-findings
persist_ref: docs/features/aiua-audit-findings
pbi_ref: docs/todos/pending/[KAIZEN] Aiúa — hallazgos auditoría live y thinking HIGH.md
document_id: PBI-AIUA-AUDIT-FINDINGS-20260908
uuid: "2fa76082-d3b1-49ed-af8e-0c1c33b7dd44"
execution_id: "1532d612-85d6-4b8e-9b9c-6c4979c3cce4"
---

# Plan — aiua-audit-findings

Corte diseño: clarify + objectives + spec + plan. **Commit planificación** antes de mutar crate/genoma/runtime.

## L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. PBI v1.0.0.

## L1 — Crate thinking (H-LLM-1b)

Tekton sobre `SddIA/tools/gemini-http-infer/src/main.rs` (ED ya forjada). Starter-kit `.env.example` (clave comentada, sin slug). Tests crate.

## L2 — Forja acciones (H-LLM-2)

`./sddia-run.sh --process entity-manager` (relay IDE) ×3 `update` + `action_body`. Si EM aborta por revoked: **stop** (DA-2). No Write sobre `SddIA/actions/`.

## L3 — Prefacio (H-LLM-3)

`handlers::aiua_stimulus::invoke_aiua_core`. Parse frontmatter. Tests handler.

## L4 — Docs + evolution + PR

`implementation.md` / `execution.md` / `validacion.md` (`CA-CI: PENDIENTE-CI` hasta verde). `sddia-qa evolution-register`. DCC. PBI a `done/` al archivar **tras** CI verde en el mismo PR.

## L5 — CI → accept-pr

`global: APTO` solo con `run_id` verde. Entonces `accept-pr`.

## Fuera

Kalma2. IOTA. Agente Tormentosa. Antigravity CLI. Retry 503. Reabrir #270.
