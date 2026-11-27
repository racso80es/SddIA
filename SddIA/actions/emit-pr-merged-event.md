---
uuid: "c0d71f2b-c1c1-4c56-8f74-2f4f41b24c4f"
name: "emit-pr-merged-event"
version: "2.0.3"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "pr-merged-event-emission"
  - "event-bus-pending-write"
  - "delegate-git-manager"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs: []
outputs: []
hash_signature: "sha256:807a4d3a13f8dfe1ba6fff2e44ace73f229ce16d22a8c76bd3c79ef59e37ed87"
minteo_maximo: null
porcentaje_de_exito: null
---

## 1. Propósito

Emite PullRequest_Merged tras accept-pr

## 2. Orquestación

1. Validar entradas.
2. Delegar vía Cúmulo.
3. Envelope JSON canónico.

## 3. Límites

Sin SO directo.
