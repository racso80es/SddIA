---
feature_name: kaizen-disk-thermodynamics-retention
created: "2026-11-01"
process: feature
branch_name: feat/kaizen-disk-thermodynamics-retention
persist_ref: docs/features/kaizen-disk-thermodynamics-retention
tracker_ref: AUT-49
pbi_uuid: "0d2c4bf9-8837-4842-89f2-5a0f874b203d"
status: IMPLEMENTED-L6-PENDING-CI
phases:
  - L0-docs-seal
  - L1-crate-jail
  - L2-slice-target
  - L3-slice-events-fs
  - L4-slice-scratch-dist
  - L5-action-handler
  - L6-qa-evolution-dcc
related:
  - docs/features/kaizen-disk-thermodynamics-syslog/
  - docs/features/eda-store-io-gate/
  - .SddIA/workspaces/refine-item/71a4fc2f-1f35-4379-9203-33c0215f0eae/item.refined.md
---

# Plan — kaizen-disk-thermodynamics-retention

Rama de entrega: **`feat/kaizen-disk-thermodynamics-retention`** desde **`main`** (gate AUT-56 ya mergeado). No acoplar el PR a `Fix_Events_Multiple` salvo necesidad explícita del operador.

**Parada actual (L0):** `objectives.md`, `clarify.md`, `spec.md`, `plan.md` en `persist_ref`. Commit de planificación antes de forja de genoma.

## L0 — Sellar planificación

| Tarea | Entregable |
|-------|------------|
| L0.1 | Frontmatter válido en los cuatro `.md` de diseño |
| L0.2 | Commit en rama feature: `docs(AUT-49): cascada planificación retención disco` |

## L1 — Crate + jail

| Tarea | Entregable | KA |
|-------|------------|-----|
| L1.1 | `entity-manager`: tool `repo-retention-purger.md` + crate bajo `SddIA/tools/repo-retention-purger/` | — |
| L1.2 | Módulo `jail`: canonicalize bajo `repo_root`, deny `..`, deny `events.redb` | RET-5 |
| L1.3 | CLI `--repo`, `--simulate`, `--apply --plan-digest`; lock en `.SddIA/runtime/` | L-TWO-PHASE |
| L1.4 | `cargo test -p repo-retention-purger` fixtures tempdir | RET-5 |

Dependencias crate: `sddia-event-store` (modo + open store read-only), `sddia-daemon-runtime` (topología bus) — evaluar acoplamiento; si el tool no debe depender del daemon crate, **copiar solo helpers de resolución de paths** en el tool (preferible extraer función compartida en fase L1 si el diff es pequeño; si no, duplicar defaults documentados en spec).

## L2 — Slice A (target)

| Tarea | Entregable | KA |
|-------|------------|-----|
| L2.1 | Enumeración deps por artefacto + incremental por crate | RET-2 |
| L2.2 | Exclusión `native-debug-bins`; test binario presente post-simulate | RET-3 |
| L2.3 | Detección `cargo`/`rustc` → abstención apply | L-TARGET |
| L2.4 | Test integración: dos builds `execute-process` en fixture → una generación deps | RET-2 |

## L3 — Slice B (residuo `.events`, solo `redb`)

| Tarea | Entregable | KA |
|-------|------------|-----|
| L3.1 | Listar JSON bajo dirs de spec; skip `processing/**` | RET-4 |
| L3.2 | Cruzar `dead_letter_index`; marcar `kept` | RET-4 |
| L3.3 | Modo `filesystem`/`dual` → slice vacío en plan | RET-4 |
| L3.4 | Abstención `WATCHER_ACTIVE` + `STORE_UNREADABLE` | RET-6 |
| L3.5 | Test: pending con JSON en `filesystem` no borra en simulate apply | RET-4 |

## L4 — Slices C y D

| Tarea | Entregable | KA |
|-------|------------|-----|
| L4.1 | Scratch allowlist + TTL + hold mtime 1980 | RET-7 |
| L4.2 | `dist/` retención por edad | RET-1 |
| L4.3 | Plan agrega `bytes_reclaimable` por slice | RET-1 |

## L5 — Action handler

| Tarea | Entregable | KA |
|-------|------------|-----|
| L5.1 | `entity-manager`: action `purge-repo-retention.md` | — |
| L5.2 | Handler en `execute-process` (patrón `purge_sandbox_cache.rs`) | RET-1 |
| L5.3 | Gate digest; subset de paths | L-TWO-PHASE |
| L5.4 | `./sddia-run.sh --process …` o invocación action documentada en `implementation.md` | — |

## L6 — Cierre

| Tarea | Entregable | KA |
|-------|------------|-----|
| L6.1 | `implementation.md` + `execution.md` (touchpoints por tarea L1–L5) | — |
| L6.2 | `SddIA/evolution/{uuid}.md` vinculado a AUT-49 | — |
| L6.3 | Dry-run en host documentado en `execution.md` (bytes por slice; sin apply en CI) | RET-1 |
| L6.4 | `validacion.md` `global: APTO` + `run_id` CI | RET-8 |
| L6.5 | `delivery-close-cycle`; AUT-49 → Done Linear | RET-8 |

## Orden de commits (Tekton)

Un commit por fase L1…L5 mínimo; anotar id de tarea en `execution.md`. Prohibido mezclar forja genoma y docs de cierre en el mismo commit sin trazabilidad.

## Defecto L2 — cerrado 2026-11-01

Slice A en `slice_target.rs`: generación por hash (`.rlib` + `.rmeta`), hold-incomplete, incremental por subdirs de crate. `live.rs`: apply aborta con `PROC_UNREADABLE` si `/proc` no es legible. `cargo test -p repo-retention-purger`: 13 passed.

## Fuera de este plan

- Daemon periódico (tick) — opcional post-MVP si el operador lo pide.
- Purga workspaces/proofs.
- Cambiar política `F-EDA-RESIDUAL-*` del watcher.
- Reabrir emisores del gate AUT-56.

## Patrones aplicados

Paridad operativa con action `purge-sandbox-cache` (UUID tool `8929eeea-5e5b-402a-aafa-4a9b429acaec`): dos tiempos, gate determinista, cero agentes en hot path.
