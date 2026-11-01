---
feature_name: telegram-route-ecst-gate
created: "2026-11-01"
process: bug-fix
base: SddIA/engine/execute-process/tests/eda_subscriber_ecst_redb.rs
scope: "Catálogo ECST y fixtures de los tests route_domain de Telegram"
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/telegram-route-ecst-gate
tracker_ref: AUT-60
historia_ref: AUT-20
---

# Spec — AUT-60

## Problema

`temp_repo_with_bus_skeleton` no copia `SddIA/events/`. `load_event_class_schemas` resuelve contra ese repo. `TelegramMessage_Received` no está catalogado y `route_domain_event` devuelve `delivery_status.ecst-gate = failed` antes del fan-out.

El payload `{ "text": "   " }` tampoco cumpliría la clase: faltan `chat_id` y `source`. `"   "` no es un campo ausente.

`route_domain_telegram_responder_receives_ecst_path` no distingue ese corte de un despacho real.

## Contrato

El esqueleto copia `SddIA/events/` del checkout (`find_repo_root`) al repo temporal, de modo que `load_event_class_schemas` encuentre `TelegramMessage_Received`.

Fixtures:

| Test | `text` | `chat_id` | `source` |
|------|--------|-----------|----------|
| `route_domain_telegram_empty_text_still_skipped` | `"   "` | presente, no vacío | presente, no vacío |
| `route_domain_telegram_responder_receives_ecst_path` | texto no vacío tras trim | presente, no vacío | presente, no vacío |

Aserciones:

- Vacío: alguna entrada de `delivery_status` es `skipped-empty-text`. Ninguna es el fallo de aduana `ecst-gate` / `failed` como único resultado. El error de la respuesta no contiene `event_type not cataloged` ni `missing required payload`.
- Texto presente: `delivery_status` no es el mapa de un solo `ecst-gate: failed`. Sigue prohibido `skipped-empty-text` y `process sin despacho ECST`.

Las marcas `enter` / `leave` de AUT-59 se conservan.

## Fuera de alcance

Cambiar `validate_ecst_instance`, exigir trim en el esquema, alinear payloads de markdown, Kalma2 o telemetría, logs de producción.
