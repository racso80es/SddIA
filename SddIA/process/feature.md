---
uuid: 1b4fa69f-4299-47ca-b2ed-380f2263239c
name: feature
version: 1.0.0
contract: process-contract v1.3.0
context:
- ecosystem-evolution
- filesystem-ops
hash_signature: sha256:04baef580b1af1c2d1bd4e3943a6e4d1b3c442234b37165225fd1be9f6404492
inputs:
- feature_name: Nombre kebab-case o etiqueta humana de la feature
- refined_requirements: Requisitos refinados producidos por Mayeuta
- cumulo_topology: Topología SSOT inyectada (paths, contratos, directorios)
- persist_ref: Carpeta de tarea para artefactos y trazabilidad
- branch_name: Rama git activa para la entrega
outputs:
- delivery_artifacts: Artefactos de código y documentación acordados
- verification_report: Resultado agregado de Argos
- pr_url: URL de PR tras cierre (vía delivery-close-cycle)
phases:
- name: Estabilización de Requisitos
  intent: Congelar alcance, criterios de aceptación y riesgos antes del diseño.
  delegates_to:
  - agent:mayeuta
- name: Diseño de Blueprint
  intent: Producir blueprint técnico alineado al SSOT y políticas de contexto.
  delegates_to:
  - agent:dedalo
- name: Ejecución
  intent: Materializar cambios en el repositorio sin terminal nativa prohibida; obedecer
    topología cumulo.
  delegates_to:
  - agent:tekton
- name: Verificación
  intent: Validar calidad, coherencia contractual y regresiones relevantes.
  delegates_to:
  - agent:argos
- name: Cierre de entrega
  intent: 'Invocar delivery-close-cycle con source_process: feature y parámetros de
    persistencia/rama.'
  delegates_to:
  - action:execute-process
minteo_maximo: null
porcentaje_de_exito: null
---

# feature

Proceso V5 para desarrollo de **features**: cadena **Mayeuta → Dedalo → Tekton → Argos → delivery-close-cycle** (`source_process: feature`). Sustituye el linaje legacy basado en fases `spec/clarify/planning/...` purgadas.

## Cierre

La última fase delega en **`action:execute-process`** con `process_name` resuelto al canónico **`delivery-close-cycle`** y `process_inputs` armados desde `persist_ref`, `branch_name` y `source_process: feature`. La resolución de identidad (incl. aliases) es previa a la ruta física (`process-contract v1.3.0`).
