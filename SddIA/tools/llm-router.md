---
uuid: "3836c0da-ef32-419b-9a73-89afed4a93ac"
name: "llm-router"
version: "1.0.0"
contract: "tools-contract v1.2.0"
domain_origin: "SddIA"
context: "system-operations"
capabilities:
  - "llm_router"
hash_signature: "sha256:d99a4a38ddb5de9c4c98899592e83f8dcfdb99d323720e7aaad7c7b87e3beca1"
implementation_path_ref: "SddIA/tools/llm-router"
---

# llm-router

Router de oráculos de instancia: selecciona adapter_ref por oracle_id/affinity y salta ante rate_limited/timeout/upstream_unavailable/network. Consume llm:infer; no lo provides.
