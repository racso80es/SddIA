---
uuid: "231b8d22-14bf-4480-a40c-21dca989a702"
name: "user-preference-store"
version: "1.0.0"
contract: "skills-contract v1.1.0"
context: "knowledge-management"
capabilities:
  - "user_preference_store"
provides:
  - id: "memory:pref-write"
    contract: "memory.pref_write"
    version: "1.0.0"
  - id: "memory:pref-query"
    contract: "memory.pref_query"
    version: "1.0.0"
hash_signature: "sha256:af9ef3fb8ba2ca4376dc487cc135c167a722bfd90a97eb279cdc810f2e943d1f"
inputs:
  - "op": "PUT | REVOKE | PURGE | QUERY | EXPORT"
  - "revision": "objeto UserPreference (PUT)"
  - "spec": "QuerySpec (QUERY)"
  - "preference_id": "string (PURGE)"
outputs:
  - "exitCode": "0 éxito"
  - "result": "preference_id / preferences / export"
---

# Skill: user-preference-store

Store local de preferencias del Vértice Biológico bajo `paths.userPreferencesStore` (`.SddIA/vector_store/user_preferences/`).

Cápsula nativa/WASI (`SddIA/skills/user-preference-store/`) sobre crate `user-preference-core`. `execute-process` conserva ingest/fractal y fallback inline si la cápsula no está compilada. Ortogonal a `ThoughtNode` y `EvolutionEvent`.

## Propósito

Store local de preferencias del Vértice Biológico bajo `paths.userPreferencesStore` (`.SddIA/vector_store/user_preferences/`).

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
