---
contract_ref: paths.processPath/process-contract.json
name: Create Pattern
persist_ref: paths.patternsPath/<uuid>
process_id: create-pattern
related_skills:
  - git-workspace-recon
  - git-branch-manager
  - git-save-snapshot
  - git-sync-remote
  - git-tactical-retreat
  - git-create-pr
spec_version: 2.0.0
phases:
  - description: >-
      Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, usar git-branch-manager para aislar el
      contexto en una rama de trabajo (p. ej. feat/create-pattern-<slug>) si la incorporación del patrón se versiona vía PR.
    id: '0'
    name: Preparar entorno
  - description: Definición del patrón (UUID, carpeta, spec.md, spec.json) según secciones siguientes.
    id: '1'
    name: Definición
  - description: >-
      Validación contractual. Durante la edición, consolidar hitos con git-save-snapshot. Ante fallo estructural,
      valorar git-tactical-retreat según política.
    id: '2'
    name: Validación
  - description: >-
      Cierre. git-sync-remote y git-create-pr incorporando la ruta paths.patternsPath/<uuid> y el resumen del patrón en el cuerpo del Pull Request.
    id: '3'
    name: Finalizar
---

# Proceso: Creación de Patrones (create-pattern)

Este documento define el **proceso** para añadir nuevos patrones de diseño y arquitectura a `SddIA/patterns/` (paths.patternsPath, Cúmulo).

## Objetivo

Estandarizar la incorporación de conocimiento sobre patrones, asegurando que cada entrada sea completa, legible y utilizable por los agentes (especialmente Arquitectos y Tekton).

## Alcance

- **Ubicación:** `SddIA/patterns/`
- **Estructura:** Carpeta nombrada con UUID.
- **Contenido:** `spec.md` (legible) y `spec.json` (metadatos).

## Fases del Proceso

### 0. Preparar entorno

1. **git-workspace-recon** (entorno limpio).
2. **git-branch-manager** para rama de trabajo si el cambio se integra vía PR (recomendado).

### 1. Definición

1.  **Generar UUID:** Cada patrón debe tener un identificador único (UUID v4).
2.  **Crear Carpeta:** Crear `SddIA/patterns/<uuid>/`.
3.  **Redactar `spec.md`:**
    *   Título y descripción detallada en Markdown.
    *   Idioma: Español (es-ES).
    *   Debe incluir referencias y contexto de uso.
4.  **Definir `spec.json`:**
    *   `id`: UUID del patrón.
    *   `title`: Título del patrón.
    *   `category`: Categoría (e.g., Arquitectura, Diseño).
    *   `tags`: Lista de etiquetas.
    *   `metadata`: Dificultad, estado, etc.
    *   `interested_agents`: Lista de agentes que deben conocer este patrón (mapeado desde la categoría).

### 2. Validación

1.  Verificar que `spec.json` cumple con `SddIA/patterns/patterns-contract.json`.
2.  Asegurar que `spec.md` tiene un formato claro y contenido útil.
3.  Consolidar hitos con **git-save-snapshot**; **git-tactical-retreat** solo ante fallo estructural y con confirmación.

### 3. Cierre

1. **git-sync-remote** para publicar la rama.
2. **git-create-pr** con referencia a `paths.patternsPath/<uuid>/` y resumen del patrón en el cuerpo del PR.

## Mapeo de Agentes Interesados

| Categoría | Agentes Sugeridos |
| :--- | :--- |
| Arquitectura de Software | `architect`, `infrastructure-architect` |
| Patrones de Diseño | `tekton-developer`, `architect` |
| Sistemas Distribuidos | `architect`, `infrastructure-architect`, `performance-engineer` |
| Domain-Driven Design | `architect`, `clarifier` |
| *Default* | `architect`, `tekton-developer` |

## Artefactos

*   `SddIA/patterns/<uuid>/spec.md`
*   `SddIA/patterns/<uuid>/spec.json`
