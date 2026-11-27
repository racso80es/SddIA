---
uuid: "67e82606-4ab4-4cd6-a1f9-e950c762999b"
name: "kaizen-idea-captured"
version: "1.1.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Kaizen_Idea_Captured"
context: "ecosystem-evolution"
capabilities:
  - "kaizen_idea_captured"
hash_signature: "sha256:c290ea7987ac673c15953f1a61a7f36415a6936232896a7fb50471d2518303ff"
---

# Event: Kaizen_Idea_Captured

Idea Kaizen capturada vía patrón `TODO:` o `IDEA:` en Telegram.

## Payload ECST

### REQUIRED
- `idea_text`
- `source`
- `raw_text`

### OPTIONAL
- *(ninguno)*

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- Proceso **`telegram-gateway`**

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

Suscriptores en catálogo: **`[]`** (sumidero intencional v1).

- **Política L-A4-SINK:** el evento se registra en el bus y completa el sweep sin fan-out a `forge-pbi` ni forja automática.
- **Uso:** traza de ideas capturadas (`TODO:` / `IDEA:`) vía `telegram-gateway`; el operador o un proceso explícito decide la forja posterior.
- **Auditoría:** `EMPTY_SUBSCRIBERS` en registro **domain** es esperado; no dispara umbral `circuit_alert` (solo orchestration vacía eleva Kaizen de catálogo).

## Propósito (complemento)

Idea Kaizen capturada vía patrón `TODO:` o `IDEA:` en Telegram.
