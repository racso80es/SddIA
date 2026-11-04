---
feature_name: aut-75-lifecycle-state-actions
created: "2026-11-04"
process: bug-fix
branch_name: Fix_Events_Multiple_03
persist_ref: docs/fixes/aut-75-lifecycle-state-actions
pbi_ref: AUT-75
execution_id: "e7138ac5-71fe-4407-b025-ab1eca978f35"
---

# Implementación — AUT-75

## T1 — `lifecycle_phase_action`

Módulo `SddIA/engine/execute-process/src/engine/lifecycle_phase_action.rs`: hitos `ForgeStarted`, `ForgeClosed`, `ValidationApto`, `ValidationNoApto`, `PrOpened`, `ReviewCiGreen`, `ReviewCiRed`. Metadatos vía parches en `item_lifecycle_frontmatter`.

## T2 — Validated

- `COLUMN_VALIDATED` + parches YAML.
- `.SddIA/project.md`: `state_map.validated: Validated`.

## T3 — Enganches

| Fase | Acción |
| --- | --- |
| `workspace-init` | `apply_forge_started_fail_soft` |
| Entrada `Verificación` | `maybe_apply_forge_closed` si `implementation.md` + `execution.md` |
| `Verificación` / `Cierre documental` | `maybe_apply_validation_from_validacion` (`global` en `validacion.md`) |
| `Sello Presentación ECST` (DCC) | `apply_pr_opened_fail_soft` |
| `accept-pr` sello merged | `apply_review_ci_fail_soft(true)` |
| PPR handoff rechazado | `apply_review_ci_fail_soft(false)` |

Barrera IDE: `Verificación` no se salta si la forja documental está lista o existe `validacion.md`.

`maybe_emit_validation_started_after_tekton_agent`: no-op (AUT-75).

## T4 — Suscriptor

`stamp_event`: no-op de estado para `Work_Initiated`, `Validation_Started`, `PullRequest_Presented`, `PullRequest_Merged`.

## T6 — Auditoría

`docs/audits/ciclo-vida-software-sddia.md` → v1.3.0.

## T7 — Tests

`cargo test -p execute-process aut75` — writer, Argos, stamp no-op, paridad genoma (`aut75_genome_narrative_parity`).

## D2–D6 — Paridad narrativa genoma

| Entrega | Rutas |
| --- | --- |
| Procesos patch | `feature.md` 1.3.4, `bug-fix.md` 1.4.4, `refactorization.md` 1.2.4 |
| Eventos | `work-initiated`, `validation-started`, `pull-request-presented` 1.2.1, `pull-request-merged` 1.1.1 |
| Suscripciones | `SddIA/core/event-domain-subscriptions.json` (4 intents AUT-75) |
| Inventario | `genome-alignment-inventory.md` |
| Evolution | `SddIA/evolution/f7e2a1b0-9c3d-4e5f-a6b7-8c9d0e1f2a3b.md` |
