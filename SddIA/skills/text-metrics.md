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
hash_signature: "sha256:0bbe1eb5bbb2ca4a6a3132e0f6fb07a0246dd6741a74f4e8ca110fbd725374fc"
inputs:
  - "text_content": "string; texto de entrada UTF-8"
outputs:
  - "word_count": "integer; número de palabras"
  - "char_count": "integer; número de caracteres"
---

# Skill: text-metrics

## Propósito

Métricas léxicas deterministas (palabras y caracteres) sobre UTF-8.

## Operación

Entrada `text_content`; salida `word_count`, `char_count` en envelope skill.

## Interfaz de entrada y salida

Recibir `text_content` y devolver `word_count` y `char_count` en envelope JSON del contrato de skills.
