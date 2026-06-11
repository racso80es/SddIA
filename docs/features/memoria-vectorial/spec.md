---
feature_name: memoria-vectorial
created: "2026-06-11"
process: feature
branch_name: feat/memoria-vectorial
persist_ref: docs/features/memoria-vectorial
---

# Especificación Técnica — Memoria Vectorial

## 1. Arquitectura

Subsistema de memoria semántica agnóstica del proyecto, implementado en `SddIA/core/memory/` bajo arquitectura hexagonal.

### 1.1 Entidad `KnowledgeChunk`

| Campo | Tipo | Regla |
|-------|------|-------|
| `id` | String | Hash SHA-256 del contenido; prohibido UUID aleatorio |
| `original_source` | String | Ruta lógica del activo origen |
| `text_content` | String | Fragmento textual puro |
| `metadata` | JSON | Etiquetas agnósticas (capacidad, estatus) |
| `embedding` | `Option<Vec<f32>>` | Vector denso semántico |

### 1.2 Puertos (Traits)

- **`EmbeddingGenerator`**: genera embeddings locales (All-MiniLM-L6-v2) sin APIs externas.
- **`VectorStore`**: `store_chunk`, `search_similar` (KNN).

## 2. Almacenamiento físico

- **Motor:** LanceDB incrustado.
- **Ruta:** `.SddIA/vector_store/` (subíndices por dominio: `evolution/`, colección de grafo).
- **Target:** `wasm32-wasip1` (Rust nativo).

## 3. Integración downstream

| Consumidor | Uso |
|------------|-----|
| Bóveda Epigenética | `EvolutionEvent` → subíndice `evolution/` |
| Grafo de Pensamiento | `ThoughtNode` → colección `thought_graph_collection` |

## 4. Eventos ECST

Tras indexación vectorial exitosa se emite **`Vector_Memory_Indexed`** (payload: `record_id`, `store_path`, `record_class`).
