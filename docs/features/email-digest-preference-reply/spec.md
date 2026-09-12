---
feature_name: email-digest-preference-reply
created: "2026-09-12"
process: feature
base: main
scope: email-digest-preference-reply
version_spec: "1.0.0"
document_id: PBI-EMAIL-DIGEST-PREFERENCE-REPLY
uuid: "bba79183-b76b-4e05-aa02-5adf1d09b500"
persist_ref: docs/features/email-digest-preference-reply
branch_name: feat/email-digest-preference-reply
execution_id: "b6930240-b8a7-440a-ab0d-f9f853b1855a"
dedalo_verdict: ok
laudos:
  - L-CHANNEL
  - L-TOKEN
  - L-KEYS
  - L-IGN
  - L-EMIT
  - L-STATE
  - L-GATE
  - L-EDA
  - L-FORGE
  - L-EXEMPT
  - L-NUCLEO
  - L-IMAP
  - L-CI
---

# Especificación — email-digest-preference-reply

## 1. Decisiones Dédalo

Ver `clarify.md`. Contrato operativo condensado abajo.

## 2. Circuito

```
email-noise-digest
  → tokens 8/12 hex + reply_markup por K líneas
  → send-telegram-notification {message, parse_mode:null, reply_markup}
telegram-watcher (sin cambio)
  → telegram-gateway XOR callback
  → TelegramCallback_Received
route-domain-event
  → email-digest-preference-reply {event_file_path}
    gate dpref: → correlaciona token → emit activate | skip
user-preference-ingest
  → Active + ExplicitUser
Email_Received posterior
  → P-EXEMPT-C (max) | P-MUTE-SENDER (mute)
```

## 3. `callback_data`

`dpref:<max|mute|ign>:<token>` ASCII. Bytes = `str.len()`. Teclado solo emite `max`/`mute`.

Token: primeros 8 hex de `canonical_subject_key_from_addr`. Si dos filas del lote comparten prefijo, usar 12 hex para ambas colisionadas (mapa 1:1).

## 4. Estado `tokens`

Path existente del digest. Skip / empty-window: clonar `tokens` previos al reescribir cursor. Notify: mapa = K listados.

Valor: `{subject_key, rule, created_at}`. Cero addr.

## 5. Markup y texto

Líneas `N. {addr} ({count}, {rule}) {subject}`. Pie: `⭐ N = priorizar (max) · 🔇 N = silenciar`.

`inline_keyboard[i] = [{text:"⭐ i", callback_data:"dpref:max:<tok>"}, {text:"🔇 i", callback_data:"dpref:mute:<tok>"}]`.

Cápsula ya acepta `reply_markup.inline_keyboard`. Extender `default_notify` / mock de tests.

## 6. Proceso réplica

Handler `email_digest_preference_reply.rs`. Input EDA: leer evento; `payload.callback_data`.

| Caso | Envelope |
|------|----------|
| no `dpref:` | success, skipped, reason=`foreign-namespace`, preference_emitted=false |
| parse inválido | failed / skipped-invalid, cero emit |
| token miss | skipped-expired, cero emit |
| `ign` | skipped, reason=`ignore`, cero emit |
| `max`/`mute` | emit OK → preference_emitted=true, target_event_id |

Emit payload (además de `operation`/`channel` que rellena la acción):

```json
{
  "subject_kind": "person",
  "subject_key": "<64 hex>",
  "predicate": "priority | mute",
  "priority_level": "max",
  "value": {"muted": true},
  "scope_type": "channel",
  "scope_id": "email",
  "source_event_id": "<TelegramCallback_Received.event_id>"
}
```

`priority_level` solo en max. `value` solo en mute.

## 7. Empaque genoma

EM create `process` / `email-digest-preference-reply`:

- `process_jurisdiction: domain`
- `process_domain_root: SddIA/library/codexes/codex-kalma2-assistant/process`
- `process_contract_version: 1.4.0`
- fases: `Gate-Callback`, `Correlacion-Token`, `Emision-Preferencia` (`delegates_to: action:emit-user-preference-change-requested`)
- `workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/`

EM update códice `codex-kalma2-assistant`: membership += nombre.

EM update process `email-noise-digest`: fase notificación menciona `reply_markup`.

Core (no EM): `email_digest_preference_reply.rs`, `email_noise_digest.rs`, `handlers/mod.rs`, `engine/mod.rs`, `route_domain_core.rs`, `event-domain-subscriptions.json`.

## 8. Fuera de especificación

Watcher, clases ECST nuevas, NL/hábitos Kalma2, cron, IMAP, acuse táctico como CA, membership de quick-action.
