---
uuid: "d5e6f7a8-b9c0-4d1e-8f2a-3b4c5d6e7f8a"
name: "emit-suite-execution-requested"
version: "2.0.2"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "suite-execution-requested-emission"
  - "event-bus-domain-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs: []
outputs: []
hash_signature: "sha256:1ff2a79285894361dd31cb845f2175f0114407e19e98c503d5558e72c72dd158"
minteo_maximo: null
porcentaje_de_exito: null
---

## 1. Propósito

Encola Suite_Execution_Requested desde despacho Aiúa.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA.
