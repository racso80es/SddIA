---
feature_name: inyeccion-di-proveedores-llm
created: "2026-11-23"
process: feature
branch_name: feat/inyeccion-di-proveedores-llm
persist_ref: docs/features/inyeccion-di-proveedores-llm
tracker_ref: AUT-122
historia_ref: AUT-121
phases:
  - id: T1
    name: "Capturar oráculo en rojo (Ola 0) en oracle-red.log y evolucionar Genoma (agents-contract v1.3.0, 7 agentes, agent.runtime.schema.json, capability-taxonomy, capability-bindings, action:switch-llm-provider) vía entity-manager"
    status: done
  - id: T2
    name: "Catalogar adaptadores SddIA/infrastructure/adapters/{cursor,antigravity,local-llm}-agent-runtime.md, actualizar cumulo.paths.json, .gitignore y crear plantilla .SddIA/llm-runtime.example.json"
    status: done
  - id: T3
    name: "Implementar AgentRuntimePort, AgentRuntimeDiFactory (evaluación perezosa), adaptadores v1 sin auto-failover y mock:in-memory-agent-runtime en agent_runtime.rs"
    status: done
  - id: T4
    name: "Implementar handler nativo action:switch-llm-provider, blindar resume_skip_or_block en executor.rs y propagar metadatos cognitivos a phase_reports.json / execution_report.json"
    status: done
  - id: T5
    name: "Extender llm_interaction_audit.rs con la sección ## Ejecuciones de Agente preservando intacta la tabla de 4 columnas y rejects_invalid_entity_type"
    status: done
  - id: T6
    name: "Registrar evolución en SddIA/evolution/, compilar binarios host, ejecutar suites cargo test + aduanas sddia-qa y sellar validacion.md en global: APTO"
    status: done
execution_id: "f1220000-2026-4123-8122-000000000122"
---
# Plan — inyeccion-di-proveedores-llm (`AUT-122`)

1. **T1 (Ola 0 y Evolución Genómica vía `entity-manager`):**
   - Capturar evidencia inicial en rojo en `docs/features/inyeccion-di-proveedores-llm/oracle-red.log`.
   - Crear `SddIA/library/norms/capability-contracts/agent.runtime.schema.json` vía `./sddia-run.sh --process entity-manager` (`entity_class: "capability-contract"`).
   - Actualizar `SddIA/library/norms/capability-taxonomy.md` (v1.0.13) vía `entity-manager` añadiendo `agent:runtime`.
   - Actualizar `SddIA/agents/agents-contract.md` (v1.3.0) y los 7 agentes (`mayeuta`, `dedalo`, `argos`, `tekton`, `cerbero`, `cumulo`, `radamanto`) con `thinking_profile` (`reasoning | audit | execution | deterministic`) vía `entity-manager` + actualizar `SddIA/agents/index.md`.
   - Crear `SddIA/actions/switch-llm-provider.md` vía `entity-manager` (`entity_class: "action"`, `lifecycle_operation: "create"`).
   - Actualizar `SddIA/core/capability-bindings.md` (v1.11.0) añadiendo `agent:runtime` con `roles` (`cursor`, `antigravity`, `local-llm`, `test`).
2. **T2 (Adaptadores de Infraestructura y Configuración Local `.SddIA/`):**
   - Crear fichas `SddIA/infrastructure/adapters/{cursor,antigravity,local-llm}-agent-runtime.md` y sus directorios verificables `cursor_agent_runtime`, `antigravity_agent_runtime`, `local_llm_agent_runtime`, actualizando `SddIA/infrastructure/adapters/index.md`.
   - Añadir `instance.llm_runtime_local` e `instance.llm_runtime_example` en `SddIA/core/cumulo.paths.json`.
   - Añadir `.SddIA/llm-runtime.local.json` a `.gitignore` y crear `.SddIA/llm-runtime.example.json`.
3. **T3 (Puerto `AgentRuntimePort` y Factoría Perezosa en `agent_runtime.rs`):**
   - Definir `ThinkingProfile`, `ProviderMode`, `UnattendedPolicy`, `AgentInvocationRequest`, `AgentExecutionResult`, `AgentRuntimePort` y `AgentRuntimeDiFactory`.
   - Implementar evaluación perezosa para `ThinkingProfile::Deterministic` y fallback limpio cuando `.SddIA/llm-runtime.local.json` no existe en CI/clones limpios.
   - Implementar adaptadores `cursor`, `antigravity`, `local-llm` y `InMemoryAgentRuntimeMock` (`mock:in-memory-agent-runtime`), con errores tipados (`LLM_MODEL_UNRESOLVED`, `CAPABILITY_BINDING_MISSING`, `PROVIDER_QUOTA_EXCEEDED`, `PROVIDER_TIMEOUT`, `PROVIDER_UNAVAILABLE`) y cero auto-failover.
4. **T4 (`action:switch-llm-provider` y Reanudación en `executor.rs`):**
   - Implementar handler nativo `switch-llm-provider` en `SddIA/engine/execute-process/src/engine/actions.rs`.
   - Blindar `resume_skip_or_block` en `executor.rs` para reanudación tras `awaiting_agents` / `simulated` con cambio de proveedor.
   - Inyectar `agent_ids`, `thinking_profile`, `llm_provider`, `resolved_model` y `provider_mode` en los reportes de fase (`phase_reports.json` y `execution_report.json`).
5. **T5 (Auditoría de Ejecuciones de Agente en `llm_interaction_audit.rs`):**
   - Añadir `AgentExecutionAuditRow`, `append_agent_execution` y `maybe_record_agent_phase_execution` bajo la sección `## Ejecuciones de Agente` en `docs/audits/{tracker_ref}.md`, manteniendo intacta la tabla de 4 columnas y `rejects_invalid_entity_type`.
6. **T6 (Evolution, Build/Paridad Host, Tests y Validación Argos):**
   - Registrar evolución en `SddIA/evolution/` y `Evolution_log.md`.
   - Compilar `cargo build --workspace` y `cargo build --workspace --release`, ejecutar `cargo test -p execute-process` y todas las aduanas `sddia-qa`, y sellar `validacion.md` en `global: APTO`.
