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
# Objetivos y Criterios de Aceptación — inyeccion-di-proveedores-llm (`AUT-122`)

## Objetivo

Desacoplar la ejecución cognitiva de los agentes del Core (`mayeuta`, `dedalo`, `tekton`, `argos` y los agentes deterministas `cerbero`, `cumulo`, `radamanto`) respecto al IDE o runtime concreto que provee el LLM (`cursor`, `antigravity`, `local-llm`), mediante una arquitectura de tres capas basada en Inyección de Dependencias (`agent:runtime`):

1. **Genoma de Agentes (`SddIA/agents/`):** Cada agente declara su perfil cognitivo abstracto mediante el enum cerrado `thinking_profile` (`reasoning | audit | execution | deterministic`), manteniendo `tier` (`high | medium | low | none`) como alias retrocompatible y prohibiendo slugs comerciales de modelos en `SddIA/`.
2. **Puerto y Adaptadores DI de Runtime (`agent:runtime`):** `agent_runtime.rs` se abstrae tras el puerto `AgentRuntimePort` y la factoría perezosa `AgentRuntimeDiFactory`, enlazados vía `capability-taxonomy.md`, `capability-bindings.md` (`roles: cursor | antigravity | local-llm | test`), el contrato `SddIA/library/norms/capability-contracts/agent.runtime.schema.json` y manifiestos `SddIA/infrastructure/adapters/{cursor,antigravity,local-llm}-agent-runtime.md`.
3. **Configuración Local de Instancia (`.SddIA/llm-runtime.local.json`) y Conmutación Determinista:** Un archivo local no versionado en Git define `active_provider`, `unattended_policy` (`pause_awaiting_agents | fail_fast`) y el mapeo `providers[provider_id].models[thinking_profile]`, con acción determinista `action:switch-llm-provider`, prohibición estricta de auto-failover y trazabilidad en `phase_reports.json`, `execution_report.json` y `llm_interaction_audit.rs`.

## Criterios de Aceptación

- [x] **AC-1 (Genoma y Taxonomía de Pensamiento):** `SddIA/agents/agents-contract.md` (v1.3.0) y los 7 manifiestos de `SddIA/agents/` declaran `llm_profile.thinking_profile` dentro del enum cerrado (`reasoning | audit | execution | deterministic`), manteniendo `tier` como alias retrocompatible, aplicando precedencia `reasoning > audit > execution > deterministic` en fases multi-agente y sin contener slugs comerciales de modelos en `SddIA/`.
- [x] **AC-2 (Capacidad DI `agent:runtime` y Configuración Local No Versionada):** `agent:runtime` (`agent.runtime` v1.0.0) está registrada en `capability-taxonomy.md`, `capability-bindings.md` (roles `cursor`, `antigravity`, `local-llm`, `test`), `agent.runtime.schema.json` y `SddIA/infrastructure/adapters/{cursor,antigravity,local-llm}-agent-runtime.md`; la selección de `active_provider`, `unattended_policy` y el mapeo `thinking_profile -> modelo` se resuelven perezosamente desde `.SddIA/llm-runtime.local.json` (ignorado en `.gitignore`, con plantilla `.SddIA/llm-runtime.example.json` versionada y ruta en `cumulo.paths.json`) y overrides en `.SddIA/.dev/.env`, fallando con `LLM_MODEL_UNRESOLVED` o `CAPABILITY_BINDING_MISSING` ante configuración explícita inválida sin romper clones limpios ni CI.
- [x] **AC-3 (Puerto `AgentRuntimePort`, Proveedores v1, `action:switch-llm-provider` y Prohibición de Auto-Failover):** `agent_runtime.rs` opera a través de `AgentRuntimePort` y `AgentRuntimeDiFactory` (con evaluación perezosa para `thinking_profile == deterministic` y `mock:in-memory-agent-runtime` en tests sin nuevas `SDDIA_LAB_*`), soporta los adaptadores v1 (`cursor`, `antigravity`, `local-llm`), incluye `action:switch-llm-provider` con cobertura EDA completa (`Domain_Entity_Created`), rechaza cualquier failover automático emitiendo códigos tipados (`PROVIDER_QUOTA_EXCEEDED`, `PROVIDER_TIMEOUT`, `PROVIDER_UNAVAILABLE`) y blinda `resume_skip_or_block` en `executor.rs` al reanudar (`resume: true`) desde `awaiting_agents` tras cambiar de proveedor.
- [x] **AC-4 (Auditoría, Telemetría y Aduanas de Cierre):** `phase_reports.json` y `execution_report.json` registran `agent_ids`, `thinking_profile`, `llm_provider`, `resolved_model` y `provider_mode` en cada fase de agente; `llm_interaction_audit.rs` registra la sección `## Ejecuciones de Agente` sin alterar la tabla de 4 columnas ni el test `rejects_invalid_entity_type`; y el cambio supera `cargo test -p execute-process`, `sddia-qa` (`verify-process-integrity`, `verify-tools-index`, `verify-capability-contracts`, `audit-eda-coverage`, `gate-evolution`) y validación de Argos `global: APTO`.
