---
feature_name: inyeccion-di-proveedores-llm
created: "2026-11-23"
process: feature
branch_name: feat/inyeccion-di-proveedores-llm
persist_ref: docs/features/inyeccion-di-proveedores-llm
tracker_ref: AUT-122
historia_ref: AUT-121
execution_id: "f1220000-2026-4123-8122-000000000122"
---
# Clarificación y Decisiones de Diseño — inyeccion-di-proveedores-llm (`AUT-122`)

- **Auditoría base:** `docs/audits/inyeccion-di-proveedores-llm-perfiles-pensamiento-20261123.md` (`audit-inyeccion-di-proveedores-llm-perfiles-pensamiento-20261123`)
- **Refinamiento previo:** `.SddIA/workspaces/refine-item/a1220000-2026-4123-8122-000000000122/clarify.md` (`PBI_Refined`: `eda://domain/48d130d9-1f96-4e0c-a8c2-0dfc491568c6`)

## Decisiones Cerradas e Invariantes

1. **D-1 — Taxonomía cerrada de pensamiento en el Genoma (`agents-contract.md` v1.3.0):**
   - `llm_profile.thinking_profile` admite exclusivamente `reasoning | audit | execution | deterministic`.
   - Mapeo canónico:
     - `reasoning`: `mayeuta`, `dedalo` (`tier: "high"`)
     - `audit`: `argos` (`tier: "medium"`)
     - `execution`: `tekton` (`tier: "low"`)
     - `deterministic`: `cerbero`, `cumulo`, `radamanto` (`tier: "none"`)
   - En fases multi-agente se resuelve por precedencia `reasoning > audit > execution > deterministic`.
   - Prohibido incrustar nombres comerciales de modelos en `SddIA/`.

2. **D-2 — Arquitectura DI (`agent:runtime`) y Configuración Local No Versionada:**
   - Capacidad `agent:runtime` (`contract: agent.runtime` v1.0.0) registrada vía `entity-manager` en:
     - `SddIA/library/norms/capability-contracts/agent.runtime.schema.json`
     - `SddIA/library/norms/capability-taxonomy.md` (v1.0.13)
     - `SddIA/core/capability-bindings.md` (v1.11.0, con `roles: cursor | antigravity | local-llm | test`)
   - Adaptadores estructurales en `SddIA/infrastructure/adapters/{cursor,antigravity,local-llm}-agent-runtime.md` + crates host ligeros bajo `SddIA/infrastructure/adapters/{cursor,antigravity,local_llm}_agent_runtime` e indexados en `SddIA/infrastructure/adapters/index.md` (`adapters-contract.md` §2 exige `impl_dir` físico verificable).
   - Configuración de instancia local en `.SddIA/llm-runtime.local.json` (registrada en `cumulo.paths.json` bajo `instance.llm_runtime_local`, ignorada en `.gitignore`, con plantilla versionada `.SddIA/llm-runtime.example.json`).
   - **Invariante de evaluación perezosa (`lazy evaluation`):** `AgentRuntimeDiFactory` se evalúa únicamente al ejecutar fases con `thinking_profile != deterministic`, y si `.SddIA/llm-runtime.local.json` no existe en un clon limpio o entorno de CI, utiliza defaults no disruptivos compatibles con el flujo existente (sin romper tests ni CI), activando errores estrictos (`CAPABILITY_BINDING_MISSING`, `LLM_MODEL_UNRESOLVED`) cuando la configuración declarada referencia un proveedor inexistente o carece de modelo para el perfil requerido en modo estricto.

3. **D-3 — Puerto `AgentRuntimePort`, `action:switch-llm-provider` y Prohibición de Auto-Failover:**
   - Trait `AgentRuntimePort` + `AgentRuntimeDiFactory` en `agent_runtime.rs`, con `InMemoryAgentRuntimeMock` (`mock:in-memory-agent-runtime`) para tests unitarios sin nuevas variables `SDDIA_LAB_*`.
   - Proveedores v1: `cursor` (CLI headless + modo interactivo), `antigravity` (relevo interactivo con pausa `awaiting_agents` + soporte CLI opcional) y `local-llm` (adaptador y contrato listos, devuelve `PROVIDER_UNAVAILABLE` si no hay endpoint configurado).
   - Sin auto-failover entre proveedores: códigos tipados `PROVIDER_QUOTA_EXCEEDED`, `PROVIDER_TIMEOUT`, `PROVIDER_UNAVAILABLE`.
   - Alta de `action:switch-llm-provider` vía `entity-manager` (`Domain_Entity_Created`, indexado en `SddIA/actions/index.md` y cubierto en `eda-coverage.json`) con handler nativo en `execute-process` que valida el proveedor contra `capability-bindings.md` y actualiza atómicamente `.SddIA/llm-runtime.local.json`.
   - Blindaje de `resume_skip_or_block` en `executor.rs` cuando una fase previa quedó en `awaiting_agents` o `simulated` y se reanuda (`resume: true`) tras cambiar de proveedor.

4. **D-4 — Auditoría y Fronteras (Out of Scope):**
   - Cada fase de agente en `phase_reports.json` y `execution_report.json` incluye `agent_ids`, `thinking_profile`, `llm_provider`, `resolved_model` y `provider_mode`.
   - `llm_interaction_audit.rs` mantiene intacta la tabla de 4 columnas `| entity_type | entity_id | invoked_by | timestamp |` y el test `rejects_invalid_entity_type`, añadiendo `append_agent_execution` que escribe en la sección `## Ejecuciones de Agente`.
   - **Fuera de alcance:** `tool:llm-router` / `.SddIA/llm-registry.json`, sincronización `.cursor/rules/` vs `.antigravity/rules/` y despliegue de servidor LLM local.
