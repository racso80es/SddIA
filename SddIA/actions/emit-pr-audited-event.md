---
uuid: "b504fe38-3519-4943-bfcd-5137c3a00f01"
name: "emit-pr-audited-event"
version: "2.0.2"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "pr-audited-event-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs: []
outputs: []
hash_signature: "sha256:ff8120a0906f6b5a10d6e40e13965ad118cfde033fd4efd948b59c00f3ebdd5a"
minteo_maximo: null
porcentaje_de_exito: null
---

## Propósito

Emitir `PullRequest_Audited` tras fase de auditoría técnica en `pull-request-review` / Argos.

## Orquestación

Handler `actions::emit_pr_audited`: sella payload ECST referenciado en `SddIA/events/domain/pull-request-audited.md` y escribe en `eda_bus.pending` vía crypto-broker + filesystem handoff.

## Límites

No fusiona PR ni invoca `accept-pr`; solo evento de dominio.
