---
uuid: "feb7314d-b86d-4653-a876-507c824ec9e2"
name: "agenda-manager"
version: "1.0.0"
contract: "skills-contract v1.1.0"
context: "filesystem-ops"
capabilities:
  - "agenda_manager"
provides:
  - id: "agenda:persist"
    contract: "agenda.persist"
    version: "1.0.0"
hash_signature: "sha256:e79d924e70c39202e13f897d9d54fe0be20ebbe9007de85d38109b0e4079b7a0"
inputs:
  - "operation": "CREATE | LIST"
  - "title": "string; CREATE"
  - "datetime": "ISO-8601; CREATE"
  - "source_ref": "opcional body_ref"
outputs:
  - "exitCode": "0 éxito"
  - "data": "agenda_entry_id o listado"
---

# Skill: agenda-manager

## Propósito

Agenda local de instancia en `{instancia}/.SddIA/agenda/`; contrato DI `agenda:persist` (MVP sin calendarios externos).

## Operación

CREATE/LIST vía cápsula; asiento materializado por `email-triage-gateway`.
