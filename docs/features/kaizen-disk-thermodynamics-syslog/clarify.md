---
feature_name: kaizen-disk-thermodynamics-syslog
created: "2026-09-13"
process: feature
purpose: Estabilización Filtro A PBI v1.2.0; laudos de silencio, unidad y techos
version_clarify: "1.0.0"
execution_id: "e3bbb4f8-b27c-4833-9750-d855b504da8b"
pbi_ref: docs/todos/pending/[KAIZEN] Termodinámica de disco — inundación syslog del event-watcher y techos de log de instancia.md
document_id: PBI-KAIZEN-DISK-THERMODYNAMICS-SYSLOG
pbi_uuid: "9454e7eb-f397-4e36-abaf-91fd91cba802"
pbi_version: "1.2.0"
---

# Clarificación — kaizen-disk-thermodynamics-syslog

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `e3bbb4f8-b27c-4833-9750-d855b504da8b`. Rama `feat/kaizen-disk-thermodynamics-syslog`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

Semilla: PBI v1.2.0 (Filtro A sobre v1.1.0). Dirty del PBI viajó en el checkout de rama. Stash local `wip-unrelated-pre-kaizen-disk` (health + Paciente 0) no entra en este ciclo.

## Decisiones

| ID | Laudo |
|----|-------|
| L-SILENCE | Ante `Some(skip)` en ambos bucles: `continue` sin I/O. Incluye `max attempts`. Cero traza periódica opcional. |
| L-LINES | Skip I/O actual: batch L505–512; fractal L580–587. No usar L480–518 / L559–590 como locus. |
| L-DL | `dead-letter kaizen` ya silencioso en skip. No-regresión. One-shot en `log_route_outcome` fuera de alcance. |
| L-FT | `fractal-terminal` solo en bucle no-batch. KA-DISK-1 no lo exige en batch. |
| L-ONESHOT | No mutar `log_route_outcome` ni println de éxito/fallo de despacho. |
| L-BOOK | `watcher_skip_reason` y `continue` intactos. Sin re-despacho de `routed_ok` remanente. |
| L-TRACE | Cero crate `tracing`. Cargo.toml del watcher sin deps nuevas. |
| L-UNIT | Canónico `sddia-event-watcher@.service`. Prohibido `sddia-daemon@event-watcher`. |
| L-RATE | `LogRateLimitIntervalSec=30s` + `LogRateLimitBurst=500` en fábrica **y** unidad instancia versionada. Fábrica cubre todos los daemons factory; email-watcher aparte (fuera). |
| L-INHERIT | Merge del template ≠ unidad viva. Este PR sincroniza `.SddIA/systemd/sddia-event-watcher@.service`. |
| L-PURGER | Veto `/var` inmutable. Diff no toca purger ni `purge_sandbox_cache.rs`. |
| L-HOST | journald/logrotate = runbook. Cero paths de host bajo `SddIA/` código. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo tras checks verdes. |
| L-DA2 | `daemons/` no está en tabla DA-2. Mutación del crate es ejecución Tekton post-topología, no forja entity-manager. |

## Fuera (este ciclo)

`tracing`; unidad legacy `sddia-daemon@`; silencio de `log_route_outcome`; plantilla email-watcher; mutar jail del purger; aplicar techos journald/rsyslog en el SO.
