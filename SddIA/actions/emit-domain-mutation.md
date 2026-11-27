---
uuid: "7e4a9c2b-1d3f-4a8e-9b6c-0f1e2d3a4b5c"
name: "emit-domain-mutation"
version: "2.0.2"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "domain-mutation-emission"
  - "event-bus-pending-write"
  - "delegate-filesystem-manager"
  - "delegate-crypto-broker"
  - "domain-event-type-translation"
inputs: []
outputs: []
hash_signature: "sha256:81f75a7db2da63ad27e79e6128b53c4826c824808eb446c49456b7d03de4308b"
minteo_maximo: null
porcentaje_de_exito: null
---

## 1. Propósito

Sello universal Domain_Entity_* tras mutación del genoma.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA.
