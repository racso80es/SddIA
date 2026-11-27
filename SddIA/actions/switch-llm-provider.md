---
uuid: "a6f5e0a4-ccc5-4076-bf8a-b60cfe1fa684"
name: "switch-llm-provider"
version: "1.0.2"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "switch_llm_provider"
inputs: []
outputs: []
hash_signature: "sha256:5af96903c82775783bbf6dcee08bb62637e9f507d723763885299bbe66458aa5"
minteo_maximo: null
porcentaje_de_exito: null
---

## 1. Propósito

Conmutación explícita del proveedor LLM activo en .SddIA/llm-runtime.local.json, validando roles en capability-bindings.md sin auto-failover.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico (success, exitCode, data, error).

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA.
