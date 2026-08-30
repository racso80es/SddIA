---
feature_name: iota-relay-heartbeat-degraded
created: "2026-08-30"
process: bug-fix
phases:
  - runtime-tick-with-status
  - audit-persist-degraded
  - mirror-color-heartbeat-degraded
  - relay-always-tick-status
  - crate-tests-ca9-ca12
  - document-and-stop-for-laudo
branch_name: fix/iota-relay-heartbeat-degraded
persist_ref: docs/fixes/iota-relay-heartbeat-degraded
---

# Plan — Ola 1 latido `degraded` (`701c77ebeab8`)

Corte de esta entrega: **Diseño (spec + plan) + commit**. Sin parche Rust, sin `delivery-close-cycle`.

## Fase 0 — Estado actual (hecho)

1. `DaemonRuntime::tick(&top)` → `emit_heartbeat(top, false)` con `"status": "alive"` cableado (payload + side-channel).
2. `record_heartbeat_at`: `missed_cycles=0`, `classification` = `healthy`\|`recovered`. No escribe `status`.
3. `color_daemon`: `green`/`heartbeat_ok` si `last_heartbeat_at` y `missed < 3`. No lee `status`.
4. Relay `decide_supervisor_tick`: `emit_heartbeat: health_ok || in_grace`. Post-gracia refused: `emit=false`, `kill=true` (CA4 Ola 0).
5. Callers `tick(&top)` en email-watcher, github-bridge-watcher, event-sweeper, event-watcher, telegram-watcher: **fuera de lock**; no mutar.

## Fase 1 — Runtime (RELAY-CA9)

`SddIA/sddia-daemon-runtime/src/lib.rs`:

```rust
pub fn tick(&mut self, top: &BusTopology) -> Result<(), String> {
    self.tick_with_status(top, "alive")
}

pub fn tick_with_status(&mut self, top: &BusTopology, status: &str) -> Result<(), String> {
    if !self.bootstrapped {
        self.bootstrap(top)?;
    }
    self.emit_heartbeat(top, false, status)?;
    Ok(())
}
```

`emit_heartbeat(&mut self, top, force, status)`: validar enum; inyectar `status` en payload y side-channel.

`bootstrap` → `emit_heartbeat(top, true, "alive")`.

Test crate (temp repo como `lock_excludes_duplicate_pid`):

| Test | Esperado |
|------|----------|
| `tick_with_status_degraded_writes_payload` | side-channel `"status": "degraded"` |
| `tick_defaults_alive` | `tick(&top)` → `"status": "alive"` |
| `tick_rejects_unknown_status` | `Err`; no escribe `alive` mentiroso |

Intervalo: `last_emit` ya nace −3600 s; primer tick emite. `heartbeat_interval_seconds: 1` en el `.md` de test si hace falta segundo emit.

## Fase 2 — Audit (RELAY-CA10)

`daemon_heartbeat.rs` `record_heartbeat_at` tras L198–207:

```text
status ← payload.status trim; vacío → no insertar (legado)
si status == "degraded" → classification = "degraded"  (ignora had_fracture→healthy)
si status == "shutting_down" → classification = "shutting_down"
si no → classification actual (healthy / recovered)
missed_cycles = 0 siempre
entry["status"] = status si no vacío
```

Test en el mismo `mod tests` (fn privada visible):

| Test | Entrada | Esperado |
|------|---------|----------|
| `degraded_not_healthy_missed_zero` | payload `status: degraded` | `classification == "degraded"`, `missed_cycles == 0`, `entry.status == "degraded"` |
| `alive_still_healthy` | `status: alive` | `classification == "healthy"` (sin fracture previa) |
| `legacy_absent_status_healthy` | sin `status` | `classification == "healthy"`; clave `status` ausente |

## Fase 3 — Espejo (RELAY-CA11)

`ecosystem-health/src/lib.rs` `color_daemon`: tras el gate `missed >= 3`, leer `entry.status`.

```text
si status == "degraded" → (yellow, heartbeat_degraded, missed, None, false)
si status == "shutting_down" → (yellow, heartbeat_shutting_down, missed, None, false)
si no → rama actual has_hb → green
```

Test junto a `ecosystem_health_daemon_red_on_missed_cycles`:

| Test | Fixture audit | Esperado |
|------|---------------|----------|
| `daemon_yellow_on_heartbeat_degraded` | `missed_cycles: 0`, `last_heartbeat_at` presente, `status: degraded` | `color == yellow`, `reason == heartbeat_degraded` |
| `daemon_red_missed_beats_degraded` | `missed_cycles: 3`, `status: degraded` | `red` / `missed_cycles` (precedencia) |
| `daemon_green_alive_compat` | `missed: 0`, sin `status` | `green` / `heartbeat_ok` |

## Fase 4 — Relay (RELAY-CA12)

Mismo `main.rs`. No crate nuevo.

```rust
struct SupervisorTickAction {
    heartbeat_status: &'static str,
    kill_child: bool,
}

fn decide_supervisor_tick(...) -> SupervisorTickAction {
    SupervisorTickAction {
        heartbeat_status: if health_ok || in_grace { "alive" } else { "degraded" },
        kill_child: !health_ok && !in_grace && child_alive,
    }
}
```

Bucle: **siempre** `centinela.tick_with_status(&top, action.heartbeat_status)` (salvo salida por `stop`). Kill inalterado.

Tests (`Duration` inyectado; no `sleep`):

| Test | Entrada | Esperado |
|------|---------|----------|
| `grace_refused_does_not_kill` | health=false, elapsed=0s | status=`alive`, kill=false |
| `post_grace_refused_kills_and_ticks_degraded` | health=false, elapsed=10s, child=true | status=`degraded`, kill=true |
| `healthy_ticks_alive_no_kill` | health=true, elapsed=60s | status=`alive`, kill=false |
| `grace_boundary_eq_is_outside` | elapsed=`GRACE_SECS` | `in_grace == false` |
| `post_grace_no_child_ticks_degraded` | health=false, child_alive=false, no grace | status=`degraded`, kill=false |

Renombrar `post_grace_refused_kills_and_omits_tick`. CA1 (`grace_refused_does_not_kill`) permanece.

## Fase 5 — Verificación (post-parche, no este commit)

```text
cd SddIA && cargo test -p sddia-daemon-runtime
cd SddIA && cargo test -p execute-process daemon_heartbeat
cd SddIA && cargo test -p ecosystem-health
cd SddIA && cargo test -p iota-publish-relay
```

Argos (`validacion.md`) y archivo PBI a `docs/todos/done/` van en el PR de código, no en este commit. CA5–CA7 = DIFERIDO en `validacion.md`.

## Fase 6 — Cierre de entrega

Solo tras CA9–CA12 verdes + `validacion.md` APTO. `delivery-close-cycle` con fases de agente ejecutadas. Este corte no lo invoca.

## Orden y dependencias

```text
spec/plan (este commit)
  → laudo Vértice Biológico
    → Fase 1 runtime (API; otros daemons intactos)
      → Fase 2 audit
        → Fase 3 espejo
          → Fase 4 relay (consume tick_with_status)
            → Fase 5 cargo test 4 crates
              → Fase 6 delivery-close
```

Fases 1–3 pueden ir en un único parche si el diff cabe; Fase 4 depende de 1. No mutar genoma. No tocar `route_domain_core.rs` (CA7 = Ola 2).
