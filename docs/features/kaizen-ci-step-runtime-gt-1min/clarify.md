---
feature_name: kaizen-ci-step-runtime-gt-1min
created: "2026-09-01"
process: feature
purpose: Estabilización Mayeuta — calor residual steps CI >60s
version_clarify: "1.1.0"
execution_id: "a13e2476-8474-49ef-ab2f-0d1fe915a21f"
pbi_ref: docs/todos/pending/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
document_id: PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN
uuid: "530039c9-100b-413a-b3d5-ca632d83acc6"
---

# Clarificación — kaizen-ci-step-runtime-gt-1min

Transcript Mayeuta. Semilla: PBI v1.1.0. Init lab `execution_id` `a13e2476-8474-49ef-ab2f-0d1fe915a21f`. Relé IDE.

## D0 — Apertura

| Pregunta | Decisión |
|----------|----------|
| Vehículo | `--process feature`. Relé `SDDIA_AGENT_RELAY_IDE=1`. Init con skip archive/DCC; DCC al cierre. |
| Rama | `feat/kaizen-ci-step-runtime-gt-1min` |
| `persist_ref` | `docs/features/kaizen-ci-step-runtime-gt-1min` |
| Superficie | YAML CI + tests integración `execute-process` + evolution + cascada. Sin mutar `sddia-qa`. |
| Stop planning | Commit de Diseño (clarify/objectives/spec/plan). Ejecución en la misma sesión tras ese commit. |

## D1 — Baseline (anti-alucinación)

Run [33477170741](https://github.com/racso80es/SddIA/actions/runs/33477170741) `pull_request`, job 99758755444 = 816 s. Tres steps >60 s: cache 62 s, verify-compiled-capsules 340 s, LanceDB 361 s. A0: ningún step hermano >60 s.

## D2 — Palancas reales (no las de v1.0.0)

| Rechazado | Motivo |
|-----------|--------|
| `-p` SCAN_ROOTS en vez de `--workspace` | El gate exige 29 bins |
| Mover ingest tests al adapter evolution | Ciclo de deps |
| Maquillar CA1 partiendo el gate a I/O | PBI §2.1 / CA1 anti-maquillaje |

| Adoptado | Motivo |
|----------|--------|
| Key `native-integrity-*` + iota `lookup-only` | First-write-wins de `native-*` compartida |
| Un `cargo build --workspace` | Elimina aduana 29 s redundante; CA1 = suma build+verify |
| `--test memory_evolution_ingest` | Evita `cfg(test)` de 366 tests |

## D3 — Forja

YAML `.github/` ∉ DA-2. `engine/execute-process` ∉ tabla DA-2. Evolution vía `sddia-qa evolution-register`. `SddIA/tools/` no se toca.

## D4 — A3.2 (PBI v1.2.0)

| Palanca | Colocación |
|---------|------------|
| `SCCACHE_GHA_ENABLED=true` | `env` job integrity + IOTA (antes del action) |
| `RUSTC_WRAPPER=sccache` | `GITHUB_ENV` **después** de `sccache-action` |
| `CARGO_INCREMENTAL=0` | `env` job (ya presente; no revertir) |
| Medición CA1/CA5 | SHA-2 del mismo PR; SHA-1 = calentamiento si stats requests > 0 |
