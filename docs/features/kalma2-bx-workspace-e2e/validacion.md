---
feature_name: kalma2-bx-workspace-e2e
process: bug-fix
branch: feature/kalma2-workspace-1xn-sequential
global: PENDIENTE_EJECUCION
pbi_archived: false
pbi_ref: docs/todos/pending/[ARQUITECTURA] Workspace 1×N — ciclo bug-fix real sobre BarcelonaXplorer.md
---

# Validación — E2E BarcelonaXplorer (pendiente)

**Cliente:** Kalma2 de Paciente 0 — `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA` (no forja lab ni `SddIA_AP`).

1. Host operativo APTO (bridge + `wui_http`; PBI operativo cerrado o residual documentado).
2. `active-domain-profile.json` en Paciente 0 con `software_forge: true`.
3. IDEs Antigravity/Cursor cerrados; `SDDIA_AGENT_RUNTIME_COMMAND` activo.
4. Kalma2: selector `barcelonaxplorer`, prompt `bug-fix` con PBI real en BX, Forjar Proceso.
5. Evidencia: diff en BarcelonaXplorer, `qa_gates` verdes, telemetría `Raw_Execution_Finished`, cierre según `delivery_mode`.

Al completar: `global: APTO`, `pbi_archived: true`, mover PBI a `done/` y archivar HU si es el último pendiente.
