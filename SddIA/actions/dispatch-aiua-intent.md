---
uuid: "a1086194-e19c-49b1-88e1-bd828211b3a8"
name: "dispatch-aiua-intent"
version: "1.1.0"
contract: "actions-contract v1.3.0"
context: "ecosystem-evolution"
capabilities:
  - "dispatch_aiua_intent"
inputs:
  - "function_call": "objeto {name, args}"
  - "correlation_id": "UUID v4 opcional"
outputs:
  - "success": "boolean"
  - "event_id": "UUID v4"
  - "target_path": "ruta relativa eda_fractal.domain"
  - "event_type": "Aiua_Process_Requested | Suite_Execution_Requested | User_Preference_Change_Requested"
  - "intent_dispatched": "nombre del tendon"
hash_signature: "sha256:9b717e070717cfbe31b580fcd1509fbfdf80548fcbdae1d08710674ee1ee6015"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: dispatch-aiua-intent

Traduce un tendón Aiúa (`function_call.name` + `args`) a ECST en `eda_fractal.domain`.

## Destinos

| Tendón | Destino |
|--------|---------|
| `ordenar_refactorizacion` / `iniciar_feature` / `iniciar_bug_fix` | `Aiua_Process_Requested` (`emitter_agent: aiua-stimulus-processing`) |
| `requerir_auditoria` | `Suite_Execution_Requested` vía `emit-suite-execution-requested` |
| `delegar_habito` | `User_Preference_Change_Requested` vía `emit-user-preference-change-requested` (`channel: kalma2`) |
| `solicitar_clarificacion` | No motor; el despacho aborta |

## Hábito

`delegar_habito` aplica matriz no-destructiva (borrar/eliminar/limpiar/delete/expunge → `predicate: mute`, `value.muted: true`). Prohibido copiar `raw_utterance` al payload ECST. Handler nativo en `execute-process` (`aiua_intent.rs`).

## Límites

Cero terminal. Cero IMAP. Cero join a `user-preference-ingest`. DA-5: éxito = ECST sellado.
