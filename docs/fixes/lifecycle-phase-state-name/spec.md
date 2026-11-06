---
feature_name: lifecycle-phase-state-name
created: "2026-11-06"
process: bug-fix
phase: design
agents: dedalo
branch_name: fix/lifecycle-phase-state-name
sighting_branch: fix/lifecycle-phase-state-name
persist_ref: docs/fixes/lifecycle-phase-state-name
document_id: PBI-adfcf4cf
uuid: adfcf4cf-ae0e-4dea-95ca-ac91e8e1071d
execution_id: "090e6ae4-6227-4ab7-b25c-64dab6741fe4"
status: dedalo_locked
sighting: "DCC AUT-76 lifecycle_pr_opened LINEAR_GRAPHQL_ERROR request.state_id obligatorio"
---

# Spec — lifecycle-phase-state-name

## 1. Misión

`lifecycle_phase_action` habla el mismo contrato Linear que `tracker-stamp` / `tracker-sync-replay`: `state_name` = valor de `state_map`, no la clave canónica.

## 2. Problema verificado

| Hecho | Evidencia |
| --- | --- |
| Manifiesto ya mapea `in_review: In Review` | `.SddIA/project.md` |
| Writer de fase envía `state_key: in_review` | `lifecycle_phase_action::tool_update_state` |
| Stamp envía `state_name` + `team_key` + `project_id` | `tracker_stamp::tool_update_state` |
| Cápsula solo acepta `state_name` o `state_id` | `linear-tracker-adapter::run_update_issue_state` |
| DCC AUT-76 | `lifecycle_pr_opened.error`: `LINEAR_GRAPHQL_ERROR: request.state_id obligatorio` |
| File-provider no está roto | `aut75_*` usan `tracker_column` canónico |

## 3. Laudos

| Ref | Decisión |
| --- | --- |
| **L-MAP** | `state_name = state_map[canonical]`. Sin clave → `state_map sin {k}`, sin invocar cápsula. |
| **L-ONE** | Un helper `pub(crate)`. Stamp, replay y lifecycle lo usan. Prohibido segundo dialecto. |
| **L-CAP** | No enseñar `state_key` al adaptador Linear. |
| **L-FILE** | `apply_file` sigue con clave canónica. |
| **L-VEH** | `bug-fix`. Sin mutación de genoma de proceso. Sin `entity-manager`. |
| **L-PR** | Ítem nuevo. No mezclar en PR #438. Sin `docs/todos/`. `tracker_ref` Linear pendiente de `forge-pbi`. |

## 4. Criterios de aceptación

1. Payload remoto de lifecycle ≡ stamp: `state_name` + `team_key` + `project_id`.
2. Test lab: `apply_pr_opened` deja `update_issue_state` con `state_name: "In Review"` y **sin** `state_key`.
3. Mapa sin `in_review` → error `state_map sin in_review`, cápsula no invocada.
4. `aut75_*` (provider file) siguen verdes.
5. `validacion.md` APTO. Un PR.

## 5. Fuera de alcance

- Alta de estados Linear.
- Cambiar valores de `.SddIA/project.md`.
- Enseñar `state_key` al adaptador.
- Cuerpo/DoR de AUT-76 y merge del PR #438.
- `linear_done_gate_both_e2e` (lab propio con `state_key`).
