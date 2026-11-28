---
context:
- event-routing
- knowledge-management
contract: process-contract v1.4.0
hash_signature: "sha256:592f7604bbc1402061e80f4f84373a6c4debd8875de72ada103fb5fcaed5b23b"
inputs:
- event_uri: URI canónica eda://domain/<event_id> (vía EventManagerPort)
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
  id: reaccion_rbac_gobernanza
porcentaje_de_exito: null
uuid: "8d40f01c-799d-4d47-9c17-dee74d970bc3"
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# cerbero-governance-react

## Propósito

Handler lab reactivo a eventos Self-Healing agnósticos. Rehabilita **solo** ante `Domain_Entity_Restored` emitido por Radamanto (D4.14).

## Operación

Invocación vía `./sddia-run.sh --process cerbero-governance-react`; handler/fases en frontmatter YAML.
