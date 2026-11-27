---
context: quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:08a2b9fd1dbc9ed362666268ee6e01f35c1d760b8c8f2db0c918973957343498"
inputs:
- instance_root: Ruta absoluta de la instancia desplegada
name: instance-health-verify
outputs:
- verdict: APTO | NO-APTO
- audit_path: Ruta relativa del acta en docs/audits
phases:
- intent: 'Handler nativo: systemctl, HTTP GET /api/system-health (WUI), acta instance-deploy-{ESC}-{ts}.md con verdict APTO|NO-APTO.'
  name: Verificar instancia
  id: verificar_instancia
uuid: 44f1b64c-8a88-4d1e-a8f5-6280b4a44ce2
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# instance-health-verify

Verificación post-deploy de instancia consumidor: unidades systemd habilitadas, NRestarts, WUI GET /api/system-health, perfil de dominio y bóveda. Emite acta en docs/audits.

## Propósito

Verificación post-deploy de instancia consumidor: unidades systemd habilitadas, NRestarts, WUI GET /api/system-health, perfil de dominio y bóveda. Emite acta en docs/audits.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
