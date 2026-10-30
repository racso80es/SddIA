---
feature_name: eda-store-io-gate
artifact: test-matrix
created: "2026-10-30"
process: feature
parent: docs/features/eda-store-io-gate
persist_ref: docs/features/eda-store-io-gate
---

# Objetivos — Matriz de tests I/O por tipo de evento

## Problema

Tras `publish_ecst` y el cierre de emisores residuales, la cobertura automatizada sigue siendo **puntual** (pocos `event_type`, un modo, rutas concretas). El códice declara **48 Clases ECST** (`SddIA/events/index.md`); los registros de suscripción definen qué tipos entran al bus **pending** vs **fractal**.

## Objetivo

Definir y ejecutar una **matriz reproducible** que, para cada `event_type` catalogado con ruta de runtime conocida, verifique en `redb`, `filesystem` y `dual`:

1. **Escritura:** medio correcto (store / JSON / ambos) y ausencia de fichero residual en `redb` puro.
2. **Lectura:** `load_dispatched_event` o consumidor representativo resuelve el ECST **sin** exigir JSON cuando el store es SSOT.

## Ley

- SSOT tipos: `SddIA/events/{telemetry,orchestration,domain}/` + claves en `SddIA/core/event-*-subscriptions.json`.
- Side-channel latidos (`.SddIA/daemons/state/heartbeats/`) y **PTC** (`progress_traces`) fuera de la matriz ECST (tests aparte).

## Alcance

- Laboratorio Rust (`cargo test` acotado) y, opcional, smoke shell con `SDDIA_EVENT_STORE=filesystem`.
- No E2E con daemons reales salvo una fila de humo por familia (watcher claim).

## Fuera de alcance

- Validar payload de negocio de cada Clase (schemas completos).
- 48 tests de suscriptor completo en un solo PR (se implementa por **olas** según `plan-test-matrix.md`).
