---
uuid: "f1a2b3c4-d5e6-4789-a0b1-c2d3e4f5a6b7"
name: "pull-request-resealed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "PullRequest_Resealed"
context: "ecosystem-evolution"
capabilities:
  - "pull_request_resealed"
hash_signature: "sha256:pending"
---

# Event: PullRequest_Resealed

Clase ECST emitida cuando un push sobre **PR OPEN** actualiza el sello `diff_digest` en `{persist_ref}/validacion.md` (pre-push AUT-42). Correlaciona el sello nuevo con el `PullRequest_Presented` original. No sustituye la presentación ni abre merge.

## Payload ECST

### REQUIRED
- `correlation_id` — `event_id` del `PullRequest_Presented` de la misma rama/PR
- `validated_head_sha` — commit de cabeza que contiene el sello actualizado
- `diff_digest` — digest sellado (excluye `validacion.md`)

### OPTIONAL
- `branch` — rama de la feature

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- `emit-pr-resealed-event` (invocado tras `reseal-diff-validacion` con `outcome: committed`)

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

SSOT: `SddIA/core/event-domain-subscriptions.json` → clave `PullRequest_Resealed`.
