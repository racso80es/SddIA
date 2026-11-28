---
context:
- chaos-engineering
- quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:ddd89de1319444375cd69464c3e121a58391f8e5656911f8f2e466ec47e5d6ac"
inputs: []
minteo_maximo: null
name: audit-thermodynamic-toll-failsoft
outputs:
- toll_failsoft_verified: Peaje completó sin abortar proceso pese a estrés E/S telemetría
phase_invocations:
- invocations:
  - capsule: tool:io-choke
    on_error: abort
    stdin_spec:
      target_file: .telemetry-stress-target
      workspace_path:
        from_process_state: workspace_path
  phase_name: Estímulo asfixia E/S
phases:
- delegates_to:
  - agent:tekton
  - tool:io-choke
  intent: Invocar io-choke sobre target dentro del workspace inyectado.
  name: Estímulo asfixia E/S
  requires_capability:
  - contract: qa.probe
    id: qa:probe
    version: '>=1.0.0'
  id: estimulo_asfixia_e_s
- delegates_to:
  - agent:argos
  intent: Verificar exit 0 y bandera fail-soft en thermodynamic_toll.
  name: Certificación Argos
  id: certificacion_argos
porcentaje_de_exito: null
uuid: 2e8cd8cd-e0cd-4b0e-ae78-09150ab9c266
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# audit-thermodynamic-toll-failsoft

## Propósito

Proceso audit atómico **Fase 2 Caos**: estresa E/S vía `io-choke` y certifica que el Peaje Termodinámico fail-soft (D3.13) preserva `exit_code: 0` del negocio.

## Operación

Invocación vía `./sddia-run.sh --process <name>`; fases en frontmatter YAML.
