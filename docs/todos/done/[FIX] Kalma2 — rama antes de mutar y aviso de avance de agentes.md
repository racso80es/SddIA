---
document_id: PBI-FIX-KALMA2-BRANCH-AND-STATUS
uuid: "4cc5d670-ea88-405c-9385-a86db7fba20c"
title: "[FIX] Kalma2 — rama antes de mutar y aviso de avance de agentes"
format: markdown
version: "1.0.0"
created: "2026-10-13"
status: done
priority: alta
process: bug-fix
dispatch: true
suggested_branch: fix/kalma2-branch-before-mutate
persist_ref: docs/fixes/kalma2-branch-before-mutate
pr_url: https://github.com/racso80es/SddIA/pull/347
closed: "2026-10-14"
source_audit: docs/audits/kalma2-pbi-telegram-pec-silence-20261013.md
incident_correlation_id: "09a61ea3-b2d9-43c3-b65d-b815385e9a22"
friction_ids:
  - F-BRANCH-NOT-FIRST
  - F-INIT-FAIL-CONTINUES
  - F-WORK-ON-MAIN
  - F-KALMA2-STATUS-STALE
  - F-KALMA2-AGENT-VERDICT-MUTE
  - F-REENTRY-SAME-CID
related:
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/progress_trace.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - interfaces/kalma2/app.js
  - SddIA/library/norms/progress-trace-contract.md
out_of_scope:
  - "No rediseñar la WUI ni sustituir el SSE de progreso."
  - "No relajar el gate dirty-worktree: la suciedad ajena sigue bloqueando el ciclo, después de existir la rama o sin mutar la base."
  - "No implementar aquí el PBI de silencio PEC / síntesis (PBI-KAIZEN-TELEGRAM-PEC-SILENCE-VALUE-ES)."
---

# [FIX] Kalma2 — rama antes de mutar y aviso de avance de agentes

## 1. Contexto

Prueba Kalma2 del PBI `PBI-KAIZEN-TELEGRAM-PEC-SILENCE-VALUE-ES` (`correlation_id` `09a61ea3-b2d9-43c3-b65d-b815385e9a22`). Evidencia: `docs/audits/kalma2-pbi-telegram-pec-silence-20261013.md`.

Los agentes escribieron código con HEAD en `main`. La WUI siguió mostrando «Ciclo bug-fix en espera de agentes IDE» mientras Diseño ya estaba `executed` y Ejecución `blocked`.

## 2. Fricciones

### 2.1. F-BRANCH-NOT-FIRST / F-WORK-ON-MAIN

En `feature`, `bug-fix` y kaizen, si el ciclo trabaja en rama, el primer paso es crearla (`checkout` con `create_if_not_exists`). Hoy `workspace-init` evalúa `dirty_paths_outside_scope` antes de ese checkout. El error `dirty-worktree` aborta la fase y la rama no nace.

En la prueba el sucio ajeno era `.SddIA/observability/ecosystem-health.json`. En la reentrada, el propio diff ya escrito en `main` (`route_domain_core.rs`, `notify_humanized_pr_merged.rs`) engrosó el mismo error.

### 2.2. F-INIT-FAIL-CONTINUES / F-REENTRY-SAME-CID

`agent_phase_blocks_downstream` solo corta tras una fase de agente `failed`, `blocked` o `awaiting_agents`. Una inicialización nativa `failed` no arma el barrier. Dedalo y Tekton mutaron `main`. El mismo `correlation_id` reentró tres veces (`3c792bba-…`, ráfaga 15:19:35Z, `f0d9a570-…`).

### 2.3. F-KALMA2-STATUS-STALE / F-KALMA2-AGENT-VERDICT-MUTE

`project_status` pinta la caja Respuesta solo desde el PEC de correlación. `awaiting_agents` deja el texto fijo de espera. Un PEC temprano posterior restaura esa frase aunque la fase ya terminó. `build_message` del PTC solo dice «Inicio fase» / «Fin fase»; no transporta el veredicto del handoff.

## 3. Criterios de aceptación

| ID | Criterio |
|----|----------|
| BR-CA1 | En `feature`, `bug-fix` y `refactorization` con `branch_name` de trabajo, el checkout de esa rama es el primer paso git de la inicialización, antes del gate dirty |
| BR-CA2 | Si ese checkout no tiene éxito, ninguna fase posterior muta el árbol y el ciclo termina `failed` sin reentrada automática sobre la rama base |
| BR-CA3 | Inicialización `failed` arma el mismo corte que una fase de agente `failed`: Diseño, Ejecución y Verificación quedan `skipped` |
| BR-CA4 | Un fin de fase `executed` o `blocked` hace que `GET /api/status` nombre fase y status. Un PEC `awaiting_agents` posterior del mismo `correlation_id` no restaura el texto de espera inicial |
| BR-CA5 | El mensaje que ve el operador incluye el veredicto corto del agente (handoff o `message` de fase), no solo «Inicio/Fin fase» |
| BR-CA6 | Tests de `workspace-init`, del barrier del executor y de `project_status` cubren CA1–CA5 |

## 4. Notas

- El gate dirty se queda. Lo que cambia es el orden: primero la rama, después el gate. Si el gate falla, no hay mutación de agentes en la base.
- Prohibido editar a mano `SddIA/norms/` o `SddIA/library/norms/`. Si BR-CA5 exige ampliar el envelope PTC, la norma sale por `entity-manager`.
- El código de silencio PEC y síntesis en castellano no forma parte de este PBI.
