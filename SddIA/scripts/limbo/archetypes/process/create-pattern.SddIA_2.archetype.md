---
contract_ref: paths.processPath/process-contract.json
name: Create Pattern
persist_ref: SddIA/patterns/<uuid>
process_id: create-pattern
related_skills:
  - git-workspace-recon
  - git-branch-manager
  - git-save-snapshot
  - git-sync-remote
  - git-tactical-retreat
  - git-create-pr
phases:
  - description: Ejecutar git-workspace-recon para validar entorno limpio. Crear rama de trabajo (p. ej. feat/pattern-<uuid-corto>) con git-branch-manager antes de añadir entradas bajo SddIA/patterns/.
    id: '0'
    name: Preparar entorno
  - description: Generar UUID, crear carpeta, redactar spec.md y spec.json.
    id: '1'
    name: Definición
  - description: Verificar patterns-contract.md. Consolidar hitos con git-save-snapshot. Ante fallo estructural, git-tactical-retreat solo con confirmación explícita.
    id: '2'
    name: Validación
  - description: Cierre. git-sync-remote; git-create-pr inyectando resumen del patrón y ruta SddIA/patterns/<uuid> en el cuerpo del Pull Request.
    id: '3'
    name: Finalizar
spec_version: 2.0.0
---

# Proceso: Creación de Patrones (create-pattern) (spec_version 2.0.0)

Este documento define el **proceso de tarea** para añadir nuevos patrones de diseño y arquitectura a `SddIA/patterns/`, con **Arsenal Táctico Git (S+)**: `git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr`.

## Objetivo

Estandarizar la incorporación de conocimiento sobre patrones, asegurando que cada entrada sea completa, legible y utilizable por los agentes (especialmente Arquitectos y Tekton).

## Alcance

- **Ubicación:** `SddIA/patterns/`
- **Estructura:** Carpeta nombrada con UUID.
- **Contenido:** `spec.md` (legible) y `spec.json` (metadatos).

## Fases del Proceso

### 0. Preparar entorno

**git-workspace-recon** y **git-branch-manager** antes de crear la carpeta del patrón.

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

Usar **git-save-snapshot** para consolidar la definición.

### 2. Validación

1.  Verificar que `spec.json` cumple con `SddIA/patterns/patterns-contract.json`.
2.  Asegurar que `spec.md` tiene un formato claro y contenido útil.

### 3. Finalizar

**git-sync-remote** y **git-create-pr** con resumen y enlace al patrón en el cuerpo del PR.

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
