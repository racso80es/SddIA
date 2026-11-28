---
uuid: "a6eb7f0c-8b2f-4c7d-ae5e-6c1b589f3c92"
name: "materialize-ci-chronic-failure-pbi"
version: "1.0.1"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "materialize_ci_chronic_failure_p"
hash_signature: "sha256:a16c348bc78fb51b24bf6b2c6d965384a8cbc4d0d3c9722705e116e226f77999"
---

## Propósito

Materializar PBI Kaizen ante `CI_Chronic_Failure_Detected`.

## Orquestación

Handler `materialize_ci_chronic_failure_pbi.rs`: lee fractura CI, escribe item backlog según tracker instancia.

## Límites

No re-ejecuta CI; solo documentación/deuda.
