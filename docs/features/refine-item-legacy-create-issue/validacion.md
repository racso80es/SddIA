---
feature_name: refine-item-legacy-create-issue
created: 2026-11-01
process: refactorization
branch_name: Fix_Events_Multiple
persist_ref: docs/features/refine-item-legacy-create-issue
tracker_ref: AUT-35
execution_id: 27ab6def-6796-4baa-a3e6-c569f96efd72
global: APTO
pbi_archived: false
branch: Fix_Events_Multiple
approval_status: aprobado
verdict: aprobado
validated_base_sha: 2d14642644f1acd7ebfa1fa32bac0f941f817736
checks:
  AUT35-T1-LEGACY-CREATE-ISSUE-REMOVED: APTO
  AUT35-T2-TRACKER-REF-PREDICATE: APTO
  AUT35-REFINE-ITEM-LIB-TESTS: APTO
  AUT35-REFINE-AC0C-TESTS: APTO
  DOC_CASCADE: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/handlers/refine.rs
- SddIA/engine/execute-process/src/engine/handlers/refine_item/tracker_ref.rs
- SddIA/engine/execute-process/src/engine/handlers/refine_item/run.rs
- SddIA/engine/execute-process/src/engine/handlers/refine_item/mod.rs
- docs/features/refine-item-legacy-create-issue/
- docs/audits/AUT-35.md
validated_head_sha: 2773cf7a6ea6764cdfc9e2da63fd3f4b00d2c44b
validated_tree_sha: 5e95979a6c60768682b7e2e4b1690b4d95196c9a
diff_digest: sha256:c0b4d4f5ea07ed2983bcd69e199a4ec738633dcd36ba72ab4163d223ace5ffb2
---

# Validación — AUT-35

## Veredicto

**APTO** — Residuo `try_create_hu_issue` retirado de `refine-hu` legacy; `refine-item` centraliza resolución remota de `tracker_ref` en predicado testeado.

## Evidencia

| Check | Medio | Resultado |
|-------|--------|-----------|
| T1 | `rg try_create_hu_issue SddIA/engine` | 0 coincidencias |
| T2 | `tracker_ref.rs` + delegación en `run.rs` | 4 tests predicado OK |
| Lib | `cargo test -p execute-process --lib refine_item` | 8/8 OK |
| Legacy HU | `cargo test -p execute-process --lib ac_0c` | 2/2 OK |

## Nota

`linear_direct_cycle_e2e` ac7/ac8/ac9 fallan por `tracker-stamp` / store (preexistente en rama); no bloquean el alcance AUT-35.

## Cierre tracker

Provider `linear`: **AUT-35** → Done con merge del PR; sin fichero en `docs/todos/done/`.