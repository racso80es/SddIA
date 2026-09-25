---
feature_name: multi-llm-router
created: "2026-09-25"
process: feature
phase: validate
branch: feat/multi-llm-router
branch_name: feat/multi-llm-router
persist_ref: docs/features/multi-llm-router
pbi_ref: docs/todos/pending/PBI-MULTI-LLM-ROUTER.md
document_id: PBI-MULTI-LLM-ROUTER
uuid: "d2e44083-ccdf-45af-b477-f6c71833fc31"
global: PENDIENTE-CI
pbi_archived: false
---

# Validación — multi-llm-router

`global: PENDIENTE-CI`. APTO + archivo PBI solo tras run GitHub `pull_request` `conclusion: success` en el `headSha` a fusionar.

| CA | Estado local | Evidencia |
|----|--------------|-----------|
| CA-CONTRACT | PENDIENTE-CI | `llm.infer.schema.json` + taxonomía v1.0.8 `llm:infer` |
| CA-REGISTRY | PENDIENTE-CI | schema + Cúmulo `instance.llm_registry` + example `model: ""` |
| CA-ROUTER | PENDIENTE-CI | tests `llm-router` (affinity, fallback 429, no-salto auth, ciclo, registry missing) |
| CA-ADAPTERS | PENDIENTE-CI | tables `classify_error_code` gemini/agy; `provides llm:infer` |
| CA-AIUA | PENDIENTE-CI | genoma `tool:llm-router`; lab fallback `routing_attempts==1` |
| CA-ORTHO | PENDIENTE-CI | sin diff `agent_runtime.rs`, `capability-bindings.md`, `SddIA/agents/`, `kalma2-bridge` |
| CA-FAILSOFT | PENDIENTE-CI | registro ausente → `llm-registry-missing`; auth corta cadena |
| CA-GOV | PENDIENTE-CI | entity-manager + evolution `d2e44083-…` |
| CA-DOC | PENDIENTE-CI | cascada completa; archivo PBI en el mismo PR tras CI verde |
