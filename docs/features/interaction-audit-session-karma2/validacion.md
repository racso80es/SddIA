---
feature_name: interaction-audit-session-karma2
created: "2026-10-28"
audited_at: "2026-10-28T20:06:00+01:00"
process: feature
branch_name: feat/interaction-audit-session-karma2
persist_ref: docs/features/interaction-audit-session-karma2
pbi_ref: AUT-10
tracker_ref: AUT-10
execution_id: "c924b989-968c-434d-a9ad-5cb5edc2ae12"
pr_url: "https://github.com/racso80es/SddIA/pull/407"
ci_run_url: "https://github.com/racso80es/SddIA/actions/runs/40750063035"
global: APTO
pbi_archived: false
---

# Validación — AUT-10

| Check | Veredicto | Notas |
| --- | --- | --- |
| Tabla karma2 en shell nuevo | APTO | `docs/audits/AUT-10.md` + test `shell_has_uuid_v4_and_karma2_header` |
| Dedup cuatro campos | APTO | test `dedup_row` |
| Enum `entity_type` | APTO | test `rejects_invalid_entity_type` |
| Peaje lifecycle | APTO | `thermodynamic` + `maybe_record_process_peaje` |
| Tests Rust | APTO | `cargo test -p execute-process llm_interaction_audit --lib` → 5/5 |
| Smoke AC-2 | APTO | `record_interaction` → fila en `docs/audits/AUT-10.md` |
| CI PR #407 | APTO | run `40750063035` — jobs `sddia-index-qa` + installer smoke verdes |

## global

**APTO** — listo para `accept-pr`. Cierre Linear **Done** tras merge (`done_gate: linear`).
