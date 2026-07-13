---
document_id: PBI-REFACTOR-PODA-PYTHON-RUST
title: "[REFACTOR] Poda ejecutables Python — adecuación de clientes a cápsulas Rust"
format: markdown
version: "2.1.0"
created: "2026-07-11"
updated: "2026-07-13"
status: "cerrado"
priority: alta
process: refactorization
related:
  - README.md
  - SddIA/CONSTITUTION_CORE.md
  - SddIA/core/cumulo.paths.json
  - docs/todos/done/[ARQUITECTURA] Migración execute-process a Rust nativo (orquestador soberano).md
  - docs/todos/done/[FIX] Porte procesos residuales capsules bridge a Rust.md
  - docs/todos/done/[FIX] Porte route-domain-event core a Rust (eliminar route bridge).md
  - docs/features/poda-python-rust-clientes/
---

# [REFACTOR] Poda ejecutables Python — adecuación de clientes a cápsulas Rust

## Contexto

El Core mantenía **96 ficheros `.py`** pese a que todas las cápsulas (daemons, skills, tools) y el orquestador `execute-process` ya disponen de crate Rust. **Olas 1–8** cerraron runtime caliente, QA/aduana, documentación genómica, verificación de binarios y gates O11/O12.

**Política v2.1.0:** cero ficheros `.py` en repo (excl. `.venv/`, `.tools/`) y cero referencias operativas a Python. Rotura de compatibilidad asumida.

## Inventario — Olas 1–8 (cerradas)

| Ola | Bloque | Estado |
|-----|--------|--------|
| 1 | Engine cores Rust | ✅ |
| 2 | Clientes shell/bat | ✅ |
| 3 | Purga limbo + SSOT | ✅ |
| 4 | 15 cores QA duplicados | ✅ |
| 5 | Capa QA/aduana + DEBT-K2 | ✅ |
| 6 | Documentación genómica | ✅ |
| 7 | Verificación binarios Rust (`verify-compiled-capsules`) | ✅ |
| 8 | Gate O11/O12 + validación + cierre | ✅ |

## Ola 7 — Validación binarios Rust (nueva)

| Artefacto | Acción |
|-----------|--------|
| `sddia-qa verify-compiled-capsules` | Descubre crates con `src/main.rs` en `engine/`, `skills/`, `tools/`, `daemons/`, `interfaces/` |
| SSOT `compiled_capsules` | Resuelve perfiles `release`/`debug` bajo `SddIA/target/` |
| CI `sddia-index-qa.yml` | `cargo build --workspace` + gate binarios |
| **CA18** | 24/24 binarios nativos presentes tras build workspace |

## Mandato

1. **Cero Python en el repo** — cumplido (O11).
2. **Cero referencias operativas a Python** — cumplido (O12).
3. **Binarios Rust coherentes** — cumplido (CA18).
4. **Paridad JSON intacta** — `cargo test -p execute-process` 51/51.

## Olas de entrega

```text
Ola 1 Engine ──► Ola 2 Clientes ──► Ola 3 Limbo/SSOT
      ──► Ola 4 Purga cores QA duplicados
      ──► Ola 5 Capa QA/aduana (hooks, tests, DEBT-K2)
      ──► Ola 6 Docs genómicas (skills, contratos)
      ──► Ola 7 Verificación binarios Rust
      ──► Ola 8 Gate cero-Python + Argos + cierre
```

## Criterios de aceptación

### Olas 1–3 (runtime caliente)

- [x] CA1–CA9 — runtime caliente sin Python.

### Olas 4–8 (cero-Python total)

- [x] **CA10 — Sin ficheros `.py`**
- [x] **CA11 — Sin referencias operativas**
- [x] **CA12 — Cores QA eliminados**
- [x] **CA13 — Capa QA/aduana sin Python**
- [x] **CA14 — DEBT-K2 cerrado**
- [x] **CA15 — Docs skills alineadas**
- [x] **CA16 — Suite verde**
- [x] **CA17 — Evolution olas 4–8**
- [x] **CA18 — Binarios Rust coherentes** (`verify-compiled-capsules` 24/24)

## Definición de Done

Un único PR mergeado en main con `validacion.md` APTO (`pbi_archived: true`, `global: APTO`) y este PBI en `docs/todos/done/` en la misma rama.
