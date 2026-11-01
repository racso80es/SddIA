---
feature_name: route-domain-process-effect
created: "2026-11-01"
process: feature
branch_name: Fix_Events_Multiple
persist_ref: docs/features/route-domain-process-effect
tracker_ref: AUT-61
historia_ref: AUT-20
---

# Spec — AUT-61

## Problema

El ensamblado ya entrega el ECST a las 15 claves. Los tests de router no comprueban el efecto. Once claves no tienen ruta. `Aiua_Process_Requested` tampoco. `override_store_mode` no cruza el `spawn` del orquestador.

## Contrato

Un caso de `route_domain_event` por fila. Repo temporal (`temp_repo_with_bus_skeleton`). El test exporta, antes del route:

- `SDDIA_LAB_ROUTE_SYNC=1`
- `SDDIA_LAB_MOCK_OUTBOUND=1`
- `SDDIA_EVENT_STORE=redb`
- `SDDIA_EXECUTE_PROCESS_BIN` = `{checkout}/SddIA/target/debug/execute-process`

Payload: campos REQUIRED de la clase en `SddIA/events/` más los que `assemble_domain_process_subscriber_inputs` lee para esa clave.

Aserción: en `data.delivery_status`, la entrada del suscriptor de proceso (`{agent}.{process}`) es `success`. Un hermano `action` o `tool` del mismo evento no entra en la aserción.

| Clave | Evento | Test |
|---|---|---|
| tracker-markdown-apply | Tracker_Issue_Changed | apretar el existente |
| memory-evolution-ingest | Domain_Entity_Telemetry_Captured | apretar el existente |
| telegram-fallback-responder | TelegramMessage_Received con texto | apretar el existente |
| task-queue-manager | Kalma2_Process_Requested | apretar el existente |
| task-queue-manager | Aiua_Process_Requested | fila nueva |
| pull-request-review | Local_QA_Requested | fila nueva |
| tracker-stamp | Work_Initiated | fila nueva |
| compile-ecosystem-map-snapshot | Domain_Entity_Created | fila nueva |
| cerbero-governance-react | Domain_Entity_Degraded | misma fila |
| fix-tool-process | Domain_Entity_Degraded | misma fila |
| execute-suite | Suite_Execution_Requested | fila nueva |
| email-triage-gateway | Email_Received | fila nueva |
| email-quick-action-ingest | Email_Quick_Action_Requested | fila nueva |
| user-preference-ingest | User_Preference_Change_Requested | fila nueva |
| email-digest-preference-reply | TelegramCallback_Received | fila nueva |
| tracker-sync-replay | Tracker_Sync_Failed | fila nueva |

`route_domain_telegram_empty_text_still_skipped` permanece en `skipped-empty-text`.

Si el fallo del handler es un path ausente en el temporal, `temp_repo_with_bus_skeleton` copia ese árbol desde el checkout. No se copia el repo entero.

`success` con mock es el efecto de laboratorio. No hay escritura remota real.

## Verificación

`cargo test -p execute-process --test eda_subscriber_ecst_redb` sobre la tabla y los cuatro tests apretados, desde `SddIA/`, `--test-threads=1`. Las 15 claves salen al menos una vez con `success`.
