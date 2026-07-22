---
uuid: de142ec3-4022-4ac1-bcf4-1b8490cabf9d
name: sddia-difusion
version: 1.1.0
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- ecosystem-evolution
hash_signature: sha256:6ea867c307d8e0abe50190423cc86ba376b90bdfaec8fa1976bad98ad0d8f2cd
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
- name: Generación de touchpoints
  intent: Escribir reglas IDE y workflows GitHub de forma idempotente donde aplique.
  requires_capability:
  - id: fs:persist
    contract: fs.persist
    version: '>=1.0.0'
- name: Verificación cruzada
  intent: Validar que los touchpoints referencian rutas SSOT coherentes.
  delegates_to:
  - agent:cumulo
- name: Snapshot
  intent: Consolidar cambios con sync git ciego según políticas.
  requires_capability:
  - id: proc:git-sync
    contract: proc.git_sync
    version: '>=1.0.0'
minteo_maximo: null
porcentaje_de_exito: null
---

# sddia-difusion

Proceso que cubre la **deuda** registrada al purgar la antigua acción de difusión: materializa la propagación controlada del Core SddIA hacia `.cursor/rules`, `.github` y homólogos, sin violar la frontera Acciones/Procesos.
