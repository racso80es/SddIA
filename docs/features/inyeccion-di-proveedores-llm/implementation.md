---
feature_name: inyeccion-di-proveedores-llm
created: "2026-11-23"
process: feature
branch_name: feat/inyeccion-di-proveedores-llm
persist_ref: docs/features/inyeccion-di-proveedores-llm
tracker_ref: AUT-122
historia_ref: AUT-121
items:
  - "SddIA/agents/agents-contract.md"
  - "SddIA/agents/mayeuta.md"
  - "SddIA/agents/dedalo.md"
  - "SddIA/agents/argos.md"
  - "SddIA/agents/tekton.md"
  - "SddIA/agents/cerbero.md"
  - "SddIA/agents/cumulo.md"
  - "SddIA/agents/radamanto.md"
  - "SddIA/agents/index.md"
  - "SddIA/library/norms/capability-contracts/agent.runtime.schema.json"
  - "SddIA/library/norms/capability-contracts/index.md"
  - "SddIA/library/norms/capability-taxonomy.md"
  - "SddIA/library/norms/index.md"
  - "SddIA/core/capability-bindings.md"
  - "SddIA/core/cumulo.paths.json"
  - "SddIA/core/eda-coverage.json"
  - "SddIA/actions/switch-llm-provider.md"
  - "SddIA/actions/index.md"
  - "SddIA/infrastructure/adapters/cursor-agent-runtime.md"
  - "SddIA/infrastructure/adapters/antigravity-agent-runtime.md"
  - "SddIA/infrastructure/adapters/local-llm-agent-runtime.md"
  - "SddIA/infrastructure/adapters/index.md"
  - ".SddIA/llm-runtime.example.json"
  - ".gitignore"
  - "SddIA/engine/execute-process/src/engine/agent_runtime.rs"
  - "SddIA/engine/execute-process/src/engine/capsule_paths.rs"
  - "SddIA/engine/execute-process/src/engine/dlt_telemetry_anchor.rs"
  - "SddIA/engine/execute-process/src/engine/executor.rs"
  - "SddIA/engine/execute-process/src/engine/actions.rs"
  - "SddIA/engine/execute-process/src/engine/llm_interaction_audit.rs"
  - "SddIA/engine/execute-process/src/forges/capability_contract.rs"
  - "SddIA/engine/execute-process/src/forges/factory.rs"
  - "SddIA/engine/execute-process/tests/dlt_telemetry.rs"
  - "SddIA/evolution/d122a000-2026-4123-8122-000000000122.md"
  - "SddIA/evolution/Evolution_log.md"
execution_id: "f1220000-2026-4123-8122-000000000122"
---
# Implementación — inyeccion-di-proveedores-llm (`AUT-122`)

1. **Taxonomía cerrada de pensamiento en el Genoma (`agents-contract.md` v1.3.0 y 7 agentes Core):**
   - Evolucionado `SddIA/agents/agents-contract.md` a v1.3.0 vía `entity-manager`, incorporando `thinking_profile` (`reasoning | audit | execution | deterministic`) y conservando `tier` (`high | medium | low | none`) como alias retrocompatible.
   - Actualizados y sellados vía `entity-manager` los 7 manifiestos (`mayeuta` y `dedalo` → `reasoning`; `argos` → `audit`; `tekton` → `execution`; `cerbero`, `cumulo` y `radamanto` → `deterministic`) y `SddIA/agents/index.md`.
2. **Capacidad DI `agent:runtime`, Adaptadores y Configuración Local No Versionada:**
   - Creado `SddIA/library/norms/capability-contracts/agent.runtime.schema.json` (v1.0.0) vía `entity-manager` (`capability-contract`).
   - Actualizada `SddIA/library/norms/capability-taxonomy.md` a v1.0.13 vía `entity-manager` y `SddIA/core/capability-bindings.md` a v1.11.0 con la fila `agent:runtime` y `roles` (`cursor`, `antigravity`, `local-llm`, `test`).
   - Catalogados los adaptadores `cursor-agent-runtime.md`, `antigravity-agent-runtime.md` y `local-llm-agent-runtime.md` en `SddIA/infrastructure/adapters/` y su `index.md`.
   - Registradas `instance.llm_runtime_local` (`.SddIA/llm-runtime.local.json`, ignorada en `.gitignore`) e `instance.llm_runtime_example` (`.SddIA/llm-runtime.example.json`, plantilla versionada) en `SddIA/core/cumulo.paths.json` (v1.13.0).
3. **Puerto `AgentRuntimePort`, Evaluación Perezosa, `action:switch-llm-provider` y Prohibición de Auto-Failover:**
   - Refactorizado `SddIA/engine/execute-process/src/engine/agent_runtime.rs` con `ThinkingProfile`, `ProviderMode`, `UnattendedPolicy`, `AgentRuntimePort`, `AgentRuntimeDiFactory`, `CursorAgentRuntimeAdapter`, `AntigravityAgentRuntimeAdapter`, `LocalLlmAgentRuntimeAdapter` e `InMemoryAgentRuntimeMock` (`with_mock_agent_runtime`).
   - Blindada la evaluación perezosa para `ThinkingProfile::Deterministic` y el fallback limpio en clones sin `.SddIA/llm-runtime.local.json`.
   - Creada `SddIA/actions/switch-llm-provider.md` vía `entity-manager` (`Domain_Entity_Created`) y su handler nativo en `actions.rs` + `agent_runtime::switch_llm_provider`.
   - Blindado `resume_skip_or_block` en `executor.rs` para avanzar al reanudar (`resume: true`) desde `awaiting_agents` / `simulated` cuando los artefactos de diseño ya están listos tras un cambio de proveedor.
4. **Auditoría y Telemetría:**
   - Inyectados `agent_ids`, `thinking_profile`, `llm_provider`, `resolved_model` y `provider_mode` en los reportes de fase (`phase_reports.json` y `execution_report.json`).
   - Añadidos `AgentExecutionAuditRow`, `append_agent_execution` y `maybe_record_agent_phase_execution` bajo la sección `## Ejecuciones de Agente` en `llm_interaction_audit.rs`, manteniendo intacta la tabla de 4 columnas y `rejects_invalid_entity_type`.
