# Clarificación — grafo-pensamiento

## Entendimiento del Problema

El objetivo es establecer la arquitectura inicial y las definiciones del "Grafo de Pensamiento Espacial", una capa cognitiva en la cual el LLM puede evaluar rutas hipotéticas de resolución antes de emitirlas como respuesta final. Se busca persistir estas ramas (nodos) de pensamiento en la Base de Datos Vectorial (LanceDB) utilizando un motor local nativo en Rust, evitando el secuestro semántico y permitiendo autoevaluación y corrección.

## Incógnitas Resueltas

- **Persistencia**: LanceDB (`thought_graph_collection`).
- **Lenguaje/Entorno**: Rust nativo, target `wasm32-wasip1`.
- **Estructura de Datos**: `ThoughtNode` derivando características de `KnowledgeChunk`.
- **Conectividad**: Navegación dual por jerarquía estricta (`parent_id`) y proximidad semántica (KNN).

## Restricciones

- Sin dependencias externas de inferencia.
- No se emitirá salida al Vértice Biológico durante el proceso de triaje.
- Se debe asegurar el "retroceso autónomo" a `parent_id` ante violaciones de filtros lógicos/éticos.
