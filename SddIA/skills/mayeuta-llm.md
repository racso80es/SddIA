---
uuid: "80c96e96-3e03-4af4-bed3-0af46d3fcf7f"
name: "mayeuta-llm"
version: "1.1.0"
contract: "skills-contract v1.1.0"
context: "ecosystem-evolution"
capabilities:
  - "llm-synthesize"
  - "llm-classify-intent"
  - "local-subprocess-inference"
provides:
  - id: "llm:interact"
    contract: "llm.interact"
    version: "1.0.0"
hash_signature: "sha256:91f775e7759518a4db96581202045c9563f920bb3355fa75edf020a4b39e8b49"
execution_contexts:
  - "local-subprocess"
inputs:
  - "operation": "SYNTHESIZE | CLASSIFY_INTENT"
  - "prompt": "texto del operador"
  - "schema": "(opcional) esquema JSON para CLASSIFY_INTENT"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "data": "objeto según operación"
  - "error": "string | null"
---

# Skill: mayeuta-llm

## Propósito

Transductor CLI local `llm:interact` (PBI-043 H10-A); comando `SDDIA_LLM_CLI_COMMAND`, sin red en genoma.

## Operación

SYNTHESIZE | CLASSIFY_INTENT sobre stdin JSON capsule-json-io.

## Interfaz de entrada y salida

| Operación | Salida `data` |
|-----------|---------------|
| `SYNTHESIZE` | `{ "text": "<respuesta>", "telemetry_receipt": { … } }` |
| `CLASSIFY_INTENT` | `{ "intent", …, "telemetry_receipt": { … } }` |
| `STREAM` | stdout línea a línea; receipt en `.SddIA/radamanto/inbox/` (no stdout) |

## Contexto de ejecución

- **local-subprocess:** invoca binario CLI configurado en bóveda; prompt por stdin; respuesta por stdout.
