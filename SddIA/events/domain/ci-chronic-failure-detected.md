---
uuid: "c55ef8cc-41b8-42af-a524-c58b847039a8"
name: "ci-chronic-failure-detected"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "CI_Chronic_Failure_Detected"
context: "quality-assurance"
capabilities:
  - "ci_chronic_failure_detected"
hash_signature: "sha256:28f0f5e370932b04c3f367f172d27848c1f89664bd943273348eee3e481862de"
---

# Event: CI_Chronic_Failure_Detected

Cuota crónica de fallos CI por job_name sin mapa job→entidad. Emisor Radamanto. Cúmulo materializa PBI Kaizen. No es DIA ni Kintsugi.

## Payload ECST

### REQUIRED
- `job_name`
- `workflow_name`
- `failure_count`
- `quota_limit`
- `sample_check_run_id`
- `sample_html_url`
- `repository`
- `head_sha`

### OPTIONAL
- `run_id`
- `step_name`

### FORBIDDEN
- `entity_id`
- `asset_id`
- `review_id`
- `process_name`

## Emisores autorizados

- `radamanto`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `CI_Chronic_Failure_Detected`.
