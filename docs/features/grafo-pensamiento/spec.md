---
feature_name: grafo-pensamiento
created: "2026-06-11"
process: feature
branch_name: feat/grafo-pensamiento
persist_ref: docs/features/grafo-pensamiento
---

# Especificación Técnica — Grafo de Pensamiento Espacial

## 1. Propósito

Contrapoder interno del Nodo de Control: grafo de pensamiento persistido en LanceDB, desacoplado del Vértice Biológico.

## 2. Entidad `ThoughtNode`

| Campo | Regla |
|-------|-------|
| `node_id` | SHA-256(`parent_id` + `content` + `friction_trace`) |
| `parent_id` | Jerarquía de ramificación |
| `content` | Borrador ciego (sin fuga a terminal biológico) |
| `metadata` | `status`: `ACTIVE` \| `PRUNED` |
| `friction_trace` | Telemetría de fricción al podar |
| `embedding` | Vector semántico pre-inserción |

## 3. Servicios de dominio

### 3.1 Triaje Predictivo (Paso 0)

`ThoughtTriageService::evaluate_and_spawn`:
1. Genera embedding local.
2. KNN contra nodos `PRUNED` → rechazo temprano (secuestro semántico).
3. Persiste nodo `ACTIVE`.
4. Emite **`Thought_Persisted`**.

### 3.2 Autopoiesis

`ThoughtTriageService::prune_thought`:
- Marca `PRUNED`, guarda `friction_trace`.
- Retorna `parent_id` para retroceso autónomo.

## 4. Puerto `ThoughtGraphRepository`

Operaciones: `store_thought`, `get_thought_by_id`, `get_children`, `search_similar_thoughts`.

## 5. Adaptador físico

`LanceDbThoughtRepo` → colección `thought_graph_collection` bajo `.SddIA/vector_store/`.

## 6. Consideraciones

- Inferencia 100% local Rust/WASI.
- Cero llamadas OpenAI u APIs externas.
- Eventos ECST estándar SddIA en cada mutación persistida.
