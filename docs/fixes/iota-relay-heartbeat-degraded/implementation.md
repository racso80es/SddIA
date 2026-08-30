---
feature_name: iota-relay-heartbeat-degraded
created: "2026-08-30"
process: bug-fix
branch_name: fix/iota-relay-heartbeat-degraded
persist_ref: docs/fixes/iota-relay-heartbeat-degraded
items:
  - sddia-daemon-runtime/tick_with_status
  - daemon_heartbeat/record_status
  - ecosystem-health/color_daemon_degraded
  - iota-publish-relay/always_tick_status
---

# Implementation — Ola 1 latido `degraded`

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/sddia-daemon-runtime/src/lib.rs` | `tick_with_status`; `tick()` → `alive`; `emit_heartbeat(status)`; enum + `Err`; bypass rate-limit si cambia status |
| `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs` | `record_heartbeat_at` persiste `status`; `degraded`/`shutting_down` ≠ `healthy`; `missed_cycles=0` |
| `SddIA/ecosystem-health/src/lib.rs` | `color_daemon`: `degraded` → `yellow`/`heartbeat_degraded`; `missed>=3` sigue `red` |
| `SddIA/daemons/iota-publish-relay/src/main.rs` | `heartbeat_status`; tick siempre; kill post-gracia intacto |

## Contrato

- Otros centinelas: `tick()` intacto ≡ `alive`.
- `bootstrap` emite `alive`.
- Cambio `alive`→`degraded` emite de inmediato (no espera intervalo).
- CA1 (`GRACE_SECS`) intacta. CA4 Ola 0 (omit-tick) sustituida.

## Tests

| ID | Test |
|----|------|
| CA9 | `tick_with_status_degraded_writes_payload`, `tick_defaults_alive`, `tick_rejects_unknown_status` |
| CA10 | `degraded_not_healthy_missed_zero`, `alive_still_healthy`, `legacy_absent_status_healthy` |
| CA11 | `daemon_yellow_on_heartbeat_degraded`, `daemon_red_missed_beats_degraded`, `daemon_green_alive_compat` |
| CA12 | `post_grace_refused_kills_and_ticks_degraded`, `grace_refused_does_not_kill`, `post_grace_no_child_ticks_degraded` |
