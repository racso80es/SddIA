---
capabilities:
- telegram-gateway
- channel-ingest
- capsule-json-io
context: ecosystem-evolution
contract: tools-contract v1.3.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA
hash_signature: "sha256:ecf4538cd60a9882cdf129cfddc4fc3352bdb06291bb2f1ff29b9bcaa33bc2a8"
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
source_sha256: sha256:a87567a55d0f6a1676c4049d338220f91ba21ad0455323ce08a3f811659b19e2
uuid: a23dda8f-b5d5-4091-a21c-f408159d3a3e
version: 1.0.0
---

# telegram-gateway

## Propósito

Ingesta Telegram → eventos domain (`channel:ingest`); XOR texto vs callback.

## Operación

Implementación Rust `telegram-gateway`; emite ECST vía runtime EDA.
