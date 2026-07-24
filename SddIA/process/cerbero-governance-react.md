---
context:
- event-routing
- knowledge-management
contract: process-contract v1.4.0
hash_signature: sha256:4dc3cde6bd2635814d445f22d30316bf700ea1f3fb4dbc2f96e0bb22022d9bb6
inputs:
- event_file_path: Ruta relativa al JSON dominio en ./.events/domain/
- event_type: Tipo ECST (Domain_Entity_Degraded, Domain_Entity_Restored, Domain_Entity_Deprecated)
minteo_maximo: null
name: cerbero-governance-react
outputs:
- governance_result: Estado revocación actualizado
phases:
- delegates_to:
  - agent:cerbero
  intent: Actualizar lista revocación/rehabilitación ante eventos Radamanto.
  name: Reacción RBAC gobernanza
  requires_capability:
  - contract: gov.rbac
    id: gov:rbac
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: 3b4c5d6e-7f8a-4b9c-0d1e-2f3a4b5c6d7e
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# cerbero-governance-react

Handler lab reactivo a eventos Self-Healing agnósticos. Rehabilita **solo** ante `Domain_Entity_Restored` emitido por Radamanto (D4.14).
