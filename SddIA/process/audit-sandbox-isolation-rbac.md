---
context:
- chaos-engineering
- quality-assurance
contract: process-contract v1.4.0
hash_signature: sha256:b2044c8aa539b2c6e13a7fe2adba8697ddb6b079e6da302792fa25ee9102d0bf
inputs: []
minteo_maximo: null
name: audit-sandbox-isolation-rbac
outputs:
- isolation_verified: Bloqueo confirmado sin marker de escape fuera del workspace
phase_invocations:
- invocations:
  - capsule: tool:sandbox-breacher
    on_error: abort
    stdin_spec:
      escape_target: ../breach-marker.txt
      workspace_path:
        from_process_state: workspace_path
  phase_name: Estímulo intento de fuga
phases:
- delegates_to:
  - agent:tekton
  - tool:sandbox-breacher
  intent: Ejecutar sandbox-breacher con escape_target por defecto.
  name: Estímulo intento de fuga
  requires_capability:
  - contract: qa.probe
    id: qa:probe
    version: '>=1.0.0'
- delegates_to:
  - agent:argos
  intent: Confirmar envelope error y ausencia de archivo fuera del workspace.
  name: Certificación Argos
porcentaje_de_exito: null
uuid: 242d937d-a0da-4d36-ab89-c0fbbc18c868
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# audit-sandbox-isolation-rbac

Proceso audit atómico **Fase 2 Caos**: intenta escape vía `sandbox-breacher` y certifica bloqueo Inocuidad (`assert_workspace_bound`) sin escritura fuera del `workspace_path`.
