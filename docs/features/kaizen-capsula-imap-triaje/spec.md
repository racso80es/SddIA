---
feature_name: kaizen-capsula-imap-triaje
created: "2026-08-19"
process: feature
base: main
scope: kaizen-capsula-imap-triaje
version_spec: "1.0.0"
document_id: PBI-KAIZEN-CAPSULA-IMAP-TRIAJE
uuid: "9c25bb52-57a4-4ede-be43-41388a7576b2"
persist_ref: docs/features/kaizen-capsula-imap-triaje
branch_name: feat/kaizen-capsula-imap-triaje
execution_id: "14fff213-bcee-4c26-ad17-53e5e585979b"
dedalo_verdict: ok
laudos:
  - L-NO-NEW-DETECT
  - L-OPTIONAL-IDENTITY
  - L-TELEGRAM-FILTER
  - L-WUI-PROJECTION
  - L-RETURN-EVENT
  - L-GLOVE
---

# Especificación — kaizen-capsula-imap-triaje

## 1. Decisiones Dedalo (handoff Mayeuta D1–D6)

| ID | Decisión | Rationale |
|----|----------|-----------|
| **L-NO-NEW-DETECT** | Cero clase `Actionable_Email_Detected` | Dualidad con `Email_Triaged.verdict=actionable`. Brecha = fan-out, no ontología |
| **L-OPTIONAL-IDENTITY** | `email-triaged` v1.1.0: OPTIONAL `from`, `subject`. UUID `6a4b0e9a-…` **inmutable** | Identidad táctica sin violar FORBIDDEN `body`/`snippet`. `event-creator` update regenera UUID (precedente `tekton-fire-and-forget`); mutación quirúrgica in-ciclo |
| **L-TELEGRAM-FILTER** | Suscriptor `argos` / `tool:send-telegram-notification` en `Email_Triaged` | Misma forma que PEC. Dispatcher: mensaje solo si `verdict=actionable`; resto → `skipped-empty-message` (AC-B1) |
| **L-WUI-PROJECTION** | `GET /api/email-inbox` lee proofs `email-triaged`; botones en Kalma2 | No sustituye `GET /api/status`. Umbral interactivo ≠ veredicto terminal (PBI-044) |
| **L-RETURN-EVENT** | Clase nueva `Email_Quick_Action_Requested` (CREATE `entity-manager`) | Intención humana ≠ detección. Payload `message_uid` + `action` ∈ `{archive,draft,delegate}`. Emisor: `kalma2-bridge` |
| **L-RETURN-CONSUMER** | Proceso `email-quick-action-ingest` (packing `codex-kalma2-assistant`) | Persiste proof de intención. **No** IMAP STORE. `draft` no SMTP |
| **L-GLOVE** | Centinela: aislamiento por UID; `--once` envelope `capsule-json-io`; loop no aborta | Daemon ≠ skill; JSON-io aplica al modo one-shot. Ceguera y RO intactos |

## 2. Circuito

```
IMAP RO → email-watcher --(Email_Received)→ email-triage-gateway
                                         → Email_Triaged (+ from, subject)
                                              │
                                              ├─ noise/passive: constancia + silencio humano
                                              └─ actionable:
                                                   ├─ send-telegram-notification (poke)
                                                   └─ proofs → GET /api/email-inbox → WUI botones
                                                        └─ POST /api/email-quick-action
                                                             → Email_Quick_Action_Requested
                                                             → email-quick-action-ingest (proof)
```

## 3. Mutaciones

| ID | Artefacto | Vía | Cambio |
|----|-----------|-----|--------|
| M1 | `SddIA/daemons/email-watcher/src/main.rs` | in-ciclo (cápsula; no clase daemon en EM) | Aislamiento UID; envelope `--once` |
| M2 | `SddIA/events/domain/email-triaged.md` + índice | quirúrgico UUID-preserve | v1.1.0 OPTIONAL `from`,`subject` |
| M3 | `engine/handlers/email_triage.rs` | in-ciclo | Copiar `from`/`subject` al veredicto |
| M4 | `engine/route_domain_core.rs` | in-ciclo | Mensaje Telegram filtrado |
| M5 | `SddIA/core/event-domain-subscriptions.json` | SSOT in-ciclo (01A) | `Email_Triaged` → telegram; `Email_Quick_Action_Requested` → ingest |
| M6 | `email-quick-action-requested` | `entity-manager` create event | Clase retorno |
| M7 | `email-quick-action-ingest` | `entity-manager` create process (jurisdiction domain, root kalma2) | Consumidor retorno |
| M8 | `engine/handlers/email_quick_action.rs` + `mod.rs` | in-ciclo | Handler nativo |
| M9 | `kalma2-bridge` + `interfaces/kalma2/` | in-ciclo | Inbox + acciones rápidas |

## 4. Contratos de payload

### 4.1 `Email_Triaged` v1.1.0

REQUIRED inalterado. OPTIONAL += `from`, `subject`. FORBIDDEN inalterado (`body`, `snippet`).

### 4.2 `Email_Quick_Action_Requested`

REQUIRED: `message_uid`, `action` (`archive` \| `draft` \| `delegate`).
OPTIONAL: `source_event_id`, `channel` (`kalma2` \| `telegram`).
FORBIDDEN: `body`, `snippet`, `credentials`.

### 4.3 Envelope `--once` (watcher)

Una línea stdout, `capsule-json-io` v2.0: `meta.schemaVersion=2.0`, `meta.entityKind=tool`, `meta.entityId=email-watcher`, `success`, `exitCode`, `message`. Loop daemon: sin JSON en stdout.

### 4.4 Telegram (solo actionable)

```
Correo accionable
from={from}
subject={subject}
uid={message_uid}
```

Sin cuerpo. Sin botones Telegram (tool ciega; umbral de clics = WUI).

### 4.5 `GET /api/email-inbox`

JSON `{success, items:[{event_id, message_uid, from, subject, verdict, timestamp, agenda_entry_id}]}`. Solo `verdict=actionable`. Cap 20, timestamp desc.

### 4.6 `POST /api/email-quick-action`

Body `{message_uid, action, source_event_id?}`. Escribe ECST en `eda_fractal.domain`. 202 + `event_id`.

## 5. Invariantes (no relajar)

- G4: watcher sin `execute-process`, sin IMAP write, sin matriz, sin ruta absoluta de host.
- G5: Triaje-C early-exit intacto.
- PBI-044: `GET /api/status` permanece.
- Matriz: no alojar ley en el centinela; no mutar buzón como efecto del veredicto.
- `telegram-watcher` intocado.

## 6. Fuera

Grafo de hábitos; SMTP; IMAP STORE; clase de detección nueva; reabrir 01A.
