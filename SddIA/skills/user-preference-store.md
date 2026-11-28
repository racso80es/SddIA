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
hash_signature: "sha256:974d85dd1478d87fa45f72a73eb1e51dcdd05d836b5f7140ef9eb11a5ece30e0"
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

## Propósito

Store local de preferencias en `paths.userPreferencesStore`; capabilities `memory:pref-write` / `memory:pref-query`.

## Operación

Ops PUT|REVOKE|PURGE|QUERY|EXPORT vía cápsula `user-preference-store` (crate `user-preference-core`).
