---
feature_name: kaizen-disk-thermodynamics-retention
created: "2026-11-01"
process: feature
branch_name: feat/kaizen-disk-thermodynamics-retention
persist_ref: docs/features/kaizen-disk-thermodynamics-retention
tracker_ref: AUT-49
pbi_uuid: "0d2c4bf9-8837-4842-89f2-5a0f874b203d"
historia_ref: AUT-20
status: planning
---

# Objetivos — kaizen-disk-thermodynamics-retention

## Misión

Reducir el disco regenerable del repositorio Core-self con inventario en seco y purga por política: generaciones Cargo en `SddIA/target`, residuo histórico JSON bajo `.events/` en modo `redb`, scratch acotado y bundles `dist/` envejecidos. Sin tocar la cola viva del bus ni los ELF publicados en `native-debug-bins`.

## Alcance

- Cápsula **nueva** `repo-retention-purger` (crate + ficha tool vía `entity-manager`); acción delgada de dos tiempos análoga a `purge-sandbox-cache`.
- Resolución de rutas: `sddia_daemon_runtime::load_bus_topology` + `load_eda_bus` (defaults `.events/…`; overrides FS solo si `cumulo.paths.json` declara rutas relativas válidas). Tablas `eda_bus.*` del store **no** son rutas de fichero.
- Slice B **no** reimplementa el gate de escritura (`eda-store-io-gate`, AUT-56, ya en `main`): solo elimina residuo FS histórico cuando el modo efectivo es `redb`.
- Invocación como proceso CLI o tick opcional de daemon; apply nunca concurrente con `cargo build` del mismo `CARGO_TARGET_DIR` ni con `event-watcher` activo sobre el repo (slice B).
- Cascada documental completa bajo `persist_ref`; cierre con `validacion.md` APTO + CI verde; AUT-49 en Done (Linear). Sin fichero en `docs/todos/pending/` ni `docs/todos/done/`.

## Fuera

- Ampliar `VETO_PREFIXES` / `ALLOW_RE` de `ephemeral-cache-purger`.
- Mutar filas de `.SddIA/runtime/events.redb` (solo lectura de `dead_letter_index` para conservar ids).
- `vector_store`, `.SddIA/services`, `.tools`, `.venv`, `.git`, workspaces (22 MB), proofs (51 MB) — slice posterior.
- Repetir kaizen syslog (`event-watcher` journal).

## Ley aplicada

- `features-documentation-pattern` v1.3.1: un PR; `validacion.md` pre-merge.
- `tracker.provider: linear` en `.SddIA/project.md`: backlog en AUT-49, no cola local.
- DA-2: genoma (`SddIA/tools/`, `actions/`, `process/`) solo vía `./sddia-run.sh` + `entity-manager`.
- `events-contract` §4: en `filesystem`/`dual` el slice B no borra; `processing/` siempre intocable.

## Criterios (KA-RET)

| ID | Criterio |
|----|----------|
| KA-RET-1 | Dry-run lista candidatos por slice (`path`, `bytes`, `policy`, `kept`); cero mutación. |
| KA-RET-2 | Tras dos builds sucesivas de `execute-process`, `debug/deps` conserva 1 generación por artefacto. |
| KA-RET-3 | `SddIA/target/native-debug-bins` intacto; `SDDIA_EXECUTE_PROCESS_BIN` resuelve. |
| KA-RET-4 | `processing/` intacto; en `filesystem`/`dual` slice B vacío; en `redb` JSON residual purgable salvo ids en `dead_letter_index`. |
| KA-RET-5 | Jail rechaza `..`, rutas fuera del repo y `events.redb` como candidato. |
| KA-RET-6 | Apply slice B aborta con `event-watcher` activo o store sin abrir en lectura. |
| KA-RET-7 | Scratch con mtime imposible (< 2000-01-01) → `hold-suspicious-mtime` salvo plan acusado con `force`. |
| KA-RET-8 | `validacion.md` `global: APTO` + CI verde (`run_id`) en la rama del PR. |

Estimación recuperable slice A ≈ 50 GB (foto auditoría 2026-10-30); slice B ≈ 291 MB de overhead FS vs JSON lógico.
