---
feature_name: kalma2-post-ev-aud-005-ola
created: "2026-08-13"
process: bug-fix
branch_name: fix/kalma2-post-ev-aud-005-ola
persist_ref: docs/fixes/kalma2-post-ev-aud-005-ola
pbi_ref: docs/todos/done/[OPERATIVO] Kalma2 — ola mejora post-auditoría EV-AUD-005 (K1–K5).md
document_id: b2e4c891-3f7a-4d2e-9c1b-8a5f6e0d2c47
parent_pbi: docs/todos/done/[FIX] execute-process — fallo de fase debe fallar ejecución global (EV-AUD-005).md
parent_pr: https://github.com/racso80es/SddIA/pull/170
scope: kalma2-orchestration-ux
base: main
---

# Spec — ola Kalma2 post-auditoría EV-AUD-005 (K1–K6)

## Problema

Ciclo Kalma2 `dcb9efed-…`: poll UI cortaba en `initialized`; watcher síncrono dejaba domain huérfano; TQM ignoraba `suggested_branch` (dual persist_ref); doble `bug-fix` por el mismo `correlation_id`; bridge podía proyectar PEC early sobre el terminal.

## Solución

| ID | Contrato |
|----|----------|
| K4 | TQM lee `suggested_branch` del frontmatter PBI → `branch_name` / slug. |
| K5 | Lock `.SddIA/daemons/state/tqm-single-flight/{correlation_id}.lock`; segundo despacho concurrente → `single_flight_hit` sin hijo. |
| K3 | `event-watcher` enruta async con `MAX_IN_FLIGHT_ROUTES=16`. |
| K2 | `emit_initialized_pec` emite `cycle_phase=awaiting_agents`. |
| K1 | UI: solo `completed`/`failed` cortan el poll; `initialized`/`awaiting_agents` extienden timeout a 30 min. |
| K6 | Bridge: PEC correlacionado = mayor `timestamp`. |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-K4 | `suggested_branch_from_pbi_frontmatter` verde. |
| CA-K5 | Segundo `try_acquire_single_flight` con guard vivo → `None`. |
| CA-K2 | `emit_initialized_pec` → `cycle_phase=awaiting_agents`. |
| CA-K6 | Fixture two-PEC → gana `completed` (timestamp posterior). |
| CA-K1 | `app.js` no retorna en `initialized`/`awaiting_agents`. |
| CA-K3 | Watcher spawnea rutas async; tope in-flight. |
| CA-ISO | PR sin `phase_terminal` ni docs EV-AUD-005. |

## Fuera de alcance

Lógica EV-AUD-005. Re-forja del PBI ya archivado. Instrumentación debug `.cursor/debug-*.log`.
