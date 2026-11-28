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
hash_signature: "sha256:f3cc2973ebedc3ed18a4fb9bc6e2e8701f0e59c72eac2fc92422382ca3e7f639"
minteo_maximo: null
porcentaje_de_exito: null
---

## Propósito

Sella `PullRequest_Presented` (DCC / `delivery-emit-pr-presented`).

## Orquestación

`emit_pr_presented`: `branch`, `signer_identity_rbac`, opcionales `pr_url`, `tracker_ref`, `qa_profile`.

## Límites

No crea PR remoto.
