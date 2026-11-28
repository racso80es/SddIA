---
uuid: "4548fdd5-7c77-4589-a289-73e622bf63df"
name: "emit-user-preference-change-requested"
version: "2.0.2"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "user-preference-change-emission"
  - "event-bus-domain-write"
inputs: []
outputs: []
hash_signature: "sha256:f7486c0c9a0a2abfbcb3aea057e1cc34b5e69847a66cca33af36b92c00bb59ed"
minteo_maximo: null
porcentaje_de_exito: null
---

## Propósito

`User_Preference_Change_Requested` (domain) para cambios de preferencia Kalma2/AIUA.

## Orquestación

`user_preference_change_requested.rs`: `operation`, `channel`, `payload`; mint UUID; usado por `dispatch-aiua-intent` y handlers email.

## Límites

Sin `raw_utterance` en ECST.
