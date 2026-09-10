---
feature_name: aiua-motor-anatomy-eda
created: "2026-09-10"
process: feature
base: main
scope: core
branch_name: feat/aiua-motor-anatomy-eda
persist_ref: docs/features/aiua-motor-anatomy-eda
execution_id: "8658220b-47ff-4513-a02e-2a3ed3e1adbe"
document_id: PBI-NUCLEO-AIUA-ANATOMIA-MOTORA-EDA
---

# Spec — aiua-motor-anatomy-eda

## 1. Genoma conscience

`SddIA/conscience/aiua_core.md` (fuera DA-2): sección **Anatomía Motora**.

- Tabla de tendones MVP (`ordenar_refactorizacion`, `iniciar_feature`, `iniciar_bug_fix`, `requerir_auditoria`, `solicitar_clarificacion`).
- Contrato del fence:

````markdown
```aiua-intent
{"name":"<tendón>","args":{}}
```
````

- Veto: no ejecutar, no tools `agy`, no terminal. Delegación = ECST.

## 2. Parser (handler)

Módulo o fn en `aiua_stimulus.rs` / `aiua_intent.rs`:

- `extract_aiua_intent(text) -> Option<{name, args}>` — primer fence `aiua-intent`.
- Lab: si `SDDIA_LAB_MOCK_OUTBOUND` y `SDDIA_LAB_MOCK_AIUA_INTENT` JSON no vacío → overlay.
- Catálogo motor SDLC / Suite / no-motor según PBI §3.

## 3. Acción `dispatch-aiua-intent`

`entity-manager` `create` `entity_class: action`.

Inputs: `function_call.name`, `function_call.args` (object), `correlation_id` opcional.  
Outputs: `success`, `event_id`, `target_path`, `event_type`, `intent_dispatched`.

Handler nativo (nuevo `aiua_intent.rs`, registro en `actions.rs` `try_run_native` + `try_action` si aplica):

- SDLC: `raw_text` = `goal` + `target_component` (y resto de args serializados de forma estable). `process` mapeado. `write_fractal_event(..., "domain")` + `validate_ecst_event`. `emitter_agent: "aiua-stimulus-processing"`. `correlation_id ≡ event_id`.
- Suite: `suite_execution_requested::run` con `suite_id`.
- Error: `success: false`, cero write.

## 4. Evento `Aiua_Process_Requested`

`entity-manager` `create` `entity_class: event`:

- `event_name: aiua-process-requested`
- `event_family: domain`
- `event_type: Aiua_Process_Requested`
- payload_required: `process`, `raw_text`
- payload_optional: `pbi_ref`, `process_inputs`, `intent_name`
- payload_forbidden: host paths, scripts, secretos, tools agy
- emitter_agents: `aiua-stimulus-processing`

Índice de familia lo sella event-creator.

## 5. Suscripción y route

`SddIA/core/event-domain-subscriptions.json`: clave nueva → TQM (`agent: tekton`). Sin IOTA.

`route_domain_core.rs` `dispatch_subscriber`: tratar `Aiua_Process_Requested` con el mismo bloque que Kalma2 (`ALLOWLIST_KALMA2` en route; el emisor Aiúa ya recorta a DISPATCHABLE).

## 6. Proceso latido

`entity-manager` `update` `aiua-stimulus-processing`:

- UUID `6c595785-e386-402f-b570-0b2aa6343051` inmutable.
- Bump `1.1.0` → `1.2.0`.
- Fase nueva `Despacho-Motor` tras combustión, antes de persistencia: `action:dispatch-aiua-intent` (opcional si no hay tendón motor).
- Outputs: `intent_dispatched`, `event_id` opcionales.
- No reenviar `process_contract_version` default 1.3.0.
- Cuerpo: mencionar despacho EDA; conservar `skill:antigravity-cli-executor`.

`run()` del handler:

1. contexto + invoke-aiua-core + agy (igual).
2. parse/overlay.
3. si motor → dispatch (no TQM).
4. persistir texto combustido; si vacío y hay despacho → sintético `intent={name}; event_id={uuid}`.
5. envelope `data.intent_dispatched` / `event_id` / `correlation_id`.

## 7. Tests

- Parser: fence ok / JSON roto / nombre desconocido / primer fence.
- Dispatch: write domain; reject args; reject pending path; suite existente; suite ausente.
- `dispatch_subscriber` lab-sync: `Aiua_Process_Requested` + `process=feature` mapea `task_text`.
- Latido lab-mock: overlay intent → acuse + archivo domain; genoma sigue sin `tool:gemini-http-infer`.

## 8. Fuera

Crates HTTP/CLI. `--print`. Kitchen router. PEC. IOTA. `route-domain-event`.
