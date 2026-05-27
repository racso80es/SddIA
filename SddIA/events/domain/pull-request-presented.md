---
uuid: "5e488ae6-7cb2-4a2c-9725-4a7d4ce239ea"
name: "pull-request-presented"
version: "1.2.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "PullRequest_Presented"
context: "ecosystem-evolution"
capabilities:
  - "pull_request_presented"
  - "dlt_oracle_route"
hash_signature: "sha256:8f3c2a1b9e4d7f6a5c0b8e2d1f4a3c6b7e8d9f0a1b2c3d4e5f6a7b8c9d0e1f2"
---

# Event: PullRequest_Presented

Clase ECST para presentación de PR en bus local. Suscriptores: aduana **`pull-request-review`** (Argos) + anclaje DLT (Cúmulo/IOTA).

## Payload ECST

### REQUIRED
- `branch`
- `status`

### OPTIONAL
- `pr_url`
- `repository` *(ruta oráculo remoto)*
- `origin_agent` *(ej. `jules`, `delivery-close-cycle`)*
- `dlt_anchor_address` *(digest u object_id IOTA — skip re-anclaje en route)*
- `signer_identity_rbac` *(identidad relay local)*

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- `emit-pr-presented-event` (invocado por `delivery-close-cycle` con `emitter_agent` del proceso)
- `github-bridge-watcher` (oráculo sensor DLT — ruta remota Jules)

## Suscripciones

Ver `SddIA/core/event-subscriptions.json` → clave `PullRequest_Presented`.

**Nota oráculo:** si `payload.dlt_anchor_address` está presente, el suscriptor IOTA en `route-domain-event` omite re-publicación (`skipped-pre-anchored`).
