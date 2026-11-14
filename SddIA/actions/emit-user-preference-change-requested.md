---
uuid: "c3d4e5f6-a7b8-4901-c234-567890ab004"
name: "emit-user-preference-change-requested"
version: "2.0.0"
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
  - "event_uri": "ruta relativa en eda_fractal.domain"
hash_signature: "sha256:ad3abf87db689ab45a760b9cfb38093f0552a1ad8832ddbea49e82ebaf57323b"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: emit-user-preference-change-requested

Emite `User_Preference_Change_Requested` en `./.events/domain/`. Handler nativo en `execute-process`.
