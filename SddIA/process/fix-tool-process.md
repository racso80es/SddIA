---
context:
- ecosystem-evolution
- filesystem-ops
- quality-assurance
contract: process-contract v1.4.0
hash_signature: sha256:0fb2e83c3658bd1e3d03dbf40fff2847f5048eca5f053fb2e45d802710ad829f
inputs:
- event_file_path: Ruta relativa al JSON Domain_Entity_Degraded
- target_entity_id: Entidad degradada
- recovery_attempt: Contador de intento
minteo_maximo: null
name: fix-tool-process
outputs:
- fix_result: Resultado reparación sandbox
phases:
- intent: Materializar entorno aislado writable.
  name: Preparación sandbox
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - agent:argos
  intent: Argos valida estructura del fix; persiste structure_valid sin emitir Status_Restored.
  name: Verificación estructural Argos
porcentaje_de_exito: null
uuid: 4c5d6e7f-8a9b-4c0d-1e2f-3a4b5c6d7e8f
version: 1.0.1
workspace_template: .SddIA/sandbox/{target_entity_id}/{recovery_attempt}/
---

# fix-tool-process

Reparación Self-Healing en sandbox estricto. Argos valida materia; **no** sella redención (D4.13).

**Nota de Compatibilidad (Gate Estricto):** Este proceso opera única y exclusivamente con entidades cuyo `entity_type` sea `"tool"`. Cualquier otro tipo (incluyendo `"skill"`) será rechazado/ignorado y resultará en un "skip".
