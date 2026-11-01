---
feature_name: reconcile-unstamped-open-budget
created: "2026-11-01"
base: main
scope: retención de events.redb en reconcile_unstamped
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/reconcile-unstamped-open-budget
tracker_ref: AUT-55
execution_id: "decb2cc8-7efb-4462-8de0-df6f9c772a66"
---

# Especificación — presupuesto de reconcile_unstamped

## Corte

HEAD `e5919040`:

```text
claim_store_pending
  reconcile_unstamped          → open, list_family, list_deliveries, writes, drop al salir
  Err                          → eprintln (AUT-32). Sin elapsed_ms ni conteos
  EventStore::open del claim
  drop(store)
  spawn route-domain-event
StoreError::Locked Display     → solo el texto de redb
open budget                    → 2000 ms
poll                           → POLL_SECONDS = 2
```

El plan de suscriptores no usa el store. El lock sigue tomado durante ese cálculo y las write-tx.

## Decisión

1. `reconcile_unstamped` abre, lista dominio y entregas, suelta el store y calcula el plan en memoria. Si el `open` de lectura falla, no hay segundo `open`. Si la lectura cabe en el presupuesto, abre otra vez solo para las write-tx y suelta al terminar. Si la lectura ya superó `SDDIA_EVENT_STORE_OPEN_BUDGET_MS`, no abre para escribir.
2. El retorno lleva `elapsed_ms`, `domain_rows`, `delivery_rows`, `touched` y `open_cause` (`None` si ambos `open` fueron bien). `open_cause` usa el `Display` nuevo de `Locked` cuando el fallo es ese.
3. El watcher imprime una línea con esos campos en cada tick que llama a la función. Si `open_cause` tiene texto, mantiene el `eprintln` actual.
4. `Display` de `StoreError::Locked` es `StoreError::Locked: {mensaje}`. El resto de variantes no cambia. El `drop(store)` anterior al spawn no se mueve.

## Regresión

| Caso | Esperado |
|------|----------|
| Registry o db ausente | informe en cero, `open_cause` vacío, sin `open` |
| `open` de lectura `Locked` | sin writes; `open_cause` contiene `StoreError::Locked` |
| Lectura por encima del presupuesto | sin segundo `open`; `elapsed_ms` en el informe |
| Lectura dentro del presupuesto | writes y `touched` como ahora |
| `Display` de `Locked("Database already open")` | empieza por `StoreError::Locked` |
| Tick del watcher | línea con `elapsed_ms`, `domain_rows`, `delivery_rows` |

## Fuera de alcance

AUT-32, presupuesto por defecto, fractal claim, ECST.

## Criterios de aceptación

Los del cuerpo refinado de AUT-55: retención acotada, log con `elapsed_ms` y causa, acuse que nombra `Locked`, `validacion.md` APTO y AUT-55 en Done.
