---
context:
- ecosystem-evolution
- filesystem-ops
- quality-assurance
contract: process-contract v1.4.0
hash_signature: "sha256:59b2cfffffb07b14c15af68db2f2571996472aaae7f436ccaa327d38d3215346"
inputs:
- 'event_uri: Token lógico EventUri (eda://domain/{event_id})'
- 'target_entity_id: Entidad degradada'
- 'recovery_attempt: Contador de intento'
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
  id: preparacion_sandbox
- delegates_to:
  - agent:argos
  intent: Argos valida estructura del fix; persiste structure_valid sin emitir Status_Restored.
  name: Verificación estructural Argos
  id: verificacion_estructural_argos
porcentaje_de_exito: null
uuid: "bc04740a-d511-4b66-8824-d1701e7a9545"
version: 2.0.0
workspace_template: .SddIA/sandbox/{target_entity_id}/{recovery_attempt}/
---

# fix-tool-process

Reparación Self-Healing en sandbox estricto. Argos valida materia; **no** sella redención (D4.13).

**Nota de Compatibilidad (Gate Estricto):** Este proceso opera única y exclusivamente con entidades cuyo `entity_type` sea `"tool"`. Cualquier otro tipo (incluyendo `"skill"`) será rechazado/ignorado y resultará en un "skip".

## Propósito

Reparación Self-Healing en sandbox estricto. Argos valida materia; **no** sella redención (D4.13).

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
