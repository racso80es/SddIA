---
feature_name: cognitive-tier-native-dispatch
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/cognitive-tier-native-dispatch
tracker_ref: AUT-26
---

# Implementación — AUT-26

## T1

`subscriber_declares_native` en `SddIA/engine/execute-process/src/engine/route_domain_core.rs`: el bucle incluye `process` junto a `action` y `tool`.

## T2

Test `store_backed_native_capsule_does_not_use_cognitive_runtime`: `process` + tier cognitivo, `action` + tier cognitivo y `process` + tier `none` no ejecutan el script de `AGENT_PHASE` cuando el evento es store-backed.

Mutex `TIER_DISPATCH_ENV_LOCK` en el módulo `tier_dispatch` para evitar carrera en `SDDIA_AGENT_RUNTIME_COMMAND` entre tests.

## Verificación

`cargo test -p execute-process tier_dispatch --lib`
