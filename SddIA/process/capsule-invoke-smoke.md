---
uuid: "f3a8c2d1-9e4b-4a7c-b6d5-1e0f9a8b7c6d"
name: capsule-invoke-smoke
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- quality-assurance
hash_signature: sha256:47871ee0fe1c4cc6994a91505415785df5fa1681d9ecc8b1a8f93711ae194d5b
inputs: []
outputs:
- capsule_invoked: Indica que io-choke respondió con éxito en el workspace inyectado
phases:
- name: Invocación io-choke
  intent: Invocar tool:io-choke sobre workspace_path inyectado (golden D-P5.2).
  delegates_to:
  - tool:io-choke
minteo_maximo: null
porcentaje_de_exito: null
---

# capsule-invoke-smoke

Proceso mínimo de laboratorio para verificar invocación nativa de cápsulas `tool:` vía orquestador Rust (P5 / D-P5.2).
