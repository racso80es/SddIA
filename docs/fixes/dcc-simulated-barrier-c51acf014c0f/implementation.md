---
feature_name: dcc-simulated-barrier-c51acf014c0f
created: "2026-08-30"
process: bug-fix
branch_name: fix/dcc-simulated-barrier-c51acf014c0f
persist_ref: docs/fixes/dcc-simulated-barrier-c51acf014c0f
items:
  - executor/simulated_relay_barrier
  - phase_capsules/delivery_close_validacion_guard
  - phase_capsules/gh_pr_telemetry
  - delivery_close/gate_block_fracture_suppression
---

# Implementation — fractura `c51acf014c0f`

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `executor.rs` | `simulated_relay_blocks_close`: si proceso con cascada documental (`bug-fix`/`feature`/`refactorization`), fase agente `simulated` y sin `{persist_ref}/validacion.md` → `barrier_prior = awaiting_agents`; salta cierre documental y DCC |
| `phase_capsules.rs` | `capsule_feature_invoke_delivery_close`: guarda `validacion.md ausente` → `awaiting_agents` (red de seguridad F2) |
| `phase_capsules.rs` | `capsule_delivery_gh_pr`: error `pr_url` incluye `gh_stdout`/`gh_stderr`/`view_stdout`/`view_stderr` truncados |
| `delivery_close.rs` | `dcc_gate_block_suppresses_fracture`: `Aduana evolution` / `Aduana EDA genómica` con `blocked` no emiten `System_Fracture_Detected` |

## Contrato (implementado)

- Relevo IDE sin `validacion.md` → acuse limpio (`success: true`), fases de cierre `skipped` vía `phase-barrier`.
- `simulated` + `validacion.md` presente → cierre permitido (sin castrar DCC verificado).
- Gate de aduana evolution/EDA conserva `status: blocked`; solo se suprime escalado a dominio.
- Telemetría de forja: error accionable sin re-ejecutar `gh`.

## Fuera de alcance (respetado)

- Sin mutación de genoma `delivery-close-cycle.md` (AEL-CA9).
- Sin armado de `core.hooksPath` en bootstrap.
- Higiene PR #231 / snapshot `b0a4bde` revertido en esta rama (instancia/genoma ajeno).
