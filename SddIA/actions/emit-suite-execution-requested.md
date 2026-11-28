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
hash_signature: "sha256:39f1c9d29000bfa68994ac5211c2c9854b137b5be1ae8a922c9e1ffc4019a7a9"
minteo_maximo: null
porcentaje_de_exito: null
---

## Propósito

Publica `Suite_Execution_Requested` para disparo de suite (AIUA).

## Orquestación

`suite_execution_requested.rs`: valida `SddIA/suites/{suite_id}.md`, ECST, fractal domain.

## Límites

No corre nodos de la suite.
