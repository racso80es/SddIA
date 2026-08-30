---
feature_name: iota-relay-heartbeat-degraded
created: "2026-08-30"
process: bug-fix
branch_name: fix/iota-relay-heartbeat-degraded
persist_ref: docs/fixes/iota-relay-heartbeat-degraded
items_applied:
  - tick_with_status
  - record_heartbeat_status
  - color_daemon_degraded
  - relay_always_tick
---

# Ejecución — Ola 1 latido `degraded`

## Fases

| Fase | Estado |
|------|--------|
| Inicialización | executed (`39567569-6670-42d6-8174-116954dda036`) |
| Diseño | `spec.md` + `plan.md` (`9fc3a4e`) |
| Ejecución | parche 4 crates + tests CA9–CA12 |
| Verificación Argos / DCC | executed — PR [#234](https://github.com/racso80es/SddIA/pull/234) |

## Comandos

```bash
cd SddIA && cargo test -p sddia-daemon-runtime
cd SddIA && cargo test -p execute-process degraded_not_healthy
cd SddIA && cargo test -p execute-process alive_still_healthy
cd SddIA && cargo test -p execute-process legacy_absent_status
cd SddIA && cargo test -p sddia-ecosystem-health daemon_yellow_on_heartbeat
cd SddIA && cargo test -p sddia-ecosystem-health daemon_red_missed_beats
cd SddIA && cargo test -p sddia-ecosystem-health daemon_green_alive
cd SddIA && cargo test -p iota-publish-relay
```

Salida: todos `ok`. Relay `7 passed`. Runtime CA9 `3/3`. Audit CA10 `3/3`. Espejo CA11 `3/3`.

## Binarios vivos

Parche en fuente. Centinelas ya levantados siguen el binario previo hasta rebuild+respawn (`iota-publish-relay`, `execute-process` audit, fusión espejo). No se matan procesos en este corte.
