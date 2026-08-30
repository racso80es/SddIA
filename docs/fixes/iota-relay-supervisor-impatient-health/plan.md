---
feature_name: iota-relay-supervisor-impatient-health
created: "2026-08-30"
process: bug-fix
phases:
  - extract-pure-decision
  - rewire-loop
  - unit-tests-ca1-ca4
  - document-and-stop-for-laudo
branch_name: fix/iota-relay-supervisor-impatient-health
persist_ref: docs/fixes/iota-relay-supervisor-impatient-health
---

# Plan — fractura `701c77ebeab8`

Corte de esta entrega: **Diseño (spec + plan) + commit**. Sin parche de `main.rs`, sin `delivery-close-cycle`.

## Fase 0 — Estado actual (hecho)

`SddIA/daemons/iota-publish-relay/src/main.rs` L190–228:

1. `need_spawn` → `spawn_node_child`; `last_restart = Instant::now()` en **éxito y fallo**.
2. `centinela.tick(&top)` incondicional.
3. `if !probe_health { if child_alive { kill } }`.

`HEALTH_TIMEOUT_MS` no cubre refused. No existe `child_spawned_at`. Tests actuales no cubren el bucle.

## Fase 1 — Fn pura (T-A/T-B/T-C)

En el mismo `main.rs` (no crate nuevo, no `lib.rs`):

```rust
const GRACE_SECS: u64 = 10;

struct SupervisorTickAction {
    emit_heartbeat: bool,
    kill_child: bool,
}

fn in_grace(child_present: bool, elapsed: Option<Duration>, grace: Duration) -> bool {
    child_present && elapsed.map(|e| e < grace).unwrap_or(false)
}

fn decide_supervisor_tick(
    health_ok: bool,
    in_grace: bool,
    child_alive: bool,
) -> SupervisorTickAction {
    SupervisorTickAction {
        emit_heartbeat: health_ok || in_grace,
        kill_child: !health_ok && !in_grace && child_alive,
    }
}
```

Sin I/O, sin `Child`, sin `DaemonRuntime`. `elapsed: None` ⇒ fuera de gracia (fail-closed).

Tests mínimos (`#[cfg(test)]`):

| Test | Entrada | Esperado |
|------|---------|----------|
| `grace_refused_does_not_kill` | health=false, elapsed=0s, child=true | emit=true, kill=false |
| `post_grace_refused_kills` | health=false, elapsed=10s, child=true | emit=false, kill=true |
| `post_grace_omits_tick` | misma que anterior | `emit_heartbeat == false` (CA4; puede fusionarse con el de kill) |
| `healthy_ticks_no_kill` | health=true, elapsed=60s | emit=true, kill=false |
| `grace_boundary_eq_is_outside` | elapsed=`GRACE_SECS` | `in_grace == false` |

`Duration` inyectado; no `thread::sleep` en unitario.

## Fase 2 — Reconectar el bucle (Tekton, post-laudo)

Estado nuevo: `child_spawned_at: Option<Instant> = None`.

| Evento | `last_restart` | `child_spawned_at` |
|--------|----------------|--------------------|
| spawn Ok | now | `Some(now)` |
| spawn Err | now | intacto (`None` si no hay hijo) |
| reap / kill | intacto | `None` |
| hijo muerto en `try_wait` | intacto | `None` + `child = None` antes del siguiente spawn |

Orden por tick **después** de spawn/reap:

```text
health_ok ← probe_health(&health)
elapsed ← child_spawned_at.map(|t| t.elapsed())
grace ← in_grace(child.is_some(), elapsed, Duration::from_secs(GRACE_SECS))
alive ← child.as_mut().map(child_alive).unwrap_or(false)
action ← decide_supervisor_tick(health_ok, grace, alive)
si action.emit_heartbeat → centinela.tick(&top)
si action.kill_child → kill; wait; child=None; child_spawned_at=None
sleep TICK_SECS
```

Backoff `RESTART_BACKOFF_SECS` inalterado. `probe_health` / `HEALTH_TIMEOUT_MS` inalterados.

Cuidado: `child_alive` consume `try_wait`; no llamarlo dos veces en el mismo tick sobre el mismo `Child` (hoy se llama en `need_spawn` y otra vez en kill). Tras reap, dejar `child = None` y usar `alive` derivado una sola vez, o aceptar que el segundo `try_wait` sobre proceso ya reaped es `Some(status)` → `alive=false` (kill no-op). Preferible: un solo `try_wait` por tick.

## Fase 3 — Verificación (post-parche)

```text
cd SddIA && cargo test -p iota-publish-relay
```

Argos (`validacion.md`) y archivo PBI a `docs/todos/done/` van en el PR de código, no en este commit.

## Fase 4 — Cierre de entrega

Solo tras T-A/T-B/T-C verdes + `validacion.md` APTO. `delivery-close-cycle` con fases de agente ejecutadas. Este corte no lo invoca.

## Orden y dependencias

```text
spec/plan (este commit)
  → laudo Vértice Biológico
    → Fase 1+2 (mismo archivo, un parche)
      → Fase 3 cargo test
        → Fase 4 delivery-close
```

Ola 1 (`degraded` en runtime/audit/espejo) no arranca hasta cerrar CA1/CA4 o PBI hijo explícito.
