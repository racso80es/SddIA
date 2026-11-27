---
context:
- chaos-engineering
- quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:ec6e7a26f1255006c6881b23e0e889fd878b2aefcb6aba32422ffa02e9249934"
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
  id: estimulo_intento_de_fuga
- delegates_to:
  - agent:argos
  intent: Confirmar envelope error y ausencia de archivo fuera del workspace.
  name: Certificación Argos
  id: certificacion_argos
porcentaje_de_exito: null
uuid: 242d937d-a0da-4d36-ab89-c0fbbc18c868
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# audit-sandbox-isolation-rbac

Proceso audit atómico **Fase 2 Caos**: intenta escape vía `sandbox-breacher` y certifica bloqueo Inocuidad (`assert_workspace_bound`) sin escritura fuera del `workspace_path`.

## Propósito

Proceso audit atómico **Fase 2 Caos**: intenta escape vía `sandbox-breacher` y certifica bloqueo Inocuidad (`assert_workspace_bound`) sin escritura fuera del `workspace_path`.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
