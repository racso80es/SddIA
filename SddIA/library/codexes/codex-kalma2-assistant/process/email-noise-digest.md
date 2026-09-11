---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: "sha256:24515d9ea999fbddb282705e42588bee8fe5211847c4c81a4a113023b4961482"
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
  intent: Poke Telegram plano parse_mode null si hay matches; cursor solo en exito.
  name: Notificacion-Digest
uuid: fc11c0d6-09ba-48e6-972c-561847f8c8ef
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# email-noise-digest

Digest batch determinista de ruido Triaje-C desde proofs email-triaged. Cero LLM. Cero IMAP.
