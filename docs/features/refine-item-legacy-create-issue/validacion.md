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
validated_base_sha: "2d14642644f1acd7ebfa1fa32bac0f941f817736"
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
validated_head_sha: "6ac2b651c54bbbab8f5b14cb1cc4eb6aaabb7fa4"
validated_tree_sha: "520520befed83c2b688eaaf303c1320b1414b6ab"
diff_digest: "sha256:0971d1c7bde471cbf30ba394620c0c3513f6cd6c3ee32f41c29969fe93134121"
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