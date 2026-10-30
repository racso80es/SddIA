---
feature_name: eda-store-io-gate
artifact: test-matrix
created: "2026-10-30"
process: feature
branch_name: Kaicen_Auditoria_Eventos_261030
persist_ref: docs/features/eda-store-io-gate
status: IMPLEMENTED
phases:
  - harness y tabla generada
  - ola pending bus
  - ola fractal
  - ola dual y filesystem regresión
  - watcher smoke por familia
  - cierre validacion TM
related:
  - docs/features/eda-store-io-gate/spec-test-matrix.md
  - docs/features/eda-store-io-gate/spec.md
  - SddIA/engine/execute-process/tests/eda_subscriber_ecst_redb.rs
  - docs/features/eda-redb-04-readers/plan.md
---

# Plan — Tests de escritura y lectura por tipo de evento

Implementación **después** del commit `189dc7dd` (fachada I/O). Tracker `linear`: extender issue de `eda-store-io-gate` o PBI hijo; no materializar en `docs/todos/`.

## Fase A — Harness y trazabilidad

1. **Hecho:** `SddIA/engine/execute-process/tests/common/mod.rs` — `lab_repo()`, `roundtrip_publish_load`, tipos pending = unión de suscripciones.
2. Tabla Markdown autogenerada (script bajo `docs/features/eda-store-io-gate/` **solo documental**, ejecutable en CI opcional) — **pendiente**; cobertura narrada en `execution-test-matrix.md`:
   - Claves de los cuatro JSON de suscripción.
   - Filas del códice `events/*/index.md`.
   - Columna `test_fn` (vacía → backlog).
3. Actualizar `spec-test-matrix.md` § Cobertura cuando exista la primera ola.

## Fase B — Ola 1: bus pending (`event-subscriptions.json`)

Para **cada** clave del registro (35 tipos):

| Paso | Acción |
|------|--------|
| B1 | Construir payload mínimo válido para stamp (suscriptores → claves fake; vacíos → `_disposition: no_subscriber`). |
| B2 | `sddia_eda_catalog::stamp_published_event` + `publish_ecst(repo, "pending", …)`. |
| B3 | Modo `redb`: assert no existe `.events/pending/{id}.json`; `load_dispatched_event` devuelve ECST. |
| B4 | Modo `filesystem`: assert JSON; store no abierto o vacío. |
| B5 | Opcional smoke: `load_ecst_from_rel` sin error de fichero (no exigir handler success). |

Agrupar en **un** test parametrizado (`rstest` o macro) para no duplicar 35 funciones.

Tipos sin suscriptor en JSON (`PullRequest_Audited`, `Thought_Persisted`, …): fila TM-7 explícita.

## Fase C — Ola 2: fractal por familia

### C1 — Telemetría (4 + vitalidad)

| event_type | Suscripción | Notas test |
|------------|-------------|------------|
| `Raw_Execution_Finished` | sí | payload peaje mínimo |
| `Daemon_Heartbeat` | sí | no confundir con side-channel |
| `CI_Job_Failed` | sí | payload github stub |
| `System_Vitality_Probed` | no en JSON | `_disposition`; store Family::Telemetry |

Por tipo: `publish_ecst(..., "telemetry", fs_dir)` + `route_fractal_event` con path lógico sin fichero en `redb` (AC-F2).

### C2 — Orquestación

Claves JSON: `Process_Execution_Completed`, `Fracture_Clarification_Requested`.

Codice adicional: `TQM_Dispatch_Discarded`, `Local_QA_Requested` — una fila cada una si el emisor documentado es fractal orchestration.

### C3 — Dominio fractal (`event-domain-subscriptions.json`)

40 claves. Excluir duplicados que **solo** viven en pending si el test pending ya cubrió el mismo `event_type` (marcar en tabla como «pending+fractal dual route» y testear solo la ruta que falte: fractal `route-domain`).

Payload: objeto vacío o campos obligatorios mínimos según Clase (leer frontmatter de un `.md` representativo por grupo: PR, Entity CRUD, Email, Kaizen).

## Fase D — Modo `dual` y regresión `filesystem`

Subconjunto **estratificado** (no 48×3 completo en CI lento):

- 5 tipos pending (con/sin suscriptor, fractura, PR, mutación).
- 2 telemetry + 1 orchestration + 2 domain fractal.

Assert TM-6: tras write dual, borrar JSON y lectura sigue desde store; borrar fila store (lab) y lectura cae a JSON.

Regresión TM-5: job CI `SDDIA_EVENT_STORE=filesystem cargo test -p execute-process --test eda_store_io_matrix_pending -- --test-threads=1` (subset o full según tiempo).

## Fase E — Watcher (smoke lectura indirecta)

Un test por familia en `event-watcher`:

- Insertar fila store con entrega Pending (o usar `record_ecst` + suscriptor fake).
- Tick `claim_store_fractal` / `claim_store_pending`.
- Verificar invocación `route-*` con path lógico (mock execute-process o assert log en lab `once`).

No sustituye matriz de tipos; valida **claim + load** end-to-end.

## Fase F — Cierre documental

- `validacion.md`: sección `checks` TM-1..TM-8.
- `execution.md`: comandos exactos y filtros `cargo test`.
- Evolución opcional si se añade crate test-only o proceso `eda-store-io-matrix` (preferir integration tests sin genoma nuevo).

## Orden y esfuerzo estimado

| Ola | PR | Tests nuevos (orden magnitud) |
|-----|-----|-------------------------------|
| A | 1 | 0 (infra) |
| B | 2 | 1 parametrizado × 3 modos ≈ 105 asserts |
| C | 2–3 | 2 binarios fractal × modos |
| D | 4 | ~15 casos dual/fs |
| E | 5 | 3–4 watcher |
| F | — | docs |

## Riesgos

- **Stamp falla** sin catálogo completo en fixture: copiar registries reales del repo (no inventar suscriptores).
- **Payload inválido** en route: matriz de I/O se limita a publish+load; tests de route solo en subset TM-8.
- **Tiempo CI**: parametrización + `--test-threads=1` en integration; olas B/C pueden dividirse por PR.

## Comandos de verificación (lab)

```bash
cd SddIA
export SDDIA_EVENT_STORE=redb   # o filesystem / dual por ola
cargo test -p sddia-event-store --test gate
cargo test -p execute-process --test eda_store_io_matrix_pending -- --test-threads=1
cargo test -p execute-process --test eda_store_io_matrix_fractal -- --test-threads=1
cargo test -p execute-process --test eda_store_io_matrix_pending tm8_presented -- --test-threads=1
./docs/features/eda-store-io-gate/gen-test-matrix-table.sh
```
