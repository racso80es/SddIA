---
contract_ref: paths.processPath/process-contract.json
name: Create Principle
persist_ref: paths.principlesPath/<principle-id>
process_id: create-principle
related_skills:
  - git-workspace-recon
  - git-branch-manager
  - git-save-snapshot
  - git-sync-remote
  - git-tactical-retreat
  - git-create-pr
phases:
  - description: Ejecutar git-workspace-recon para validar entorno limpio. Crear rama de trabajo (p. ej. feat/principle-<principle-id>) con git-branch-manager antes de crear artefactos en paths.principlesPath.
    id: '0'
    name: Preparar entorno
  - description: Elegir principle_id, crear carpeta, redactar spec.md y spec.json según principles-contract.
    id: '1'
    name: Definición
  - description: Verificar cumplimiento del contrato; validar blocking_for_pr y checks de validate si aplica. Durante redacción, consolidar hitos con git-save-snapshot. Ante fallo estructural, git-tactical-retreat solo con confirmación explícita.
    id: '2'
    name: Validación
  - description: Cierre. git-sync-remote; git-create-pr inyectando resumen del principio y rutas bajo paths.principlesPath en el cuerpo del Pull Request.
    id: '3'
    name: Finalizar
spec_version: 2.0.0
---

# Proceso: Creación de Principio (create-principle) (spec_version 2.0.0)

Este documento define el **proceso de tarea** para añadir nuevos principios técnicos a `paths.principlesPath` (SddIA/principles/), con **Arsenal Táctico Git (S+)**: `git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr`.

## Objetivo

Estandarizar la incorporación de principios (guías técnicas, normas de nomenclatura, etc.), asegurando que cada entrada cumpla el contrato de principios y sea utilizable por Arquitecto, Tekton, Cúmulo y, si aplica, por la validación pre-PR.

## Alcance

- **Ubicación:** paths.principlesPath (Cúmulo) = SddIA/principles/
- **Estructura:** Carpeta `<principle-id>` en kebab-case.
- **Contenido:** spec.md (legible) y spec.json (metadatos). Ambos obligatorios según principles-contract.

## Fases del Proceso

### 0. Preparar entorno

**git-workspace-recon** y **git-branch-manager** (rama dedicada recomendada) antes de crear carpetas bajo paths.principlesPath.

### 1. Definición

1. **Elegir principle_id:** Identificador en kebab-case (ej. `nomenclatura`, `norma-nomenclatura`).
2. **Crear carpeta:** paths.principlesPath/\<principle-id\>/.
3. **Redactar spec.md:**
   - Título, categoría, resumen y objetivo.
   - Contenido del principio (reglas, criterios).
   - Aplicación para Arquitecto y Tekton (y, si aplica, para Cúmulo o QA Judge).
   - Referencias.
   - Idioma: español (es-ES).
   - Pie: *Definición en paths.principlesPath/\<principle-id\>/ (contrato paths.principlesPath/principles-contract.md).*
4. **Crear spec.json:**
   - **id:** UUID v4 del principio.
   - **principle_id:** mismo que el nombre de la carpeta (kebab-case).
   - **title:** Título legible.
   - **category:** Categoría (Clean Code, Principios SOLID, Normas SddIA, etc.).
   - **tags:** Lista de etiquetas.
   - **metadata:** difficulty (Beginner | Intermediate | Advanced), status (Draft | Published | Deprecated).
   - **contract_ref:** paths.principlesPath/principles-contract.json.
   - **blocking_for_pr:** (opcional) true si el PR no debe aprobarse si no se cumple este principio. La acción validate debe incluir un check para este principio.
   - **defined_by_agent:** (opcional) Agente que define/custodia el principio (ej. cumulo).

Tras hitos de definición, **git-save-snapshot** (commits atómicos).

### 2. Validación

1. Verificar que spec.json cumple SddIA/principles/principles-contract.json.
2. Asegurar que spec.md tiene formato claro y contenido útil.
3. Si **blocking_for_pr** es true: confirmar que la acción validate (paths.actionsPath/validate/) incluye la comprobación de este principio y que el resultado es bloqueante (global fail / blocking).

### 3. Finalizar

**git-sync-remote** y **git-create-pr** con resumen del principio y enlaces a spec.md en el cuerpo del PR.

## Principio bloqueante para PR

Si el principio debe impedir que un PR pase cuando no se cumple (ej. norma de nomenclatura):

- En spec.json: `"blocking_for_pr": true`.
- En spec.md: indicar explícitamente que *Un PR no debe aprobarse si no se cumple esta norma*.
- La acción **validate** debe tener un check (ej. `nomenclatura` o `principle_<principle-id>`) que falle cuando se incumpla y que figure como obligatorio/blocking en validacion.md (frontmatter + cuerpo).

## Artefactos

- paths.principlesPath/\<principle-id\>/spec.md
- paths.principlesPath/\<principle-id\>/spec.json

## Referencia

- Contrato: SddIA/principles/principles-contract.json y principles-contract.md.
- Cúmulo: paths.principlesPath.
