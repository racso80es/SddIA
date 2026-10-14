---
feature_name: kalma2-branch-before-mutate
created: "2026-10-14"
process: bug-fix
branch_name: fix/kalma2-branch-before-mutate
persist_ref: docs/fixes/kalma2-branch-before-mutate
pbi_ref: docs/todos/done/[FIX] Kalma2 — rama antes de mutar y aviso de avance de agentes.md
document_id: PBI-FIX-KALMA2-BRANCH-AND-STATUS
uuid: "4cc5d670-ea88-405c-9385-a86db7fba20c"
---

# Objetivos — kalma2-branch-before-mutate

## Misión

Corregir el orden de `workspace-init` (rama de trabajo antes del gate dirty), cortar el ciclo cuando la inicialización falla, y proyectar en Kalma2 el fin de fase y el veredicto del agente sin regresar a la espera `awaiting_agents`. Auditoría: `docs/audits/kalma2-pbi-telegram-pec-silence-20261013.md`.

## Alcance (manifiesto)

Criterios BR-CA1–BR-CA6 del PBI. Fuera de alcance: rediseño WUI, relajar dirty-worktree, silencio PEC Telegram.

## Ley aplicada

- Git vía `skill:git-manager`.
- PTC: veredicto en `message` / `metadata.verdict` sin mutar `SddIA/library/norms/` a mano.
