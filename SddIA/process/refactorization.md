---
uuid: ae01e3ff-af68-4b94-90b3-97e5c03d75ee
name: refactorization
version: 1.0.0
contract: process-contract v1.3.0
context:
- ecosystem-evolution
- filesystem-ops
hash_signature: sha256:8eb0a6da06fdc6b49d759d63f63d3d7c89125941b7a68f04e7ce0548703b3182
inputs:
- refactor_goal: Objetivo estructural (legibilidad, modularización, deuda técnica)
    sin nueva funcionalidad
- refined_constraints: Restricciones de compatibilidad y alcance acordadas con Mayeuta
    cuando aplique
- cumulo_topology: Topología SSOT inyectada
- persist_ref: Carpeta de tarea
- branch_name: Rama git activa
outputs:
- refactor_artifacts: Cambios estructurales y pruebas de regresión
- verification_report: Salida de Argos
- pr_url: URL de PR tras cierre
phases:
- name: Estabilización de alcance
  intent: Fijar límites del refactor y superficies tocadas (Mayeuta cuando el alcance
    sea ambiguo).
  delegates_to:
  - agent:mayeuta
- name: Diseño de refactor
  intent: Blueprint de reorganización segura y migraciones incrementales.
  delegates_to:
  - agent:dedalo
- name: Ejecución
  intent: Aplicar cambios estructurales con Tekton respetando SSOT.
  delegates_to:
  - agent:tekton
- name: Verificación
  intent: Argos valida invariantes, contratos y calidad.
  delegates_to:
  - agent:argos
- name: Cierre de entrega
  intent: 'Invocar delivery-close-cycle con source_process: refactorization.'
  delegates_to:
  - action:execute-process
minteo_maximo: null
porcentaje_de_exito: null
---

# refactorization

Proceso V5 para **refactorización** con **process_id canónico preservado** (`refactorization`). Paralelo a `feature`, enfatiza cambios **sin nueva capacidad funcional** y cierra con **`delivery-close-cycle`** (`source_process: refactorization`).
