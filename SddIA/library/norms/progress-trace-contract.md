---
uuid: "7d4e9f12-3a6b-4c5d-9e0f-1a2b3c4d5e6f"
name: "progress-trace-contract"
version: "1.0.0"
nature: "tactical-norm"
author: "tekton"
scope: "agnostic"
category: "architecture"
dependencies: []
hash_signature: "sha256:45bd8b4496ea0eeb95d50489f78502e302a4ec5fbfd933ced5df7e610377c106"
source_feature: "kalma2-canal-telemetria-progreso"
---

# progress-trace-contract

Norma táctica para **Progress Trace Capsule (PTC)** — canal efímero de observabilidad UI, **distinto** del peaje ECST (`eda_fractal.telemetry`) y del dominio (`eda_fractal.domain`).

## Directriz Core

1. PTC es schema **no-ECST**: prohibidos en raíz `event_type`, `emitter_agent`, `payload`, `delivery_state`, `event_family`.
2. Hoja Cúmulo: `eda_fractal.progress` → `./.events/progress` (sin `*_subscriptions`).
3. Join key saga Kalma2: `correlation_id` (= `event_id` de `Kalma2_Process_Requested`).
4. Emisión solo si `process_inputs.correlation_id` no vacío; best-effort (IO fallido no tumba `execute-process`).
5. Bridge **solo lee** la hoja; proyección SSE `GET /api/progress/stream?correlation_id=<uuid>` complementa (no sustituye) `GET /api/status`.

## Envelope runtime (un archivo = una traza)

Ruta: `{repo}/{eda_fractal.progress}/{correlation_id}/{trace_id}.json`

```json
{
  "trace_id": "<uuid-v4>",
  "correlation_id": "<uuid-v4>",
  "timestamp": "<ISO-8601 UTC>",
  "phase": "spec | clarify | plan | implementation | validation | closure",
  "severity": "info | warn | error | kaizen_alert",
  "source_agent": "cerbero | mayeuta | dedalo | tekton | argos | cumulo | orchestrator | <indexado>",
  "message": "<texto legible del hito>",
  "metadata": {}
}
```

| Campo | Obligatorio | Regla |
|-------|:-----------:|-------|
| `trace_id` | Sí | UUID v4 de la cápsula (≠ `event_id` ECST) |
| `correlation_id` | Sí | Saga Kalma2 / ejecución |
| `timestamp` | Sí | ISO-8601 UTC |
| `phase` | Sí | Enum UI cerrado; mapear desde `phase.name` del proceso |
| `severity` | Sí | Enum cerrado |
| `source_agent` | Sí | Badge UI; agente de `delegates_to` o `orchestrator` |
| `message` | Sí | No vacío tras trim |
| `metadata` | No | Extensible (`process_name`, `phase_name_raw`, `moment`, …) |

## Mapa fase proceso → `phase` UI

| `phase.name` | `phase` PTC |
|--------------|-------------|
| Inicialización Git / init | `spec` |
| Estabilización de Requisitos / Clarify* | `clarify` |
| Diseño de Blueprint / Diseño del fix | `plan` |
| Ejecución | `implementation` |
| Verificación | `validation` |
| Cierre documental* / Cierre de entrega / finalize* | `closure` |
| Desconocido | `implementation` + `metadata.phase_name_raw` |

Emisión mínima: **inicio y fin** de cada fase orquestada con `correlation_id`.

## Restricciones Duras (Aduana de Fricción)

- Prohibido emitir PTC en `eda_fractal.telemetry`, `eda_bus.pending` o `telemetry_compliance.emitted_registry`.
- Prohibido catalogar PTC como Clase ECST bajo `SddIA/events/`.
- Sweeper poda `{progress}/{correlation_id}/` por PEC terminal o TTL; **nunca** mezclar con purge peaje.

## Referencias

- Feature: `docs/features/kalma2-canal-telemetria-progreso/spec.md`
- SSOT paths: `SddIA/core/cumulo.paths.json` → `eda_fractal.progress`
