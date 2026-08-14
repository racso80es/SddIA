---
feature_name: kalma2-phase-barrier-timeout-persist
created: "2026-08-14"
process: refactorization
phases: T0-T5
branch_name: refactor/kalma2-phase-barrier-timeout-persist
persist_ref: docs/features/kalma2-phase-barrier-timeout-persist
pbi_ref: docs/todos/done/[REFACTOR] Kalma2 — serialización de fases, timeout y rama refactor (KALMA2-AUD-4b9de6).md
document_id: 1de0bdd1-6144-4e45-8efa-92db0f399147
agents: dedalo
execution_id: d630a6cf-1767-4751-a2b9-b1f4210a01fb
---

# Plan — kalma2-phase-barrier-timeout-persist

## T0 — Prefijo y persist_ref (fundación 003/004)

1. `workspace_init.rs`: `canonicalize_branch_name`; slug `refactor/` en `workspace_task_name`.
2. `task_queue_manager.rs`: default `refactor/{slug}`; inyectar `persist_ref`.
3. `agent_runtime.rs`: fallback persist desde `inputs` / `state.workspace`.
4. `eda_bus_topology.rs`: `refactor/` → `featurePath`.
5. Tests unitarios TQM + workspace-init.

## T1 — Barrera de fase (002)

1. Helpers en `executor.rs` (`agent_phase_blocks_downstream`, skip entry).
2. Loop `run_generic` con barrera.
3. Skip lab cierre también para `refactorization`.
4. Test: fase Ejecución failed → Verificación skipped.

## T2 — Timeout runtime (001/005)

1. `is_soft_config_error` sin `"timeout"`.
2. `resolve_timeout_secs(phase)` + override Ejecución.
3. `build_prompt`: persist desde `inputs` si el top-level está vacío.
4. Docstring env. Test Python importlib.

## T3 — Compilar y unit

```text
cargo test -p execute-process --lib
python3 SddIA/scripts/tools/test_kalma2_runtime_timeout.py
```

## T4 — Documental Tekton

`implementation.md` + `execution.md`.

## T5 — Verificación / cierre (Argos + PBI archive)

Fuera de esta tanda de forja: Argos escribe `validacion.md`; PBI a `docs/todos/done/`; `delivery-close-cycle`.

## Orden innegociable

T0 → T1 → T2 → T3. No subir timeout default. No tocar `docs/features/evolution-history-normalization/`.
