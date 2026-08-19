---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: "sha256:6568daa668e929ef221954067c5450ae749c816d0b8b8dffc95eb283d09a677c"
inputs:
- event_file_path: Ruta de la instancia ECST Email_Quick_Action_Requested
name: email-quick-action-ingest
outputs:
- recorded: true si se escribió proof de intención
phases:
- intent: Validar message_uid y action en {archive,draft,delegate}; abort silencioso si inválido.
  name: Gate
- delegates_to:
  - skill:filesystem-manager
  intent: Escribir proof de intención bajo eda_instance.proofs/email-quick-action. Prohibido IMAP STORE y SMTP.
  name: Persistencia
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
uuid: e11c4348-29b5-45cd-bac8-f33f40e18a12
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# email-quick-action-ingest

Consume Email_Quick_Action_Requested: valida acción, persiste proof de intención. No muta IMAP. No SMTP.
