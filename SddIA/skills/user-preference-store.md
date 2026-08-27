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
hash_signature: "sha256:pending-forge"
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

MVP: handler nativo `user-preference-store-core` en `execute-process`. Ortogonal a `ThoughtNode` y `EvolutionEvent`.
