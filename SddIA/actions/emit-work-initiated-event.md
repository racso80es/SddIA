---
uuid: "c8d9e0f1-a2b3-4c5d-8e7f-6a5b4c3d2e1f"
name: "emit-work-initiated-event"
version: "2.0.1"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "work-initiated-event-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs: []
outputs: []
hash_signature: "sha256:24718fc01e7b5fe8e4666b41d915067ebb6451baca670a0d1bfc7f01e65b2a3a"
minteo_maximo: null
porcentaje_de_exito: null
---

## 1. Propósito

Emite Work_Initiated al cerrar workspace-init en feature, bug-fix y refactorization.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico (success, exitCode, data, error).

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA.
