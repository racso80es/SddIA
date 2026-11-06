---
feature_name: lifecycle-phase-state-name
created: "2026-11-06"
process: bug-fix
phase: blueprint
agents: dedalo
phases: T0-T4
branch_name: fix/lifecycle-phase-state-name
sighting_branch: fix/lifecycle-phase-state-name
persist_ref: docs/fixes/lifecycle-phase-state-name
document_id: PBI-adfcf4cf
uuid: adfcf4cf-ae0e-4dea-95ca-ac91e8e1071d
execution_id: "090e6ae4-6227-4ab7-b25c-64dab6741fe4"
---

# Plan — lifecycle-phase-state-name

Parada actual (**L0**): `objectives.md`, `spec.md`, `plan.md`. Rama: `fix/lifecycle-phase-state-name`. Base: `main` (sin el motor AUT-76). Código en el estímulo de Ejecución.

## T0 — Sellar planificación (esta entrega)

| Paso | Acción |
| --- | --- |
| T0.1 | Frontmatter coherente (`execution_id` `090e6ae4-…`, `document_id`, `uuid`). |
| T0.2 | `spec.md` + `plan.md`. Prohibido `implementation.md` en este sello. |
| T0.3 | Commit de planificación cuando el operador lo pida. |

## T1 — Helper único

1. Extraer de `tracker_stamp::tool_update_state` un `pub(crate)` que traduzca `canonical` → `state_name` vía `state_map` y arme el request (`state_name`, `team_key`, `project_id`, `issue_ref`).
2. `lifecycle_phase_action::tool_update_state` delega. Deja de emitir `state_key`.
3. `tracker-sync-replay` usa el mismo helper si el JSON se construye a mano.

## T2 — Test

1. Lab outbound: `apply_pr_opened_fail_soft` (o `apply_remote`) registra `update_issue_state` con `state_name: "In Review"` y sin `state_key`.
2. `state_map` sin `in_review` → `state_map sin in_review`; la cápsula no se invoca.
3. `cargo test -p execute-process --lib aut75` sigue verde (file-provider).

## T3 — Evolution

Alta canónica al tocar `execute-process`. Ligar `document_id` PBI-adfcf4cf y el sighting AUT-76 / PR #438.

## T4 — Cierre

`implementation.md` + `execution.md`. `validacion.md` APTO. Un PR. Sin `docs/todos/`.

Verificación viva de columna Linear: post-merge o issue de lab; fuera de T1–T2.
