---
feature_name: grafo-pensamiento
created: "2026-06-04"
process: feature
branch_name: feat/grafo-pensamiento
persist_ref: docs/features/grafo-pensamiento
---

# Objetivos — grafo-pensamiento

## Misión

Implementar PBI `docs/todos/kitchen/Grafo de Pensamiento.md`: Instanciación del Contrapoder Interno (Grafo de Pensamiento Espacial) - v2.

## Alcance (manifiesto)

Dotar al Nodo de Control de un contrapoder interno independiente mediante un **Grafo de Pensamiento Espacial** persistido en la Base de Datos Vectorial (LanceDB). Esto incluye:
- Triaje Predictivo Vectorial (Paso 0) para poda instántanea usando embeddings.
- Borrador Ciego para aislar la ramificación del output del Vértice Biológico.
- Autopoiesis (Retroceso Autónomo) ante colisiones lógicas o éticas.
- Telemetría de Fricción Vectorizada para registrar pensamientos descartados.
- Estructura `ThoughtNode` en Rust integrada con LanceDB.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Inferencia Nativa local estricta en Rust (sin llamadas a APIs externas como OpenAI).
- Implementación hexagonal (puertos/adaptadores en WASI).
- Sin explicaciones verbosas.
