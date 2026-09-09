---
feature_name: kalma2-aiua-perceptive-bridge
created: "2026-09-09"
process: feature
phase: validate
agents: argos
branch: feat/kalma2-aiua-perceptive-bridge
branch_name: feat/kalma2-aiua-perceptive-bridge
persist_ref: docs/features/kalma2-aiua-perceptive-bridge
pbi_ref: docs/todos/done/[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI.md
document_id: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
uuid: "d7192a54-7389-4b68-b3d4-b91c0e35921a"
global: APTO
pbi_archived: true
pr_url: "https://github.com/racso80es/SddIA/pull/276"
ci_run_id: "34322877409"
ci_run_url: "https://github.com/racso80es/SddIA/actions/runs/34322877409"
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  CA-5: APTO
  CA-6: APTO
  CA-7: APTO
  CA-8: APTO
  CA-9: APTO
  CA-10: APTO
  CA-CI: APTO
git_changes:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - interfaces/kalma2/index.html
  - interfaces/kalma2/app.js
  - docs/features/kalma2-aiua-perceptive-bridge/
  - SddIA/evolution/bdd512bd-42ea-4c45-ab97-2e0fa8ea37c8.md
  - SddIA/evolution/Evolution_log.md
  - docs/todos/done/[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI.md
---

# Validacion — kalma2-aiua-perceptive-bridge

`global: APTO`. PBI archivado en `docs/todos/done/` en esta rama. CA-CI: run `34322877409` (PR #276, head `492c673`).

## Checks

| CA | Veredicto | Evidencia |
|----|-----------|-----------|
| CA-1 | APTO | Ruta solo en `kalma2-bridge`. `cargo check` verde. Cero Python en pasarela. |
| CA-2 | APTO | Body `{prompt}`. Test WUI: cero `localStorage`. |
| CA-3 | APTO | `setBusy` deshabilita `#aiua-pulse` + tres botones previos. |
| CA-4 | APTO | `handle_aiua_interact` spawnea `aiua-stimulus-processing`. Flatten envelope. Helper Mayeuta conserva `kalma2-interact`. |
| CA-5 | APTO | Flatten test `lab-mock: latido`. Live no gate. |
| CA-6 | APTO | PTC `source_agent: aiua`. `#cognitive-pulse` no se pisa. |
| CA-7 | APTO | `Ctrl+Enter` → `enviarChat`. Rutas chat/forge/sync intactas. |
| CA-8 | APTO | `resolve_client_timeout_secs(120, 180) == 180`. |
| CA-9 | APTO | Grep `kalma2-interact` / `caja de texto` en proceso/acciones/tool. |
| CA-10 | APTO | PBI en `docs/todos/done/`; `pbi_archived: true`. |
| CA-CI | APTO | Run [34322877409](https://github.com/racso80es/SddIA/actions/runs/34322877409): `sddia-index-integrity`, `wasi-runtime-smoke`, `eda-iota-smoke-simulate`, `eda-bus-e2e-smoke`, `eda-iota-physical` SUCCESS. |

## Tests

```text
TMPDIR=<home> CARGO_TARGET_DIR=SddIA/target cargo test --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml --bin kalma2-bridge
# 32 passed
```
