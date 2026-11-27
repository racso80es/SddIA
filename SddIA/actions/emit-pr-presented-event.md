---
uuid: "a1b2c3d4-e5f6-4789-a012-3456789abcde"
name: "emit-pr-presented-event"
version: "2.0.2"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "pr-presented-event-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs: []
outputs: []
hash_signature: "sha256:f52711fda1a1a5e49b1afd28eb8cf24ab45c1fc548a7083d1fc83afa67428260"
minteo_maximo: null
porcentaje_de_exito: null
---

## 1. Propósito

Emite PullRequest_Presented en eda_bus.pending vía EventManagerPort.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico (success, exitCode, data, error).

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA.
