---
feature_name: aut-52-refine-fracture-route
created: "2026-11-02"
process: bug-fix
phase: blueprint
agents: dedalo
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-52-refine-fracture-route
pbi_ref: AUT-52
tracker_ref: AUT-52
uuid: c55f04de-f956-4f1e-89ab-580b2c5bab34
execution_id: "00df8b64-e5b1-42e0-be0f-5d684f7eefc9"
---

# Plan — aut-52-refine-fracture-route

Orden: C1 → C4 (catálogo) → C2 → C3 → tests → C5 verificación → evolution → validacion. Rama: `Fix_Events_Multiple_02`. Sin proceso genómico nuevo.

## C1 — refine-item / fractura lock

1. Payload `emit_store_lock_fracture`: `friction_id: F-REFINE-ITEM-REDB-LOCK`, `error_code: STORE_LOCK_CONTENTION`.
2. Sidechannel FS en `write_pending_domain_event_file` si `record_ecst` falla por lock al emitir `System_Fracture_Detected`.
3. Lab: `SDDIA_LAB_SIMULATE_STORE_LOCK` → envelope honesto.

## C2 — route / identidad evento redb

1. Modo store-only: path virtual `{pending}/{event_id}.json`; dedupe → id existente.
2. Rechazo explícito de ruta `events.redb` en `route_domain_event`.
3. `route_domain_batch`: errores acumulados; `success: false` si falla carga o ruta DB.

## C3 — capsules

1. `tool_invoke_error_message`: `code: message` salvo `LINEAR_AUTH_MISSING`.
2. Test `aut52_tool_invoke_error_message_includes_linear_transport_detail`.

## C4 — fracture-signatures.json

1. Entrada `F-REFINE-ITEM-REDB-LOCK` (consumers mayeuta/cumulo).
2. Evolution + nota corpus/unclassified en validacion si no hay fixture dedicado.

## C5 — materialize-fracture-pbi

1. Verificar nativo `run_remote`: `build_pbi_body` + labels `pbi`+`fix` (sin cambio si paridad OK).

## Cierre Tekton

1. `cargo test aut52` en `execute-process`.
2. `SddIA/evolution/c55f04de-f956-4f1e-89ab-580b2c5bab34.md` + `Evolution_log.md`.
3. `implementation.md`, `execution.md`, `validacion.md` (APTO + AUT52-CA*).
