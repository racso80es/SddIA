---
feature_name: kaizen-capsula-imap-triaje
created: "2026-08-19"
process: feature
phases: "T1-guante,T2-identidad-triaged,T3-fanout-telegram,T4-evento-retorno,T5-ingest,T6-wui,T7-aduana"
uuid: "9c25bb52-57a4-4ede-be43-41388a7576b2"
persist_ref: docs/features/kaizen-capsula-imap-triaje
branch_name: feat/kaizen-capsula-imap-triaje
dedalo_verdict: ok
---

# Plan — kaizen-capsula-imap-triaje

## T1 · Guante IMAP

- Aislar error por UID en `poll_once` (MIME/fetch/eml/emit no abortan el lote).
- Loop: `poll_once` Err → log stderr; heartbeat sigue.
- `--once`: stdout envelope JSON-io; `exitCode` 0 iff `success`.
- Tests crate `email-watcher`.
- `delegates_to` implícito: edición cápsula daemon (no clase EM).

## T2 · Identidad en veredicto

- Quirúrgico `email-triaged.md` v1.1.0 + fila índice. UUID preservado.
- `emit_triaged` copia `from`/`subject` del `Email_Received`.
- Test handler: payload OPTIONAL presente; `snippet` ausente.

## T3 · Fan-out Telegram

- `build_telegram_message_from_event`: rama `Email_Triaged`; `None` si verdict ≠ `actionable`.
- Suscripción SSOT: `agent:argos`, `tool:send-telegram-notification`.
- Test: noise → None; actionable → texto con from/subject/uid.

## T4 · Clase retorno

- `./sddia-run.sh --process entity-manager` create `event` `email-quick-action-requested`.
- Familia `domain`. Emisor `kalma2-bridge`.

## T5 · Ingest retorno

- `entity-manager` create `process` `email-quick-action-ingest`, `process_jurisdiction: domain`, `process_domain_root`: packing kalma2-assistant.
- Handler nativo + wiring `execute-process` `mod.rs`.
- Proof `{eda_instance.proofs}/email-quick-action/{event_id}.json`.
- Suscripción `Email_Quick_Action_Requested` → este proceso.

## T6 · WUI Kalma2

- Bridge: `GET /api/email-inbox`, `POST /api/email-quick-action` (escritura fractal domain vía Cúmulo).
- UI: panel inbox + botones Archivar / Generar borrador / Delegar.
- Tests bridge (list filter + POST envelope).

## T7 · Aduana documental

- `implementation.md` / `execution.md` / `validacion.md`.
- PBI → `docs/todos/done/` + `pbi_archived: true` en el mismo PR.
- Evolution bajo `directories.evolution`.

## Orden y dependencias

```
T1 ∥ T2
T2 → T3
T4 → T5 → T6
T1–T6 → T7
```

## Riesgo conocido

`entity-manager` update de eventos regenera UUID. T2 no usa update. T4/T5 son create.
