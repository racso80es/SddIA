---
uuid: "7a8da3ad-4916-4ee3-8407-aa1ecdc7ecba"
name: "gemini-http-infer"
version: "1.1.0"
contract: "tools-contract v1.2.0"
domain_origin: "SddIA"
context: "system-operations"
capabilities:
  - "gemini_http_infer"
  - "llm:infer"
provides:
  - id: "llm:infer"
    contract: "llm.infer"
    version: "1.0.0"
hash_signature: "sha256:347053b46b6cbdfae90bf4354d3c25e964e2319ad14ce1d3d08b2d0351c3c0a5"
implementation_path_ref: "SddIA/tools/gemini-http-infer"
---

# gemini-http-infer

## Propósito

Inferencia HTTP Gemini (`llm:infer`) para notificaciones y skills delegadas.

## Operación

POST `generateContent`; mocks de lab vía `SDDIA_LAB_MOCK_GEMINI_URL`. Implementación `SddIA/tools/gemini-http-infer`.
