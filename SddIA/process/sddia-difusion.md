---
uuid: de142ec3-4022-4ac1-bcf4-1b8490cabf9d
name: sddia-difusion
version: 1.1.0
contract: process-contract v1.4.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
context:
- ecosystem-evolution
hash_signature: "sha256:30acb9f13f5ce5f00cd7399f0319fef2bca403e4c0c9fd3e35dcadac312bb11c"
inputs:
- core_root: Raíz del Core resuelta por Cumulo (directories / constitution)
- target_repo: Repositorio o workspace destino de la difusión
outputs:
- touchpoints_report: Resumen de artefactos escritos (.cursor/rules, .github, etc.)
- sync_snapshot: Referencia de commit o tag de cierre
phases:
- name: Lectura del Core
  intent: Inventariar normas, contratos y procesos relevantes para export contextual.
  delegates_to:
  - agent:cumulo
  id: lectura_del_core
- name: Generación de touchpoints
  intent: Escribir reglas IDE y workflows GitHub de forma idempotente donde aplique.
  requires_capability:
  - id: fs:persist
    contract: fs.persist
    version: '>=1.0.0'
  id: generacion_de_touchpoints
- name: Verificación cruzada
  intent: Validar que los touchpoints referencian rutas SSOT coherentes.
  delegates_to:
  - agent:cumulo
  id: verificacion_cruzada
- name: Snapshot
  intent: Consolidar cambios con sync git ciego según políticas.
  requires_capability:
  - id: proc:git-sync
    contract: proc.git_sync
    version: '>=1.0.0'
  id: snapshot
minteo_maximo: null
porcentaje_de_exito: null
---

# sddia-difusion

Proceso que cubre la **deuda** registrada al purgar la antigua acción de difusión: materializa la propagación controlada del Core SddIA hacia `.cursor/rules`, `.github` y homólogos, sin violar la frontera Acciones/Procesos.

## Propósito

Proceso que cubre la **deuda** registrada al purgar la antigua acción de difusión: materializa la propagación controlada del Core SddIA hacia `.cursor/rules`, `.github` y homólogos, sin violar la frontera Acciones/Procesos.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
