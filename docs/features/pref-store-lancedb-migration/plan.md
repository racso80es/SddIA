---
feature_name: pref-store-lancedb-migration
created: "2026-09-08"
process: feature
phases:
  - L0-design-commit
  - L1-domain-trait
  - L2-lancedb-adapter
  - L3-migrate-parity
  - L4-execute-process-wire
  - L5-evolution-docs
  - L6-dcc-ci-accept
branch_name: feat/pref-store-lancedb-migration
persist_ref: docs/features/pref-store-lancedb-migration
pbi_ref: docs/todos/pending/[ARQUITECTURA] Migración del Store de Preferencias de Usuario a LanceDB.md
document_id: PBI-PREF-STORE-LANCEDB-MIGRATION
uuid: "c79e6f1a-821b-4d7a-9a84-0b1e32d56a77"
execution_id: "338b68e4-de96-4303-93a7-d0c0e40acb9c"
---

# Plan — pref-store-lancedb-migration

Corte Diseño: **clarify + objectives + spec + plan + commit**. Ejecución L1–L6 después.

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `338b68e4-de96-4303-93a7-d0c0e40acb9c`.

## L0 — Diseño (esta parada)

Artefactos bajo `persist_ref` + PBI v1.2.0. Commit de planificación **antes** de mutar crates.

## L1 — Dominio (CA-1, CA-2 parcial, CA-11)

`SddIA/user-preference-core/src/lib.rs`:

1. Campo `embedding` con serde default/skip.
2. Trait `UserPreferenceStore`.
3. `JsonUserPreferenceStore { repo: PathBuf }` implementando el trait; `get_active` sobre `head_index`.
4. `QuerySpec` + campos vectoriales ignorados.
5. Funciones libres intactas; tests existentes + `embedding_absent_deserializes_to_none` + `get_active_skips_revoked`.
6. Actualizar literales `UserPreference { ... }` en tests del crate y de `email_triage` / `user_preference` / `telegram_fallback` (campo nuevo o `..` si aplica). Preferir `embedding: None` explícito para no ocultar el contrato.

`cargo test -p user-preference-core --lib`

## L2 — Adaptador (CA-2, CA-3, CA-4, CA-5, CA-6, CA-7)

1. Crate `SddIA/infrastructure/adapters/lancedb_preferences_repo/` pin `lancedb = "=0.37.1"`, deps: `sddia-core-memory`, `user-preference-core`, `tokio`, `futures`, `serde_json`.
2. `LanceDbPreferenceAdapter`: `open`, `table_exists`, `ensure_table`, upsert, query filtro, knn, purge, `migrate_from_json(repo)`.
3. Ficha `lancedb-preferences-repo.md` uuid `4c0103c3-5683-4219-aafd-1370c2b63c69`.
4. Fila `index.md` + member `SddIA/Cargo.toml`.
5. Tests tempfile (patrón thought).

`cargo test -p sddia-infrastructure-lancedb-preferences --lib`

`protoc` en PATH (ya exigido por hermanos).

## L3 — Paridad (CA-8)

Test en el crate adaptador: poblar JSON via `user_preference_core::put_revision`, `migrate_from_json`, comparar `query` filtro (IDs, values, orden de scope). Assert ficheros JSON siguen en disco.

## L4 — Wiring host (CA-9)

`execute-process/Cargo.toml`: path-dep al crate nuevo.

`user_preference.rs`:

- `put_revision_durable` / `purge_durable` para ingest/`run_capsule`.
- Lectura: si `table_exists(lancedb_uri(repo))` usar adaptador; si error → bloque vacío; si no tabla → JSON.
- Conservar fallback de cápsula.

No tocar la lógica de reglas de `email_triage.rs`. Actualizar structs de test con `embedding: None`.

```text
cd SddIA && cargo test -p execute-process --lib -- email_triage
cd SddIA && cargo test -p execute-process --lib -- user_preference
```

## L5 — Evolution + docs de ejecución

`sddia-qa evolution-register` ligando PBI + crate + ficha. `implementation.md` + `execution.md`.

Si el diff toca `directories.evolution`: `sddia-qa gate-evolution --json --range` antes de push (DA-6).

## L6 — Cierre, DCC, CI, accept-pr

1. PBI → `docs/todos/done/` + `validacion.md` con `pbi_archived: true`. CA-10 = `PENDIENTE-CI` hasta run verde; `global` no APTO hasta entonces.
2. `./sddia-run.sh --process delivery-close-cycle` (sin skip).
3. Un log de checks del PR. Rojo → parche + un push. Verde → `run_id` + `global: APTO` + `accept-pr`.

## Fuera

Mutación EM de la skill. Reglas KNN en triaje. Deprecar JSON.
