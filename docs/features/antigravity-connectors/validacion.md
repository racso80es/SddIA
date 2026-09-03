---
feature_name: antigravity-connectors
created: "2026-09-03"
process: feature
phase: validate
agents: argos
branch: feat/antigravity-connectors-8989250975201761652
branch_name: feat/antigravity-connectors-8989250975201761652
persist_ref: docs/features/antigravity-connectors
pbi_ref: "docs/todos/done/[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI).md"
document_id: PBI-CAPSULES-ANTIGRAVITY-NATIVE
uuid: "7f966f32-5502-4bd7-b252-44849f29f5d3"
global: APTO
pbi_archived: true
checks:
  CA_forge: APTO
  CA_compile: APTO
  CA_blindness: APTO
  CA_http_lab: APTO
  CA_cli_argv: APTO
  CA_envelope: APTO
  CA_di: APTO
  CA_index: APTO
git_changes:
  - SddIA/tools/gemini-http-infer.md
  - SddIA/tools/gemini-http-infer/
  - SddIA/tools/index.md
  - SddIA/skills/antigravity-cli-executor.md
  - SddIA/skills/antigravity-cli-executor/src/main.rs
  - SddIA/skills/index.md
  - SddIA/skills/antigravity-http-connector.md
  - SddIA/sddia-io/src/outbound_lab.rs
  - SddIA/scripts/qa/build-wasi-capsules.sh
  - SddIA/core/eda-coverage.json
  - SddIA/Cargo.lock
  - docs/features/antigravity-connectors/
  - docs/todos/done/[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI).md
---

# Validacion — antigravity-connectors

`global: APTO` por CAs de laboratorio (PBI §7). Red Google / `agy` autenticado: fuera de gate.

## Checks

| CA | Veredicto | Evidencia |
|----|-----------|-----------|
| Forge | APTO | create tool `7a8da3ad-…`; skill sellada `d8b07e6f-…`; delete HTTP Jules `b548b894-…` |
| Compile | APTO | `cargo test -p gemini-http-infer -p antigravity-cli-executor` 11 passed |
| Ceguera | APTO | sin `find_repo_root`; env `GEMINI_API_KEY` / `SDDIA_AGY_PATH` |
| HTTP lab | APTO | `SDDIA_LAB_MOCK_OUTBOUND=1` success; sin key `success:false` |
| CLI argv | APTO | tests: sandbox default; skip solo doble opt-in; stub spawn JSON |
| Envelope | APTO | smoke stdout `meta.schemaVersion=2.0` |
| DI | APTO | cero `provides`; taxonomía/bindings intactos |
| Índice/EDA | APTO | `verify-tools-index` OK; `verify-process-integrity` OK; `orphan_count: 0` |

## Fuera de gate

CI GitHub Actions del PR: no es CA de red Gemini. Mock lab cubre el contrato.
