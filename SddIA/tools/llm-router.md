---
uuid: "3836c0da-ef32-419b-9a73-89afed4a93ac"
name: "llm-router"
version: "1.0.0"
contract: "tools-contract v1.2.0"
domain_origin: "SddIA"
context: "system-operations"
capabilities:
  - "llm_router"
hash_signature: "sha256:902fcf38120b8fd2db67bf04e537ac30381196721ce929b910b4eda194f74782"
implementation_path_ref: "SddIA/tools/llm-router"
---

# llm-router

Router de oráculos de instancia: selecciona adapter_ref por oracle_id/affinity y salta ante rate_limited/timeout/upstream_unavailable/network. Consume llm:infer; no lo provides.

## Propósito

Router de oráculos de instancia: selecciona adapter_ref por oracle_id/affinity y salta ante rate_limited/timeout/upstream_unavailable/network. Consume llm:infer; no lo provides.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
