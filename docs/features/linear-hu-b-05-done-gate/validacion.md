---
feature_name: linear-hu-b-05-done-gate
persist_ref: docs/features/linear-hu-b-05-done-gate
pbi_ref: docs/todos/done/[ARQUITECTURA] Linear HU-B 05 — Gate de Done leído desde Linear.md
document_id: PBI-LINEAR-B-05-DONE-GATE
branch: feat/linear-hu-b-05-done-gate
global: APTO
pbi_archived: true
---
# Validación — HU-B 05

| Criterio | Estado |
|----------|--------|
| AC-17 | APTO — `done_gate::tests::ac17_git_still_requires_pbi_archived` |
| AC-18 | APTO — `ac18_linear_done_defers_archive`, `ac18_linear_in_review_rejects` |
| AC-19 | APTO — `ac19_both_linear_down_fallback_git_archive` |
| AC-20 | APTO — `ac20_pre_push_reads_local_proof_without_network` |
| AC-23 | APTO — `ac23_resolve_linear_gate_from_fixture_manifest` + binding test |
| CI PR | PENDIENTE — merge gate |
