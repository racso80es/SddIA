---
uuid: "a23dda8f-b5d5-4091-a21c-f408159d3a3e"
name: "telegram-gateway"
version: "1.0.0"
contract: "tools-contract v1.3.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "ecosystem-evolution"
capabilities:
  - "telegram-gateway"
  - "channel-ingest"
  - "capsule-json-io"
provides:
  - id: "channel:ingest"
    contract: "channel.ingest"
    version: "1.0.0"
hash_signature: "sha256:e72d4f34012f875ea0c231950aec59fe60e3cf268568e8ebe973c1a72c8a5ad9"
implementation_path_ref: "SddIA/tools/telegram-gateway"
outputs:
  - "success": "boolean"
  - "emitted": "boolean"
  - "event": "object; ECST domain"
  - "event_type": "string"
  - "error": "string; diagnóstico si aplica"
---

# telegram-gateway

Tool de aduana aferente Telegram (PBI-045 H11-D · laudo Racso). Transmuta `text` en evento domain (`Kaizen_Idea_Captured` / `Manual_Task_Requested`).

Proveedor canónico de `channel:ingest`. Implementación Rust bajo `implementation_path_ref`.
