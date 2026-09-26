---
feature_name: sddia-deterministic-teardown
created: "2026-09-26"
process: feature
purpose: Limpieza determinista simétrica al installer; defaults PR #300 no Paciente 0
version_clarify: "1.0.0"
execution_id: "0e8d3078-8bcb-4d45-91b9-5dade5b6b641"
pbi_ref: docs/todos/pending/[DEUDA] Paciente 0 — prompt de teardown.md
document_id: PBI-DT-PACIENTE0-UNDEPLOY-PROCESS
---

# Clarificación — sddia-deterministic-teardown

Init: `./sddia-run.sh --process feature` + relevo IDE + skips + `ALLOW_DIRTY` (PBI destilación untracked, no entra). `execution_id` `0e8d3078-8bcb-4d45-91b9-5dade5b6b641`.

## Decisiones

| ID | Laudo |
|----|-------|
| L-NO-DUP | Cero segundo motor. Wipe = `sddia-installer.sh teardown`. |
| L-DEFAULT | Mismos defaults que deploy: `--root` > `SDDIA_INSTALL_ROOT` > `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`. **No** `/home/racso/Proyectos/SddIA_AP`. |
| L-PROTO | Deuda: SIGTERM `{ROOT}/start-sddia.sh` → stop/disable/reset-failed `@${ESC}` → residuales cwd/exe ∈ ROOT → `rm -rf ROOT`. No factory. No unidades de forja (ESC distinto). |
| L-PKILL | Residuales vía `/proc` + locks. Prohibido `pkill -f` ciego. |
| L-SAFE | Abort ROOT vacío, `/`, `$HOME`, forja o bajo forja. No borrar vaults `*.deploy-vault` / `.dev` de forja. |
| L-FORCE | CLI `teardown` sin `--force` → exit 3. Atajo `SddIA_Teardown.sh` sin args inyecta `--force` (el gesto es el consentimiento). `--dry-run` no exige `--force`. |
| L-DA2 | No forjar `paciente0-undeploy`. Deuda permanece pending. |
| L-HOST | Atajo `/home/racso/Aplicaciones/SddIA/SddIA_Teardown.sh` (fuera de git). |

## Fuera

Proceso Core undeploy. Deploy live. Wipe `SddIA_AP` salvo `--root` explícito.
