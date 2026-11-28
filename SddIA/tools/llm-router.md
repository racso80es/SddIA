---
uuid: "3836c0da-ef32-419b-9a73-89afed4a93ac"
name: "llm-router"
version: "1.0.0"
contract: "tools-contract v1.2.0"
domain_origin: "SddIA"
context: "system-operations"
capabilities:
  - "llm_router"
hash_signature: "sha256:9571c39ff6b3c88c3a5a1038bd8b668c96b037a516b9dc3ac695d5301d2a4670"
implementation_path_ref: "SddIA/tools/llm-router"
---

# llm-router

## Propósito

Enrutamiento de inferencia LLM entre adapters declarados en runtime local.

## Operación

Selecciona `adapter_ref`; fail-over por afinidad documentada; consume `llm:infer`.
