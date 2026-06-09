---
uuid: "b21e89f7-66a8-4235-950c-d9c9efbd6359"
name: "pull-request-audited"
version: "1.0.0"
contract: "events-contract v1.2.0"
event_family: "domain"
event_type: "PullRequest_Audited"
context: "quality-assurance"
capabilities:
  - "pull_request_audited"
  - "argos_verdict_ecst"
hash_signature: "sha256:b12f541439b490b282cef9c8fcf338c685c18f2a24bc5ec122f14c3a208876dd"
---

# Event: PullRequest_Audited

Clase ECST emitida cuando **Argos** finaliza el escrutinio determinista de una Pull Request o rama candidata. Patrón Event-Carried State Transfer: el payload contiene el veredicto completo sin consultas posteriores.

## Payload ECST

### REQUIRED
- `audit_event_reference` — UUID o hash canónico unívoco de la ejecución de auditoría
- `target_entity_id` — identificador del artefacto evaluado (branch, PR o path)
- `resolution` — enum estricto: `PASS` | `REJECT` | `FLAG`

### OPTIONAL
- `violated_rules` — array de strings con trazabilidad normativa (si aplica)

### FORBIDDEN
- Cualquier valor de `resolution` fuera de `PASS`, `REJECT`, `FLAG`

## Invariantes

- `audit_event_reference` correlaciona la ejecución específica de Argos (típicamente `correlation_id` de la aduana).
- Argos deposita el evento y termina jurisdicción (Ceguera Espacial).

## Emisores autorizados

- `emit-pr-audited-event` (invocado desde `pull-request-review` / fase Veredicto con `emitter_agent: argos`)

## Suscripciones

Ver `SddIA/core/event-subscriptions.json` → clave `PullRequest_Audited`.
