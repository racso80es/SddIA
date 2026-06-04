# Plan de Ejecución — grafo-pensamiento

## Fases de Implementación (Para Iteración Posterior)

1. **Definición de Entidad de Dominio Central:**
   - Crear el modelo de datos `ThoughtNode` en la capa central (e.g. `SddIA/core/models/thought_node.rs`), garantizando que pueda ser seralizado y que posea los campos estipulados (`node_id`, `parent_id`, `content`, `metadata`, `friction_trace`, `embedding`).
   - El identificador `node_id` debe emplear _hashing_ determinista (SHA-256) sobre el contenido y metadatos, evitando usar identificadores puramente aleatorios como UUID.

2. **Desarrollo del Puerto (Trait) en el Core:**
   - Definir la interfaz (Trait) `ThoughtGraphRepository` en Rust para las operaciones requeridas sobre el grafo: guardar nodo, obtener por ID, obtener por `parent_id` y buscar por similitud semántica.
   - Definir los errores de dominio esperados devolviendo estructuras `Result<T, E>`.

3. **Implementación de la Memoria Espacial con LanceDB:**
   - Desarrollar el adaptador físico para LanceDB (e.g. en `SddIA/infrastructure/adapters/lancedb_thought_repo.rs`) que implemente el trait `ThoughtGraphRepository`.
   - Garantizar la creación de la colección `thought_graph_collection` si no existe.

4. **Integración de Inferencia Nativa:**
   - Conectar con el puerto `EmbeddingGenerator` de la memoria vectorial para calcular los embeddings de los `ThoughtNode` (utilizando modelos locales, e.g. all-MiniLM-L6-v2) previo a la inserción en la BDV.
   - Implementar la lógica del "Triaje Predictivo Vectorial" y "Retroceso Autónomo".

5. **Aislamiento y Eventos:**
   - Asegurarse de que durante el flujo de ramificación se disparen eventos (e.g. `Thought_Spawned`, `Thought_Pruned`) usando el formato estándar de Entidad de Dominio SddIA.
   - Confirmar el desacople estricto de la salida del terminal de manera que no hayan "fugas" de los nodos inmaduros.

6. **Auditoría (Argos):**
   - Escribir pruebas unitarias y de integración para validar la jerarquía, el guardado correcto, las colisiones semánticas y la serialización sin uso de elementos no deterministas.

*(Nota: Esta iteración en sí no implementa código de las fases antes mencionadas; únicamente establece este artefacto descriptivo).*
