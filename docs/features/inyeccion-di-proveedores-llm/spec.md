---
feature_name: inyeccion-di-proveedores-llm
created: "2026-11-23"
process: feature
branch_name: feat/inyeccion-di-proveedores-llm
persist_ref: docs/features/inyeccion-di-proveedores-llm
tracker_ref: AUT-122
historia_ref: AUT-121
base: main
scope: "SddIA/agents/*, capability-taxonomy, capability-bindings, capability-contracts, infrastructure/adapters, execute-process (agent_runtime, executor, llm_interaction_audit, actions)"
execution_id: "f1220000-2026-4123-8122-000000000122"
---
# Especificación Técnica — inyeccion-di-proveedores-llm (`AUT-122`)

## 1. Contrato de Agentes (`SddIA/agents/agents-contract.md` v1.3.0 y Catálogo)

- **Esquema `llm_profile` en `agents-contract.md` v1.3.0:**
  ```yaml
  llm_profile:
    thinking_profile: "reasoning" # reasoning | audit | execution | deterministic
    tier: "high"                  # high | medium | low | none (alias retrocompatible v1)
    description: "texto breve de la carga cognitiva"
  ```
- **Mapeo en los 7 agentes Core:**
  - `mayeuta.md`, `dedalo.md` → `thinking_profile: "reasoning"`, `tier: "high"`
  - `argos.md` → `thinking_profile: "audit"`, `tier: "medium"`
  - `tekton.md` → `thinking_profile: "execution"`, `tier: "low"`
  - `cerbero.md`, `cumulo.md`, `radamanto.md` → `thinking_profile: "deterministic"`, `tier: "none"`
- **Regla de precedencia multi-agente:**
  - `ThinkingProfile::rank`: `Reasoning (3) > Audit (2) > Execution (1) > Deterministic (0)`.
  - Si un manifiesto antiguo solo define `tier`, se deriva `thinking_profile` (`high -> reasoning`, `medium -> audit`, `low -> execution`, `none -> deterministic`).

## 2. Capacidad DI `agent:runtime`, Adaptadores y Configuración Local

- **Contrato de capacidad (`SddIA/library/norms/capability-contracts/agent.runtime.schema.json`):**
  - Creado vía `entity-manager` (`entity_class: "capability-contract"`, `entity_name: "agent.runtime"`, `schema_version: "1.0.0"`).
- **Taxonomía (`SddIA/library/norms/capability-taxonomy.md` v1.0.13):**
  - Añade entrada `{ id: "agent:runtime", contract: "agent.runtime", version: "1.0.0", description: "Puerto AgentRuntimePort de ejecución cognitiva de agentes del Core por perfil de pensamiento y proveedor DI sin auto-failover." }`.
- **Bindings (`SddIA/core/capability-bindings.md` v1.11.0):**
  - Añade fila `capability_id: "agent:runtime"` con `contract: "agent.runtime"`, `contract_version: "1.0.0"`, `contract_schema: "agent.runtime.schema.json"`, `provider: "adapter:cursor-agent-runtime"`, `provider_version: ">=1.0.0"` y mapa `roles`:
    - `cursor` → `adapter:cursor-agent-runtime`
    - `antigravity` → `adapter:antigravity-agent-runtime`
    - `local-llm` → `adapter:local-llm-agent-runtime`
    - `test` → `mock:in-memory-agent-runtime`
- **Adaptadores de infraestructura (`SddIA/infrastructure/adapters/`):**
  - Fichas `cursor-agent-runtime.md`, `antigravity-agent-runtime.md`, `local-llm-agent-runtime.md` (con sus directorios verificables `cursor_agent_runtime/`, `antigravity_agent_runtime/`, `local_llm_agent_runtime/` y filas en `SddIA/infrastructure/adapters/index.md` según `adapters-contract.md` §1–§2), declarando únicamente `provider_id`, `transport_mode` y `supports_unattended` sin slugs comerciales de modelos.
- **Topología y Configuración Local (`.SddIA/llm-runtime.local.json`):**
  - `SddIA/core/cumulo.paths.json`: añade `instance.llm_runtime_local: ".SddIA/llm-runtime.local.json"` e `instance.llm_runtime_example: ".SddIA/llm-runtime.example.json"`.
  - `.gitignore`: ignora `.SddIA/llm-runtime.local.json`.
  - `.SddIA/llm-runtime.example.json` (versionado):
    ```json
    {
      "schema_version": "1.0.0",
      "active_provider": "cursor",
      "unattended_policy": "pause_awaiting_agents",
      "providers": {
        "cursor": {
          "transport_mode": "headless_cli",
          "models": {
            "reasoning": "model-reasoning-id",
            "audit": "model-audit-id",
            "execution": "model-execution-id"
          }
        },
        "antigravity": {
          "transport_mode": "interactive_relay",
          "models": {
            "reasoning": "model-reasoning-id",
            "audit": "model-audit-id",
            "execution": "model-execution-id"
          }
        },
        "local-llm": {
          "transport_mode": "headless_api",
          "endpoint": "",
          "models": {
            "reasoning": "local-reasoning-model",
            "audit": "local-audit-model",
            "execution": "local-execution-model"
          }
        }
      }
    }
    ```

## 3. Puerto `AgentRuntimePort`, Evaluación Perezosa y `action:switch-llm-provider`

- **Tipos y Trait en `SddIA/engine/execute-process/src/engine/agent_runtime.rs`:**
  - `ThinkingProfile` (`Reasoning`, `Audit`, `Execution`, `Deterministic`)
  - `ProviderMode` (`InteractiveRelay`, `HeadlessCli`, `HeadlessApi`, `Deterministic`)
  - `UnattendedPolicy` (`PauseAwaitingAgents`, `FailFast`)
  - `AgentInvocationRequest` y `AgentExecutionResult`
  - `pub trait AgentRuntimePort: Send + Sync`
  - `pub struct AgentRuntimeDiFactory`:
    - **Invariante de evaluación perezosa (`lazy evaluation`):**
      1. Si la fase tiene `ThinkingProfile::Deterministic` (`all_none`), cortocircuita antes de consultar `.SddIA/llm-runtime.local.json` y devuelve `status: "executed"`, `note: "deterministic-agent-no-llm"`, `thinking_profile: "deterministic"`, `llm_provider: "deterministic-none"`, `resolved_model: null`, `provider_mode: "deterministic"`.
      2. Si hay un mock inyectado en el hilo actual (`with_mock_agent_runtime_port`), usa `mock:in-memory-agent-runtime` (sin variables `SDDIA_LAB_*`).
      3. Si existe `.SddIA/llm-runtime.local.json` (o `inputs.execution_profile.llm_runtime`), valida que `active_provider` exista en `roles` de `agent:runtime` en `capability-bindings.md` (si no existe → error `CAPABILITY_BINDING_MISSING`), y resuelve el modelo con precedencia:
         `SDDIA_LLM_MODEL_{PROVIDER}_{PROFILE}` (o legacy `SDDIA_LLM_TIER_*`) > `llm-runtime.local.json: providers[active_provider].models[thinking_profile]`. Si el archivo exige resolución estricta (`require_model: true` o mapa de modelos presente sin la clave del perfil y sin env override) → error `LLM_MODEL_UNRESOLVED`.
      4. Si `.SddIA/llm-runtime.local.json` **no existe** (CI / clon limpio), resuelve sin fallar usando el proveedor por defecto (`SDDIA_LLM_ACTIVE_PROVIDER` o `"cursor"`), preservando la semántica de `SDDIA_AGENT_RELAY_IDE` y `SDDIA_AGENT_RUNTIME_COMMAND`.
- **Adaptadores v1 sin auto-failover:**
  - `CursorAgentRuntimeAdapter`: usa CLI (`SDDIA_AGENT_RUNTIME_COMMAND` o `providers.cursor.command`) en modo `HeadlessCli` (inyectando `thinking_profile`, `llm_provider: "cursor"`, `resolved_model` en el payload `AGENT_PHASE`) o `InteractiveRelay` cuando no hay comando configurado (`pause_awaiting_agents` → `simulated`/`awaiting_agents`, `fail_fast` → `failed` con `PROVIDER_UNATTENDED_UNSUPPORTED`).
  - `AntigravityAgentRuntimeAdapter`: modo `InteractiveRelay` por defecto (`pause_awaiting_agents` → pausa controlada para relevo en IDE Antigravity; `fail_fast` → `PROVIDER_UNATTENDED_UNSUPPORTED`) o `HeadlessCli` si se configura comando CLI.
  - `LocalLlmAgentRuntimeAdapter`: verifica `endpoint` / `command` en `providers["local-llm"]` o `SDDIA_LOCAL_LLM_ENDPOINT`; si está vacío o inaccesible, falla determinísticamente con `error_code: "PROVIDER_UNAVAILABLE"` sin saltar a otro proveedor.
  - Clasificación de errores de transporte sin failover: `rate_limit` / `quota` → `PROVIDER_QUOTA_EXCEEDED`; timeout → `PROVIDER_TIMEOUT`; endpoint/binario ausente → `PROVIDER_UNAVAILABLE`.
- **Acción `action:switch-llm-provider`:**
  - Registrada en `SddIA/actions/switch-llm-provider.md` vía `entity-manager` (`Domain_Entity_Created`, indexada en `SddIA/actions/index.md` y cubierta en `eda-coverage.json`).
  - Handler nativo en `SddIA/engine/execute-process/src/engine/actions.rs`: recibe `provider` (`cursor | antigravity | local-llm`), opcionalmente `unattended_policy` y `models`, valida que `provider` esté registrado bajo `agent:runtime.roles` en `capability-bindings.md` (rechaza proveedores desconocidos con `CAPABILITY_BINDING_MISSING`) y persiste atómicamente `.SddIA/llm-runtime.local.json`.
- **Blindaje de `resume_skip_or_block` en `executor.rs`:**
  - Al reanudar (`resume: true`) una ejecución cuya fase previa quedó en `awaiting_agents` o `simulated`:
    - Si es una fase de diseño (`design` / `Diseño de Blueprint` / `Diseño del fix` / `Diseño de refactor`) y los artefactos de diseño (`spec.md` y `plan.md` cuando aplica) **ya fueron materializados** durante el relevo interactivo previo, marca la fase como `executed` (`handler: "execution-resume"`, `reason: "interactive_relay_artifacts_ready"`) y avanza a la siguiente fase con el nuevo proveedor activo, reconciliando previamente `execution_id` en `persist_ref` para evitar falsos positivos de `persist-execution-id-conflict`.

## 4. Auditoría y Telemetría (`phase_reports.json`, `execution_report.json`, `llm_interaction_audit.rs`)

- **Campos en cada reporte de fase de agente (`phase_reports.json` y `execution_report.json`):**
  - `agent_ids`: array de strings (`["mayeuta"]`, etc.)
  - `thinking_profile`: `"reasoning" | "audit" | "execution" | "deterministic"`
  - `llm_provider`: `"cursor" | "antigravity" | "local-llm" | "deterministic-none" | "mock"`
  - `resolved_model`: string o `null`
  - `provider_mode`: `"interactive_relay" | "headless_cli" | "headless_api" | "deterministic"`
- **Sección `## Ejecuciones de Agente` en `llm_interaction_audit.rs`:**
  - La tabla principal de 4 columnas (`| entity_type | entity_id | invoked_by | timestamp |`) y `KARMA2_ENTITY_TYPES = &["skill", "tool", "action", "process"]` (junto con el test `rejects_invalid_entity_type`) permanecen **intactos**.
  - Se añade `AgentExecutionAuditRow` y `append_agent_execution(repo, tracker_ref, &row, require_active)` que añade (con deduplicación) filas en una sección separada al final de `docs/audits/{tracker_ref}.md`:
    ```markdown
    ## Ejecuciones de Agente

    | agent_id | thinking_profile | llm_provider | resolved_model | phase_id | timestamp |
    | --- | --- | --- | --- | --- | --- |
    ```
  - `executor.rs` invoca best-effort `maybe_record_agent_phase_execution` tras cada fase solo-agente cuando hay `tracker_ref` / `pbi_ref` y `llm_audit != inactive`.

## 5. Oráculos Mecánicos Ejecutables (Ola 0)

| ID | Criterio | Oráculo mecánico |
| :--- | :--- | :--- |
| AC-1 | `thinking_profile` en `agents-contract.md` v1.3.0 y 7 agentes Core + precedencia multi-agente | `cargo test --manifest-path SddIA/Cargo.toml -p execute-process agent_runtime::tests::thinking_profile` |
| AC-2 | Capacidad `agent:runtime` en taxonomy/bindings/contracts + adaptadores + `.SddIA/llm-runtime.local.json` gitignored | `sddia-qa verify-capability-contracts && cargo test --manifest-path SddIA/Cargo.toml -p execute-process agent_runtime::tests::di_factory` |
| AC-3 | `AgentRuntimePort`, proveedores v1 (`cursor`, `antigravity`, `local-llm`), sin auto-failover, `switch-llm-provider` y resume desde `awaiting_agents` | `cargo test --manifest-path SddIA/Cargo.toml -p execute-process agent_runtime::tests::no_auto_failover` + `executor::tests::resume_after_provider_switch` |
| AC-4 | Campos en `phase_reports.json` + sección `## Ejecuciones de Agente` en `llm_interaction_audit.rs` + aduanas `sddia-qa` | `cargo test --manifest-path SddIA/Cargo.toml -p execute-process llm_interaction_audit` + `sddia-qa verify-process-integrity && sddia-qa verify-tools-index && sddia-qa audit-eda-coverage --scan --json && sddia-qa gate-evolution --json --range --sync-base` |
