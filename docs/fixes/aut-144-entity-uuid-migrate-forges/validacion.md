---
feature_name: aut-144-entity-uuid-migrate-forges
created: "2026-11-28"
updated: "2026-11-28T13:35:00Z"
process: bug-fix
phase: Verificación
branch: fix/aut-144-entity-uuid-migrate-forges
branch_name: fix/aut-144-entity-uuid-migrate-forges
persist_ref: docs/fixes/aut-144-entity-uuid-migrate-forges
tracker_ref: AUT-144
tracker_provider: linear
historia_ref: AUT-133
source_process: bug-fix
execution_id: d0b4e016-3133-4fdd-ba51-38cd393156cc
global: APTO
pbi_archived: false
git_manager_invoked: true
git_evidence_source: skill:git-manager (cápsula directa)
formal_execute_process: true
ac5_scope: motor_y_regresiones
ac5_genome_remediation_delegada: AUT-133
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
  AC-5: APTO
  global: APTO
  branch: APTO
  git_changes: APTO
  TECH_FORMAL_EXECUTE_PROCESS: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
validated_base_sha: 5b0f4a0b2a5f5a48d78705b34afb52530cc95fc3
---

# Validación — AUT-144

## Veredicto

`global: APTO` — canal `entity_uuid_migrate` en process/event/agent/skill/suite con tests de forja dedicados + regresión action; **AC-5** acotado en Linear a motor/regresiones (remediación de 20 paths + `fail-fast-lab` en **AUT-133**).

## AC-5 en Linear

`tracker-description-sync` (outbound) el 2026-11-28: AC-5 ya no exige aplicar migración en genoma dentro de AUT-144.

## Tests (`cargo test -p execute-process`)

| Test / ámbito | Familia | Resultado |
|---------------|---------|-----------|
| `migrate_entity_uuid_syncs_index_and_coverage` | action | OK |
| `process_forge_entity_uuid_migrate_syncs_index_and_coverage` | process | OK |
| `event_forge_entity_uuid_migrate_syncs_index_and_coverage` | event | OK |
| `agent_forge_entity_uuid_migrate_syncs_index_and_coverage` | agent | OK |
| `suite_forge_entity_uuid_migrate_syncs_index_and_coverage` | suite | OK |
| `migrate_entity_uuid_record_rejects_non_v4` | validación UUID | OK |
| `suite_forge_migrate_rolls_back_artifact_on_index_sync_failure` | rollback | OK |
| `with_paths_rollback_restores_all_on_error` | rollback | OK |

Suite lib: **974 passed / 2 failed / 1 ignored** — fallos aislados `ac18_linear_in_review_rejects`, `ac21_merge_stamp_then_apply_archives_without_manual_pbi_move` (done_gate Linear en instancia; no atribuibles a diffs AUT-144).

## Evolución DA-2

- `SddIA/evolution/9f67afce-39f1-4103-979a-cb5a78360bfb.md` + fila en `Evolution_log.md`.
- `sddia-qa gate-evolution --range origin/main...HEAD` tras commit conjunto (evolution + motor).

## Git (`skill:git-manager`)

Invocación cápsula directa `SddIA/target/release/git-manager` (`operation_type: status` / `commit`). Sin `--tool git-manager`.

## Diff esperado (sin genoma operativo)

- `SddIA/engine/execute-process/src/forges/{common,factory}.rs`
- `SddIA/evolution/*`
- `docs/fixes/aut-144-entity-uuid-migrate-forges/*` (sin `_agent_handoff.md`)
- Excluido: `SddIA/suites/*`, 20 paths, `.SddIA/`, `docs/audits/AUT-144.md`