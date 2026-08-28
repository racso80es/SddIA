---
feature_name: kaizen-fracture-fanout-idempotencia
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-fracture-fanout-idempotencia
persist_ref: docs/fixes/kaizen-fracture-fanout-idempotencia
pbi_ref: docs/todos/done/[KAIZEN] Fan-out de fractura sin idempotencia real — PBI cerrados resucitados y Mayeuta en dead-letter.md
document_id: PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA
items:
  - l0-genome-emit-backfill
  - l1-core-resolver
  - l2-handlers-precedence
  - l3-mayeuta-no-dl
  - l4-event-content-hash
  - tests-unit
  - ca9-stale-purge
  - l5-action-contracts
  - ca8-dl-purge
  - actions-index-sync
  - cierre-documental
---

# Implementación — Fan-out de fractura: idempotencia por genoma

## Touchpoints

| Área | Archivo |
|------|---------|
| SSOT rutas | `SddIA/core/cumulo.paths.json` → `paths.todos.pending` / `done` |
| Resolutor Core | `SddIA/engine/execute-process/src/core/fracture_pbi.rs` |
| Cúmulo | `SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs` |
| Mayeuta | `SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs` |
| Event dedup | `SddIA/engine/execute-process/src/engine/route_domain_core.rs` (`write_pending_domain_event_file`) |
| Forja acción update | `SddIA/engine/execute-process/src/forges/common.rs` (`patch_action_content_update`, `sync_action_index_row`) |
| Backfill one-shot | `docs/fixes/kaizen-fracture-fanout-idempotencia/backfill-fracture-genome.py` |
| Purga DL CA8 | `docs/fixes/kaizen-fracture-fanout-idempotencia/purge-mayeuta-enrich-dl.py` |

## Comportamiento

- **Precedencia §4.1:** `already_open` → `deduped_by_process` → `regression_opened` → `materialized`.
- **Ceguera nominal:** barrido YAML por `fracture_hash` / `fracture_process`; extracción lineal tolerante a frontmatter malformado.
- **Regresión:** PBI nuevo en `pending/` con `document_id` `PBI-FIX-FRACTURE-<hash>-R<n>`, `regression_of` al canónico en `done/` (inmutable).
- **Mayeuta:** `no_target` sin dead-letter; resolución vía Core.
- **Eventos:** `System_Fracture_Detected` por hash de contenido + `create_new(true)`.
- **Telemetría:** `Fracture_Pbi_Resolver_Scan` (`docs_scanned`, `bytes_read`, `duration_ms`).

## L5 — Contratos de acción (CA11)

| Acción | Versión | `event_id` |
|--------|---------|------------|
| `materialize-fracture-pbi` | 1.1.0 | `9622c410-a9c8-49ae-8c56-75a167cd50fa` |
| `enrich-fracture-pbi-kaizen` | 1.1.0 | `ab2f5558-16ae-462a-8a1f-ac49811893d2` |

## Cierre

`validacion.md` APTO; PBI en `docs/todos/done/`; índice `SddIA/actions/index.md` alineado con capabilities v1.1.0.
