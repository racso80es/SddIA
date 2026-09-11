---
feature_name: email-noise-heuristic-digest
created: "2026-09-11"
process: feature
base: main
scope: email-noise-heuristic-digest
version_spec: "1.0.0"
document_id: PBI-EMAIL-NOISE-HEURISTIC-DIGEST
uuid: "bd5eab3f-0693-408a-ab04-de8b43a36c54"
persist_ref: docs/features/email-noise-heuristic-digest
branch_name: feat/email-noise-heuristic-digest
execution_id: "8fa709ed-6c4a-4b71-bac1-a2b69ecb0bf3"
dedalo_verdict: ok
laudos:
  - L-BATCH
  - L-HANDLER
  - L-FILTER
  - L-NORM
  - L-CURSOR
  - L-TG
  - L-FORGE
  - L-METRIC
  - L-IMAP
  - L-LLM
  - L-CHILD
  - L-CI
---

# Especificación — email-noise-heuristic-digest

## 1. Decisiones Dédalo

| ID | Decisión | Rationale |
|----|----------|-----------|
| **L-BATCH** | CLI `execute-process --process email-noise-digest`. Sin suscripción. | No es poke por `Email_Triaged`. Timer = instancia. |
| **L-HANDLER** | Nuevo módulo nativo. Reusar `proofs_root`. | Precedente `email_triage.rs`. Path vía `eda_instance.proofs`. |
| **L-FILTER** | Conjunto cerrado de tres reglas C + noise + deterministic. | Matriz SSOT; glob `C-*` adelantaría reglas futuras. |
| **L-NORM** | `normalize_email_addr` sin RFC 2047. | Decode ya en persistencia del proof. Función crate no decodifica. |
| **L-CURSOR** | `until` frontera + clamp `since`. | Evita recontar solapes. Skip barato. |
| **L-TG** | `parse_mode: null` explícito. | Contrato `.md` ≠ cápsula; el null es el contrato del digest. |
| **L-FORGE** | EM create con root kalma2 + contrato 1.4.0. | Default creator `[0]` = software-engineering. EM default contrato `1.3.0`. |
| **L-METRIC** | `events_scanned` = matches. | Alinea CA-4 con outputs. |
| **L-IMAP** | Cero mutación de buzón. | Matriz; D2. |
| **L-LLM** | Cero inferencia. | D2 cuarentena asíncrona. |
| **L-CHILD** | Copy de réplica inerte. | `blocks_on` el PBI hijo. |
| **L-CI** | APTO solo con run verde. | `features-documentation-pattern` v1.2.1. |

## 2. Circuito

```
CLI/timer instancia
  → execute-process --process email-noise-digest
       { since, until } RFC3339; since < until
  → cursor {daemons_instance.state}/email-noise-digest.json
       until <= last_until → skip
       since < last_until < until → effective_since = last_until
  → {eda_instance.proofs}/email-triaged/*.json
  → filtro §3 → aggregate §4
  → 0 matches → persist cursor, notified=false
  → N matches → send-telegram-notification {message, parse_mode: null}
       fail → abort, cursor intacto
       ok → persist cursor last_until=until
```

Handler: `canonical == "email-noise-digest"` en `engine/mod.rs`.

## 3. Proof y filtro

Envelope (escrito por `persist_email_triaged_proof`):

```json
{
  "kind": "email-triaged-proof",
  "event_id": "<uuid>",
  "event_type": "Email_Triaged",
  "timestamp": "2026-09-06T12:00:00Z",
  "payload": { "verdict": "noise", "decision_path": "deterministic", "matched_rule": "C-NOREPLY", "from": "Shop <noreply@shop.tld>", "subject": "…" }
}
```

`timestamp` raíz, no payload. `from`/`subject` ya pasan por `decode_rfc2047` en el gateway.

Inclusión: `kind` + `event_type` + `ts ∈ [effective_since, until)` + payload § L-FILTER.

JSON roto / timestamp inválido → skip de ese fichero, no abort del lote.

## 4. Agregación

```
key = normalize_email_addr(from) || "_unknown"
count += 1
matched_rule = argmax freq; tie → lex
subject = subject of max(timestamp); tie → min(event_id)
sort: count DESC, key ASC
```

`normalize_email_addr`: trim; si hay `<…>` extrae interior; `to_ascii_lowercase`. No RFC 2047.

## 5. Mensaje Telegram

```text
Ruido Triaje-C {YYYY-MM-DD de since UTC}
eventos={events_scanned} remitentes={senders}
- {key} ({count}, {rule}) {subject≤80}
¿Inyectar priority:max? Respuesta en ciclo aparte.
```

Construcción ≤ 4000. Overflow: header + footer + `+ {M} remitentes omitidos`; caben K líneas del ranking.

Cápsula: `invoke_capsule_json(repo, "send-telegram-notification", {message, parse_mode: null}, false)`.

## 6. Cursor

Path: `{daemons_instance.state}/email-noise-digest.json` (`.gitignore`).

Campos: `last_until`, `last_run`, `last_events_scanned`, `last_senders_count`, `last_notified`.

Skip envelope: `success=true`, `skipped=true`, `reason=already-processed`, contadores 0, `notified=false`.

## 7. Empaque genoma

`entity-manager` create `process` / `email-noise-digest`:

- `process_jurisdiction: domain`
- `process_domain_root: SddIA/library/codexes/codex-kalma2-assistant/process`
- `process_contract_version: 1.4.0`
- `process_context: ecosystem-evolution`
- fases: `Agregacion-Cuarentena`, `Notificacion-Digest` (`delegates_to: tool:send-telegram-notification`)
- `workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/`

Después EM update códice `codex-kalma2-assistant`: `process_membership` += nombre; cuerpo nombra el digest.

Coverage: sello `emit-domain-mutation` del EM. Prohibido `Write` sobre `directories.process_domain_roots` / `library_codexes`.

Core (no EM): `handlers/mod.rs`, `engine/mod.rs`, `email_noise_digest.rs`, tests.

## 8. Fuera de especificación

Timer systemd, réplica, botonera, daemon, evento nuevo, membership de `email-quick-action-ingest`, alineación contrato/cápsula Telegram.
