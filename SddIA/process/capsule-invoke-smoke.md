---
context:
- quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:6b61755dd588327bfcec63cc567ce2d68082c6e80b264a0f85d8a6dff819d6f3"
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
  id: invocacion_io_choke
porcentaje_de_exito: null
uuid: f3a8c2d1-9e4b-4a7c-b6d5-1e0f9a8b7c6d
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# capsule-invoke-smoke

Proceso mínimo de laboratorio para verificar invocación nativa de cápsulas `tool:` vía orquestador Rust (P5 / D-P5.2).

## Propósito

Proceso mínimo de laboratorio para verificar invocación nativa de cápsulas `tool:` vía orquestador Rust (P5 / D-P5.2).

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
