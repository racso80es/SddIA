---
feature_name: refine-item-legacy-create-issue
created: "2026-11-01"
base: c008493b
scope: alta HU legacy en handlers/refine.rs
process: refactorization
branch_name: aut-35
persist_ref: docs/features/refine-item-legacy-create-issue
tracker_ref: AUT-35
execution_id: "27ab6def-6796-4baa-a3e6-c569f96efd72"
---

# Especificación — alta HU legacy (AUT-35)

## Corte en HEAD

`refine_item/run.rs` (líneas del sello remoto): si `remote` y el frontmatter no trae `tracker_ref`, usa `snap.identifier`. Si trae otro valor, `status: conflict` y retorno sin `update_issue_description`. No hay `create_issue` en ese módulo.

`refine.rs::run_hu`: con `remote` y `tracker_ref` vacío llama a `try_create_hu_issue` (`create_issue`, etiqueta `label_hu`). El `tracker_ref` sellado pasa a ser el issue nuevo. El test `ac_0c_creates_tracker_ref_when_missing` fija `create_issue_invoked: true` y un identificador `OSC-`.

`ac_0c_skips_create_when_tracker_ref_present` ya exige que no haya alta si el frontmatter trae `tracker_ref`. Se conserva.

`linear_direct_cycle_e2e` siembra `tracker_ref` antes de `run_hu` en el ciclo con tracker. `ac10_core_self_without_tracker_still_green` usa proyecto sin tracker (`local_only`): no entra en `try_create_hu_issue`.

## Cambio

1. Eliminar `try_create_hu_issue` y la rama `tracker_ref.is_none() && remote` que lo invoca. `create_issue_invoked` queda en false cuando no hubo alta. Sin `tracker_ref` remoto no se inventa identificador.
2. `ac_0c_creates_tracker_ref_when_missing` espera `create_issue_invoked: false` y no un prefijo `OSC-`.
3. Predicado puro junto a `refine_item` (mismo criterio que el `if remote` de `run.rs`): entrada `fm_tracker_ref: Option<&str>`, `identifier: &str` → `Seal` o `Conflict`. Tests: `None` y cadena vacía → `Seal(identifier)`; igualdad ASCII case-insensitive → `Seal`; distinto → `Conflict`. `run.rs` delega en ese predicado. Sin llamada de red en el test.

## Criterios

- CA-1: `rg try_create_hu_issue SddIA/engine` sin coincidencias.
- CA-2: `ac_0c_creates_tracker_ref_when_missing` en verde con `create_issue_invoked: false`.
- CA-3: tests del predicado cubren ausencia, igualdad y divergencia.
- CA-4: `cargo test -p execute-process --lib handlers::refine` y `handlers::refine_item` en verde. `linear_direct_cycle_e2e` que llama `run_hu` sigue en verde.
