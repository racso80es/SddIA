---
uuid: "c3d4e5f6-a7b8-4901-c234-567890ab004"
name: "emit-user-preference-change-requested"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "user-preference-change-emission"
  - "event-bus-domain-write"
inputs:
  - "operation": "propose | activate | revoke | purge | ignore"
  - "channel": "string"
  - "payload": "objeto ECST sin campos FORBIDDEN"
outputs:
  - "success": "boolean"
  - "event_id": "UUID v4"
  - "target_path": "ruta relativa en eda_fractal.domain"
---

# Acción: emit-user-preference-change-requested

Emite `User_Preference_Change_Requested` en `./.events/domain/`. Handler nativo en `execute-process`.
