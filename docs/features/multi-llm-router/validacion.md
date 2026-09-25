---
feature_name: multi-llm-router
created: "2026-09-25"
process: feature
phase: validate
branch: feat/multi-llm-router
branch_name: feat/multi-llm-router
persist_ref: docs/features/multi-llm-router
pbi_ref: docs/todos/done/PBI-MULTI-LLM-ROUTER.md
document_id: PBI-MULTI-LLM-ROUTER
uuid: "d2e44083-ccdf-45af-b477-f6c71833fc31"
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/297
ci_run_id: "36110156034"
ci_run_url: https://github.com/racso80es/SddIA/actions/runs/36110156034
ci_head_sha: "db374b3df52939a46fd42483a8a84e0ec67c364f"
checks:
  CA-CONTRACT: APTO
  CA-REGISTRY: APTO
  CA-ROUTER: APTO
  CA-ADAPTERS: APTO
  CA-AIUA: APTO
  CA-ORTHO: APTO
  CA-FAILSOFT: APTO
  CA-GOV: APTO
  CA-DOC: APTO
git_changes:
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/library/norms/capability-contracts/llm.infer.schema.json
  - SddIA/library/norms/capability-contracts/llm-registry.schema.json
  - SddIA/tools/llm-router.md
  - SddIA/tools/llm-router/src/main.rs
  - SddIA/tools/gemini-http-infer.md
  - SddIA/skills/antigravity-cli-executor.md
  - SddIA/process/aiua-stimulus-processing.md
  - SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs
  - SddIA/evolution/d2e44083-ccdf-45af-b477-f6c71833fc31.md
  - docs/features/multi-llm-router/
  - docs/todos/done/PBI-MULTI-LLM-ROUTER.md
---

# Validación — multi-llm-router

`global: APTO`. Run [36110156034](https://github.com/racso80es/SddIA/actions/runs/36110156034) evento `pull_request`, head `db374b3df52939a46fd42483a8a84e0ec67c364f`, `conclusion: success`.

| CA | Estado | Evidencia |
|----|--------|-----------|
| CA-CONTRACT | APTO | `llm.infer.schema.json` + taxonomía v1.0.8 `llm:infer` |
| CA-REGISTRY | APTO | schema + Cúmulo `instance.llm_registry` + example `model: ""` |
| CA-ROUTER | APTO | tests `llm-router` (affinity, fallback 429, no-salto auth, ciclo, registry missing) |
| CA-ADAPTERS | APTO | tables `classify_error_code` gemini/agy; `provides llm:infer` |
| CA-AIUA | APTO | genoma `tool:llm-router`; lab `lab_rate_limited_primary_falls_back_to_gemini` |
| CA-ORTHO | APTO | sin diff `agent_runtime.rs`, `capability-bindings.md`, `SddIA/agents/`, `kalma2-bridge` |
| CA-FAILSOFT | APTO | registro ausente → `llm-registry-missing`; auth corta cadena |
| CA-GOV | APTO | entity-manager + evolution `d2e44083-…` hash `sha256:951fe4d1…` |
| CA-DOC | APTO | cascada completa; PBI en `done/` en este PR |
