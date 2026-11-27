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
hash_signature: "sha256:32978484ed1a25670ed6f583d25062cbbdc0c155539db5ea18fba2308df26298"
inputs:
  - "prompt": "texto de inferencia"
  - "model": "opcional; override"
  - "effort": "high|medium|low"
outputs:
  - "success": "boolean"
  - "error_code": "rate_limited|timeout|upstream_unavailable|auth|malformed_response|network|unknown"
---

# Skill: antigravity-cli-executor

Transductor nativo de agy en print/headless. Request `llm.infer` + legacy (`parameters`). Argv --output-format json; prompt en -p. Binario SDDIA_AGY_PATH o PATH. Default --sandbox; skip-permissions solo doble opt-in. Lab mock/stub. Sobre capsule-json-io 2.0. Sin provides llm:interact. Provides llm:infer.

## Propósito

Transductor nativo de agy en print/headless. Request `llm.infer` + legacy (`parameters`). Argv --output-format json; prompt en -p. Binario SDDIA_AGY_PATH o PATH. Default --sandbox; skip-permissions solo doble opt-in. Lab mock/stub. Sobre capsule-json-io 2.0. Sin provides llm:interact. Provides llm:infer.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
