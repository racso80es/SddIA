---
uuid: "242d937d-a0da-4d36-ab89-c0fbbc18c868"
name: audit-sandbox-isolation-rbac
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- chaos-engineering
- quality-assurance
hash_signature: sha256:f693f65ea7b2df4f4e4072f1d4cab81f4c55450235ae16d62fe3fbf158f1db14
inputs: []
outputs:
- isolation_verified: Bloqueo confirmado sin marker de escape fuera del workspace
phases:
- name: Estímulo intento de fuga
  intent: Ejecutar sandbox-breacher con escape_target por defecto.
  delegates_to:
  - agent:tekton
  - tool:sandbox-breacher
- name: Certificación Argos
  intent: Confirmar envelope error y ausencia de archivo fuera del workspace.
  delegates_to:
  - agent:argos
phase_invocations:
- phase_name: Estímulo intento de fuga
  invocations:
  - capsule: tool:sandbox-breacher
    stdin_spec:
      workspace_path:
        from_process_state: workspace_path
      escape_target: "../breach-marker.txt"
    on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# audit-sandbox-isolation-rbac

Proceso audit atómico **Fase 2 Caos**: intenta escape vía `sandbox-breacher` y certifica bloqueo Inocuidad (`assert_workspace_bound`) sin escritura fuera del `workspace_path`.
