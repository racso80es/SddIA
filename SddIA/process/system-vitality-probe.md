---
context: quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:43dc6f73d159486c68cc2d60a2e8f1a7d43316ace2a7c295057035d4187e7ced"
name: system-vitality-probe
phases:
- intent: Correr bus.topology, cumulo.tools_index, cerbero.config, kalma2.http con causa física.
  name: Ejecutar sondas
  id: ejecutar_sondas
- intent: Escribir System_Vitality_Probed en eda_fractal.telemetry.
  name: Emitir telemetría
  id: emitir_telemetria
- intent: Sonda roja → System_Fracture_Detected; reset al volver a verde.
  name: Fractura idempotente
  id: fractura_idempotente
uuid: b215b373-f6d3-4fb1-9d55-60eb260df5cc
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# system-vitality-probe

## Propósito

Ejecutar sondas deterministas sobre invariantes del repositorio y de Kalma2, persistir estado por `probe_id` y emitir telemetría/fractura en el bus fractal. Handler nativo en `SddIA/engine/execute-process/src/engine/handlers/system_vitality.rs`; invocado también desde `event-watcher` vía `invoke_named_probe`.

## Sondas (`probe_id`)

| id | Comprobación física |
| --- | --- |
| `bus.topology` | `SddIA/core/cumulo.paths.json` legible; suscripciones EDA referenciadas existen bajo el repo |
| `cumulo.tools_index` | Coherencia del índice de tools declarado en Cúmulo |
| `cerbero.config` | Configuración Cerbero / execution-contexts resoluble |
| `kalma2.http` | Socket HTTP hacia endpoint Kalma2 configurado (`SDDIA_*` / `.SddIA`) |

## Orquestación y sellado EDA

1. Ejecuta cada sonda y agrega `ProbeResult { ok, cause }`.
2. Escribe `System_Vitality_Probed` en `eda_fractal.telemetry` con detalle por sonda.
3. Si una sonda pasa de verde a rojo, emite `System_Fracture_Detected` idempotente por `probe_id`; al recuperar verde, resetea el contador en `.SddIA/daemons/state/vitality-probe.json` (`STATE_FILE` = `vitality-probe.json`).
4. No alimenta el panel Espejo; cadencia externa vía `SDDIA_VITALITY_PROBE_SECONDS` (default 300, piso 30).
