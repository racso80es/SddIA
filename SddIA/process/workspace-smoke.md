---
context:
- quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:2450e36cabfe73ebd61ba86f8b7971da9e1d921d212a08eb6d822fd06568ac4b"
inputs: []
minteo_maximo: null
name: workspace-smoke
outputs:
- workspace_verified: Indica que el marker fue escrito en el workspace inyectado
phases:
- intent: Escribir marker `.workspace_ok` en workspace_path inyectado por CLI (proceso no ligado a feature/fix).
  name: Verificación de workspace
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
  id: verificacion_de_workspace
porcentaje_de_exito: null
uuid: c4e8a1b2-3f5d-4a9c-8e7b-2d1f0a9b6c3e
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# workspace-smoke

Proceso mínimo de laboratorio para **AC2.1** (Fase 2): verifica instanciación de workspace dinámico sin depender de rutas `docs/features/{slug}`.

## Propósito

Proceso mínimo de laboratorio para **AC2.1** (Fase 2): verifica instanciación de workspace dinámico sin depender de rutas `docs/features/{slug}`.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
