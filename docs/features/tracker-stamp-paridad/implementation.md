---
feature_name: tracker-stamp-paridad
created: "2026-10-02"
process: feature
---

# Implementación

| Área | Cambio |
|------|--------|
| `handlers/tracker_stamp.rs` | Preflight AC-18, R-2 HU desde backlog, R-5 hijos `done` |
| `tracker_pbi_meta.rs` | `attach_tracker_ref_from_persist`, herencia desde `PullRequest_Presented` |
| `linear-tracker-adapter` | Fixtures `LAB-PBI-*` / `LAB-HU-*` en lab inline |
| `phase_capsules.rs` | `tracker_ref` en `emit-pr-presented-event` |
| `accept_pr.rs` | `tracker_ref` en `emit-pr-merged-event` |
| Eventos PR | Campo OPTIONAL `tracker_ref` |

Tests: `tracker_stamp::tests::*`, `actions::pr_emits_carry_tracker_ref_when_provided`.
