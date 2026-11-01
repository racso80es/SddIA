---
feature_name: kaizen-disk-thermodynamics-retention
created: "2026-11-01"
process: feature
branch_name: Fix_Events_Multiple
persist_ref: docs/features/kaizen-disk-thermodynamics-retention
tracker_ref: AUT-49
items_applied:
  - L0-docs-planificacion
  - L1-entity-manager-tool
  - L2-L4-slices-crate
  - L5-entity-manager-action
  - L2-defecto-hash-generation-2026-11-01
---

# Ejecución — kaizen-disk-thermodynamics-retention

## Commits (AUT-49)

| Commit | Tarea |
|--------|--------|
| `0477a760` | L0 cascada objectives/clarify/spec/plan |
| `6cd30417` | L1 tool + jail/lock/plan vacío |
| `7ffbab11` | L2–L4 slices target/events/scratch/dist + live checks |
| `8204a2f8` | L5 action `purge-repo-retention` + handler |

## Forja

```bash
./sddia-run.sh --process entity-manager --inputs-file docs/features/kaizen-disk-thermodynamics-retention/_lab/entity-manager-repo-retention-purger.json
./sddia-run.sh --process entity-manager --inputs-file docs/features/kaizen-disk-thermodynamics-retention/_lab/entity-manager-purge-repo-retention.json
```

## Verificación local

```bash
cd SddIA && cargo test -p repo-retention-purger
cd SddIA && cargo check -p execute-process
```

Dry-run (sin apply) vía action:

```bash
echo '{"simulate_only":true,"repo_root":"'"$(pwd)"'"}' | \
  ./sddia-run.sh --action purge-repo-retention
```

(Requiere binario `repo-retention-purger` compilado en `SddIA/target/debug/`.)

## L2 — corrección slice A (2026-11-01)

Paridad con operador `clean-project-ephemeral-cargo-deps.py` (AUT-79):

- `slice_target.rs`: agrupación por hash; tests `dep_generation_keeps_rlib_and_rmeta`, `dep_hold_incomplete_generation`, `incremental_keeps_two_sessions_per_crate`.
- `live.rs` + `plan.rs`: `PROC_UNREADABLE` aborta apply (fail-closed).
- Verificación: `cd SddIA && cargo test -p repo-retention-purger` → 13 passed.

## Cierre

- **Rama actual:** `Fix_Events_Multiple` (multi-ámbito: AUT-21…34 + AUT-49). DCC/PR dedicado solo a retención: bifurcar `feat/kaizen-disk-thermodynamics-retention` desde `main` o cherry-pick commits AUT-49 antes de `delivery-close-cycle`.
- **Linear AUT-49 → Done:** tras merge del PR que incluya este `persist_ref` y `validacion.md` con `global: APTO` + CI verde (`run_id`).
