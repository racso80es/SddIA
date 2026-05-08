---
uuid: ac8d078c-9785-490b-9f43-ad310fe9df9d
name: bug-fix
version: 1.0.0
contract: process-contract v1.3.0
context:
- ecosystem-evolution
- filesystem-ops
hash_signature: sha256:490fc97d2dccee28d446c920ed4d3bca7e7879bc052a442951d28038559b6731
inputs:
- bug_summary: Descripción del defecto y pasos de reproducción (o triaje directo)
- cumulo_topology: Topología SSOT inyectada
- persist_ref: Carpeta de tarea
- branch_name: Rama git activa
outputs:
- fix_artifacts: Parches y pruebas asociadas
- verification_report: Salida de Argos
- pr_url: URL de PR tras cierre
phases:
- name: Diseño del fix
  intent: Acotar causa raíz y estrategia de corrección mínima viable.
  delegates_to:
  - agent:dedalo
- name: Ejecución
  intent: Implementar la corrección y ajustes de prueba/documentación necesarios.
  delegates_to:
  - agent:tekton
- name: Verificación
  intent: Validar ausencia de regresión y cumplimiento de criterios de calidad.
  delegates_to:
  - agent:argos
- name: Cierre de entrega
  intent: 'Invocar delivery-close-cycle con source_process: bug-fix.'
  delegates_to:
  - action:execute-process
minteo_maximo: null
porcentaje_de_exito: null
---

# bug-fix

Proceso V5 para **corrección de defectos** con **process_id canónico preservado** (`bug-fix`). Omite la fase pesada de requisitos cuando el triaje es directo; mantiene **Dedalo → Tekton → Argos → delivery-close-cycle** con `source_process: bug-fix`.
