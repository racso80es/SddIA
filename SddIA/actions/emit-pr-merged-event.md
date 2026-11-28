---
uuid: "c0d71f2b-c1c1-4c56-8f74-2f4f41b24c4f"
name: "emit-pr-merged-event"
version: "2.0.2"
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
hash_signature: "sha256:8a0ddb36a4991c83d308cff778fcd68e2338ca2ee564b69c144af347c03a30f9"
minteo_maximo: null
porcentaje_de_exito: null
---

## Propósito

Emite `PullRequest_Merged` en `eda_bus.pending` tras merge (`accept-pr`). Handler `actions::emit_pr_merged`.

## Orquestación

Exige `merge_commit_hash`/`hash_signature`, ramas y `correlation_id`; delegaciones declaradas en capabilities.

## Límites

No ejecuta merge Git.
