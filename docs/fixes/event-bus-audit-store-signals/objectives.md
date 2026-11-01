---
feature_name: event-bus-audit-store-signals
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/event-bus-audit-store-signals
pbi_ref: AUT-27
tracker_ref: AUT-27
execution_id: "6cd0a99f-1d58-43ee-8e30-31e9e13a090b"
document_id: PBI-1778fb70
---

# Objetivos — event-bus-audit-store-signals

## Misión

Cerrar AUT-27: en modo redb, `event-bus-audit` señala entregas `success` sin efecto, ids de `subscriber_delivery` fuera del catálogo, suscriptores del catálogo sin fila, `store_unavailable` cuando `EventStore::open` falla, y los JSON residuales de `.events/pending/` ligados a `F-EDA-RESIDUAL-FILESYSTEM`.

## Alcance

`audit_state_from_store` y el informe de `tool:event-bus-audit`. El cruce usa el mismo `subscriber_id` que el catálogo (`agente.cápsula`). Casos vivos: merge `3803aeae-7a1e-4c15-b02f-8c72d16fd8ff` y presentado `BVLgK63NjKvYhH1Zt4LGPE2KbrSCAhQaBTnJmfng8xCU`.

## Ley aplicada

- Git solo vía `skill:git-manager`. Este ciclo no movió HEAD: `SDDIA_LAB_SKIP_GIT` para permanecer en `Fix_Events_Multiple`. El init etiquetó `fix/event-bus-audit-store-signals`; esa rama no existe y no se abre.
- El cambio de comportamiento vive en la cápsula ya indexada `SddIA/tools/event-bus-audit/src/main.rs`. El frontmatter de `event-bus-audit.md` no se reescribe.
- Provider `linear`: cierre en el issue, sin fichero nuevo en `docs/todos/pending/` ni `docs/todos/done/`.
- Fuera: AUT-26, AUT-21, AUT-25, AUT-16 y `reconcile_unstamped`.
