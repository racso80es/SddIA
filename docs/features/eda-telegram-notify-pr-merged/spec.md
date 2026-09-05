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

## Fuera de spec (ola 1)

Mutar `send-telegram-notification`. Inyectar `pr_url` en `accept-pr`. Versionar Clase a 1.1.0.

---

# Spec ola 2 — síntesis humanizada

PBI v1.2.0. El fan-out `tool: send-telegram-notification` de `PullRequest_Merged` **se sustituye** (no se suma) por `action: notify-humanized-pr-merged`.

## Contrato de fan-out ola 2

```json
{
  "agent": "argos",
  "action": "notify-humanized-pr-merged",
  "intent": "Resumen ejecutivo post-merge: metadatos estáticos + síntesis de valor (fail-soft LLM)."
}
```

`dispatch_subscriber`: si `action == "notify-humanized-pr-merged"` → `notify_humanized_pr_merged::run_from_event(repo, event)` **antes** del `try_run_native` payload-only.

## Handler

1. `static_msg = build_telegram_message_from_event(event)` (función crate-visible). `None` → `skipped-empty-message` (paridad tool).
2. Invocar `gemini-http-infer` con `request.prompt` = bloque O2-DD-3 + hechos ECST (sin diffs). `temperature` ≤ 0.2. Timeout: respetar tool (default 30s); no retry.
3. Si inferencia `success` y `result.text` trim no vacío: truncar a 2 líneas / 400 chars; `msg = static + "\n\n🧠 Síntesis de Valor: " + text`.
4. Si inferencia falla (exit≠0, HTTP, timeout, vacío, key ausente fuera de lab-mock): `msg = static`.
5. `invoke_send_telegram_notification(repo, msg)`. Fallo Telegram → testigo failed (ola 1). Gemini no marca failed.

## Prompt base (contrato action)

```text
[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. MAX 2 LINES]
Return only business value of this merge. Do not restate hash, auditor, branch, or correlation.
Do not invent files, commits, or intent absent from CONTEXT.
CONTEXT:
source_branch=…
target_branch=…
merge_commit_hash=…
author=…
auditor=…
policy=…
pr_url=…   (omit line if empty)
repository_name=… (omit if empty)
```

Este bloque es **prompt Gemini**, no el prefijo creator de `external-ai-constraints`.

## Tests ola 2

1. CA2 lab-mock: prompt contiene `MAX 2 LINES` + `source_branch` del fixture; se invoca la tool.
2. CA4: infer `Err` / exit 1 → mensaje enviado **sin** `Síntesis de Valor`; status success.
3. CA3: estático ignora `traceability_*`; correlación desde envelope.
4. CA7: un invoke Telegram; JSON sin `tool: send-telegram-notification` bajo `PullRequest_Merged`.
5. CA5: IOTA subscriber intacto.

## Fuera de spec ola 2

Entrenar LLM; retry API; payload ECST 1.1.0 con diffs; ampliar `git-manager`; `diff_name_only` en prompt; `accept-pr`; init feature desde `main`.
