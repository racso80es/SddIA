---
id: lancedb-evolution-repo
uuid: "ab9bef02-c2c1-426b-a2b2-ca1cc170f21c"
type: infrastructure-adapter
version: "1.1.0"
status: active
crate_name: sddia-infrastructure-lancedb-evolution
impl_dir: lancedb_evolution_repo
contract: "adapters-contract v1.0.0"
port: "EvolutionStore"
context: ecosystem-evolution
---

# Adaptador: lancedb-evolution-repo

Implementación host del puerto `EvolutionStore` (`SddIA/core/memory/`) sobre LanceDB nativo.

## Estado

**Active.** Tabla `evolution` en URI `{paths.vectorStore}/lancedb/`. Upsert idempotente por `id`. Lectura por id y KNN. Sin JSON como SSOT. Target `wasm32-wasip1` no soportado. Compilación exige `protoc`.

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `sddia-infrastructure-lancedb-evolution` |
| Directorio | `SddIA/infrastructure/adapters/lancedb_evolution_repo/` |
| Substrato | Host nativo (no WASI) |
| Importador legado | `import_legacy_evolution_json` (explícito, no automático) |
