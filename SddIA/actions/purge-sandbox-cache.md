---
uuid: "37454c3a-4590-4dfb-a439-2014876a0138"
name: "purge-sandbox-cache"
version: "1.0.2"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "purge_sandbox_cache"
  - "delegate-ephemeral-cache-purger"
inputs: []
outputs: []
hash_signature: "sha256:2c5b60c2294e60eb4f02e01bafc8d82b2de5f8eea0d9cf038024d3cbc2f65096"
minteo_maximo: null
porcentaje_de_exito: null
---

## 1. Propósito

Orquesta limpieza de caché sandbox autorizada.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA.
