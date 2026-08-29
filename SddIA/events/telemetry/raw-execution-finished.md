---
uuid: "5a02d313-685d-4464-84c1-ffe16ef6ba6d"
name: "raw-execution-finished"
version: "1.1.0"
contract: "events-contract v1.1.0"
event_family: "telemetry"
event_type: "Raw_Execution_Finished"
context: "system-operations"
capabilities:
  - "raw_execution_finished"
  - "thermodynamic_toll"
hash_signature: "sha256:1966b61a60f674771546de1f248bed6a4957a31f74ca16ad65ff1580e9405cf7"
---

# Event: Raw_Execution_Finished

Telemetría física emitida por el CLI al finalizar el Peaje Termodinámico (cronómetro, `exit_code`, `asset_id`). Instancias en `./.events/telemetry/` (`eda_fractal.telemetry`).

## Payload ECST

### REQUIRED

- `asset_id`
- `exit_code`
- `duration_ms`
- `process_name`

### OPTIONAL

- `telemetry_receipt` — objeto JSON con métricas cognitivas LLM (SSOT único; ver abajo)
- `capsule_id` — skill/tool/action invocada (resolución contrato ED)
- `execution_id`
- `workspace_path`
- `cycle_phase`, `lab_hollow`, `detached_child`, `failed_phase*` — supervivencia / ciclo de vida

### `telemetry_receipt` (v1.1.0)

| Campo | Obligatorio compliance* | Descripción |
|-------|-------------------------|-------------|
| `prompt_tokens` | sí* | Tokens de entrada |
| `completion_tokens` | sí* | Tokens de salida |
| `llm_model` | no | Modelo del proveedor |
| `tier` | no | Tier / plan |
| `provider_latency_ms` | no | Latencia del proveedor (≠ `duration_ms` del Peaje) |
| `cognitive-degraded` | no | `true` si la prótesis omitió `usage` (tokens a cero; no hard fail) |

\* Solo si la entidad declara `telemetry_provided: true` en su contrato ED.

Mapeo interno de cápsula: `thermodynamic_cost.tokens_in` → `prompt_tokens`, `tokens_out` → `completion_tokens`, `duration_ms` → `provider_latency_ms`. Prohibido `cognitive_metrics` como tercer nombre.

### FORBIDDEN

- *(ninguno en v1.1.0)*

## Emisores autorizados

- `execute-process`
- `execute-action`
- Procesos/cápsulas CLI indexados que implementen Peaje Termodinámico (`execute_process_capsules`)

## Suscripciones

`SddIA/core/event-telemetry-subscriptions.json` → `route-telemetry`:

| Suscriptor | Proceso | Intención |
|------------|---------|-----------|
| `radamanto` | `radamanto-batch` | Acumulado estadístico; umbrales; gobernanza + `Domain_Entity_Telemetry_Captured` |
| `argos` | `telemetry-compliance-audit` | Cruce recibo vs contrato ED; breach si aplica |
