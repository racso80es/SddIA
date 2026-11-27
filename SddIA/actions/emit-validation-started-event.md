---
uuid: "e84218a8-7e40-4436-8ae7-a3fcc3396ad7"
name: "emit-validation-started-event"
version: "2.0.1"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "validation-started-event-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs: []
outputs: []
hash_signature: "sha256:4d9eb47c0be4dd1eab75a366007cc8333f6c17ff22f96007e673a060951283a2"
minteo_maximo: null
porcentaje_de_exito: null
---

## 1. Propósito

Emite Validation_Started (fail-soft) al entrar en Verificación.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA.
