---
feature_name: aut-52-refine-fracture-route
created: "2026-11-02"
updated: "2026-11-02"
process: bug-fix
phase: Diseño del fix
agents: dedalo
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-52-refine-fracture-route
pbi_ref: AUT-52
tracker_ref: AUT-52
uuid: c55f04de-f956-4f1e-89ab-580b2c5bab34
execution_id: "f2880068-385c-4403-8c31-9d11bd4b08a1"
---
# Plan — aut-52-refine-fracture-route

**No es blueprint genómico** (sin proceso nuevo). Secuencia de cortes para Tekton en rama `Fix_Events_Multiple_02`.

Orden: **C1 → C4 → C2 → C3 → tests → C5 verificación → evolution → validacion**.

## C1 — refine-item / fractura lock

1. Payload `emit_store_lock_fracture`: `friction_id: F-REFINE-ITEM-REDB-LOCK`, `error_code: STORE_LOCK_CONTENTION`.
2. Sidechannel FS en `write_pending_domain_event_file` si `record_ecst` falla por lock al emitir `System_Fracture_Detected`.
3. Lab: `SDDIA_LAB_SIMULATE_STORE_LOCK` → envelope honesto + fractura recuperable.

## C4 — fracture-signatures.json (antes de materializar cuerpos)

1. Entrada `F-REFINE-ITEM-REDB-LOCK` (consumers mayeuta/cumulo; scope `refine-item` / `STORE_LOCK_CONTENTION`).
2. Nota corpus/`unclassified` en evolution o validacion si no hay fixture dedicado.

## C2 — route / identidad evento redb

1. Modo store-only: path virtual `{pending}/{event_id}.json`; dedupe → id existente; **nunca** `runtime_db_path`.
2. Rechazo explícito de ruta `events.redb` en `route_domain_event`.
3. `route_domain_batch`: errores acumulados; `success: false` si falla carga o ruta DB.
4. Tests: detección path DB + batch rechaza `.redb`.

## C3 — capsules

1. `tool_invoke_error_message`: `code: message` salvo auth-only (`LINEAR_AUTH_MISSING`).
2. Test `aut52_tool_invoke_error_message_includes_linear_transport_detail`.

## C5 — materialize-fracture-pbi

1. Verificar nativo `run_remote`: `build_pbi_body` + labels `pbi`+`fix` (sin cambio si paridad OK).

## Cierre

1. `cargo test aut52` en `execute-process`.
2. Evolution `c55f04de-f956-4f1e-89ab-580b2c5bab34` + `Evolution_log.md`.
3. `implementation.md` / `execution.md` / `validacion.md` (CA1–CA8; CA4 integración y CA8 Linear Done pueden quedar salvedades operativas).
4. Sin writes bajo `docs/todos/`.
