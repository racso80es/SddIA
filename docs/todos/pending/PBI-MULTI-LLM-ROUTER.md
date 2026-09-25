---
document_id: PBI-MULTI-LLM-ROUTER
uuid: "d2e44083-ccdf-45af-b477-f6c71833fc31"
title: "[ARQUITECTURA] Adaptador Multi-LLM, Registro de Oráculos de instancia y Fail-Soft encadenado"
format: markdown
version: "1.2.0"
created: "2026-08-25"
updated: "2026-09-25"
refined: "2026-09-25"
status: pending
priority: alta
type: arquitectura
process: feature
feature_name: multi-llm-router
branch_name: feat/multi-llm-router
persist_ref: docs/features/multi-llm-router
absorbs:
  - PBI-LLM-REGISTRY-RESILIENCE
depends_on:
  - PBI-ARQUITECTURA-LLM-TIERS
  - PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR
related:
  - "docs/todos/done/[ARQUITECTURA] Inyección de perfiles LLM (Tiers) en contratos de agentes SddIA y Resolución Dinámica.md"
  - docs/features/arquitectura-llm-tiers/spec.md
  - docs/features/aiua-antigravity-cli-vector/
  - docs/features/nucleo-aiua-tormentosa-motor/auditoria.md
  - docs/features/adecuar-ed-telemetry/objectives.md
  - docs/features/telemetria-cognitiva-llm-kalma2/spec.md
  - SddIA/agents/agents-contract.md
  - SddIA/core/capability-bindings.md
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/library/norms/capability-contracts/llm.interact.schema.json
  - SddIA/tools/gemini-http-infer.md
  - SddIA/skills/antigravity-cli-executor.md
  - SddIA/skills/mayeuta-llm.md
  - SddIA/process/aiua-stimulus-processing.md
  - SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs
  - SddIA/engine/execute-process/src/engine/capability_di_resolver.rs
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/scripts/starter-kit/.dev/.env.example
  - SddIA/scripts/starter-kit/.SddIA/.dev/.env.example
  - SddIA/evolution/1dc4055c-b0c8-40ff-a30d-d257152fb8df.md
refinement_notes: "v1.2.0 (2026-09-25). Filtro A sobre v1.1.0 + anexo PBI-LLM-REGISTRY-RESILIENCE + transcripción 2026-09-09. Unificado en un solo PBI. Purgados: combustión Aiúa = gemini-http-infer (obsoleto desde 1dc4055c, 2026-09-09: combustión = antigravity-cli-executor); SDDIA_GEMINI_MODEL ausente (hoy definido en bóveda instancia); llm.interact.schema.json 'se crea' (ya existe v1.0.0, sobre-only); eventos Tool_Degraded/Status_Restored (fósiles purgados por adecuar-ed-telemetry → Domain_Entity_Degraded/Restored); registro con slugs-ley en SddIA/core (viola H7/H12 tiers y agnosticismo Core); Ollama/DeepSeek como oráculo existente (no existe); matriz por agente Tekton/Argos (fases agent: ya resueltas por tiers, fuera). Promovido a pending."
---

# [ARQUITECTURA] Adaptador Multi-LLM, Registro de Oráculos de instancia y Fail-Soft encadenado

**Ciclo:** `feature` · `feat/multi-llm-router` · `docs/features/multi-llm-router/`  
**Absorbe:** `PBI-LLM-REGISTRY-RESILIENCE` (anexo 2026-09-09, borrador Tormentosa) — unificado aquí; no existe como PBI independiente.  
**Depende de (Done):** `PBI-ARQUITECTURA-LLM-TIERS` (tiers `llm_profile`), `PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR` (combustión Aiúa vía `agy`).

## 0. Intención estratégica

Erradicar la dependencia de un proveedor único de inferencia. La fricción real (2026-09-09, Vértice Biológico): la licencia del vector actual de la Aiúa tiene cuota muy baja; ante agotamiento o caída, el latido debe **saltar** a otro oráculo accesible sin intervención humana y sin colapsar el hilo. Conmutación por **bóveda/configuración de instancia**, nunca por edición de genoma. Cero slugs comerciales como valor-ley en `SddIA/`.

Principio rector (laudo Racso 2026-09-09): **DIP ontológico** — los consumidores de inteligencia conocen un contrato; las cápsulas de proveedor son adaptadores intercambiables; el enrutamiento y el fallback viven en un órgano Core agnóstico.

## 1. Hallazgos Filtro A (v1.1.0 + anexo) — purgados en esta versión

| # | Afirmación previa | Veredicto | Hecho verificado (2026-09-25) |
|---|-------------------|-----------|-------------------------------|
| F1 | «`tool:gemini-http-infer` = única combustión Aiúa»; «`antigravity-cli-executor` no está en `delegates_to`» | **Obsoleto** | `aiua-stimulus-processing` v1.2.0 fase `Combustion-Inferencia` → `skill:antigravity-cli-executor`. Handler `aiua_stimulus.rs::infer_antigravity_cli`. Test `process_genome_combustion_is_antigravity_cli` prohíbe `gemini-http-infer` en ese genoma. Evolution `1dc4055c` (H-AIUA-VECTOR laudado = CLI). |
| F2 | «`SDDIA_GEMINI_MODEL` ausente en bóveda» | **Obsoleto** | Definido en `.SddIA/.dev/.env` de esta instancia. `invoke-aiua-core` lo lee como fallback de `inputs.model`; se propaga a `agy --model`. |
| F3 | «Se crea `llm.interact.schema.json`» | **Inexacto** | Existe (v1.0.0). Solo valida el sobre (`success`, `data`, `error`, `additionalProperties: true`). No define `system_prompt`, `temperature`, `expected_format`. Cualquier cambio = mutación de `directories.library_norms` (DA-2, gobernada) + bump. |
| F4 | Eventos `Tool_Degraded` / `Status_Restored` | **Fósil** | Purgados por `adecuar-ed-telemetry`. Canónicos: `Domain_Entity_Degraded` / `Domain_Entity_Restored` / `Domain_Entity_Deprecated` con `entity_type`/`entity_id` en payload. Suscriptores vigentes: Cerbero `cerbero-governance-react` (revoca RBAC), Dédalo `fix-tool-process`, Radamanto `iota-immutable-publisher`. |
| F5 | `SddIA/core/llm-registry.json` con claves `gemini-flash-3.8`, `antigravity-client` y `fallback_route` a slug | **Slug-ley en Core** | Contradice H7/H12 (PBI tiers), §5 Agnosticismo y el propio §0. El **esquema** del registro es Core; la **instancia** del registro (oráculos, model-ids, credenciales) es bóveda `.SddIA/`. |
| F6 | «Oráculo Local Soberano: Ollama + DeepSeek» como catálogo accesible | **Alucinación** (reconocida en transcripción) | No hay inferencia local. Racso: «llm local no tenemos… tenemos los facilitados por Cursor». Fase 2 diferida. |
| F7 | Matriz «Tekton → Antigravity, Argos → Ollama» | **Fuera de alcance** | Fases `agent:` se resuelven por `llm_profile.tier` → `SDDIA_LLM_TIER_*` → harness `kalma2-agent-runtime-cursor.py` (`cursor-agent --print`). Done. Este PBI no enruta fases `agent:`. |
| F8 | «Las Skills emiten intención al bus; el resolver DI escucha `domain_affinity`» | **Incoherente con DI vigente** | `capability_di_resolver` resuelve `requires_capability` → **un** provider por binding (`capability-bindings.md`), sin afinidad ni reintento. `mayeuta-llm` **es** el provider de `llm:interact`, no un emisor al bus. Introducir afinidad/fallback en el resolver = nueva semántica de contrato de proceso; ver H-LOCUS. |
| F9 | «Cerbero bloquea el oráculo; Radamanto sella `Tool_Degraded`» como órgano nuevo | **Ya existe** (con nombres canónicos) | Pipeline `Domain_Entity_Degraded` → Cerbero RBAC / Radamanto DLT operativo. Este PBI **reutiliza**, no reimplementa. |
| F10 | «HTTP 429 → `exitCode` que denote cuota» | **Sin base** | `capsule-json-io`: `exitCode === 0 ⟺ success`; no hay taxonomía de causas. `gemini-http-infer` codifica `http-status-{code}` y `gemini-model-unavailable` en `error` (texto). `antigravity-cli-executor` emite `agy-failed`. La clasificación 503/red vive hoy en la **epidermis** `kalma2-bridge` (`is_gemini_upstream_unavailable`, sanitize agy network) — deuda: la causa debe viajar tipada desde el adaptador. |
| F11 | `sddia-client-bridge.py`, Tormentosa como `agent:`, inyección de Constitución por puente | **Fósiles v1.0.0** | Ya laudado en v1.1.0; se mantiene el veto (§3). |

## 2. Estado as-is (2026-09-25) — superficies LLM ortogonales

**No fusionar.** Solo las marcadas *Dentro* entran en Fase 1.

| Superficie | Órgano | Resolución de modelo hoy | Vars bóveda | Fase 1 |
|------------|--------|--------------------------|-------------|--------|
| Fases `agent:` (feature / bug-fix / refactorization / PPR) | `agent_runtime.rs` → harness `kalma2-agent-runtime-cursor.py` (`resolve_phase_model`) | tier → `SDDIA_LLM_TIER_*` → `SDDIA_AGENT_RUNTIME_MODEL` (default harness `composer-2.5`); CLI `SDDIA_AGENT_RUNTIME_CLI` ∥ `SDDIA_LLM_CLI_COMMAND` ∥ `cursor-agent --print` | `SDDIA_LLM_TIER_*`, `SDDIA_AGENT_RUNTIME_*` | **Fuera** (Done tiers) |
| Capacidad `llm:interact` | `capability-bindings` → `skill:mayeuta-llm` (SYNTHESIZE / CLASSIFY_INTENT / STREAM) | Spawnea `SDDIA_LLM_CHAT_COMMAND` ≻ `SDDIA_LLM_CLI_COMMAND`; prompt stdin, texto stdout; `telemetry_receipt.llm_model` best-effort | esas dos (+ `SDDIA_LLM_INFER_COMMAND`, `SDDIA_LLM_REQUIRE_INFER` en Kalma2 CHAT_STREAM) | **Fuera** del rebind; **dentro** como consumidor opcional del router (H-MAYUTA-CLI, sin tocar la skill) |
| Latido Aiúa `Combustion-Inferencia` | `aiua-stimulus-processing` v1.2.0 → `skill:antigravity-cli-executor`; handler `aiua_stimulus.rs` | `inputs.model` ∥ `SDDIA_GEMINI_MODEL` → `agy --model`; `effort` → `SDDIA_AGY_EFFORT` | `SDDIA_AGY_PATH`, `SDDIA_AGY_EFFORT`, `SDDIA_AGY_TIMEOUT_SECS`, sesión `agy` | **Dentro** — consumidor primario del router |
| Martillo HTTP Gemini | `tool:gemini-http-infer` v1.0.0 (L-MODEL) | `request.model` ∥ `SDDIA_GEMINI_MODEL` obligatorio; `SDDIA_GEMINI_THINKING_LEVEL` | `GEMINI_API_KEY`, `SDDIA_GEMINI_API_BASE_URL`, `SDDIA_GEMINI_HTTP_TIMEOUT_SECS`, `SDDIA_GEMINI_MODEL` | **Dentro** — adaptador |
| Martillo CLI Antigravity | `skill:antigravity-cli-executor` v1.0.0 | `params.model` → `--model`; `effort`; `print_timeout` | `SDDIA_AGY_*`, `SDDIA_AGY_ALLOW_SKIP_PERMISSIONS` | **Dentro** — adaptador |
| Síntesis Argos post-merge | `notify-humanized-pr-merged` → `gemini-http-infer`; fail-soft `skipped` si falta `SDDIA_GEMINI_MODEL` | directo | `SDDIA_GEMINI_*` | **Fuera** (fail-soft ya existe; migrar = Fase 1b) |
| Embeddings memoria | `SddIA/core/memory` `sddia-local-hashing-v1` | n/a | — | **Fuera** (no es chat) |

Tiers en genoma (`agents-contract.md` §5): Mayeuta/Dédalo `high`; Argos `medium`; Tekton `low`; Cerbero/Cúmulo/Radamanto `none`. Intactos.

## 3. Fósiles — no implementar

- `.SddIA/client/sddia-client-bridge.py` (podado en `kalma2-bridge-rust`, `40ef941`).
- Tormentosa/Aiúa como `agent:` del catálogo (`SddIA/agents/`). La Aiúa es `paths.directories.conscience` / `aiua_core.md`.
- Inyección de `CONSTITUTION_CORE.md` por petición (H-LLM-4: solo `aiua_core.md`; la Constitución es parámetro de sesión).
- `Tool_Degraded` / `Status_Restored` / `Tool_Deprecated`.
- `SddIA/core/llm-registry.json` con slugs.
- Reabrir `llm_profile` / `resolve_phase_model` / veto `none`.
- Acoplar `kalma2-bridge` (WUI) al router.
- Sustituir Cerbero/Cúmulo/Radamanto por LLM. Unificar embeddings con chat.

## 4. Alcance Fase 1 (este ciclo)

### 4.1 Contrato de adaptador (Core, gobernado)

Un contrato I/O único que **todo adaptador de proveedor** consume y devuelve. Campos mínimos de request: `prompt` (obligatorio), `system_prompt?`, `model?`, `effort?` (`high|medium|low`), `timeout_ms?`, `expected_format?` (`text|json`). Respuesta: sobre `capsule-json-io` 2.0 con `data.text`, `data.telemetry_receipt` (`llm_model`, `provider`, `prompt_tokens?`, `completion_tokens?`, `provider_latency_ms`) y, si `success:false`, `data.error_code` tipado:

`rate_limited | timeout | upstream_unavailable | auth | malformed_response | network | unknown`

Locus del contrato: `SddIA/library/norms/capability-contracts/` (H-TERM decide si es `llm.infer.schema.json` nuevo o bump de `llm.interact` a 1.1.0). Alta/bump **vía entity-manager** (DA-2). Si es término nuevo, alta en `capability-taxonomy.md` con laudo (AC-NO-INVENT).

### 4.2 Registro de oráculos (esquema Core, instancia en bóveda)

- **Core:** `llm-registry.schema.json` (JSON Schema) junto al contrato; Cúmulo clave `instance.llm_registry` → ruta de instancia (default `.SddIA/llm-registry.json`, gitignored). Starter-kit: `.SddIA/llm-registry.example.json` con `adapter_ref` a entidades del genoma (`tool:gemini-http-infer`, `skill:antigravity-cli-executor`) y **`model` vacío**.
- **Entrada del registro:** `oracle_id` (libre, de instancia), `adapter_ref` (entidad Core), `model` (slug de instancia, opcional si el adaptador tiene default de bóveda), `affinity[]` (etiquetas libres; el consumidor pide una), `timeout_ms`, `fallback` (`oracle_id` o `null`), `status` (`active|disabled`).
- **Ningún** `oracle_id` ni slug se referencia desde `SddIA/`.

### 4.3 Router (Core, cápsula Rust)

`tool:llm-router` — cápsula Rust (`capsule-json-io` 2.0, stdin/stdout). Entrada: request del contrato §4.1 + `affinity?` + `oracle_id?`. Comportamiento:

1. Carga registro de instancia vía Cúmulo. Sin registro → `success:false`, `error_code: unknown`, mensaje `llm-registry-missing` (fail-soft del consumidor decide).
2. Selecciona oráculo: `oracle_id` explícito ≻ primer `active` con `affinity` ≻ primer `active`.
3. Invoca el adaptador (`adapter_ref`) por stdin/stdout con `model`/`timeout_ms` del registro.
4. Si `success:false` y `error_code ∈ {rate_limited, timeout, upstream_unavailable, network}` → sigue `fallback` (cadena acíclica; máx. N saltos; ciclo = error de configuración).
5. Devuelve la primera respuesta `success:true` con `telemetry_receipt` del oráculo efectivo + `attempts[]` (oráculo, `error_code`, latencia) de los fallidos. Si todos fallan → `success:false` con `attempts[]` completo.
6. Telemetría: cada intento fallido queda en `attempts[]`; el Peaje Termodinámico del proceso consumidor agrega `cognitive-degraded: true` si hubo salto. La emisión de `Domain_Entity_Degraded` (entity_type `tool|skill`, entity_id = adaptador) se delega al pipeline vigente de Radamanto sobre telemetría; el router **no** emite eventos ECST directamente (H-EMIT).

Sin lógica de dominio: el router no conoce Aiúa, Kalma2 ni Argos.

### 4.4 Adaptadores existentes (mutación gobernada, DA-2)

- `tool:gemini-http-infer` v1.1.0: acepta request §4.1 (mapeo `prompt`/`system_prompt`/`model`/`effort`→`thinkingLevel`); `error_code` tipado (429 → `rate_limited`, 503 → `upstream_unavailable`, 401/403 → `auth`, timeout ureq → `timeout`, candidato vacío → `malformed_response`). Compatibilidad con request legacy.
- `skill:antigravity-cli-executor` v1.1.0: request §4.1 (`effort`, `model`, `timeout_ms`→`print_timeout`); `error_code` tipado (network issue / dial tcp → `network`; timeout → `timeout`; cuota agy → `rate_limited` si el CLI lo distingue, else `upstream_unavailable`).
- Ninguno `provides llm:interact` (L-ORTHOGONAL-INTERACT se mantiene). `provides` del término de §4.1 según H-TERM.

### 4.5 Consumidor primario: latido Aiúa

`aiua-stimulus-processing` v1.3.0 (entity-manager): `Combustion-Inferencia` → `tool:llm-router` con `affinity: "aiua"` (etiqueta; la instancia decide qué oráculo la sirve). Handler `aiua_stimulus.rs`: `infer_antigravity_cli` → `infer_via_router`; `model`/`effort` de inputs se pasan como *override* opcional. Test de genoma actualizado. Sin registro de instancia → fail-soft: error tipado hacia `kalma2-bridge`, sin retry.

### 4.6 Bóveda / starter-kit

Familias de variables comentadas, vacías, sin slug: `SDDIA_LLM_REGISTRY_PATH?` (override de la clave Cúmulo), familias existentes `SDDIA_GEMINI_*`, `SDDIA_AGY_*`. Nota explícita: el registro decide el oráculo; las vars de familia solo aportan credenciales/defaults del adaptador.

### 4.7 Fuera de Fase 1 (Fase 1b / Fase 2, deuda explícita)

- `mayeuta-llm` como consumidor del router (modo texto plano o `SDDIA_LLM_CLI_COMMAND` apuntando a un shim). No se toca la skill.
- Migrar `notify-humanized-pr-merged` al router.
- Circuit breaker de estado (Cerbero revoca RBAC del adaptador degradado y el router lo salta hasta `Domain_Entity_Restored`): depende de la emisión de degradación desde telemetría; se diseña, no se implementa.
- Adaptador local (Ollama/vLLM) — Fase 2. El registro ya lo admite (`adapter_ref` nuevo + `BASE_URL` de familia).
- Adaptador Cursor (`cursor-agent --print`) como oráculo del registro — solo si laudo; hoy es harness de fases `agent:`.

## 5. Hipótesis a laudar en clarify (Mayeuta)

| ID | Pregunta | Propuesta v1.2.0 | Riesgo si se decide distinto |
|----|----------|------------------|------------------------------|
| **H-LOCUS** | ¿Router en el motor (`capability_di_resolver`/inyección) o cápsula Rust? | **Cápsula** `tool:llm-router`. Respeta «agentes orquestan, cápsulas ejecutan»; no reabre el resolver DI (1 provider por binding); es el binario al que una instancia puede apuntar `SDDIA_LLM_CLI_COMMAND` (H-MAYUTA-CLI). | Motor: nueva semántica `affinity` en `requires_capability`, reintento en la capa de inyección, contrato de proceso 1.5.0. Trabajo ×3. |
| **H-TERM** | ¿Nuevo término `llm:infer` (adaptador) o bump `llm.interact` 1.1.0? | **Nuevo término** `llm:infer` / `llm.infer`: `llm:interact` es interacción gobernada (síntesis/intención, provider mayeuta-llm); `llm:infer` es inferencia cruda de proveedor. Evita ambigüedad del resolver (`CAPABILITY_PROVIDER_AMBIGUOUS`). Requiere laudo AC-NO-INVENT. | Bump: mezcla sobre de skill con sobre de adaptador; adaptadores pasarían a `provides llm:interact` (rompe L-ORTHOGONAL-INTERACT). |
| **H-REGISTRY-LOCUS** | ¿Registro en `SddIA/core/` o `.SddIA/`? | Esquema en Core; **datos en `.SddIA/llm-registry.json`** (Cúmulo `instance.llm_registry`). | Core con slugs = F5. |
| **H-EMIT** | ¿El router emite `Domain_Entity_Degraded`? | **No.** Devuelve `attempts[]` tipados; Radamanto decide degradación desde telemetría (umbral, no un fallo aislado). | Router emisor = acoplamiento a bus y falsos positivos por un 429. |
| **H-AIUA-OVERRIDE** | ¿`inputs.model` de `aiua-stimulus-processing` sigue vivo? | Sí, como override explícito hacia el oráculo elegido; `SDDIA_GEMINI_MODEL` deja de ser el default del latido (lo pone el registro). | Romper `api-aiua-interact` que hoy puede pasar `model`. |

## 6. Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-CONTRACT | Contrato de adaptador (`llm.infer` o bump laudado) en `library_norms/capability-contracts`, creado vía entity-manager, con `error_code` enum y `telemetry_receipt` definidos. Taxonomía actualizada si hay término nuevo. |
| CA-REGISTRY | `llm-registry.schema.json` en Core; Cúmulo `instance.llm_registry`; example en starter-kit con `model` vacío. `rg` de slugs `gemini-|llama-|composer-|deepseek` en `SddIA/` (excl. tests/lab mocks) = 0 nuevos. |
| CA-ROUTER | `tool:llm-router` Rust; tests: selección por `oracle_id`/`affinity`, salto por `rate_limited`/`timeout`/`upstream_unavailable`/`network`, no salto por `auth`/`malformed_response`, cadena acíclica, registro ausente → fail-soft tipado, `attempts[]` completo. |
| CA-ADAPTERS | `gemini-http-infer` y `antigravity-cli-executor` aceptan request del contrato y devuelven `error_code` tipado; requests legacy siguen funcionando (tests). Sin `provides llm:interact`. |
| CA-AIUA | `aiua-stimulus-processing` v1.3.0 → `tool:llm-router`; handler y test de genoma actualizados; lab mock: simulación de `rate_limited` en primario devuelve `success:true` por secundario y `telemetry.attempts.len()==1`. |
| CA-ORTHO | Superficies §2 «Fuera» sin diff: `agent_runtime.rs`, `resolve_phase_model`, `capability-bindings` `llm:interact`, `SddIA/agents/*.md`, `kalma2-bridge` (salvo lectura de `error_code` si ya se expone). |
| CA-FAILSOFT | Sin registro o con todos los oráculos caídos, el consumidor recibe `success:false` + `error_code` + `attempts[]`; ningún fallo deglutido sin `telemetry_receipt`. |
| CA-GOV | Toda mutación de `tools/`, `skills/`, `process/`, `library/norms/` vía entity-manager; evolution con uuid de este PBI. |
| CA-DOC | Cascada `docs/features/multi-llm-router/` completa; PBI en `done/` y `validacion.md` APTO en el mismo PR (`task-closure-documental`). |

## 7. Laudos del Vértice extraídos de la transcripción (2026-09-09)

1. No existe LLM local; existen los oráculos facilitados por Cursor. No diseñar sobre Ollama/DeepSeek.
2. Se requiere un **listado/configuración** de LLMs accesibles (hoy inexistente; el enrutamiento es estático por cápsula).
3. Contratos de LLM **desacoplados** de implementaciones (`cursor-cli`, `antigravity-cli`, `gemini-http`), inyectables en consumidores.
4. Aprobado preparar el PBI para refinamiento (este documento).

## 8. Referencias de contraste

- Tiers done: `docs/features/arquitectura-llm-tiers/`
- Vector Aiúa CLI: `docs/features/aiua-antigravity-cli-vector/`, evolution `1dc4055c`
- Auditoría Aiúa: `docs/features/nucleo-aiua-tormentosa-motor/auditoria.md` (H-LLM-1…4; H-LLM-1 cerrado por 1dc4055c)
- Telemetría cognitiva: `docs/features/telemetria-cognitiva-llm-kalma2/spec.md` (L2/L3/L5: `llm_model`, `provider_latency_ms`, `cognitive-degraded`)
- Taxonomía ECST vigente: `docs/features/adecuar-ed-telemetry/`
- DI: `README.md` § Inyección de dependencias por capacidades; `capability_di_resolver.rs`
