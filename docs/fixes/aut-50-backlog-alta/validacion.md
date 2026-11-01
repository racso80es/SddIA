---
feature_name: aut-50-backlog-alta
created: 2026-11-01
process: bug-fix
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-50-backlog-alta
tracker_ref: AUT-50
execution_id: 82abfda6-47a2-41f6-855f-bddbc8546bda
global: APTO
pbi_archived: false
branch: Fix_Events_Multiple_02
approval_status: aprobado
verdict: aprobado
checks:
  AUT50-CA1: APTO
  AUT50-CA2: APTO
  AUT50-CA3: APTO
  AUT50-CA5: APTO
  AUT50-T4-STORE: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs
- SddIA/tools/linear-tracker-adapter/src/main.rs
- SddIA/engine/execute-process/src/engine/handlers/forge_hu.rs
- SddIA/engine/execute-process/src/engine/handlers/refine_item/run.rs
- SddIA/sddia-event-store/src/error.rs
- SddIA/sddia-event-store/src/lib.rs
- SddIA/library/codexes/codex-software-engineering/process/forge-hu.md
- SddIA/evolution/a1b2c3d4-e5f6-4789-a012-3456789abcde.md
- docs/fixes/aut-50-backlog-alta/
validated_base_sha: c86fec895b9878cd6d80007553c161789561a93c
validated_head_sha: 1025c59afbaf2922580b3a1bc7958d0d7b9f5d33
validated_tree_sha: 936e44fafdd1ad51b7be1cc578b524dfe6155f52
diff_digest: sha256:6f32515f5921ad766fae55bdbbc0035ea8ab7a00176eea40ba2df995d8a60d55
---

# Validación — AUT-50

## Veredicto

**APTO** — T1–T4 del plan implementados; tests `aut50` en verde (serial); genoma `forge-hu` indexado vía entity-manager.

## Evidencia

| Check | Medio | Resultado |
|-------|--------|-----------|
| CA1/CA2 | `cargo test -p execute-process aut50` (forge-pbi) | OK |
| CA3 | unit test `list_issues_query` | OK |
| CA5 | `aut50_ca5_forge_hu_returns_tracker_ref` | OK |
| T4 | `StoreError::Display` + budget 10s + refine lock path | OK (código + evolution) |

## Cierre tracker

Provider `linear`: **AUT-50** → Done con el PR que incluya este diff; sin fichero en `docs/todos/done/`.