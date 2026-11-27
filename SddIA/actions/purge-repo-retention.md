---
uuid: "e4b84b41-c804-4c34-8fea-8b0f6bbcb573"
name: "purge-repo-retention"
version: "1.0.1"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "purge_repo_retention"
inputs: []
outputs: []
hash_signature: "sha256:99a65fa97b45fe55ecdfa0728cb42b64cc3edec07566bced2cdb3dfe472e7c01"
minteo_maximo: null
porcentaje_de_exito: null
---

## 1. Propósito

Orquesta repo-retention-purger simulate o apply.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA.
