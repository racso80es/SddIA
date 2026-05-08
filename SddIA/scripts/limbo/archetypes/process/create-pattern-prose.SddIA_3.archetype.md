# Proceso: Creaci├│n de Patrones (create-pattern)

Este documento define el **proceso** para a├▒adir nuevos patrones de dise├▒o y arquitectura a `SddIA/patterns/`.

## Objetivo

Estandarizar la incorporaci├│n de conocimiento sobre patrones, asegurando que cada entrada sea completa, legible y utilizable por los agentes (especialmente Arquitectos y Tekton).

## Alcance

- **Ubicaci├│n:** `SddIA/patterns/`
- **Estructura:** Carpeta nombrada con UUID.
- **Contenido:** `spec.md` (legible) y `spec.json` (metadatos).

## Fases del Proceso

### 1. Definici├│n

1.  **Generar UUID:** Cada patr├│n debe tener un identificador ├║nico (UUID v4).
2.  **Crear Carpeta:** Crear `SddIA/patterns/<uuid>/`.
3.  **Redactar `spec.md`:**
    *   T├¡tulo y descripci├│n detallada en Markdown.
    *   Idioma: Espa├▒ol (es-ES).
    *   Debe incluir referencias y contexto de uso.
4.  **Definir `spec.json`:**
    *   `id`: UUID del patr├│n.
    *   `title`: T├¡tulo del patr├│n.
    *   `category`: Categor├¡a (e.g., Arquitectura, Dise├▒o).
    *   `tags`: Lista de etiquetas.
    *   `metadata`: Dificultad, estado, etc.
    *   `interested_agents`: Lista de agentes que deben conocer este patr├│n (mapeado desde la categor├¡a).

### 2. Validaci├│n

1.  Verificar que `spec.json` cumple con `SddIA/patterns/patterns-contract.json`.
2.  Asegurar que `spec.md` tiene un formato claro y contenido ├║til.

## Mapeo de Agentes Interesados

| Categor├¡a | Agentes Sugeridos |
| :--- | :--- |
| Arquitectura de Software | `architect`, `infrastructure-architect` |
| Patrones de Dise├▒o | `tekton-developer`, `architect` |
| Sistemas Distribuidos | `architect`, `infrastructure-architect`, `performance-engineer` |
| Domain-Driven Design | `architect`, `clarifier` |
| *Default* | `architect`, `tekton-developer` |

## Artefactos

*   `SddIA/patterns/<uuid>/spec.md`
*   `SddIA/patterns/<uuid>/spec.json`
