---
uuid: "1c026b2b-5ee1-40ff-940d-e214ba98c494"
name: "ci-job-failed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "telemetry"
event_type: "CI_Job_Failed"
context: "system-operations"
capabilities:
  - "ci_job_failed"
hash_signature: "sha256:9133f675771088b24aae96333d65514254128f2ccdd9ba64c2471dd7e0c2d827"
---

# Event: CI_Job_Failed

Telemetría física de un Check Run de GitHub Actions con conclusion=failure asimilado por github-bridge-watcher. No es Peaje Termodinámico ni estímulo de dominio.

## Payload ECST

### REQUIRED
- `repository`
- `head_sha`
- `workflow_name`
- `job_name`
- `conclusion`
- `html_url`
- `check_run_id`

### OPTIONAL
- `pr_url`
- `step_name`
- `run_id`

### FORBIDDEN
- `entity_id`
- `asset_id`
- `exit_code`
- `process_name`

## Emisores autorizados

- `github-bridge-watcher`

## Suscripciones

Ver `SddIA/core/event-telemetry-subscriptions.json` → clave `CI_Job_Failed`.
