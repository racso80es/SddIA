---
uuid: "f1a2b3c4-d5e6-4789-a012-3456789ab01"
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
hash_signature: "sha256:45f128ce99ee838398316d5b990792c6c52f1b058ffe87b83d4638dc52e119c6"
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
