---
feature_name: boveda-evolucion-epigenetica
created: "2024-06-04"
process: feature
---

# Implementación Técnica — Bóveda de Evolución Epigenética

## 1. Entidades de Dominio (Core)

Se implementó el dominio base en `SddIA/core/memory/src/models/evolution_node.rs`:
- **`SpatialPolarity`**: Enum (`EfficientSymmetry`, `StructuralFracture`) para categorizar las trazas.
- **`EvolutionEvent`**: Estructura que almacena el payload original del evento, metadatos operativos y la polaridad asignada.

Se implementaron funciones auxiliares para generar el SHA-256 determinista basado en el contenido del evento (sin entropía aleatoria).

## 2. Adaptador de Almacenamiento Vectorial (LanceDB)

Se forjó el nuevo adaptador en `SddIA/infrastructure/adapters/lancedb_evolution_repo/`:
- **Tecnología**: Rust nativo apuntando a `wasm32-wasip1`.
- **Ruta Fija**: Aislamiento estricto en `.SddIA/vector_store/evolution/`.
- **Estructura**: Mapeo y persistencia de las entidades `EvolutionEvent` asegurando que la clave primaria `id` es un hash SHA-256.

## 3. Aduana Semántica y Proxy

Se diseñó la integración para la inferencia semántica local (All-MiniLM-L6-v2) a fin de extraer los vectores densos de cada evento antes de su persistencia, preservando la inmutabilidad de los datos. Se han establecido los cimientos para el Proxy de Captura Pasiva encargado de leer los eventos emitidos hacia `.SddIA/events/`.
