---
uuid: "2e8cd8cd-e0cd-4b0e-ae78-09150ab9c266"
name: audit-thermodynamic-toll-failsoft
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- chaos-engineering
- quality-assurance
hash_signature: sha256:6a3d76c76059ed243680ec22d29ab0c374d769183ce5f8213da271e0e374aec4
inputs: []
outputs:
- toll_failsoft_verified: Peaje completó sin abortar proceso pese a estrés E/S telemetría
phases:
- name: Estímulo asfixia E/S
  intent: Invocar io-choke sobre target dentro del workspace inyectado.
  delegates_to:
  - agent:tekton
  - tool:io-choke
- name: Certificación Argos
  intent: Verificar exit 0 y bandera fail-soft en thermodynamic_toll.
  delegates_to:
  - agent:argos
phase_invocations:
- phase_name: Estímulo asfixia E/S
  invocations:
  - capsule: tool:io-choke
    stdin_spec:
      workspace_path:
        from_process_state: workspace_path
      target_file: ".telemetry-stress-target"
    on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# audit-thermodynamic-toll-failsoft

Proceso audit atómico **Fase 2 Caos**: estresa E/S vía `io-choke` y certifica que el Peaje Termodinámico fail-soft (D3.13) preserva `exit_code: 0` del negocio.
