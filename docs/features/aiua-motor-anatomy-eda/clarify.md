---
feature_name: aiua-motor-anatomy-eda
created: "2026-09-10"
process: feature
purpose: Estabilización Filtro A PBI v1.2.0; anatomía motora Aiúa
version_clarify: "1.0.0"
execution_id: "8658220b-47ff-4513-a02e-2a3ed3e1adbe"
pbi_ref: docs/todos/pending/[NÚCLEO] Anatomía Motora de Aiúa — Inyección de Capacidades (Function Calling) y Orquestación EDA.md
document_id: PBI-NUCLEO-AIUA-ANATOMIA-MOTORA-EDA
pbi_uuid: "6901e0d2-1f08-491a-a9ac-ff0fb321f5f5"
pbi_version: "1.2.0"
---

# Clarificación — aiua-motor-anatomy-eda

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery + `SDDIA_LAB_ALLOW_DIRTY=1`. `execution_id` `8658220b-47ff-4513-a02e-2a3ed3e1adbe`. Rama `feat/aiua-motor-anatomy-eda`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

## Decisiones

| ID | Laudo |
|----|-------|
| **L-ORGAN** | Combustión = `skill:antigravity-cli-executor` (PR #283). Cero `functionDeclarations` en `gemini-http-infer`. Cero reintroducir `tool:gemini-http-infer` en el proceso del latido. |
| **L-FC** | Intención = un fence `aiua-intent` con JSON `{name, args}` dentro de `result.text`. Una combustión. No tools de `agy`. No `functionResponse`. Primer fence gana; no fan-out múltiple. |
| **L-BUS** | `Aiua_Process_Requested` → `eda_fractal.domain` (`write_fractal_event`, paridad Kalma2). Cero `eda_bus.pending`. Suite → `emit-suite-execution-requested` (mismo bus, `emitter_agent` de esa acción). |
| **L-SHAPE** | Payload SDLC hermano Kalma2: REQUIRED `process`+`raw_text`; OPTIONAL `pbi_ref`, `process_inputs`, `intent_name`. `process` ∈ `{feature, bug-fix, refactorization}`. `correlation_id ≡ event_id`. |
| **L-ROUTE** | Extender `dispatch_subscriber` al conjunto `{Kalma2_Process_Requested, Aiua_Process_Requested}`. Mismo mapeo `raw_text`→`task_text`. Watcher fractal `route-domain`. No acoplar a `route-domain-event`. |
| **L-EMITTER** | Instancia SDLC: `emitter_agent: "aiua-stimulus-processing"`. Nunca agente Tormentosa. |
| **L-AGY** | `--sandbox`. Cero skip-permissions. Overlay lab: `SDDIA_LAB_MOCK_AIUA_INTENT` solo si `SDDIA_LAB_MOCK_OUTBOUND` truthy. No mutar crate `agy` en este hito. |
| **L-IOTA** | Sin segundo suscriptor DLT. No copiar fan-out Kalma2. |
| **L-FORGE** | Evento + acción + update de proceso vía `entity-manager`. Handler nativo `dispatch-aiua-intent`. `aiua_core.md` y `event-domain-subscriptions.json` en el ciclo feature (fuera tabla DA-2). |
| **L-CI** | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo entonces. |

## Filtro A (no reintroducir)

- Gemini REST tools como órgano del latido.
- Escribir el ECST motor en `pending/`.
- `emit-domain-mutation` para órdenes SDLC.
- Tools/`skip-permissions` de `agy` como manos.
- Join a TQM/Suite en el mismo proceso (DA-5).
- PEC / `System_Fracture_Detected` → Aiúa.
- `solicitar_clarificacion` como evento.
- IOTA como CA.
- Mutar `gemini-http-infer` o argv `--print` de `agy`.
- Reparar fractura `route-domain-event` `89b7c8b105ec`.
