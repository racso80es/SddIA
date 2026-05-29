---
uuid: "3b4c5d6e-7f8a-4b9c-0d1e-2f3a4b5c6d7e"
name: cerbero-governance-react
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- event-routing
- knowledge-management
hash_signature: sha256:74455f32d07f5047ccc8aec1b522a8b368dc31767a338c04eafa1be17a68b90e
inputs:
- event_file_path: Ruta relativa al JSON dominio en ./.events/domain/
- event_type: Tipo ECST (Domain_Entity_Degraded, Domain_Entity_Restored, Domain_Entity_Deprecated)
outputs:
- governance_result: Estado revocación actualizado
phases:
- name: Reacción RBAC gobernanza
  intent: Actualizar lista revocación/rehabilitación ante eventos Radamanto.
  delegates_to:
  - agent:cerbero
minteo_maximo: null
porcentaje_de_exito: null
---

# cerbero-governance-react

Handler lab reactivo a eventos Self-Healing agnósticos. Rehabilita **solo** ante `Domain_Entity_Restored` emitido por Radamanto (D4.14).
