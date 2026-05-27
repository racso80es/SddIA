---
uuid: "c4e8a1b2-3f5d-4a9c-8e7b-2d1f0a9b6c3e"
name: workspace-smoke
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- quality-assurance
hash_signature: sha256:72596a9d97a834207b245926057eca1a703ed4494875fa46b86d4079ad7cc7d5
inputs: []
outputs:
- workspace_verified: Indica que el marker fue escrito en el workspace inyectado
phases:
- name: Verificación de workspace
  intent: Escribir marker `.workspace_ok` en workspace_path inyectado por CLI (proceso no ligado a feature/fix).
  delegates_to:
  - skill:filesystem-manager
minteo_maximo: null
porcentaje_de_exito: null
---

# workspace-smoke

Proceso mínimo de laboratorio para **AC2.1** (Fase 2): verifica instanciación de workspace dinámico sin depender de rutas `docs/features/{slug}`.
