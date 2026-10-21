---
feature_name: kaizen-ci-chronic-eda-bus-e2e-smoke
created: "2026-10-21"
process: feature
purpose: Estabilización del PBI v1.1.0; separar cuota histórica del rojo WASI vigente
execution_id: "0db56ca9-0fe4-4e81-96cd-88f5c52fcdd4"
pbi_ref: docs/todos/pending/[KAIZEN] CI crónica — eda-bus-e2e-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-EDA-BUS-E2E-SMOKE
pbi_uuid: "f2bbed15-2cca-49ae-a439-aff1a935e43b"
pbi_version: "1.1.0"
---

# Clarificación — kaizen-ci-chronic-eda-bus-e2e-smoke

Init: `./sddia-run.sh --process feature` con `execution_id` `0db56ca9-0fe4-4e81-96cd-88f5c52fcdd4`. Rama `feat/kaizen-ci-chronic-eda-bus-e2e-smoke`. Mayeuta simulada; Dedalo…cierre en phase-barrier (`prior_agent_phase_not_executed`). Relevo IDE. Sin `project_slug`: el slug `core` dispara `DOMAIN_AUTHORITY_DENIED` porque el perfil de instancia no tiene `software_forge`. Core-self entra por autoridad legada (`git_required: true`, `codex_slug` vacío).

## Decisiones

| ID | Laudo |
|----|-------|
| L-JOB | El job se llama `eda-bus-e2e-smoke`. La muestra `119107147785` y los push de `main` desde #356 mueren en `Build WASI capsules`, antes del lab EDA. |
| L-QUOTA | `failure_count: 3` son tres `head_sha`. Las filas `111402814783` y `111409283409` son el lab Linear del 2026-10-04. La fila `119107147785` es el `E0554` de `redb`. |
| L-HIST | El lab Linear no se reabre. El run de #355 (`39554780138`) dejó `eda-bus-e2e-smoke` en success. |
| L-LIVE | Objeto del ciclo: `event-bus-audit` entra en el descubrimiento WASI y arrastra `redb =2.6.4`. En `wasm32-wasip1`, `target_os = wasi` y el canal stable rechaza `feature(wasi_ext)`. |
| L-EXCLUDE | El arreglo es una línea en `EXCLUDE`, mismo criterio que `thought-graph-access`. Pin y toolchain quietos. |
| L-SIBLING | `wasi-runtime-smoke` falla en el mismo step. CA4 lo cubre. No se abre otro PBI crónico. |
| L-CI | `global: APTO` y archivo del PBI solo tras run `pull_request` con `eda-bus-e2e-smoke` y `wasi-runtime-smoke` en success. |
