---
uuid: "e7f1a2b3-c4d5-4e6f-9a0b-8c7d6e5f4a3b"
name: "local-qa-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "orchestration"
event_type: "Local_QA_Requested"
context: "local-quality-gate"
capabilities:
  - "local_qa_requested"
  - "pre_push_blocking"
hash_signature: "sha256:pending"
---

# Event: Local_QA_Requested

Estímulo síncrono emitido por el hook `pre-push` vía `route_domain_event_core.py --event Local_QA_Requested --blocking`. Activa la aduana de fricción local antes de permitir el push al repositorio remoto. Suscriptor: **Argos** → `pull-request-review`.

## Payload ECST

### REQUIRED

- `branch`

### OPTIONAL

- `emitter_context`

### FORBIDDEN

- *(ninguno)*

## Emisores autorizados

- `git-hook-pre-push` (vía `route_domain_event_core.py __main__`)

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Local_QA_Requested`.

**Modo de dispatch:** `SDDIA_LAB_ROUTE_SYNC=1` (síncrono bloqueante); el exit code propaga a Git.
