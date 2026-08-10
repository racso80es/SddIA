---
feature_name: ocean-dpp-v2
created: "2026-08-10"
process: feature
phase: Ejecución correcciones
agent: tekton
branch: feat/ocean-dpp-v2-6416012709005756507
persist_ref: docs/features/ocean-dpp-v2
---

# Implementation — correcciones Ocean DPP V2

## Cambios aplicados (plan C1–C5)

### C1 — `route_domain_core.rs`
- `is_valid_iota_anchor`: rechaza `"batched-digest"`; acepta digest real / `merkle_anchored`.
- Pre-lote Merkle sella `cumulo`, `transaction_digest` real, `merkle_root`, `merkle_anchored`.
- `batch_mode_iota`: consume ancla previa (`batched-preanchored`); **nunca** inventa digest.
- Proofs vía `resolve_eda_proofs_dir` (SSOT `eda_instance.proofs`).
- Eliminado `thread_local! BATCH_MODE`.

### C2 — `eda_sweep.rs`
- Restaurado bucle `try_sweep_event` + `SweepReport` (paridad `main`).
- Sweeper no invoca `route-domain-event`.

### C3 — `event-watcher`
- Chunk físico ≤50 → `event_file_paths` (batch semántico dentro del orquestador).
- Fractal permanece unitario.

### C4 — `cumulo.paths.json`
- `eda_instance.proofs: ".SddIA/proofs"`.

### C5 — Tests / docs
- Unit `merkle_array_payload_returns_root_and_proofs` (env serializado).
- Topología `docs/features/ocean-dpp-v2/` + evolution.

## Fuera de este corte
- ZKP Groth16.
- Archivado PBI / `validacion` APTO global (pendiente re-auditoría Argos tras CI).
