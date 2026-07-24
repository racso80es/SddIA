---
context:
- quality-assurance
contract: process-contract v1.4.0
hash_signature: sha256:76456a7a5d4030da2b773344000285d42bbb64a30e07b5cc645966e0b4a8cc23
inputs: []
minteo_maximo: null
name: capsule-invoke-smoke
outputs:
- capsule_invoked: Indica que io-choke respondió con éxito en el workspace inyectado
phases:
- delegates_to:
  - tool:io-choke
  intent: Invocar tool:io-choke sobre workspace_path inyectado (golden D-P5.2).
  name: Invocación io-choke
  requires_capability:
  - contract: qa.probe
    id: qa:probe
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: f3a8c2d1-9e4b-4a7c-b6d5-1e0f9a8b7c6d
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# capsule-invoke-smoke

Proceso mínimo de laboratorio para verificar invocación nativa de cápsulas `tool:` vía orquestador Rust (P5 / D-P5.2).
