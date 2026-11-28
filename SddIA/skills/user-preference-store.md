---
uuid: "2fa83fc2-9897-498a-bda8-4eb155212a0d"
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
hash_signature: "sha256:96f44d8fc47a8536765c241b4c59c76f62cf4c9533ab22b934e69897220837de"
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
