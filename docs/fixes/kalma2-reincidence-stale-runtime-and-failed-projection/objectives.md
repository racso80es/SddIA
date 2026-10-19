---
feature_name: kalma2-reincidence-stale-runtime-and-failed-projection
created: "2026-10-15"
process: bug-fix
branch_name: fix/kalma2-reincidence-stale-runtime-and-failed-projection
persist_ref: docs/fixes/kalma2-reincidence-stale-runtime-and-failed-projection
pbi_ref: docs/todos/pending/[FIX] Kalma2 — reincidencia PR 347 runtime fósil e init failed en status.md
execution_id: "381178a8-8a56-4a06-a8ce-d47be87242f2"
---

# Objetivos — kalma2-reincidence-stale-runtime-and-failed-projection

## Misión

PBI-FIX-KALMA2-REINCIDENCE-STALE-RUNTIME-AND-FAILED-PROJECTION (e3b27269-6736-4735-ba6f-fd94bb87d1c2). Reincidencia del PR #347: binarios execute-process y kalma2-bridge anteriores al merge; phase_end ignora phase_status failed y GET /api/status queda en awaiting_agents; dirty-worktree tras checkout no restaura prior_branch ni borra la rama nueva; /api/progress/stream no entrega el replay. Criteri

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
