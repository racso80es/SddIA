---
uuid: de142ec3-4022-4ac1-bcf4-1b8490cabf9d
name: sddia-difusion
version: 1.0.0
contract: process-contract v1.3.0
context:
- ecosystem-evolution
hash_signature: sha256:c19c0b7f31b41bb22f6db89a9992a0aa78dab0f9322ccd0034e604e3e8d52b5f
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
  delegates_to:
  - skill:filesystem-manager
- name: Verificación cruzada
  intent: Validar que los touchpoints referencian rutas SSOT coherentes.
  delegates_to:
  - agent:cumulo
- name: Snapshot
  intent: Consolidar cambios con git-manager según políticas.
  delegates_to:
  - skill:git-manager
minteo_maximo: null
porcentaje_de_exito: null
---

# sddia-difusion

Proceso que cubre la **deuda** registrada al purgar la antigua acción de difusión: materializa la propagación controlada del Core SddIA hacia `.cursor/rules`, `.github` y homólogos, sin violar la frontera Acciones/Procesos.
