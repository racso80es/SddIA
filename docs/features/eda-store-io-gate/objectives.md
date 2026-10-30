---
feature_name: eda-store-io-gate
created: "2026-10-30"
process: feature
branch_name: Kaicen_Auditoria_Eventos_261030
persist_ref: docs/features/eda-store-io-gate
---

# Objetivos — I/O de eventos solo vía `SDDIA_EVENT_STORE`

## Problema (origen)

Con `SDDIA_EVENT_STORE=redb` (default de `gate.rs` y bóveda de instancia) el SSOT es `.SddIA/runtime/events.redb`. Antes del corte de esta feature, varios emisores y lectores tocaban `.events/` sin consultar `writes_store` / `writes_filesystem` (JSON residual, idempotencia ciega, fractal en store no consumido por el watcher).

## Estado en rama

Implementado en `189dc7dd` (fachada + call sites + fractal claim) y verificado en `validacion.md` (`global: APTO`). Higiene Kaizen del bus: `docs/audits/kaizen-auditoria-eventos-261030.md`. Deriva documental residual: `docs/audits/eda-doc-drift-eda-store-io-gate-261030.md`.

## Objetivo

Todo camino de producción que persista o recupere un ECST (dominio, fractal, progreso, testigo de entrega) decide el medio **solo** con `sddia_event_store::{store_mode, writes_store, writes_filesystem}` (o fachada única que los encapsule). `redb` no crea ni lee JSON de bus como fuente de verdad. `filesystem` no abre `events.redb`. `dual` escribe y lee ambos, con store como autoridad si la clave existe.

## Ley

- `SddIA/events/events-contract.md` §4.
- `SddIA/sddia-event-store/src/gate.rs`.
- Side-channel de latido (`.SddIA/daemons/state/heartbeats/`) **no** es bus ECST; permanece fuera de este corte.

## Fuera de alcance

- Purga histórica de `.events/` ya archivada.
- Cambiar el default del gate (sigue `redb`).
- Cola `docs/todos/` (tracker `linear`: alta vía `forge-pbi`).
- Reescribir `event-sweeper` más allá de no tratar FS como SSOT en `redb`.

## Hecho previo (no repetir)

- `write_pending_kaizen` (`event-bus-audit`) alineado en commit `9d17ece8`.
- Emisores gated: `actions`, `domain_mutation`, `linear_bridge`, `github_bridge`, `materialize_pending_domain_event`, `progress_trace`, `write_fractal_event`.
