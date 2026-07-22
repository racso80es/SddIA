---
feature_name: inyeccion-dependencias-barrido-creators
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-barrido-creators
persist_ref: docs/features/inyeccion-dependencias-barrido-creators
document_id: PBI-042-BARRIDO-CREATORS
execution_id: c9d1e4f2-7a8b-4c5d-9e0f-1a2b3c4d5e6f
runtime: tekton-kalma2-cursor
verdict: ready_for_argos
phase: r14-done
---

# Implementation — Barrido creators residuales DI (Hito 6 R14)

## Veredicto

**ready_for_argos** — R14 materializado: forge update preservante + ola `N_ola=4` + sellos EDA + orphan 0 + regresión DI.

## Engine (habilitador)

| Archivo | Cambio |
|---------|--------|
| `forges/common.rs` | `patch_process_phases_update`, `bump_semver_patch`, `update_process_index_version` |
| `forges/factory.rs` | update + `process_phases` → patch; sin phases → hash-only |
| `entity_manager.rs` | propaga `process_phases` / `process_version` solo si vienen en seed |

Tests: `process_forge_update_with_phases_*` + `process_forge_update_without_phases_*` **ok**.

## Genoma R14

| ED | Antes | Después | DI |
|----|-------|---------|-----|
| `norm-creator` | 1.2.0 + `skill:filesystem-manager` | 1.2.1 | Materialización + Indexación → `fs:persist` ciego |
| `codex-creator` | 1.1.0 | 1.1.1 | idem |
| `daemon-creator` | 1.0.0 | 1.0.1 | Forja mixto crypto+`fs:persist`; Indexación ciego |
| `suite-creator` | 1.0.0 | 1.0.1 | Materialización + Indexación ciego |

`skill:filesystem-manager` eliminado de fases FS (0 matches). Índice process actualizado.

## Sellos

4× `Domain_Entity_Updated` en `.events/pending/` con hash post-mutación (ver `execution.md`).

## Regresión

- `cargo test -p execute-process capability_di` → 17 ok
- `cargo test -p execute-process cerbero_di` → 7 ok
- Smoke: `process-creator` conserva `fs:persist` (H5)
- `audit-eda-coverage --scan --json` → `orphan_count: 0`
- `verify-process-integrity` → OK
