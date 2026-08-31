---
id: lancedb-thought-repo
uuid: "0a22c260-2c5a-4aaa-a632-2c9a78e983e4"
type: infrastructure-adapter
version: "1.1.0"
status: active
crate_name: sddia-infrastructure-lancedb-thought
impl_dir: lancedb_thought_repo
contract: "adapters-contract v1.0.0"
port: "ThoughtGraphRepository"
context: ecosystem-evolution
---

# Adaptador: lancedb-thought-repo

Implementación host del puerto `ThoughtGraphRepository` (`SddIA/core/memory/`) sobre LanceDB nativo.

## Estado

**Active.** Tabla `thought_graph_collection` en URI `{paths.vectorStore}/lancedb/`. Store, get-by-id, hijos por `parent_id`, KNN. Target `wasm32-wasip1` no soportado. Compilación exige `protoc`.

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `sddia-infrastructure-lancedb-thought` |
| Directorio | `SddIA/infrastructure/adapters/lancedb_thought_repo/` |
| Substrato | Host nativo (no WASI) |
