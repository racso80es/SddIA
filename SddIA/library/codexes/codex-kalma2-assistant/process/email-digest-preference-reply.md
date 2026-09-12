---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: "sha256:fdfabc7ca44dddb2c30c6dba16fb182e30edf3f81c0caaf4a4eb2a85e8fa4132"
inputs:
- event_file_path: Ruta de la instancia ECST TelegramCallback_Received
name: email-digest-preference-reply
outputs:
- success: boolean
- skipped: true si namespace ajeno, ignore o token ausente
- preference_emitted: true si se emitió User_Preference_Change_Requested
- operation: activate | ignore
- target_event_id: UUID del evento emitido
phases:
- intent: Filtrar namespace dpref:; skipped si callback ajeno.
  name: Gate-Callback
- intent: Resolver token en estado del digest; fail-closed si ausente.
  name: Correlacion-Token
- delegates_to:
  - action:emit-user-preference-change-requested
  intent: Emitir activate max|mute vía emit-user-preference-change-requested; ign no emite.
  name: Emision-Preferencia
uuid: c77f96ad-ad01-466e-87cf-950b1976555b
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# email-digest-preference-reply

Consume TelegramCallback_Received dpref:*; correlaciona token del digest y emite User_Preference_Change_Requested. Cero IMAP. Cero LLM.
