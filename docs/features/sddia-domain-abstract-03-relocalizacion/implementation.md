---
feature_name: sddia-domain-abstract-03-relocalizacion
created: "2026-08-09"
updated: "2026-08-09"
process: refactorization
branch_name: feat/sddia-domain-abstract-03-relocalizacion
persist_ref: docs/features/sddia-domain-abstract-03-relocalizacion
pbi_ref: docs/todos/done/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-03 — Relocalización física process software.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
laudo: L-PACK-MULTIROOT-SIX-MOVE
phase: T0-T3
status: implemented
agents: tekton
---

# Implementation — sddia-domain-abstract-03-relocalizacion

## T0 — Resolución multi-root

| Path | Cambio |
|------|--------|
| `SddIA/core/cumulo.paths.json` | `version` 1.5.3→**1.6.0**; +`directories.process_domain_roots` |
| `SddIA/engine/execute-process/src/core/paths.rs` | `load_paths_config` (Cúmulo + overlay `.SddIA/local.paths.json`) |
| `SddIA/engine/execute-process/src/core/resolver.rs` | `process_search_roots` + `resolve_process_path` domain-first |
| `workspace.rs` / `verify_process_integrity.rs` / `eda_coverage.rs` | Consumen multi-root |
| Normas / códice | Nota packing + `process_domain_roots` (DA-2) |

Evidencia: `cargo test -p execute-process ac_resolve` → **5 passed**.

## T1 — Move físico + índice

Origen → destino (UUID conservados):

```text
SddIA/process/{feature,bug-fix,refactorization,pull-request-review,accept-pr,delivery-close-cycle}.md
→ SddIA/library/codexes/codex-software-engineering/process/
```

- Índice Core: filas de los 6 retiradas + nota ABSTRACT-03.
- Índice dominio: `…/codex-software-engineering/process/index.md` (6 filas).
- Sin stubs en Core.

## T2 — Referencias

Reactor DI y normas path alineados a resolución Cúmulo (sin hardcode Core de los 6).

## T3 — Compat

| Check | Evidencia |
|-------|-----------|
| AC-RESOLVE | tests `ac_resolve_*` OK |
| AC-RUN | `./sddia-run.sh --process feature --inputs '{}'` → `INPUT_VALIDATION` (resuelve dominio; falla inputs) |
| AC-TQM | `kalma2-interact` OK post-move |
| AC-BUILD | `cargo build -p execute-process --release` OK |

## T4 — PBI

Promovido kitchen → `docs/todos/pending/` con UUID `7ade2a5f-be13-41ef-8b11-deb96fd58be3`.
