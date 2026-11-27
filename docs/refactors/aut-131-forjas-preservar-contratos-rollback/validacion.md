---
feature_name: aut-131-forjas-preservar-contratos-rollback
created: "2026-11-27"
updated: "2026-11-27T08:45:00+01:00"
process: refactorization
branch: refactor/aut-131-forjas-preservar-contratos-rollback
branch_name: refactor/aut-131-forjas-preservar-contratos-rollback
persist_ref: docs/refactors/aut-131-forjas-preservar-contratos-rollback
tracker_ref: AUT-131
tracker_provider: linear
source_process: refactorization
global: PARCIAL
pbi_archived: false
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
  AC-5: APTO
  AC-6: APTO
  AC-EM: APTO
  global: PARCIAL
  F3_TECH_GATE: APTO
  DOC_CASCADE: APTO
  DOC_EVOLUTION: APTO
---

# Validación local — AUT-131 (pre-PPR)

`global: PARCIAL` — evidencia Tekton local; **pendiente** `pull-request-review` (Argos) y CI GitHub.

## Evidencia ejecutada

| Oráculo | Comando | Resultado |
| --- | --- | --- |
| AC-6 / KA-FORGE-6 | `cd SddIA/engine && cargo test -p execute-process forges` | **PASS** — `65 passed; 0 failed; 1 ignored` (lib) |
| AUT-131 focal | `cargo test -p execute-process aut131` | **PASS** — `14 passed; 0 failed` |
| AC-EM | `cargo test -p execute-process action_update_inputs_carry_only_seed_fields` | **PASS** — `1 passed` |
| Evolution | `SddIA/target/debug/sddia-qa gate-evolution --json --range --sync-base` | **PASS** — `reason_codes: ["EVOL_OK"]` |

## Matriz AC ↔ tests

| AC | Test(s) | Estado |
| --- | --- | --- |
| AC-1 | `action_update_preserves_frontmatter_not_in_payload`, `action_update_preserves_contract_and_index_tail` | APTO |
| AC-2 | `sync_action_index_row_last_row_keeps_trailing_sections` | APTO |
| AC-3 | `action_update_anemic_body_rolls_back_file_and_index` | APTO |
| AC-4 | `suite_update_applies_yaml_and_body_together`, `suite_update_failing_body_rolls_back_yaml` | APTO |
| AC-5 | `agent_update_yaml_only_refreshes_hash`, `agent_update_failure_restores_agent` | APTO |
| AC-6 | filtro `forges` (suite completa) | APTO |

## Git (rama de trabajo)

- Base observada: `main` @ `dea3917b` (merge PR #480).
- Rama: `refactor/aut-131-forjas-preservar-contratos-rollback` @ `dea3917b` + cambios locales sin push.

## Pendientes antes de auditoría

1. Commit + push rama.
2. PR hacia `main` con tracker `AUT-131`.
3. CI verde (`sddia-index-qa`, tests Rust).
4. `./sddia-run.sh --process pull-request-review`.
5. `delivery-close-cycle` y issue Linear → `Done`.