---
feature_name: event-pending-sweeper
created: "2026-05-22"
process: bug-fix
branch_name: fix/event-pending-sweeper
persist_ref: docs/fixes/event-pending-sweeper
related_incident: "Padre ECST permanece en pending/ tras enrutamiento exitoso del watcher"
pbi_ref: docs/todos/pending/[FIX] event-pending-sweeper — padre permanece en pending tras enrutamiento.md
---

# Objetivos — event-pending-sweeper

## Misión

Cerrar el ciclo operativo del bus EDA V3+ en entornos donde solo corre `event-watcher`: cuando `route-domain-event` alcanza consenso de suscriptores, el padre debe purgarse de `.events/pending/` sin exigir una invocación manual de `event-sweeper.py`.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Reproducibilidad | Smoke documentado: emit → watcher `--once` → padre persiste (baseline pre-fix) |
| **O2** | Cierre automático post-route | Tras fix: mismo smoke → padre ausente en `pending/` si testigos OK |
| **O3** | Semántica dead-letter | Eventos con testigo en `dead-letter/subscribers/` **no** se purgan; alerta Kaizen preservada |
| **O4** | Compatibilidad sweeper | `event-sweeper.py` sigue siendo válido como recolector periódico de eventos stale |
| **O5** | UX operador | Log del watcher refleja purga o estado terminal real (no mensaje ambiguo) |

## No objetivos

- Cambiar inmutabilidad del padre **durante** fan-out (permanece en `pending/` mientras hay suscriptores in-flight).
- Refactor global del bus ni migración de topología legacy `docs/events/`.
- Resolver fallos de suscriptores (`sync-entity-index`, ECST gate) — quedan como deuda Kaizen separada.

## Ley aplicada

- Proceso `bug-fix` v1.2.0
- `SddIA/events/events-contract.md` §4 (ciclo de vida V3+)
- `SddIA/norms/execution-contexts.md` §2.7 (`event-routing`)
