---
contract_ref: paths.processPath/process-contract.json
inputs:
  description: Descripción breve del fin concreto. Obligatorio.
  process_ref: Proceso que orquesta (feature, correccion-auditorias, etc.). Obligatorio.
  template_id: kebab-case. Obligatorio.
name: Create Template
paths:
  featurePath_ref: paths.featurePath (Cúmulo)
  processPath_ref: paths.processPath (Cúmulo)
  templatesPath_ref: paths.templatesPath (Cúmulo)
persist_ref: paths.featurePath/create-template-<template-id>
phases:
  - description: Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, crear rama feat/create-template-<template-id> desde master usando git-branch-manager.
    id: '0'
    name: Preparar entorno
  - description: Objetivos y spec de la plantilla (objectives.md, spec.md).
    id: '1'
    name: Objetivos y especificación
  - description: Redactar entregable en paths.templatesPath; consolidar hitos con git-save-snapshot. Ante fallo estructural, git-tactical-retreat (con confirmación explícita).
    id: '2'
    name: Implementación de plantilla
  - description: Validación frente a templates-contract.
    id: '3'
    name: Validar
  - description: Cierre. git-sync-remote; git-create-pr inyectando objectives.md y validacion.md en el cuerpo del PR.
    id: '4'
    name: Finalizar
process_doc_ref: paths.processPath/create-template/
process_id: create-template
process_interface_compliance: 'Genera en carpeta de la tarea un .md por acción con YAML Frontmatter (objectives.md, spec.md, validacion.md); no ficheros .json separados. Entregable: carpeta en paths.templatesPath/<template-id>/ con spec.md (YAML Frontmatter). Norma: features-documentation-frontmatter.md.'
related_actions:
  - spec
  - validate
  - finalize-process
related_skills:
  - git-workspace-recon
  - git-branch-manager
  - git-save-snapshot
  - git-sync-remote
  - git-tactical-retreat
  - git-create-pr
  - documentation
spec_version: 2.0.0
templates_contract_ref: SddIA/templates/templates-contract.json
triggers:
  - Crear nueva plantilla en paths.templatesPath
  - Solicitud de creación de plantilla con template-id
---
# Proceso: Creación de plantillas (create-template) (spec_version 2.0.0)

Este documento define el **proceso de tarea** para crear una nueva plantilla (template) (**spec_version 2.0.0**), con **Arsenal Táctico Git (S+)**: `git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr`. Ubicación: paths.processPath/create-template/ (Cúmulo). Rutas: **Cúmulo** (paths.templatesPath).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): la tarea de creación genera en la carpeta de la tarea (Cúmulo) un **`.md` por acción** con **YAML Frontmatter** (objectives.md, spec.md, validacion.md). No ficheros .json separados. El **resultado** es la carpeta en **paths.templatesPath/<template-id>/** con spec.md (YAML Frontmatter), cumpliendo SddIA/templates/templates-contract. Norma: SddIA/norms/features-documentation-frontmatter.md.

## Propósito

El proceso **create-template** define el procedimiento para incorporar una nueva plantilla al ecosistema de paths.templatesPath (Cúmulo): desde la definición del objetivo hasta la plantilla lista con spec.md y spec.json. Garantiza que cada plantilla cumpla SddIA/templates/templates-contract.json y que quede registrada en Cúmulo (paths.templatesPath). Las plantillas procedimentan el uso de procesos mediante configuraciones predefinidas con un fin concreto.

## Alcance del procedimiento

- **Documentación de la tarea:** Cúmulo (paths.featurePath/create-template-<template-id>/).
- **Entregable:** paths.templatesPath/<template-id>/ con spec.md y spec.json (y opcionalmente config.json).

**Flujo Git (S+):** Fase 0 — **git-workspace-recon** y **git-branch-manager**. Implementación — **git-save-snapshot**; emergencia — **git-tactical-retreat** (confirmación). Cierre — **git-sync-remote** y **git-create-pr** (objectives + validacion en el PR).

Fases: 0 Preparar entorno | 1 Objetivos y especificación de la plantilla | 2 Redactar spec.md y spec.json según templates-contract | 3 Validación | 4 Cierre (sync + PR).

## Restricciones

- template_id en kebab-case. Rama feat/create-template-<template-id>. Windows 11, PowerShell 7+. Contrato templates (SddIA/templates/templates-contract.json) obligatorio. process_ref debe existir en paths.processPath.

## Referencias

- Contrato: SddIA/templates/templates-contract.json, templates-contract.md.
- Cúmulo: paths.templatesPath, paths.featurePath, paths.processPath.
- Proceso machine-readable: paths.processPath/create-template/spec.json.
