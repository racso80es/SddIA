---
feature_name: eda-telegram-notify-pr-merged
created: "2026-09-05"
process: feature
base: main
scope: core
document_id: PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
execution_id: "fccb9d32-8996-4594-8293-71c27926a017"
---

# Spec — eda-telegram-notify-pr-merged

## Contrato de fan-out

`route-domain-event` carga `eda_bus.subscriptions` → `SddIA/core/event-domain-subscriptions.json`. Si el suscriptor declara `tool: send-telegram-notification`, despacha `build_telegram_message_from_event` → `invoke_send_telegram_notification`. `None` → `skipped-empty-message`. Fallo de la tool no altera el testigo DLT (fan-out independiente).

## Mensaje `PullRequest_Merged`

```
✅ PR Fusionado — {source_branch}
━━━━━━━━━━━━━━━━━━━━━━━━
📦 Commit: {hash7} ({target_branch})
👤 Integrador: {author}
🔐 Auditor: {auditor} · {policy}
[{pr_url}]
[🔗 Correlación: {cid8}…]
```

| Campo | Fuente | Regla |
| :--- | :--- | :--- |
| `source_branch` | payload | `"?"` si ausente |
| `target_branch` | payload | fallback `"main"`; no hardcodear si el payload lo trae |
| `merge_commit_hash` | payload | `&h[..7.min(h.len())]` |
| `author` | payload | `"?"` si ausente |
| `security_clearance` | payload REQUIRED | línea omitida solo si el objeto falta (malformado) |
| `pr_url` | payload OPTIONAL | línea si trim no vacío |
| `correlation_id` | envelope | línea si `len >= 8` |
| `traceability_*` | extra-contractual | no renderizar |
| `timestamp` | envelope | no renderizar |

## Suscripciones JSON

Añadir bajo `PullRequest_Merged` (después de IOTA, sin tocarlo):

```json
{
  "agent": "argos",
  "tool": "send-telegram-notification",
  "intent": "Resumen ejecutivo post-merge al Vértice Biológico: branch, commit y auditor."
}
```

Ficheros: `event-domain-subscriptions.json` (SSOT) y `event-subscriptions.json` (paridad).

## Clase ECST

`entity-manager` `entity_class: event` `lifecycle_operation: update` `hash` vía `markdown_body_replacements`:

`from`: `Ver \`SddIA/core/event-subscriptions.json\` → clave \`PullRequest_Merged\`.`

`to`: tabla IOTA + Telegram + línea SSOT domain.

UUID `cfb8ce66-784e-4826-8a0a-a20c671e3a60` inmutable. Versión 1.0.0 (schema payload intacto). `hash_signature` y `eda-coverage` last_hash los sella el gestor.

## Tests

`build_telegram_message_from_event`:

1. Instancia well-formed sin `pr_url` → encabezado, hash 7, target, autor, auditor, correlación; sin URL; sin `traceability`.
2. Con `pr_url` → URL en texto.
3. Con `traceability_anomaly` en payload → texto no contiene `merge_huérfano` ni la nota.

## Fuera de spec

Mutar `send-telegram-notification`. Inyectar `pr_url` en `accept-pr`. Versionar Clase a 1.1.0.
