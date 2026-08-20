---
feature_name: feature-revoked-registry-rehab
created: "2026-08-20"
process: refactorization
phase: execution
agents: tekton
items_applied:
  - T0-fail-soft-padre
  - T1-poda-hollow
  - T2-instance-rehab
  - T3-docs
branch_name: refactor/feature-revoked-registry-rehab
persist_ref: docs/features/feature-revoked-registry-rehab
document_id: PBI-FEATURE-185-REVOKED-REGISTRY
uuid: c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f
execution_id: "3027bd2e-295e-4c24-8955-a9a913ae896e"
---

# Execution — feature-revoked-registry-rehab

## T0 / T1 (motor)

- DCC: `delivery_push` copiado a `data`.
- Padre: `capsule_feature_invoke_delivery_close` → `invoke_process_full`; `feature_dcc_parent_fail_soft` (veto causal + fallback publicación remota).
- Residual: sin patch (copia `Ok.fail_soft`; `Err` causal).
- REF: `cycle_phase` + `lab_hollow` solo skip de cierre.
- Batch: `survival_hollow` antes de `samples`.
- Tests añadidos: `hard_when_push_but_apertura_failed_without_fail_soft`, `physical_fallback_from_remote_phase_executed`, `hollow_initialized_is_cycle_phase_not_lab_flag`, `thresholds_110_process_intact`.

`cargo test -p execute-process --lib -- …` en esta sesión: **Shell Rejected**. Sin acuse de tests. No se declara APTO por cifra de tests.

## T2 (instancia · fuera del PR)

Locus Cúmulo: `radamanto.revoked_entities` = `.SddIA/cerbero/revoked_entities.json`; `radamanto.stats` = `.SddIA/radamanto/stats.json`.

| Check | Resultado |
|-------|-----------|
| `permanent.feature` | **ausente** (`permanent: {}`) |
| `revoked.feature` | **ausente** |
| `revoked.bug-fix` / `emit-pr-audited-event` | intactos |
| stats raíz `feature` | `healthy` · `recovery_attempts: 0` · `consecutive_success_count: 0` · `degraded_at: null` · `samples: []` · `rehab_laudo: PBI-FEATURE-185-REVOKED-REGISTRY` · `rehabilitated_at: 2026-08-20T05:40:37Z` |
| fósil `entities.feature` / `process:feature` | no mutados |
| ontología Cerbero | clave `feature` no reescrita (rehab = borrado); `entity_type: process` no regresionado a `tool` |

## T3

Cascada `implementation.md` / `execution.md` + evolution `c041bfd2-3be0-4956-83ec-be28fadee390`. Umbrales JSON no tocados.

Git `./sddia-run.sh --tool git-manager`: no ejecutado en esta sesión (Shell Rejected previo en ciclo; T5 = commit/PR). PBI sigue en `docs/todos/pending/` (T5).

## Pendiente runtime

T4 Argos → `validacion.md`. T5 archive PBI + DCC. Tests crate: re-ejecutar cuando el invocador Shell esté habilitado.
