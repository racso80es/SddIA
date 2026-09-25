---
feature_name: multi-llm-router
created: "2026-09-25"
process: feature
base: docs/features/multi-llm-router/objectives.md
scope: contrato-registro-router-adaptadores-aiua
branch_name: feat/multi-llm-router
persist_ref: docs/features/multi-llm-router
document_id: PBI-MULTI-LLM-ROUTER
agents: dedalo
---

# Spec — multi-llm-router

## 1. Contrato `llm.infer` v1.0.0

Archivo: `SddIA/library/norms/capability-contracts/llm.infer.schema.json` (alta vía entity-manager `norm`/`library_norms`, según creator aplicable; si el creator no cubre `capability-contracts/`, alta gobernada + evolution). Término `llm:infer` en `capability-taxonomy.md` v1.0.8: «Inferencia cruda de proveedor LLM vía adaptador (`tool:`/`skill:`); sin gobernanza de intención; consumida por `tool:llm-router`».

### 1.1 Request (`request`)

| Campo | Tipo | Oblig. | Nota |
|-------|------|--------|------|
| `prompt` | string | sí | Texto ya ensamblado por el consumidor |
| `system_prompt` | string | no | Adaptadores sin soporte lo anteponen al `prompt` |
| `model` | string | no | Slug de instancia; si falta, default del adaptador (bóveda) |
| `effort` | `high\|medium\|low` | no | Gemini → `thinkingLevel`; agy → `--effort` |
| `timeout_ms` | integer ≥ 1000 | no | Gemini → timeout ureq; agy → `print_timeout` |
| `expected_format` | `text\|json` | no | Default `text`; `json` = el adaptador intenta parsear y falla con `malformed_response` |

### 1.2 Response (sobre `capsule-json-io` 2.0)

```json
{
  "success": true,
  "exitCode": 0,
  "data": {
    "text": "…",
    "raw": {},
    "telemetry_receipt": {
      "provider": "gemini-http-infer",
      "llm_model": "…",
      "prompt_tokens": 0,
      "completion_tokens": 0,
      "provider_latency_ms": 0
    }
  },
  "error": null
}
```

Fallo: `success:false`, `exitCode:1`, `data.error_code` ∈ `rate_limited | timeout | upstream_unavailable | auth | malformed_response | network | unknown`, `data.telemetry_receipt.provider` presente, `error` texto humano (mantiene `http-status-{code}` / `agy-failed` para compatibilidad de epidermis).

`telemetry_receipt` alineado con `telemetria-cognitiva-llm-kalma2` L2/L3 (`llm_model`, `provider_latency_ms`, tokens best-effort).

## 2. Registro de oráculos

### 2.1 Esquema Core

`SddIA/library/norms/capability-contracts/llm-registry.schema.json` (JSON Schema draft 2020-12):

```json
{
  "registry_version": "1.0.0",
  "oracles": {
    "<oracle_id>": {
      "adapter_ref": "tool:gemini-http-infer | skill:antigravity-cli-executor | …",
      "model": "",
      "affinity": ["aiua", "triage"],
      "timeout_ms": 30000,
      "fallback": "<oracle_id> | null",
      "status": "active | disabled"
    }
  }
}
```

Reglas: `adapter_ref` con prefijo `tool:`/`skill:` y nombre kebab-case existente en el catálogo (validación runtime por el router: `.md` bajo `directories.tools|skills`); `fallback` debe existir en `oracles`; grafo de `fallback` acíclico (validación router). `model` vacío permitido.

### 2.2 Localización

- Cúmulo: `instance.llm_registry: ".SddIA/llm-registry.json"` (edición directa de `SddIA/core/cumulo.paths.json` — Core SSOT, no genoma protegido DA-2; verificar con `sddia-qa`).
- Override: env `SDDIA_LLM_REGISTRY_PATH` (bóveda instancia).
- Starter-kit: `SddIA/scripts/starter-kit/.SddIA/llm-registry.example.json` con `oracle-agy` → `skill:antigravity-cli-executor` (`affinity: ["aiua"]`, `fallback: "oracle-gemini"`) y `oracle-gemini` → `tool:gemini-http-infer` (`fallback: null`); `model: ""`.
- Instancia (L-VAULT, no genoma): `.SddIA/llm-registry.json` misma topología. Gitignored.
- `.gitignore`: entrada explícita `.SddIA/llm-registry.json` (`.SddIA/` no se ignora entera).

## 3. Cápsula `tool:llm-router`

Ruta fuente: `SddIA/tools/llm-router/` (Cargo, deps `sddia-io`, `serde_json`; sin red). Entidad `SddIA/tools/llm-router.md` vía entity-manager (`entity_class: tool`, `capabilities: [llm_route]`). El router **consume** `llm:infer`; no lo `provides` (eso es de los adaptadores). Las fases lo referencian por `delegates_to: tool:llm-router`.

### 3.1 Entrada

```json
{ "request": { …llm.infer… }, "affinity": "aiua", "oracle_id": null, "max_hops": 3 }
```

### 3.2 Algoritmo

1. Resolver ruta del registro: `SDDIA_LLM_REGISTRY_PATH` ≻ Cúmulo `instance.llm_registry` (relativo a repo root recibido en `repo_root` o cwd). Ausente/ilegible → `success:false`, `error_code: unknown`, `error: "llm-registry-missing"`, `attempts: []`.
2. Validar: esquema mínimo, `fallback` existentes, aciclicidad. Fallo → `error_code: unknown`, `error: "llm-registry-invalid: …"`.
3. Selección inicial: `oracle_id` ≻ primer `active` con `affinity` ∋ solicitada (orden de declaración) ≻ primer `active`. Ninguno → `error_code: unknown`, `error: "llm-registry-no-active-oracle"`.
4. Bucle (≤ `max_hops`, default 3): construir request al adaptador (`model` del registro salvo override explícito `request.model`; `timeout_ms` del registro salvo override); invocar `adapter_ref` por binario compilado (`compiled_capsules.native_root/{profile}/{name}`, release ≻ debug; paridad con `capsules.rs`) con stdin JSON; parsear sobre.
5. `success:true` → devolver `data` del adaptador + `data.routing: { "oracle_id", "hops", "attempts": [...] }`.
6. `success:false` con `error_code ∈ {rate_limited, timeout, upstream_unavailable, network}` y `fallback` no nulo → anotar `attempts[]` (`oracle_id`, `adapter_ref`, `error_code`, `provider_latency_ms`) y continuar. Otro `error_code` o sin `fallback` → devolver el fallo del adaptador enriquecido con `data.routing.attempts`.
7. Agotado → `success:false`, `error_code` del último intento, `data.routing.attempts` completo.

Laboratorio: única variable propia `SDDIA_LLM_ROUTER_ADAPTER_BIN_DIR` (directorio con binarios/scripts stub nombrados como el adaptador) para tests herméticos del salto; documentada como lab, no producto. Los mocks de proveedor existentes (`SDDIA_LAB_MOCK_OUTBOUND`, mock agy) siguen sirviendo para smokes de adaptador.

### 3.3 Salida de error tipada

Idéntica a §1.2 fallo; `data.routing.attempts` obligatorio (posiblemente vacío).

## 4. Adaptadores

### 4.1 `tool:gemini-http-infer` v1.1.0

- Acepta `request` §1.1 y legacy (`request.prompt` + `request.model` + `request.parameters`).
- Mapeo: `effort` → `thinkingLevel` (respeta `SDDIA_GEMINI_THINKING_LEVEL` como default); `system_prompt` → `systemInstruction`; `timeout_ms` → timeout ureq (default `SDDIA_GEMINI_HTTP_TIMEOUT_SECS`).
- `error_code`: 429 → `rate_limited`; 500/502/503/504 → `upstream_unavailable`; 401/403 → `auth`; 404 modelo (`gemini-model-unavailable`) → `upstream_unavailable`; ureq timeout/transport → `timeout` / `network`; candidato vacío o JSON inválido → `malformed_response`; resto → `unknown`.
- `telemetry_receipt.provider = "gemini-http-infer"`; `llm_model` = modelo efectivo; tokens desde `usageMetadata` si existe.
- `.md`: bump vía entity-manager (`lifecycle_operation: update`).

### 4.2 `skill:antigravity-cli-executor` v1.1.0

- Acepta `request` §1.1 y legacy (`request.prompt` + `request.parameters.{model,effort,print_timeout}`).
- Mapeo: `timeout_ms` → `print_timeout` (`"{s}s"`); `system_prompt` → prefijo del prompt (agy print no expone system).
- `error_code`: timeout del proceso → `timeout`; stderr/JSON con `network issue|dial tcp|ECONN` → `network`; `429|quota|rate limit` → `rate_limited`; `5xx|unavailable` → `upstream_unavailable`; salida no JSON → `malformed_response`; binario ausente → `unknown` (no salta: es bóveda).
- `telemetry_receipt.provider = "antigravity-cli-executor"`.

Ninguno añade `provides llm:interact`. Ambos añaden `provides: [{ id: "llm:infer", contract: "llm.infer", version: "1.0.0" }]` — dos providers del mismo término **no** rompen el resolver mientras ningún process declare `requires_capability: llm:infer` sin `delegates_to` (el router los invoca por nombre, no por DI). Documentar en `.md` de ambos.

## 5. Consumidor Aiúa

- `SddIA/process/aiua-stimulus-processing.md` v1.3.0 (entity-manager update): fase `Combustion-Inferencia` `delegates_to: [tool:llm-router]`, intent «Única llamada al LLM vía router de oráculos de instancia (`affinity: aiua`); Peaje Termodinámico del adaptador efectivo». Input `model` documentado como override.
- `aiua_stimulus.rs`: `infer_antigravity_cli` → `infer_via_router(repo, prompt, model_override, effort, timeout)`; `invoke_tool_capsule_json(repo, "llm-router", …)`. `resolve_print_timeout` → `timeout_ms`. Telemetría del proceso: `telemetry.model` = `telemetry_receipt.llm_model`; `telemetry.provider`; `telemetry.routing_attempts = attempts.len()`; `cognitive-degraded: attempts.len() > 0`.
- Sin registro → `Err("llm-registry-missing")` → sobre `success:false` hacia `api-aiua-interact`/`kalma2-bridge` (sin retry; bridge intacto).
- Tests: `process_genome_combustion_is_antigravity_cli` → `process_genome_combustion_is_llm_router` (prohíbe `antigravity-cli-executor` y `gemini-http-infer` literales en el genoma del proceso); lab: registro temporal con `primary` → stub `rate_limited`, `secondary` → stub ok; assert `success`, `routing.attempts.len()==1`, `hops==2`.

## 6. Bóveda y documentación

- `.env.example` (ambas): bloque `# --- Registro de oráculos LLM (tool:llm-router) ---` con `# SDDIA_LLM_REGISTRY_PATH=` y nota «el registro elige oráculo; las familias `SDDIA_GEMINI_*` / `SDDIA_AGY_*` aportan credenciales/defaults del adaptador».
- README § DI: fila `llm:infer` en catálogo vigente; nota router.
- Evolution: entrada con uuid `d2e44083-ccdf-45af-b477-f6c71833fc31` relacionada; `sddia-qa gate-evolution --json --range` antes del push.

## 7. Invariantes (CA-ORTHO)

Cero diff en: `agent_runtime.rs`, `kalma2-agent-runtime-cursor.py`, `capability-bindings.md` (fila `llm:interact`), `SddIA/agents/*.md`, `SddIA/skills/mayeuta-llm*`, `SddIA/interfaces/kalma2-bridge/`, `notify_humanized_pr_merged.rs`.
