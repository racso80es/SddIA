---
uuid: "b0c1d2e3-f4a5-4678-9012-3456789abcde"
name: "emit-tracker-sync-failed"
version: "2.0.1"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "tracker-sync-failed-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs: []
outputs: []
hash_signature: "sha256:044deccae7ef02afe8527f5f013a46dcf8449e559d66de007d23aa4a105475cb"
minteo_maximo: null
porcentaje_de_exito: null
---

## 1. Propósito

Registra Tracker_Sync_Failed para tracker-sync-replay.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA.
