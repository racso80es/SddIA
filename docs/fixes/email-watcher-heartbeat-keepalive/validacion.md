---
feature_name: email-watcher-heartbeat-keepalive
created: "2026-08-31"
process: bug-fix
branch: fix/email-watcher-heartbeat-keepalive
global: APTO
pbi_archived: true
checks:
  CA1-build: pass
  CA2-tests: pass
  CA3-keepalive-continuous: pass
  CA4-once-no-keepalive: pass
  CA5-no-threshold-mutation: pass
  CA6-pbi-archived: pass
git_changes:
  - SddIA/daemons/email-watcher/src/main.rs
  - SddIA/evolution/6b8504a4-b45f-468f-8a9f-7b188a8ca4a1.md
  - SddIA/evolution/Evolution_log.md
  - docs/fixes/email-watcher-heartbeat-keepalive/
  - docs/todos/done/[FIX] email-watcher — fractura sistémica (6c0db1296181).md
---

# Validación — email-watcher heartbeat keepalive (`6c0db1296181`)

**Veredicto global: APTO**

## Criterios de aceptación (spec.md)

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA1 | `cargo build -p email-watcher` | ✅ | compiló en `cargo test -p email-watcher` |
| CA2 | `cargo test -p email-watcher` verdes | ✅ | 21 passed; 0 failed |
| CA3 | Modo continuo: keepalive ≤10 s aunque `poll_once` bloquee | ✅ | `spawn_heartbeat_worker`; mutex no retenido durante I/O IMAP |
| CA4 | `--once` sin hilo keepalive; envelope JSON-IO | ✅ | `once_branch_does_not_spawn_keepalive_worker`; `once_envelope_json_io_contract` |
| CA5 | No mutar umbrales Argos | ✅ | diff acotado a cápsula daemon + docs + evolution |
| CA6 | `pbi_archived: true`; PBI en `docs/todos/done/` | ✅ | este archivo + PBI archivado en la rama |

## Causa raíz cerrada

Starvation de latido: `poll_once` IMAP síncrono sin `tick()` intermedio → `missed_cycles≥3` con PID vivo (sello `6c0db1296181`). Corregido con hilo keepalive desacoplado del poll, paridad centinelas.

## Cierre documental

| Paso | Estado |
|------|--------|
| PBI → `docs/todos/done/` | ✅ |
| `pbi_archived: true` | ✅ |
| PR único pre-merge | ⏳ `delivery-close-cycle` |
