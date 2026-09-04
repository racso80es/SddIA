---
feature_name: arquitectura-llm-tiers
created: "2026-09-04"
process: feature
items:
  - agents-contract-llm-profile
  - agent-yaml-llm-profile
  - agent-runtime-payload
  - harness-resolve-model
  - starter-kit-tier-env
---

# Implementation — arquitectura-llm-tiers

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/agents/agents-contract.md` | v1.1.0; § `llm_profile` opcional |
| `SddIA/agents/{cerbero,cumulo,radamanto,dedalo,mayeuta,argos,tekton}.md` | `llm_profile` en YAML; UUID intacto |
| `SddIA/agents/argos.md` | L-ARGOS-SYNTHESIS en doctrina |
| `SddIA/agents/index.md` | Nota de perfiles (sin columna nueva) |
| `agent_runtime.rs` | `load_llm_profiles` + veto none + campo payload |
| `kalma2-agent-runtime-cursor.py` | `resolve_phase_model`; SDK usa el id |
| starter-kit `.env.example` | `SDDIA_LLM_TIER_*` comentadas |

`entity-manager update` de agente **no** usado: `run_agent_forge` en update regenera UUID.
