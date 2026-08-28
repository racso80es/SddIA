---
feature_name: kaizen-fracture-fanout-idempotencia
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-fracture-fanout-idempotencia
persist_ref: docs/fixes/kaizen-fracture-fanout-idempotencia
pbi_ref: docs/todos/pending/[KAIZEN] Fan-out de fractura sin idempotencia real — PBI cerrados resucitados y Mayeuta en dead-letter.md
document_id: PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA
uuid: 48c64f40-6016-46c9-bb1e-e976ec39d89a
scope: fracture-fanout-idempotencia
base: main
execution_id: "57a510b9-f288-4569-8ff5-067c0c614d1a"
---

# Spec — Fan-out de fractura: identidad en genoma e idempotencia

## Problema

El fan-out de `System_Fracture_Detected` materializa y enriquece el PBI con dos handlers que **no comparten resolutor** y **no leen `docs/todos/done/`**. Identidad del defecto hoy: nombre de fichero y cadenas embebidas. Incidente `6a49e0ad310e`: PBI cerrado (PR #210) reaparece untracked en `pending/`; 351 DL de Mayeuta por ruta reconstruida que no existe.

## Causa raíz

| ID | Hecho | Trabajo |
|----|-------|---------|
| D1 | `build_pbi_body` no declara `fracture_hash` ni `fracture_process` en YAML | Emitir genoma (CA11) + backfill (CA12) |
| D2 | `target.is_file()` y `find_open_fracture_pbi` por prefijo de nombre | Ceguera nominal: solo YAML (CA10) |
| D3 | `done/` invisible | Precedencia §4.1 (CA1) + regresión (CA2/CA13) |
| D4 | Resolutor duplicado en `engine/`; `enrich` importa `materialize` | Módulo único en `core/` + claves Cúmulo (CA3/CA4) |
| D5 | Mayeuta `Err` si el fichero derivado no está | `success: true` + `reason: no_target` (CA5) |
| D6 | `materialize_pending_domain_event` nombra `{uuid}.json` | Hash de contenido + `create_new` (CA6) |

## Precisión CA12 (Filtro A)

`document_id: PBI-FIX-FRACTURE-<12hex>` **no** contiene el proceso fracturado. El campo YAML `process:` es `bug-fix` (clase del PBI), no el daemon colapsado. Backfill:

- `fracture_hash` ← sufijo hex de `document_id` (única lectura autorizada de identidad no-genómica).
- `fracture_process` ← slug del título `[FIX] {process} — fractura sistémica` o celda «Proceso» del cuerpo. No usar `process: bug-fix`.

## Solución

### Identidad (genoma)

YAML de primer nivel: `fracture_hash` (12 hex SHA-256 de `error_trace`), `fracture_process` (slug), `regression_of` solo en regresiones. `document_id` de apertura: `PBI-FIX-FRACTURE-<hash>`; de regresión: `PBI-FIX-FRACTURE-<hash>-R<n>` con `n` = 1 + máximo `R` ya visto para ese hash (pending+done). El nombre de fichero es presentación; el motor no lo parsea.

### Resolutor Core (`execute-process/src/core/`)

Nuevo módulo junto a `paths.rs` / `parser.rs`. Consume `cumulo.paths.json` claves nuevas `todos.pending` / `todos.done` (alta; hoy no existen). API:

1. Barrido de frontmatter (corte en `---` de cierre; ignorar docs sin `fracture_hash`).
2. Precedencia CA1: `already_open` → `deduped_by_process` → `regression_opened` → `materialized`.
3. Telemetría por invocación: `docs_scanned`, `bytes_read`, `duration_ms` (familia `telemetry`; umbral de índice = `PBI-DT-FRACTURE-RESOLVER-SCAN-LINEAL`, **prohibido indexar aquí**).

Handlers `materialize_fracture_pbi` y `enrich_fracture_pbi_kaizen` **solo** consumen esa API. Cero `super::materialize_fracture_pbi` desde enrich. Cero literales `docs/todos/pending`.

### Mayeuta

Cascada: `cumulo_pbi_path` si el fichero existe → hash abierto en pending → process abierto en pending → **no** escribe sobre `done/`. Si Cúmulo acaba de abrir regresión, enriquece esa ruta. Si no hay target: `success: true`, `reason: no_target`.

### Eventos (CA6)

Ámbito: escritura de `System_Fracture_Detected` (punto caliente: `materialize_pending_domain_event` y el emit ad-hoc en `route_domain_core`). Identidad de contenido = SHA-256 canónico de `{event_type, payload}` **sin** `event_id` ni `timestamp` de sobre. Nombre `{hash12}.json` + `OpenOptions::create_new(true)`. `AlreadyExists` = duplicado, `success` sin segunda copia. `event_id` interno puede seguir siendo UUID; el fichero no lo usa. No aplica a PBI (asimetría §5.4 del PBI).

### Contratos de acción

`SddIA/actions/materialize-fracture-pbi.md` y `enrich-fracture-pbi-kaizen.md` vía `entity-manager` (DA-2). Documentar campos YAML, `reason` y resolución por genoma. Prohibido «se resuelve por hash de nombre».

## Fuera de alcance

Relay IOTA #208. TQM single-flight. Índice de `done/`. Mutar estado/cuerpo de PBI en `done/` salvo anotación de identidad CA12. Poda de `done/`.
