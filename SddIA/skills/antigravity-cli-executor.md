---
uuid: "d8b07e6f-1cc0-4b6f-a789-02ade10471f5"
name: "antigravity-cli-executor"
version: "1.1.0"
contract: "skills-contract v1.4.0"
context: "system-operations"
capabilities:
  - "antigravity_cli_executor"
  - "llm:infer"
provides:
  - id: "llm:infer"
    contract: "llm.infer"
    version: "1.0.0"
hash_signature: "sha256:f364da3d3223232c5c4a775f62795b59c3ccc07ad8e466d329d9e132a159ee89"
inputs:
  - "prompt": "texto de inferencia"
  - "model": "opcional; override"
  - "effort": "high|medium|low"
outputs:
  - "success": "boolean"
  - "error_code": "rate_limited|timeout|upstream_unavailable|auth|malformed_response|network|unknown"
---

# Skill: antigravity-cli-executor

## Propósito

Transductor CLI agy (headless) para capability `llm:infer`; envelope capsule-json-io v2.

## Operación

Argv `--output-format json`, `-p` prompt; sandbox por defecto; crate nativo bajo skills.
