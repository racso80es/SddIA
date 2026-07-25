---
feature_name: kalma2-agent-runtime-cursor-f3-km-residual
created: "2026-07-24"
process: feature
branch_name: feat/kalma2-agent-runtime-cursor-f3-km-residual
persist_ref: docs/features/kalma2-agent-runtime-cursor-f3-km-residual
document_id: PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL
pbi_uuid: 3d9bb1de-e45d-49fe-99f7-9b0b31d79c1d
spec_uuid: f3a91c2e-8b47-4d6e-a1c5-9e0d7b4f2a68
phase: Ejecución
agents: tekton
items:
  - T1 Evidence Bridge prótesis
  - T2 agent_runtime state forward
  - T3 smoke script
---

# Implementation — kalma2-agent-runtime-cursor-f3-km-residual

## Resumen

Evidence Bridge en path `kalma2-agent-runtime-cursor`: la prótesis materializa R1/R2 por subprocess (no Shell IDE); `agent_runtime.rs` reenvía flags nativos #125; prompt Argos acota KM a `docs/todos/**` (R3).

| # | Artefacto | Cambio | Estado |
|---|-----------|--------|--------|
| T1 | `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py` | `materialize_runtime_evidence`, bloque schema v1, gate Verificación/argos, prompt KM scoped | done |
| T2 | `SddIA/engine/execute-process/src/engine/agent_runtime.rs` | `inject_runtime_evidence_from_state` + test `runtime_evidence_forwards_native_state_flags` | done |
| T3 | `SddIA/scripts/tools/kalma2-evidence-bridge-smoke.sh` | Smoke MOCK/native/prompt/subprocess (L-TRUTH) | done · host 2026-07-25 `SMOKE T3 OK` |

## Contrato evidencia (v1)

Append en `{persist_ref}/_agent_handoff.md`:

- `schema: kalma2-agent-runtime-evidence/v1`
- `source: native_state | prosthesis_subprocess | none`
- Checks `TECH_FORMAL_EXECUTE_PROCESS` / `GIT_EVIDENCE_VIA_GIT_MANAGER`
- MOCK → `evidence_materialized: false`, `notes: mock`, ambos NO_APTO

## Explicitamente no tocado

Handlers `ppr-prep-branch` / `ppr-tech-triage` · genoma `pull-request-review.md` · DCC revoked/signer · PBI-042.
