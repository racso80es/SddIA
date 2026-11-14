---
uuid: "3836c0da-ef32-419b-9a73-89afed4a93ac"
name: "llm-router"
version: "1.0.0"
contract: "tools-contract v1.2.0"
domain_origin: "SddIA"
context: "system-operations"
capabilities:
  - "llm_router"
hash_signature: "sha256:f4627c7316153eb6741b5838e507d935b5339867c01228763be08a6fa9a6d7a9"
implementation_path_ref: "SddIA/tools/llm-router"
---

# llm-router

Router de oráculos de instancia: selecciona adapter_ref por oracle_id/affinity y salta ante rate_limited/timeout/upstream_unavailable/network. Consume llm:infer; no lo provides.
