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
hash_signature: "sha256:8db2eca9f1814fa3c3cc965e411fa15690f26bd99c14473b404eb42ae5a8c431"
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

## Propósito

Calcula métricas básicas de un texto (número de palabras y caracteres).

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
