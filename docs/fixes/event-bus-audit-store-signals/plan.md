---
feature_name: event-bus-audit-store-signals
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/event-bus-audit-store-signals
tracker_ref: AUT-27
execution_id: "6cd0a99f-1d58-43ee-8e30-31e9e13a090b"
phases:
  - id: T1
    title: store_unavailable cuando open falla
  - id: T2
    title: Cruce de catálogo por evento de dominio
  - id: T3
    title: success_sin_efecto y residuo pending
  - id: T4
    title: Regresión de las cuatro señales
---

# Plan — AUT-27

Rama `Fix_Events_Multiple`. Un commit por tarea cuando se ejecute. Sin abrir `fix/event-bus-audit-store-signals`. Sin tocar el frontmatter de `event-bus-audit.md`.

## T1 — Apertura

En `audit_state_from_store`, si el fichero de la base existe y `EventStore::open` devuelve error, registrar anomalía `store_unavailable` con el mensaje del error y cortar el agregado. `build_report_md` añade una sección que nombra `store_unavailable` antes de los conteos.

Si el fichero no existe, el retorno vacío actual se queda: `open` no se invoca.

## T2 — Catálogo

Tras un `open` correcto, para cada evento de `Family::Domain`:

1. Cargar `SddIA/core/event-domain-subscriptions.json`. Si el manifiesto declara `codex_slug: codex-software-engineering`, cargar también el registro del códice que ya usa `sddia_eda_catalog`.
2. `plan_route` y `subscriber_applies_to_topology` producen el conjunto esperado. El id es `subscriber_id`.
3. Comparar con las filas de `list_deliveries` de ese `event_id`.
4. Anomalía `delivery_fuera_de_catalogo` por cada id de fila ausente del conjunto. Anomalía `suscriptor_sin_fila` por cada id esperado sin fila.

Los conteos de estado que ya existen se conservan.

## T3 — Efecto y residuo

`success_sin_efecto` sobre filas `Success` de eventos de dominio:

- El agente es el tramo anterior al primer punto de `subscriber_id`. El tier sale de `llm_profile.tier` en `SddIA/agents/{agente}.md`. Cognitivo: `high`, `medium`, `low`.
- El directorio `.SddIA/workspaces/eda-dispatch/{event_id}/{agente}` no existe o no contiene ficheros.
- Si el tramo de cápsula es `tracker-stamp`, leer `tracker_ref` del ECST. Hay efecto si `.SddIA/state/tracker-outbound/{tracker_ref}.jsonl` es un fichero. Sin `tracker_ref`, no hay rastro que apague la marca.

`residual_filesystem` solo en modo redb: cada `*.json` del pending resuelto por Cúmulo. Si un `System_Fracture_Detected` del store trae `payload.friction_id` `F-EDA-RESIDUAL-FILESYSTEM` y `error_trace` contiene el nombre del fichero, el detalle de la anomalía incluye ese `event_id`.

Estas kinds no se añaden al filtro de `needs_kaizen` ni a `implicated_files` del Kaizen.

## T4 — Regresión

En los tests de `event-bus-audit`, con `override_store_mode(Redb)` y base temporal:

- `open` fallido → `store_unavailable` en anomalías e informe.
- Merge fixture: dos `success_sin_efecto`; el jsonl de Tekton apaga solo su fila.
- Presentado fixture: ids `argos` y `cumulo` fuera de catálogo, y los tres ids de `PullRequest_Presented` sin fila.
- Pending con JSON y fractura asociada → `residual_filesystem`.

`cargo test -p event-bus-audit`.

La pasada sobre `.SddIA/runtime/events.redb` del repo queda para la ejecución, con el store libre. Busca los ids `3803aeae-7a1e-4c15-b02f-8c72d16fd8ff` y `BVLgK63NjKvYhH1Zt4LGPE2KbrSCAhQaBTnJmfng8xCU`. Si `open` está bloqueado, el resultado de esa pasada es `store_unavailable`.
