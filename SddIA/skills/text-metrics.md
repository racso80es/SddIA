---
uuid: "0c84d99b-aa67-4b27-abb6-7133867c5102"
name: "text-metrics"
version: "1.0.0"
contract: "skills-contract v1.2.0"
context: "ecosystem-evolution"
telemetry_provided: true
telemetry_schema:
  - prompt_tokens
  - completion_tokens
capabilities:
  - "text-metrics"
hash_signature: "sha256:35655c727052107076e200512466d12ca1de115ba01a07d9b0e82fa25280aa1e"
inputs:
  - "text_content": "string; texto de entrada UTF-8"
outputs:
  - "word_count": "integer; número de palabras"
  - "char_count": "integer; número de caracteres"
---

# Skill: text-metrics

Calcula métricas básicas de un texto (número de palabras y caracteres).

## Ejecución (LLM-native)

Recibir `text_content` y devolver `word_count` y `char_count` en envelope JSON del contrato de skills.
