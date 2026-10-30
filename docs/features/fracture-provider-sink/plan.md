---
feature_name: fracture-provider-sink
created: "2026-10-30"
process: refactorization
phases:
  - id: T1
    title: Sumidero y alta remota
  - id: T2
    title: Hecho, route y depósito
---

# Plan

## T1 — Sumidero y alta remota

Slug auto-hospedado. `Unconfigured` no escribe fichero. Alta linear por el puerto con cuerpo, etiquetas y dedupe por descripción. `list_issues` devuelve `description`.

## T2 — Hecho, route y depósito

Clase `Fracture_Pbi_Materialized`. Enrich suscribe la referencia. Route ejecuta acción o tool declarada. Deny visible. Watcher suelta el store.

## Verificación

Tests de `materialize_fracture`, `enrich_fracture`, resolver del puerto y deny, con `SDDIA_EVENT_STORE=filesystem` en el laboratorio que no carga la bóveda. `cargo check -p event-watcher`.
