---
uuid: "4c5d6e7f-8a9b-4c0d-1e2f-3a4b5c6d7e8f"
name: fix-tool-process
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/sandbox/{target_entity_id}/{recovery_attempt}/"
context:
- ecosystem-evolution
- filesystem-ops
- quality-assurance
hash_signature: sha256:b3571c3033c572f4d25a4440ed36d9f59b5e21e6c5b95d4b99caed5eca90a2b0
inputs:
- event_file_path: Ruta relativa al JSON Tool_Degraded
- target_entity_id: Entidad degradada
- recovery_attempt: Contador de intento
outputs:
- fix_result: Resultado reparación sandbox
phases:
- name: Preparación sandbox
  intent: Materializar entorno aislado writable.
  delegates_to:
  - skill:filesystem-manager
- name: Verificación estructural Argos
  intent: Argos valida estructura del fix; persiste structure_valid sin emitir Status_Restored.
  delegates_to:
  - agent:argos
minteo_maximo: null
porcentaje_de_exito: null
---

# fix-tool-process

Reparación Self-Healing en sandbox estricto. Argos valida materia; **no** sella redención (D4.13).
