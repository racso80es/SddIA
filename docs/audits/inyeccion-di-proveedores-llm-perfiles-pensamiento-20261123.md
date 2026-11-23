---
id: "audit-inyeccion-di-proveedores-llm-perfiles-pensamiento-20261123"
title: "Auditoría Técnica: Inyección de Dependencias de Proveedores LLM y Perfiles de Pensamiento por Agente (AUT-122)"
type: "audit"
version: "1.0.0"
created: "2026-11-23"
mode: "read-only"
tracker_ref: "AUT-122"
parent_ref: "AUT-121"
scope:
  - "SddIA/engine/execute-process/src/engine/agent_runtime.rs"
  - "SddIA/engine/execute-process/src/engine/executor.rs"
  - "SddIA/engine/execute-process/src/engine/phase_capsules.rs"
  - "SddIA/engine/execute-process/src/engine/phase_terminal.rs"
  - "SddIA/agents/*"
  - "SddIA/core/capability-bindings.md"
  - "SddIA/core/cumulo.paths.json"
  - "SddIA/library/norms/capability-taxonomy.md"
  - "SddIA/engine/execute-process/src/engine/capability_di_resolver.rs"
  - ".SddIA/active-domain-profile.json"
  - ".SddIA/project.md"
  - ".SddIA/instances.json"
  - ".SddIA/llm-registry.json"
  - "SddIA/engine/execute-process/src/engine/domain_profile.rs"
  - "SddIA/engine/execute-process/src/engine/project_binding.rs"
  - "SddIA/engine/execute-process/src/core/env.rs"
  - "SddIA/engine/execute-process/src/engine/llm_interaction_audit.rs"
  - "SddIA/engine/execute-process/src/engine/handlers/llm_interaction_audit.rs"
  - "SddIA/engine/execute-process/src/engine/execution_registry.rs"
  - "SddIA/engine/execute-process/src/engine/execution_workspace_report.rs"
---

# Auditoría Técnica (Solo Lectura): Inyección de Dependencias de Proveedores LLM y Perfiles de Pensamiento por Agente (`AUT-122`)

> **Fecha:** 2026-11-23
> **PBI:** `AUT-122` (Padre: `AUT-121`)
> **Alcance:** Inspección directa de código fuente de `execute-process`, contratos y manifiestos de agentes en `SddIA/agents/`, SSOT DI (`capability-bindings.md`, `capability-taxonomy.md`, `cumulo.paths.json`), configuración de instancia (`.SddIA/`) y módulos de auditoría/telemetría.

---

## 1. Hallazgos verificados

### Bloque 1 — Mecánica actual de ejecución de agentes (`agent_runtime.rs`, `executor.rs`, `phase_capsules.rs`, `phase_terminal.rs`)

1. **Cómo delega hoy `execute-process` una fase de agente:**
   - En `SddIA/engine/execute-process/src/engine/executor.rs` (L633–L728), `execute_phase_body` detecta fases solo-agente vía `delegates_are_only_agents(&delegates)` (todos los elementos empiezan por `agent:`).
   - Si `agent_runtime::is_configured()` es `true`, invoca `agent_runtime::invoke_agent_phase(...)`; si es `false`, marca la fase directamente con `status: "simulated"` y `note: "agentes IDE; sin SDDIA_AGENT_RUNTIME_COMMAND"`.
   - En `SddIA/engine/execute-process/src/engine/agent_runtime.rs` (L597–L966), `invoke_agent_phase` soporta **únicamente un modo de transporte real (CLI externo por stdio)** más 3 rutas de cortocircuito:
     - **Relevo IDE (`SDDIA_AGENT_RELAY_IDE=1|true|yes|on`)**: `lab_relay_active()` fuerza `is_configured() == false` y retorna `status: "simulated"` con `note: "SDDIA_AGENT_RELAY_IDE=1; relevo IDE"`.
     - **Guarda de reentrada (`SDDIA_AGENT_RUNTIME_DEPTH >= 1`)**: `agent_runtime_depth()` evita recursión devolviendo `status: "simulated"`.
     - **Agentes deterministas (`tier: "none"`)**: `load_llm_profiles` lee `llm_profile` de `SddIA/agents/{name}.md` (fallback a `DETERMINISTIC_AGENTS = ["cerbero", "cumulo", "radamanto"]`). Si `all_none == true`, retorna `status: "executed"` con `note: "deterministic-agent-no-llm"` sin spawnear proceso.
     - **CLI externo (`SDDIA_AGENT_RUNTIME_COMMAND`)**: `build_agent_command` ejecuta el binario/script configurado pasándole un JSON `operation: "AGENT_PHASE"` por `stdin` (con `agents`, `llm_profiles`, `persist_ref`, `branch_name`, `execution_id`, `mcp_servers`, `di_binding`, etc.) y lee la última línea no vacía de `stdout` como JSON (`success`, `data.status` normalizado a `executed | awaiting_agents | failed | blocked`, `data.message`, `error`). Aplica timeouts por `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` (660s), `_REFINAMIENTO` (900s) y `_EJECUCION` (suelo mínimo `MIN_EXECUTION_TIMEOUT_SECS = 2400s`), más smoke-check JSON-RPC `initialize` sobre `sddia-workspace-server` salvo `SDDIA_LAB_SKIP_MCP_SMOKE=1`.
     - **No hay cliente HTTP/API nativo en `agent_runtime.rs`**: la resolución de modelos y llamadas a CLI/SDK viven hoy fuera del motor Rust, en `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py` (L336–L372: `TIER_ENV_KEYS = SDDIA_LLM_TIER_HIGH|MEDIUM|LOW`, fallback `SDDIA_AGENT_RUNTIME_MODEL` / `composer-2.5`).
   - En `SddIA/engine/execute-process/src/engine/phase_capsules.rs` se gestionan fases de cápsulas/aduana (`delivery-snapshot-final`, `eda-genomic-audit`, `index-integrity-audit`, `evolution-audit`, etc.), no la delegación de agentes.
   - En `SddIA/engine/execute-process/src/engine/phase_terminal.rs` (L98–L134), `aggregate_execution_terminal` trata `failed` y `blocked` (sin `fail_soft: true`) como fallo global (`success: false, status_code: 1`), mientras que `skipped`, `simulated`, `awaiting`, `awaiting_agents` y `executed` son neutrales (`success: true, status_code: 0`).

2. **Cómo distingue hoy entorno interactivo (IDE) vs headless:**
   - **No existe detección de IDE ni selector de proveedor**. La única bifurcación es la ausencia de `SDDIA_AGENT_RUNTIME_COMMAND` o la activación de `SDDIA_AGENT_RELAY_IDE=1`:
     - Cuando una fase devuelve `status: "simulated"` en procesos con cascada documental (`feature`, `bug-fix`, `refactorization`) y no existe `{persist_ref}/validacion.md`, `simulated_relay_blocks_close` en `executor.rs` (L131–L141) y `barrier_from_phase_entry` (`executor.rs`, L194–L199) levantan una barrera `("awaiting_agents", "prior_agent_phase_not_executed")` que salta las fases posteriores de agente y cierre (`Verificación`, `Cierre documental en rama`, `Cierre de entrega`).
     - Un CLI headless configurado vía `SDDIA_AGENT_RUNTIME_COMMAND` puede devolver `status: "executed"` (continúa el bucle) o `status: "awaiting_agents"` (activa la misma barrera de parada controlada).

3. **Interfaz mínima (`trait` / puerto) necesaria en `agent_runtime.rs`:**
   - Para eliminar la condicionalidad en `executor.rs` (que hoy llama directamente a `agent_runtime::is_configured()` e `invoke_agent_phase`), basta un puerto `AgentRuntimePort` resuelto una vez por una factoría DI (`AgentRuntimeDiFactory`, análoga a `EventManagerDiFactory` usada por `eda:event-manager`) que devuelva un resultado tipado (`Executed`, `AwaitingAgents`, `Blocked`, `Failed`) junto con metadatos de resolución (`provider_id`, `thinking_profile`, `resolved_model`, `provider_mode`).

---

### Bloque 2 — Contrato de agentes y resolución DI (`SddIA/agents/`, `capability-bindings.md`, `capability_di_resolver.rs`)

1. **Campos actuales en `agents-contract.md` y los 7 agentes Core:**
   - `SddIA/agents/agents-contract.md` (v1.2.0, L1–L57) define:
     - Obligatorios: `uuid`, `name`, `version`, `contract`, `allowed_policies`, `inputs`, `outputs`, `hash_signature` (obligatorio en prod).
     - Opcionales ya existentes:
       - **`llm_profile`** (§5, L36–L47): `{ tier: "high" | "medium" | "low" | "none", description: "..." }`. Prohíbe explícitamente slugs comerciales, IPs o URLs.
       - **`cognitive_role_descriptor`** (§6, L49–L57): línea descriptiva industrial.
   - Estado verificado en los 7 manifiestos actuales:

     | Agente | `llm_profile.tier` | `llm_profile.description` | `cognitive_role_descriptor` |
     | :--- | :--- | :--- | :--- |
     | `SddIA/agents/mayeuta.md` | `high` | Mayéutica, refinamiento de requisitos y síntesis de intenciones | Estabilización de requisitos y auditoría Kaizen de fracturas |
     | `SddIA/agents/dedalo.md` | `high` | Diseño arquitectónico y especificación formal de procesos | Diseño de especificación y blueprint de proceso |
     | `SddIA/agents/argos.md` | `medium` | Síntesis de `audit_report_md` y `correction_blueprint_md` tras evidencia de cápsulas | Juez de artefactos; síntesis solo tras evidencia de cápsulas |
     | `SddIA/agents/tekton.md` | `low` | Forja táctica según especificación; sin decisiones de arquitectura | Ejecución táctica según especificación |
     | `SddIA/agents/cerbero.md` | `none` | RBAC determinista; prohibido LLM | Peaje RBAC determinista |
     | `SddIA/agents/cumulo.md` | `none` | Topología y SSOT; prohibido LLM | SSOT de topología e índices |
     | `SddIA/agents/radamanto.md` | `none` | Actuario de umbrales; prohibido LLM | Actuario de umbrales sobre telemetría |

2. **Encaje DI en `capability-bindings.md`, `cumulo.paths.json`, `capability-taxonomy.md` y `capability_di_resolver.rs`:**
   - `SddIA/library/norms/capability-taxonomy.md` (L35–L42) ya distingue `llm:interact` (`skill:mayeuta-llm`, síntesis/intención) y `llm:infer` (inferencia cruda consumida por `tool:llm-router`; **nota**: `llm-router` hace auto-failover entre oráculos de `.SddIA/llm-registry.json`, lo cual es ortogonal a la regla de no auto-failover de `AUT-122`).
   - `SddIA/core/capability-bindings.md` (L77–L93) ya estableció el precedente arquitectónico para adaptadores multiproveedor con `eda:event-manager`: prefijo `adapter:` / `mock:` + mapa `roles:` + resolución en runtime vía config de instancia.
   - En `SddIA/engine/execute-process/src/engine/capability_di_resolver.rs` (L109–L133), `is_capsule_provider` y `provider_fs_rel` hoy solo resuelven `skill:`, `action:` y `tool:` (los `adapter:` de `eda:event-manager` se resuelven en su propia factoría `EventManagerDiFactory` leyendo `capability-bindings.md` y la config de instancia).
   - `SddIA/core/cumulo.paths.json` (L40–L41, L74) ya declara `directories.infrastructure_adapters: "SddIA/infrastructure/adapters"` y `contracts.infrastructure_adapters: "SddIA/infrastructure/adapters/adapters-contract.md"`.

3. **Taxonomía cerrada de perfiles de pensamiento:**
   - El catálogo actual de los 7 agentes ya particiona el dominio de forma exacta en 4 roles cognitivos sin solapamiento (mapeables 1:1 con los tiers existentes para mantener retrocompatibilidad):
     - `reasoning` (equiv. `high`): `mayeuta`, `dedalo` (análisis de requisitos, arquitectura, especificación).
     - `audit` (equiv. `medium`): `argos` (síntesis de evidencias deterministas y emisión de veredicto/blueprint correctivo).
     - `execution` (equiv. `low`): `tekton` (forja táctica acotada a especificación).
     - `deterministic` (equiv. `none`): `cerbero`, `cumulo`, `radamanto` (prohibido LLM; 0 tokens).

---

### Bloque 3 — Configuración de instancia y conmutación de proveedor (`.SddIA/`)

1. **Estructura actual y lectores Rust:**
   - `.SddIA/active-domain-profile.json`:
     `{ "codex_slug": "codex-software-engineering", "git_required": true, "software_forge": true }`.
     Leído y validado por `domain_profile::resolve_execution_profile` en `SddIA/engine/execute-process/src/engine/domain_profile.rs` (L12–L120) (`ExecutionProfile`: `codex_slug`, `codex_uuid`, `git_required`, `software_forge`, `allowed_policies`, `source`), con precedencia `inputs.execution_profile` > `inputs.active_codex_ref` > `.SddIA/active-domain-profile.json` > default.
   - `.SddIA/project.md`:
     Frontmatter YAML (`id`, `uuid`, `git_remote`, `default_branch`, `delivery_mode`, `contract_version: "1.3.0"`, `codex_slug`, `tracker.*`, `docs_layout.*`, `mcp.allowed_executables`). Leído y validado estrictamente por `SddIA/engine/execute-process/src/engine/project_binding.rs` (L70–L350) (`BoundProject`, `TrackerConfig`, `effective_provider` solo acepta `file | linear`). Es configuración **de proyecto** (compartida en el repo), no de entorno/IDE del operador.
   - `.SddIA/instances.json`:
     Registro de instancias en el host (`{ "<instance-key>": { created_at, mailbox_fingerprint, profile, root, wui_port } }`). **No lo lee `execute-process`**: solo lo manipulan los scripts del instalador (`SddIA/scripts/sddia-installer.sh`, `installer_io_lib.sh`).
   - `.SddIA/llm-registry.json`:
     Registro de oráculos `oracle-agy` (`skill:antigravity-cli-executor`, `fallback: "oracle-gemini"`) y `oracle-gemini` (`tool:gemini-http-infer`). Leído por `SddIA/tools/llm-router/src/main.rs` y materializado por `handlers/instance_creator.rs`. Diseñado para inferencia de oráculos con failover automático (`aiua-stimulus-processing`), no para fases de agentes del Core.
   - `SddIA/engine/execute-process/src/core/env.rs` (L17–L84) y `.dev/.env.example`:
     Carga jerárquica `.dev/.env` < `.SddIA/.dev/.env` < entorno SO (excepto `VAULT_PRECEDENCE_KEYS = ["SDDIA_LAB_SIMULATE_IOTA", "SDDIA_IOTA_TIMEOUT_SECONDS"]` donde la bóveda pisa el entorno).

2. **Dónde debe residir el selector explícito de proveedor activo:**
   - **No en `.SddIA/project.md`**: `project.md` describe el proyecto (git, tracker, layout) y es versionado en repositorio.
   - **No en `.SddIA/instances.json`**: es inventario de host del instalador.
   - **Sí en `.SddIA/active-domain-profile.json`**: aprovecha un archivo que `execute-process` ya carga en cada invocación (`domain_profile.rs`), es específico de instancia (no del Genoma `SddIA/`), y permite además override determinista de credenciales/endpoints locales en `.SddIA/.dev/.env` (bóveda no versionada).

3. **Interacción de la conmutación manual entre fases con `execution_registry.rs`, `phase_reports.json` y `stop_after`:**
   - `SddIA/engine/execute-process/src/engine/execution_registry.rs` (L65–L156) persiste atómicamente `{workspace}/phase_reports.json` tras cada fase y mantiene `.SddIA/state/active-executions.json` (`process_name`, `workspace_path`, `persist_ref`, `status`, `stop_after`, `updated_at`).
   - En `executor.rs` (L288–L339 y L861–L901), al reanudar (`resume: true` + `execution_id`):
     - Fases previas en `executed` se saltan (`handler: "execution-resume"`, `reason: "resume_already_executed"`), preservando íntegro su objeto en `phase_reports.json` (incluyendo los campos del proveedor con el que se ejecutaron).
     - Fases marcadas `skipped` por `reason: "stop_after"` o `in_progress | failed | blocked` se relanzan desde cero con el proveedor **actualmente activo** en el momento del `resume`.
     - Si una fase quedó en `status: "simulated"` o `status: "awaiting_agents"` (relevo IDE), `resume_skip_or_block` retorna `Ok(None)` (la relanza), y si la barrera se activó por `stop_after`, `stop_after_frontier_consumed` (`executor.rs`, L246–L265) desactiva la barrera para avanzar. Por tanto, **cambiar el proveedor activo entre un `stop_after: ["design"]` y el siguiente `resume: true` funciona de forma natural**, y cada entrada de `phase_reports.json` puede registrar el proveedor/modelo exacto que ejecutó esa fase concreta.

---

### Bloque 4 — Auditoría y telemetría de interacciones LLM (`llm_interaction_audit.rs`, `execution_registry.rs`, `execution_workspace_report.rs`)

1. **Qué registra hoy `llm_interaction_audit` y dónde persiste:**
   - `SddIA/engine/execute-process/src/engine/llm_interaction_audit.rs` (L20–L67) y su handler `handlers/llm_interaction_audit.rs` (L20–L61) persisten un Markdown por `tracker_ref` en `{paths.auditsPath}/{tracker_ref}.md` (resuelto desde `cumulo.paths.json` → `docs/audits/{tracker_ref}.md`).
   - Solo registra dos estructuras:
     - `InteractionRow`: tabla Markdown de 4 columnas `| entity_type | entity_id | invoked_by | timestamp |` (donde `normalize_entity_type` **solo acepta `skill | tool | action | process`** y rechaza explícitamente `"agent"`, ver test `rejects_invalid_entity_type` en L470).
     - `FrictionRow`: tabla `## Fricción — ./sddia-run.sh` con `comando`, `success`, `error_code`, `handler`, `mensaje`.
   - `SddIA/engine/execute-process/src/engine/execution_workspace_report.rs` (L15–L57) persiste `{workspace}/execution_report.json` con `process_name`, `execution_id`, `correlation_id`, `status_code`, `duration_ms` y `phases` (copia de `phase_reports`, donde `inject_elapsed_ms` añade `elapsed_ms`).
   - **Ninguno de los tres** (`docs/audits/{tracker_ref}.md`, `phase_reports.json`, `execution_report.json`) registra hoy `thinking_profile`, `llm_provider` ni `resolved_model`.

2. **Campos exactos a añadir para trazabilidad auditable:**
   - En cada entrada de fase de `phase_reports.json` y `execution_report.json` (generada en `agent_runtime.rs` / `executor.rs`):
     - `agent_ids: Vec<String>` (alineado con `delegates_to`)
     - `thinking_profile: String` (`reasoning | audit | execution | deterministic`)
     - `llm_provider: String` (`cursor | antigravity | local-llm | deterministic-none`)
     - `resolved_model: Option<String>` (`null` cuando es `deterministic` o relevo interactivo sin modelo informado por el runtime)
     - `provider_mode: String` (`interactive_relay | headless_cli | headless_api | deterministic`)
   - En `llm_interaction_audit.rs`:
     - Añadir sección/tabla dedicada `## Ejecuciones de Agente` (para no romper `parse_existing_rows` de 4 columnas ni el test `rejects_invalid_entity_type`) con columnas `| agent_id | thinking_profile | llm_provider | resolved_model | phase_id | timestamp |`, más un hook automático post-fase en `executor.rs` cuando exista `tracker_ref` y `llm_audit != inactive`.

---

## 2. Propuesta técnica cerrada (para los 4 puntos abiertos del refino)

### Punto 1 — Ubicación y esquema de configuración y contratos DI

- **Taxonomía y Bindings (`SddIA/` — Genoma):**
  - Registrar capacidad `agent:runtime` (contrato `agent.runtime`, versión `1.0.0`, schema `SddIA/library/norms/capability-contracts/agent.runtime.schema.json`) en `capability-taxonomy.md` (vía `entity-manager`, prohibida edición manual).
  - Registrar en `SddIA/core/capability-bindings.md` una única fila `agent:runtime` siguiendo el patrón `roles:` ya vigente en `eda:event-manager`:
    ```yaml
    - capability_id: "agent:runtime"
      contract: "agent.runtime"
      contract_version: "1.0.0"
      provider: "adapter:cursor-agent-runtime"
      provider_version: ">=1.0.0"
      roles:
        cursor:
          provider: "adapter:cursor-agent-runtime"
          provider_version: ">=1.0.0"
        antigravity:
          provider: "adapter:antigravity-agent-runtime"
          provider_version: ">=1.0.0"
        local-llm:
          provider: "adapter:local-llm-agent-runtime"
          provider_version: ">=1.0.0"
        test:
          provider: "mock:in-memory-agent-runtime"
          provider_version: ">=1.0.0"
    ```
  - Cada adaptador vive bajo `SddIA/infrastructure/adapters/{provider}-agent-runtime.md` (cumpliendo `adapters-contract.md`) y declara su **mapeo por defecto** `thinking_profile -> model` y su `transport_mode` (`interactive_relay | headless_cli | headless_api`):
    ```yaml
    provider_id: "cursor"
    transport_mode: "headless_cli" # o "interactive_relay"
    supports_unattended: true
    default_model_map:
      reasoning: "claude-opus-4-6"
      audit: "claude-sonnet-4-6"
      execution: "composer-2.5"
    ```
- **Selector activo y overrides de instancia (`.SddIA/`):**
  - En `.SddIA/active-domain-profile.json` añadir el bloque explícito:
    ```json
    {
      "codex_slug": "codex-software-engineering",
      "git_required": true,
      "software_forge": true,
      "llm_runtime": {
        "active_provider": "antigravity",
        "unattended_policy": "pause_awaiting_agents",
        "model_overrides": {
          "antigravity": {
            "reasoning": "gemini-2.5-pro",
            "audit": "gemini-2.5-pro",
            "execution": "gemini-2.5-flash"
          }
        }
      }
    }
    ```
  - **Precedencia determinista de resolución de modelo** (sin salto de proveedor):
    1. Override en `.SddIA/active-domain-profile.json` → `llm_runtime.model_overrides[active_provider][thinking_profile]`
    2. Override de bóveda local `.SddIA/.dev/.env` → `SDDIA_LLM_MODEL_{PROVIDER}_{PROFILE}` (opcional; migración limpia desde `SDDIA_LLM_TIER_*`)
    3. `default_model_map[thinking_profile]` del adaptador activo en `SddIA/infrastructure/adapters/`
    4. Si falta → error explícito `LLM_MODEL_UNRESOLVED` (nunca fallback silencioso a otro proveedor).

### Punto 2 — Interfaz del puerto en `agent_runtime.rs` y mecánica IDE interactivo vs headless

- Definir en `agent_runtime.rs` el puerto y DTOs:
  ```rust
  pub enum ThinkingProfile { Reasoning, Audit, Execution, Deterministic }
  pub enum ProviderMode { InteractiveRelay, HeadlessCli, HeadlessApi, Deterministic }

  pub struct AgentInvocationRequest {
      pub process_name: String,
      pub phase_id: Option<String>,
      pub phase_name: String,
      pub agents: Vec<String>,
      pub thinking_profile: ThinkingProfile,
      pub resolved_model: Option<String>,
      pub payload: Value, // AGENT_PHASE + mcp_servers + di_binding
      pub timeout: Duration,
  }

  pub struct AgentExecutionResult {
      pub status: AgentPhaseStatus, // Executed | AwaitingAgents | Blocked | Failed
      pub provider_id: String,
      pub provider_mode: ProviderMode,
      pub thinking_profile: ThinkingProfile,
      pub resolved_model: Option<String>,
      pub message: Option<String>,
      pub error_code: Option<String>,
      pub error: Option<String>,
      pub runtime_response: Option<Value>,
  }

  pub trait AgentRuntimePort: Send + Sync {
      fn provider_id(&self) -> &'static str;
      fn mode(&self) -> ProviderMode;
      fn supports_unattended(&self) -> bool;
      fn invoke(&self, req: &AgentInvocationRequest) -> AgentExecutionResult;
  }
  ```
- **Comportamiento cuando el proveedor activo no soporta ejecución desatendida** (ej. IDE interactivo sin CLI headless):
  - Gobernado explícitamente por `llm_runtime.unattended_policy`:
    - `"pause_awaiting_agents"` (default interactivo): el adaptador devuelve `AgentPhaseStatus::AwaitingAgents` con `provider_mode: InteractiveRelay` y `message: "relevo interactivo requerido en proveedor '<id>'"` → `executor.rs` levanta barrera limpia y deja el proceso reanudable con `resume: true`.
    - `"fail_fast"` (entornos CI/daemon desatendidos): devuelve `AgentPhaseStatus::Failed` con `error_code: "PROVIDER_UNATTENDED_UNSUPPORTED"`.
- **Factoría DI (`AgentRuntimeDiFactory::resolve(repo, inputs)`):**
  - Lee `active_provider` desde `ExecutionProfile` (`.SddIA/active-domain-profile.json`, sin aceptar fallback a otro proveedor).
  - Si todos los agentes de la fase son `ThinkingProfile::Deterministic`, cortocircuita con `DeterministicNoLlmAdapter` (`status: Executed`, `llm_provider: "deterministic-none"`).
  - Si `active_provider` no existe en `capability-bindings.md` → fallo duro `CAPABILITY_BINDING_MISSING` (sin auto-failover).

### Punto 3 — Taxonomía cerrada de tipos de pensamiento en `agents-contract.md`

- Evolucionar `agents-contract.md` a **v1.3.0** extendiendo `llm_profile` con `thinking_profile` como enum cerrado (y manteniendo `tier` como alias deprecado en v1 para cero regresiones):
  ```yaml
  llm_profile:
    thinking_profile: "reasoning"  # reasoning | audit | execution | deterministic
    tier: "high"                   # alias retrocompatible (high->reasoning, medium->audit, low->execution, none->deterministic)
    description: "..."
  ```
- Asignación cerrada sobre todo el catálogo Core:
  - `reasoning`: `mayeuta`, `dedalo`
  - `audit`: `argos`
  - `execution`: `tekton`
  - `deterministic`: `cerbero`, `cumulo`, `radamanto`
- Regla para fases multi-agente: se selecciona el perfil de mayor precedencia cognitiva (`reasoning > audit > execution > deterministic`).

### Punto 4 — Mecánica de conmutación explícita sin auto-failover

- **Prohibición estricta de auto-failover:** ante errores de cuota (`rate_limited`), timeout o fallo de transporte, el adaptador retorna `Failed` con `error_code` tipado (`PROVIDER_QUOTA_EXCEEDED`, `PROVIDER_TIMEOUT`, `PROVIDER_UNAVAILABLE`). El motor **jamás** reintenta contra otro proveedor.
- **Conmutación manual entre fases:**
  - Operador/Kalma2 cambia `llm_runtime.active_provider` en `.SddIA/active-domain-profile.json` (opcionalmente vía una acción/tool determinista `switch-llm-provider` que valide contra `capability-bindings.md` y escriba atómicamente el JSON).
  - Luego relanza el proceso con `resume: true` y el mismo `execution_id`: `executor.rs` conserva las fases ya `executed` (con su `llm_provider` y `resolved_model` originales intactos en `phase_reports.json`) y ejecuta las fases pendientes con el nuevo proveedor activo.

---

## 3. Riesgos y fricciones detectadas

1. **Divergencia real entre reglas de `.cursor/rules/` y `.antigravity/rules/`:**
   - Verificado en el repo: `.antigravity/rules/` contiene 5 ficheros `.md` (`[ARQUITECTURA] constitution_core.md`, `[OPERATIVO] protocolo_ejecucion.md`, `kintsugi-fracture-protocol.md`, `task-closure-documental.md`, `tekton-fire-and-forget.md`), mientras que `.cursor/rules/` solo tiene 3 `.mdc` (`kintsugi-fracture-protocol.mdc`, `task-closure-documental.mdc` y `tekton-fire-and-forget.mdc` con distinto tamaño: 830B vs 1466B).
   - **Impacto:** si un operador conmuta de `cursor` a `antigravity` (o viceversa) en modo interactivo a mitad de un proceso, cada IDE inyectará reglas de sistema parcialmente divergentes.
2. **Estado `awaiting_agents` / `simulated` al cambiar de IDE en medio de un proceso:**
   - En `executor.rs` (L324–L338), `resume_skip_or_block` solo verifica `design_artifacts_ready` (`spec.md` + `plan.md`) cuando la fase previa quedó en `in_progress | failed | blocked`, pero si quedó en `simulated` o `awaiting_agents` cae al `Ok(None)` final sin comprobar si el IDE anterior ya escribió artefactos parciales bajo `persist_ref`. Al cambiar de proveedor tras un `awaiting_agents`, hay riesgo de sobrescribir trabajo parcial o de que `check_persist_execution_id_conflict` en `agent_runtime.rs` (L199–L241) dispare `persist-execution-id-conflict` si se reanudó sin pasar el mismo `execution_id`.
3. **Coexistencia de dos subsistemas LLM distintos (`agent_runtime` vs `llm-router` / `.SddIA/llm-registry.json`):**
   - `.SddIA/llm-registry.json` + `tool:llm-router` **sí implementan failover automático** (`oracle-agy` → `oracle-gemini`) para `aiua-stimulus-processing`. El PBI debe dejar explícita la frontera: `agent:runtime` (fases de agentes del Core, sin failover automático) es ortogonal a `llm:infer` / `llm-router` (oráculos de estímulo Aiúa).
4. **Impacto en tests de laboratorio y variables `SDDIA_AGENT_*`:**
   - Los tests actuales en `agent_runtime.rs` (L969–L1100) dependen directamente de `SDDIA_AGENT_RUNTIME_COMMAND`, `SDDIA_AGENT_RELAY_IDE`, `SDDIA_AGENT_RUNTIME_DEPTH`, `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS` y `SDDIA_LAB_SKIP_MCP_SMOKE`. En línea con `AUT-116` (erradicación de `SDDIA_LAB_*` en favor de DI), el nuevo puerto deberá inyectarse vía `mock:in-memory-agent-runtime` en tests unitarios para no introducir nuevas variables `SDDIA_LAB_*`.
5. **Rechazo actual de `entity_type: "agent"` en `llm_interaction_audit.rs`:**
   - `llm_interaction_audit.rs` (L18) restringe `KARMA2_ENTITY_TYPES` a `["skill", "tool", "action", "process"]` y tiene un test unitario explícito (`rejects_invalid_entity_type`, L470) que exige fallar con `"agent"`. Registrar ejecuciones de agente en `docs/audits/{tracker_ref}.md` debe hacerse mediante una tabla/sección separada o actualizando explícitamente el contrato y el test en el plan de `AUT-122`.
