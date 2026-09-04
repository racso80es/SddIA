---
feature_name: arquitectura-llm-tiers
created: "2026-09-04"
process: feature
branch_name: feat/arquitectura-llm-tiers
persist_ref: docs/features/arquitectura-llm-tiers
pbi_ref: docs/todos/pending/PBI-ARQUITECTURA-LLM-TIERS.md
execution_id: "ff0407a0-0458-4461-acc3-1beeb94e1aa0"
phases:
  - A-genome
  - B-orchestrator
  - C-harness
  - D-starter-kit
  - E-test
---

# Plan — arquitectura-llm-tiers

## A

`agents-contract.md` + siete agentes (`llm_profile`). UUID intacto. Nota en `index.md`.

## B

`SddIA/engine/execute-process/src/engine/agent_runtime.rs`: carga, payload, veto. Tests Cargo.

## C

`kalma2-agent-runtime-cursor.py`: `resolve_phase_model`. Selftest env.

## D

Ambos `.env.example` del starter-kit.

## E

`cd SddIA && cargo test -p execute-process llm_profile`
`SDDIA_HARNESS_SELFTEST=1 python3 SddIA/scripts/tools/kalma2-agent-runtime-cursor.py`
