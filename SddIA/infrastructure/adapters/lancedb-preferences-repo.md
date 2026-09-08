---
id: lancedb-preferences-repo
uuid: "4c0103c3-5683-4219-aafd-1370c2b63c69"
type: infrastructure-adapter
version: "1.0.0"
status: active
crate_name: sddia-infrastructure-lancedb-preferences
impl_dir: lancedb_preferences_repo
contract: "adapters-contract v1.0.0"
port: "UserPreferenceStore"
context: ecosystem-evolution
---

# Adaptador: lancedb-preferences-repo

Implementación host del puerto `UserPreferenceStore` (`SddIA/user-preference-core/`) sobre LanceDB nativo.

## Estado

**Active.** Tabla `user_preferences` en URI `{paths.vectorStore}/lancedb/`. Upsert idempotente por `revision_id`. Lectura por filtro, `get_active` y KNN. JSON permanece SSOT de escritura; este adaptador es réplica + superficie vectorial. Target `wasm32-wasip1` no soportado. Compilación exige `protoc`.

## Delivery

| Campo | Valor |
|-------|-------|
| Crate | `sddia-infrastructure-lancedb-preferences` |
| Directorio | `SddIA/infrastructure/adapters/lancedb_preferences_repo/` |
| Substrato | Host nativo (no WASI) |
| Migración | `migrate_json_to_lancedb` (explícita, no borra JSON) |
