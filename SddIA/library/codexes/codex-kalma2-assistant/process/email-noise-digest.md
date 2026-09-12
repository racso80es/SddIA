---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:c8ed5433e9c7d800857278ab5bd51e144629ac3d519eacd88c503001ce167af2
inputs:
- since: RFC3339 inclusivo
- until: RFC3339 exclusivo
name: email-noise-digest
outputs:
- events_scanned: matches del filtro
- senders: remitentes unicos
- notified: true si poke
- skipped: true si cursor
phases:
- intent: Escanear proofs, filtrar conjunto cerrado C, normalizar y agregar.
  name: Agregacion-Cuarentena
- delegates_to:
  - tool:send-telegram-notification
  intent: Poke Telegram parse_mode null + reply_markup por remitente listado; cursor y tokens solo en exito.
  name: Notificacion-Digest
uuid: fc11c0d6-09ba-48e6-972c-561847f8c8ef
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# email-noise-digest

Digest batch determinista de ruido Triaje-C desde proofs email-triaged. Cero LLM. Cero IMAP.
