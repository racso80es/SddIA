---
capabilities:
- telegram-gateway
- channel-ingest
- capsule-json-io
context: ecosystem-evolution
contract: tools-contract v1.3.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA
hash_signature: sha256:b396955f07a35637a65f4a1a40438f6520259c8325a5430c570bb497fc42ec5c
implementation_path_ref: SddIA/tools/telegram-gateway
name: telegram-gateway
outputs:
- success: boolean
- emitted: boolean
- event: object; ECST domain
- event_type: string
- error: string; diagnóstico si aplica
provides:
- contract: channel.ingest
  id: channel:ingest
  version: 1.0.0
source_sha256: sha256:9f5cbfa4b625e4ddb34fd9158c4f639435dc39536c2a6c383dd47bd8ea9ca56c
uuid: a23dda8f-b5d5-4091-a21c-f408159d3a3e
version: 1.0.0
---

# telegram-gateway

Tool de aduana aferente Telegram. XOR: `text` → `Kaizen_Idea_Captured` / `Manual_Task_Requested`; `callback_data` → `TelegramCallback_Received`.

Proveedor canónico de `channel:ingest`. Implementación Rust bajo `implementation_path_ref`.
