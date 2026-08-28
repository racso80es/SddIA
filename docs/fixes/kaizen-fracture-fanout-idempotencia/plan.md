---
feature_name: kaizen-fracture-fanout-idempotencia
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-fracture-fanout-idempotencia
persist_ref: docs/fixes/kaizen-fracture-fanout-idempotencia
pbi_ref: docs/todos/pending/[KAIZEN] Fan-out de fractura sin idempotencia real — PBI cerrados resucitados y Mayeuta en dead-letter.md
document_id: PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA
uuid: 0f61ee39-f715-4f1e-9a7f-0804aa88bf1b
execution_id: "57a510b9-f288-4569-8ff5-067c0c614d1a"
phases:
  - l0-genome-emit-backfill
  - l1-core-resolver
  - l2-handlers-precedence
  - l3-mayeuta-no-dl
  - l4-event-content-hash
  - l5-action-contracts
  - tests-unit
  - ca8-ca9-ops
---

# Plan — Fan-out de fractura: genoma, Core, regresión

Orden: L0 → L1 → L2 → L3 → tests unitarios (CA7) → L4 → L5 → CA8/CA9. Cierre documental y `delivery-close-cycle` **después** de ejecución/Argos. Este commit sella Diseño (`spec.md` + `plan.md`).

## Fase L0 — Genoma y backfill (CA11/CA12)

1. `build_pbi_body` declara `fracture_hash`, `fracture_process`; regresiones añaden `regression_of` y `document_id` `…-R<n>`.
2. Script/paso de migración **único** (fuera de `SddIA/tools/` si es one-shot de este fix; si se indexa como tool → `entity-manager`):
   - Selección: `document_id` ~ `^PBI-FIX-FRACTURE-[0-9a-f]{12}`.
   - Escribir solo claves de identidad ausentes. No tocar `status`, `closed`, `merged_pr`, cuerpo.
   - Hash desde `document_id`; proceso desde título, nunca desde `process: bug-fix`.
3. Incluye el stale `docs/todos/pending/[FIX] route-domain-event — fractura sistémica (6a49e0ad310e).md` (CA9 lo purga **después** de que el resolutor esté vivo, no en L0).

## Fase L1 — Resolutor Core (CA3)

Archivos: `SddIA/engine/execute-process/src/core/fracture_pbi.rs` (nuevo), `core/mod.rs`, `SddIA/core/cumulo.paths.json`.

1. Claves `todos.pending` / `todos.done` (rutas relativas `docs/todos/pending`, `docs/todos/done`). Resolver vía `load_paths_config`; prohibido literal en handlers.
2. Reutilizar `parser::parse_frontmatter` (no un cuarto parser). Lectura: bytes hasta el segundo `---`.
3. `scan_fracture_ledger(repo) -> Scan { pending, done, docs_scanned, bytes_read, duration_ms }`.
4. `resolve_materialize(scan, hash, process) -> Resolve { reason, target_path, canonical_ref? }` con precedencia §4.1 del PBI.
5. `next_regression_id(scan, hash) -> n`.
6. Emitir telemetría (familia `telemetry`) con los tres contadores. Sin índice.

## Fase L2 — Handlers + ceguera nominal (CA1/CA2/CA10/CA13)

`materialize_fracture_pbi.rs`:

1. Eliminar `target.is_file()`, `fracture_pbi_path` como clave de existencia, y `find_open_fracture_pbi` por prefijo.
2. Consumir `resolve_materialize`. Ramas: `already_open` / `deduped_by_process` (cero writes); `regression_opened` (write PBI nuevo + `canonical_ref`); `materialized` (write apertura).
3. Nombre de fichero de **write**: sigue siendo humano (`[FIX]` / `[REGRESIÓN]` + slug + hash + `-R<n>` si aplica) **solo al crear**; nunca al buscar.

## Fase L3 — Mayeuta (CA4/CA5)

`enrich_fracture_pbi_kaizen.rs`:

1. Quitar `use super::materialize_fracture_pbi`.
2. Resolver vía Core: path opcional válido → hash pending abierto → process pending abierto. Homólogo solo en `done/` **no** se enriquece (`no_target` si aún no hay regresión en pending).
3. `Err` solo I/O real. Falta de PBI → `success: true`, `reason: no_target`.

## Fase tests unitarios (CA7) — antes de L4

Tempdir con YAML, nombres deliberadamente **distintos** del patrón actual.

- (a) done cerrado mismo hash → `regression_opened`, done intacto.
- (b) pending abierto mismo hash → `already_open`, mtime/contenido iguales.
- (c) 7× mismo input tras (a) → un solo fichero de regresión.
- (d) mismo `fracture_process`, hash distinto, pending abierto → `deduped_by_process`.
- (e) fichero renombrado a `zzz.md` con genoma intacto → (b) sigue.
- (f) Cúmulo dedupe por process → Mayeuta enriquece esa ruta, no la derivada del hash.

Comando: `cd SddIA && cargo test -p execute-process materialize_fracture_pbi enrich_fracture_pbi_kaizen fracture_pbi`.

## Fase L4 — Colisión física de evento (CA6)

Punto de escritura: `materialize_pending_domain_event` (`route_domain_core.rs` ~L130) y el emit de fractura ~L494. Patrón: `OpenOptions::new().write(true).create_new(true)` (precedente `task_queue_manager`).

Identidad de fichero: 12 hex de SHA-256(`event_type` + JSON canónico de `payload`). `AlreadyExists` → return Ok de la ruta existente, no overwrite.

No cambiar `write_fractal_event` genérico de telemetría/orquestación en este PBI (esas familias no son el storm medido). Si el emit de fractura pasa por fractal, sí aplicar el mismo `create_new` **solo** a `System_Fracture_Detected`.

## Fase L5 — Contratos de acción (CA11)

```text
./sddia-run.sh --process entity-manager --inputs '{… update materialize-fracture-pbi / enrich-fracture-pbi-kaizen …}'
```

Prohibido `Write` directo sobre `SddIA/actions/`.

## Fase CA8 / CA9 (ops, tras binario)

- CA9: borrar el stale de `pending/` de `6a49e0ad310e` (el canónico vive en `done/` con genoma backfilled). Verificar que una re-emisión de la misma traza abre **regresión**, no un segundo `[FIX]` idéntico.
- CA8: barrido documentado de los 351 DL `mayeuta.enrich-fracture-pbi-kaizen`; reproceso o cierre; `orphan_count` estable en `event-bus-audit`. Puede quedar residual en `execution.md` si el bus de instancia no está en este worktree.

## Cierre (fuera de esta parada)

`implementation.md` + `execution.md` → Argos `validacion.md` → PBI a `done/` → `delivery-close-cycle`.
