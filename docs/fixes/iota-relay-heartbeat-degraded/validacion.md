---
feature_name: iota-relay-heartbeat-degraded
created: "2026-08-30"
updated: "2026-08-30"
process: bug-fix
branch_name: fix/iota-relay-heartbeat-degraded
persist_ref: docs/fixes/iota-relay-heartbeat-degraded
pbi_ref: docs/todos/done/[FIX] iota-publish-relay — Ola 1 latido degradado (701c77ebeab8).md
document_id: PBI-FIX-FRACTURE-701c77ebeab8-OLA1
global: APTO
pbi_archived: true
branch: fix/iota-relay-heartbeat-degraded
checks:
  RELAY-CA9: APTO
  RELAY-CA10: APTO
  RELAY-CA11: APTO
  RELAY-CA12: APTO
  RELAY-CA1: INTACTO
  RELAY-CA2: DIFERIDO
  RELAY-CA3: DIFERIDO
  RELAY-CA5: DIFERIDO
  RELAY-CA6: DIFERIDO
  RELAY-CA7: DIFERIDO
  CASCADE_SPEC: APTO
  CASCADE_PLAN: APTO
  CASCADE_IMPLEMENTATION: APTO
  CASCADE_EXECUTION: APTO
  CASCADE_VALIDACION: APTO
git_changes:
  - SddIA/sddia-daemon-runtime/src/lib.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/ecosystem-health/src/lib.rs
  - SddIA/daemons/iota-publish-relay/src/main.rs
  - docs/fixes/iota-relay-heartbeat-degraded/
  - docs/todos/done/[FIX] iota-publish-relay — Ola 1 latido degradado (701c77ebeab8).md
  - SddIA/evolution/2381303e-5db6-4f35-b33f-70388e09295e.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — Ola 1 latido `degraded` (Argos)

## Veredicto

**APTO** — Ola 1 en rama `fix/iota-relay-heartbeat-degraded`. Cadena runtime → audit → espejo → relay. CA9–CA12 unitario verde. CA1 intacta. CA2/CA3/CA5–CA7 DIFERIDO (olas 1b/2; no esta entrega).

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| RELAY-CA9 | APTO | `tick_with_status_degraded_writes_payload`, `tick_defaults_alive`, `tick_rejects_unknown_status` |
| RELAY-CA10 | APTO | `degraded_not_healthy_missed_zero`, `alive_still_healthy`, `legacy_absent_status_healthy` |
| RELAY-CA11 | APTO | `daemon_yellow_on_heartbeat_degraded`, `daemon_red_missed_beats_degraded`, `daemon_green_alive_compat` |
| RELAY-CA12 | APTO | `post_grace_refused_kills_and_ticks_degraded`, `grace_refused_does_not_kill`, `post_grace_no_child_ticks_degraded` |
| RELAY-CA1 | INTACTO | `grace_refused_does_not_kill` (gracia no reabierta) |
| RELAY-CA2 / CA3 | DIFERIDO | Ola 1b observación (E2E bind/respawn) |
| RELAY-CA5 / CA6 | DIFERIDO | Ola 1b (log hijo; cola re-anclaje) |
| RELAY-CA7 | DIFERIDO | Ola 2 (`route_domain_core.rs`; no este PR) |
