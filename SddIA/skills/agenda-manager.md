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
hash_signature: "sha256:195ab0330d1a3b72eb37f16c52ae3d5618ac368a16f17270349974fdc4983af4"
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

Agenda local de instancia. Persistencia en `{instancia}/.SddIA/agenda/`. Cero calendarios externos en MVP.

MVP: el asiento lo materializa el handler nativo `email-triage-gateway` bajo `requires_capability: agenda:persist`. Esta skill es el contrato DI (pivote de proveedor).
