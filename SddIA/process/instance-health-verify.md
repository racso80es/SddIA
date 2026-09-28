---
context: quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:314f1414b86745c6c71e5d491264db9e0b009c73676c57c4d073d388679a5f92"
inputs:
- instance_root: Ruta absoluta de la instancia desplegada
name: instance-health-verify
outputs:
- verdict: APTO | NO-APTO
- audit_path: Ruta relativa del acta en docs/audits
phases:
- intent: 'Handler nativo: systemctl, HTTP GET /api/system-health (WUI), acta instance-deploy-{ESC}-{ts}.md con verdict APTO|NO-APTO.'
  name: Verificar instancia
uuid: 44f1b64c-8a88-4d1e-a8f5-6280b4a44ce2
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# instance-health-verify

Verificación post-deploy de instancia consumidor: unidades systemd habilitadas, NRestarts, WUI GET /api/system-health, perfil de dominio y bóveda. Emite acta en docs/audits.
