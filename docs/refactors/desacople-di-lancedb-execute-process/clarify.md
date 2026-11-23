---
feature_name: desacople-di-lancedb-execute-process
created: "2026-11-23"
process: refactorization
branch_name: refactor/desacople-di-lancedb-execute-process
persist_ref: docs/refactors/desacople-di-lancedb-execute-process
tracker_ref: AUT-119
purpose: Clarificaciones y decisiones de arquitectura para desacople DI de LanceDB
---
# Clarificación — desacople-di-lancedb-execute-process (`AUT-119`)

- **Decisión Opción A (Satélite Nativo Unificado):** Se unifican los tres dominios vectoriales (`evolution`, `preferences`, `thought`) en un único binario nativo `tool:lancedb-memory-adapter` (`SddIA/tools/lancedb-memory-adapter/`), evitando triplicar el binario de ~264 MB (~792 MB) y preservando `skill:user-preference-store` como cápsula WASI ultraligera (JSON) para `memory:pref-write` y `memory:pref-query`.
- **Sin feature flags en `execute-process`:** Se eliminan por completo `sddia-infrastructure-lancedb-evolution` y `sddia-infrastructure-lancedb-preferences` de `SddIA/engine/execute-process/Cargo.toml`. `execute-process` queda libre de `lancedb`, `lance`, `arrow`, `datafusion` y `tokio`.
- **Embeddings encapsulados:** El cálculo de embeddings (`LocalHashingEmbedder`, 384 dims) se encapsula en el proveedor satélite; `execute-process` envía payload/texto lógico sin serializar vectores crudos por JSON `stdin`.
- **Purga de acoplamiento oculto a Redb:** `lancedb_thought_repo` deja de depender de `sddia-event-manager-host`; el evento `Thought_Persisted` pasa a emitirse desde la capa orquestadora (`execute-process` / `handlers/aiua_stimulus.rs`) tras confirmación `success: true` del satélite.
- **Fast-check en `handlers/user_preference.rs`:** Antes de invocar al satélite para preferencias vectoriales, `execute-process` verifica en filesystem la existencia del directorio de tabla `user_preferences.lance` bajo la ruta SSOT de `cumulo.paths.json`; si no existe, degrada directamente a `skill:user-preference-store` / `user-preference-core` sin spawn innecesario.
- **Fuera de alcance:** Sustituir LanceDB, migrar datos existentes en `.SddIA/vector_store/`, migrar `skill:user-preference-store` a `sddia-io` o configurar `sccache`.
