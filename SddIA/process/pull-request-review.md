---
uuid: 6d59f23b-df29-4be5-9bb9-29cede3474b9
name: pull-request-review
version: 1.0.0
contract: process-contract v1.3.0
context:
- quality-assurance
- source-control
hash_signature: sha256:660853a9c494625b0b10ae8579307a0d630d6476e7103bfdb33e87fd497ecac9
inputs:
- pr_id_or_path: Identificador o ruta lógica del PR
- pr_branch: Rama asociada al PR
- code_diff: Diff o referencia al cambio bajo revisión
- tasks_path: Ruta de tareas semillas resuelta vía Cumulo
- document_context: Contexto documental opcional (normas, ADRs)
outputs:
- validacion.md: Informe estructurado de revisión
- verdict: aprobado | requiere_cambios | rechazado
- kaizen_seeds: Semillas Kaizen persistidas bajo tasks_path
phases:
- name: Preparación de rama
  intent: Alinear checkout, fetch y estado limpio para inspección reproducible.
  delegates_to:
  - skill:git-manager
- name: Escrutinio de arquitectura
  intent: Evaluar coherencia estructural y de diseño (absorbe rol legacy architect).
  delegates_to:
  - agent:dedalo
- name: Escrutinio de calidad y seguridad
  intent: Evaluar calidad, riesgos y controles de seguridad (absorbe qa-judge y security-engineer).
  delegates_to:
  - agent:argos
- name: Veredicto consensuado
  intent: Sintetizar dictamen único y trazable.
  delegates_to:
  - agent:argos
- name: Persistencia y Kaizen
  intent: Materializar validacion.md y semillas de mejora continua en tasks_path.
  delegates_to:
  - skill:filesystem-manager
minteo_maximo: null
porcentaje_de_exito: null
---

# pull-request-review

Proceso V5 que **transmuta** el legacy `validate-pull-requests`. Los tres escrutinios históricos (`architect`, `qa-judge`, `security-engineer`) quedan **absorbidos** por **Dedalo** (arquitectura) y **Argos** (calidad + seguridad + veredicto).

## Kaizen

La persistencia final debe enlazar hallazgos accionables con el sistema de tareas bajo `tasks_path`, sin romper el SSOT de normas ni contratos.
