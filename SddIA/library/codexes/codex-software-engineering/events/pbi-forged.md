---
uuid: "9a0c1d2e-3f4b-4a5c-8d6e-7f8091a2b3c4"
name: "pbi-forged"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "PBI_Forged"
context: "knowledge-management"
---

# Event: PBI_Forged

Sello de PBI materializado por `forge-pbi` en el pending del proyecto cliente.

## Payload ECST

### REQUIRED
- `document_id`
- `project_slug`

### OPTIONAL
- `process`
- `artifact_path`

### FORBIDDEN
- `hash_signature`

## Emisores autorizados

- `forge-pbi`
