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
uuid: 4c5d6e7f-8a9b-4c0d-1e2f-3a4b5c6d7e8f
version: 2.0.0
workspace_template: .SddIA/sandbox/{target_entity_id}/{recovery_attempt}/
---

# fix-tool-process

## Propósito

Self-healing acotado a entidades `tool` degradadas: materializa sandbox bajo `.SddIA/sandbox/{target_entity_id}/{recovery_attempt}/`, valida estructura con Argos y **no** emite `Status_Restored` (D4.13). Handler `fix_tool_process_core.rs`; ruta fractal `route-domain` / `route-fractal`.

## Orquestación

1. Fase `Preparación sandbox` (`fs:persist`): crea árbol writable y artefacto `fix-artifact.md` vía `materialize_sandbox` (raíz `sandbox_root` desde config Radamanto).
2. Fase `Verificación estructural Argos`: delega a `agent:argos` para `structure_valid` sin sellado de redención.
3. Entradas: `event_uri` (dominio), `target_entity_id`, `recovery_attempt`. Salida: `fix_result`.
4. Gate estricto: rechaza/skip si `entity_type` ≠ `tool` (skills ignorados).

## Límites

Escritura confinada al sandbox (`SDDIA_SANDBOX_STRICT`); no muta genoma de producción fuera del área aislada.
