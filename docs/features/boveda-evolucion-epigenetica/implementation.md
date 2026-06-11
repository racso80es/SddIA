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

- **`inference_binding.rs`**: integración para inferencia semántica local (All-MiniLM-L6-v2) y extracción de vectores densos previos a persistencia.
- **`evolution_proxy.rs`**: `EvolutionProxyService` — proxy de captura pasiva; triaje de polaridad vía metadata `success`; delega en `EvolutionStore`.

## 4. Matriz de archivos core (`SddIA/core/memory/`)

| Archivo | Rol |
|---------|-----|
| `src/models/evolution_node.rs` | `EvolutionEvent`, `SpatialPolarity`, id SHA-256 |
| `src/ports.rs` | Trait `EvolutionStore` |
| `src/services/evolution_proxy.rs` | Captura pasiva y triaje |
| `src/services/inference_binding.rs` | Binding semántico |
| `src/models/mod.rs` | Reexportes modelos |
| `src/services/mod.rs` | Reexportes servicios |
| `src/lib.rs` | API pública crate |
| `Cargo.toml` / `Cargo.lock` | Dependencias (`sha2`, `serde`, …) |

> Archivos coalescentes del grafo (`thought_node.rs`, `thought_triage.rs`) documentados en `docs/features/grafo-pensamiento/` y `docs/features/memoria-vectorial/`.
