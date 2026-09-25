---
feature_name: kaizen-ci-chronic-wasi-runtime-smoke
created: "2026-09-25"
process: feature
purpose: Estabilización del PBI v1.1.0; separar cuota histórica de rojo vigente
execution_id: "915ff612-6aad-4d82-b92a-6d455d2cfe3d"
pbi_ref: docs/todos/pending/[KAIZEN] CI crónica — wasi-runtime-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-WASI-RUNTIME-SMOKE
pbi_uuid: "30a026cf-8227-4315-a521-53ec34a8cc0a"
pbi_version: "1.1.0"
---

# Clarificación — kaizen-ci-chronic-wasi-runtime-smoke

Init: `./sddia-run.sh --process feature` con `execution_id` `915ff612-6aad-4d82-b92a-6d455d2cfe3d`. Rama `feat/kaizen-ci-chronic-wasi-runtime-smoke`. Mayeuta simulada; Dedalo…cierre en phase-barrier (`prior_agent_phase_not_executed`). Relevo IDE. Stash `wip-ajeno-antes-kaizen-wasi` queda fuera del ciclo.

## Decisiones

| ID | Laudo |
|----|-------|
| L-JOB | El job se llama `wasi-runtime-smoke`. En la muestra `102973508239` y en el run de `main` `35526123813` el step `WASI CI smoke` acabó en success. |
| L-QUOTA | `failure_count: 3` es cardinalidad de `check_run_id`. SHA únicos: 2. Los checks `102973487853` y `102973508239` son push y pull_request del mismo `8ea6be51`. |
| L-HIST-WASI | La fila `102049913900` es compile `zstd-sys` → `wasm32-wasip1` (`bits/libc-header-start.h`). Mitigada al excluir `thought-graph-access`. No se reabre. |
| L-HIST-DELTA | Las filas gemelas fallan en `evolution gate (delta)` / `EVOL_MATERIAL_UNREGISTERED` (9 paths). No se reescribe esa evolución: el SHA no es `HEAD`. |
| L-LIVE | Objeto del ciclo: `83d6eb73-0936-4acb-9f1e-5d519987f1ab.md` con `hash_integrity: sha256:pending-pr-292`. Universo del run `35526123813`: un solo finding `EVOL_HASH_MISMATCH`. Delta de ese run: `EVOL_OK`. |
| L-REHASH | Hash solo vía `sddia-qa evolution-rehash --id`. Prohibido pegar un sha256 inventado. |
| L-MSG | El mensaje de `f3b45954` no describe el diff (edita el PBI Paciente 0). No introdujo el placeholder (`6c9194f`, PR #292). No se revierte. |
| L-SENSOR | Deduplicar el ledger por `head_sha` y separar el evolution gate del job quedan fuera. No son el rojo de `main`. |
| L-CI | `global: APTO` y archivo del PBI solo tras run `pull_request` con `wasi-runtime-smoke` en success. |
