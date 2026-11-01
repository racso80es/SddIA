---
feature_name: reconcile-unstamped-open-budget
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/reconcile-unstamped-open-budget
tracker_ref: AUT-55
execution_id: "decb2cc8-7efb-4462-8de0-df6f9c772a66"
phases:
  - id: T1
    title: Soltar el store entre lectura y escritura
  - id: T2
    title: Línea de tick y Display de Locked
  - id: T3
    title: Nota evolution AUT-55
---

# Plan — AUT-55

Rama `Fix_Events_Multiple` @ `e5919040`. Un commit por tarea. El motor, con git activo, habría creado `fix/reconcile-unstamped-open-budget`. No se hace ese checkout.

## T1 — Retención

En `SddIA/sddia-eda-catalog/src/lib.rs`, `reconcile_unstamped`:

- Tras `list_family(Domain)` y `list_deliveries`, soltar el `EventStore` antes del bucle que calcula `plan_route`.
- Aplicar `upsert_delivery` / `set_delivery` en un segundo `open`, y soltarlo al acabar el lote.
- Si el primer `open` falla, no hay segundo. Si `elapsed_ms` de la lectura es mayor o igual que `SDDIA_EVENT_STORE_OPEN_BUDGET_MS` (el mismo default 2000 que `open_budget_ms` en `sddia-event-store`), no hay segundo `open`.
- Sustituir `Result<usize, String>` por un informe con `elapsed_ms`, `domain_rows`, `delivery_rows`, `touched` y `open_cause: Option<String>`. El único llamador es `event-watcher`.

Test del catálogo con store temporal: registry ausente no abre; un caso con filas vacías devuelve contadores en cero y `open_cause` vacío. Sin Linear.

## T2 — Acuse

En `SddIA/daemons/event-watcher/src/main.rs`, después de la llamada, imprimir:

`[WATCHER] reconcile_unstamped elapsed_ms=… domain_rows=… delivery_rows=… touched=… open=ok|{causa}`

Si `open_cause` tiene texto, conservar `eprintln!("[WATCHER] reconcile_unstamped: {causa}")`.

En `SddIA/sddia-event-store/src/error.rs`, `Display` de `Locked` antepone `StoreError::Locked: `. Test en `sddia-event-store/tests/store.rs` o en el módulo de error: el texto contiene `StoreError::Locked` y el mensaje interno.

No mover el `drop(store)` que precede al `thread::spawn` de `route-domain-event`.

## T3 — Evolution

Nota nueva en `SddIA/evolution/` con UUID, vinculada a AUT-55, al catálogo, al watcher y a `error.rs`.

## Hecho que este plan no repite

`eprintln` del `Err`, `drop` antes del spawn y los logs `claim_store_pending` / `route done` (AUT-32).
