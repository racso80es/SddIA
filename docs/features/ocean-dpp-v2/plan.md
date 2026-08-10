---
feature_name: ocean-dpp-v2
created: "2026-08-10"
process: feature
phase: Plan de corrección
agent: dedalo
branch: feat/ocean-dpp-v2-6416012709005756507
persist_ref: docs/features/ocean-dpp-v2
---

# Plan de corrección — Ocean DPP V2

Orden táctico derivado de `auditoria.md`.

## C1 — Digest e idempotencia (`route_domain_core.rs`)

1. Tras publish Merkle exitoso, sellar por UUID:
   - `delivery_state.cumulo = "success"`
   - `delivery_state.transaction_digest = <digest real>`
   - `delivery_state.merkle_root = <root>`
   - `delivery_state.merkle_anchored = true`
2. En `batch_mode_iota`, el subscriber IOTA:
   - Si ya hay ancla válida → `success` sin republicar ni pisar digest.
   - Nunca emitir `"batched-digest"`.
3. Gate pre-lote: ancla válida = digest no vacío ≠ `"batched-digest"` **o** `merkle_anchored == true` **o** claves `cumulo*`.
4. Eliminar `thread_local! BATCH_MODE` muerto.

## C2 — Restaurar sweeper (`eda_sweep.rs`)

1. Recuperar bucle `try_sweep_event` + población de `SweepReport` (paridad `main`).
2. Prohibido invocar `route-domain-event` desde el sweeper.
3. Mantener `sweep_fractal_bus` al final.

## C3 — Chunk físico en watcher (`event-watcher`)

1. Para `route-domain-event`, agrupar paths pendientes en chunks ≤50.
2. Invocar `--inputs '{"event_file_paths":[...]}'`.
3. Fractal / otros procesos: sin cambio (unitario).

## C4 — SSOT proofs

1. Añadir `eda_instance.proofs: ".SddIA/proofs"` en `cumulo.paths.json`.
2. Resolver directorio desde config en `route_domain_batch` (fallback documentado al path canónico).

## C5 — Tests y docs

1. Unit test publisher: array → `merkle_root` + N proofs; lab-simulate.
2. `implementation.md` + `validacion.md` + entrada `SddIA/evolution/`.
3. Smoke local: build publisher + validate-merkle-proof.py.

## No tocar en este corte

- Norma DEP (ya sellada).
- ZKP.
- Mutación de índices de agentes vía creators (mandatos Jules ya en rama).
