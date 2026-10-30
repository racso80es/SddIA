---
feature_name: eda-subscriber-ecst-delivery
created: "2026-10-29"
process: feature
phase: Verificación
branch: main
global: APTO
tracker_ref: AUT-46
pbi_archived: false
execution_id: "8b7e8827-f4b9-4cbb-a40a-4f68361de101"
approval_status: apto
merged_pr: "https://github.com/racso80es/SddIA/pull/412"
merge_commit: "da9ee2a248c24b5a213aa7edc3a4c542f6184941"
checks:
  AC-SPEC: PRESENT
  AC-OBJECTIVES: PRESENT
  capsules-qa: PASS
  git-evidence: PASS
pr_url: "https://github.com/racso80es/SddIA/pull/412"
---

# Validación — AUT-46

Criterios 1–5 y 7 del PBI refinado cubiertos en código y tests locales. Criterio 6 documental en AUT-45; código `terminal_witness_exists` heredado de #411.

## Evidencia

| Check | Resultado |
|-------|-----------|
| `eda_subscriber_ecst_redb` (5 tests) | PASS |
| `event_store_writers` terminal + skipped | PASS |
| CI PR #412 (run 40956409647) | PASS (incl. eda-bus-e2e-smoke) |
| `accept-pr` | merge `da9ee2a2` en main |
| `PullRequest_Merged` | evento `1d8e221d-35ec-4ecd-a86f-398eb934c2b2`; `tracker-stamp` remoto `stamped: true` |
| Linear AUT-46 | **Done** (API) |

## Notas

- **AUT-46** cerrado en Linear (`done_gate: linear`).
- **AUT-20** (HU padre) no pasa a Done por roll-up automático mientras queden hijos abiertos en Linear (p. ej. AUT-21, AUT-26, AUT-27, AUT-34 además de AUT-45/46).
- Hotfix `tracker_stamp.rs`: inferir `project_slug: core` en Core-self cuando `PullRequest_Merged` no trae slug (sello remoto sin lab).
