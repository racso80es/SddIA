---
feature_name: thermal-resilience-heartbeat-mayeuta
created: "2026-09-07"
process: bug-fix
phases:
  - host-boot-gate
  - orphan-lock-cube
  - unit-tests
  - entity-manager-bumps
  - verify-and-document
  - delivery-close
  - post-pr-ci
  - accept-pr
branch_name: fix/thermal-resilience-heartbeat-mayeuta
persist_ref: docs/fixes/thermal-resilience-heartbeat-mayeuta
pbi_ref: docs/todos/done/[FIX] Resiliencia Térmica en Heartbeat Audit y Poda Ontológica en Mayeuta.md
document_id: PBI-FIX-THERMAL-RESILIENCE-HEARTBEAT-MAYEUTA
execution_id: "0e0f6614-f1bd-40c5-9f63-af8e0a482786"
---

# Plan — thermal-resilience-heartbeat-mayeuta

Corte 1 (este commit): **Diseño** (`objectives.md` + `spec.md` + `plan.md`) + PBI v1.1.0. Sin código motor.

Corte 2 (mismo estímulo Racso): implementación → DCC → CI verde → `accept-pr`.

## Fase 1 — Puerta `btime` (CA-1, CA-2, CA-3)

`SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs` (no genoma).

```text
fn parse_proc_stat_btime(stat: &str) -> Option<DateTime<Utc>>
fn host_boot_time_utc() -> Option<DateTime<Utc>>  // lee /proc/stat
fn lock_predates_host_boot(started_at, last_hb, boot) -> bool
```

`audit_running_daemon`: si lock predates boot → `host_reboot_stale_lock`; `remove_lock` solo si `!pid_alive`; **no** `emit_orphan_lock_fracture`. Inyectar `boot_time: Option<DateTime<Utc>>` en el ramal de auditoría (tests) para no acoplar al `btime` del runner.

Fail-open: `boot_time=None` (sin `/proc/stat`) → comportamiento L3 actual.

## Fase 2 — Cubo `orphan_lock` (CA-4, CA-5)

`enrich_fracture_pbi_kaizen.rs`:

```text
fn is_orphan_lock_trace(error_trace: &str) -> bool
```

Anclas: `Centinela `, `lock huérfano`, `PID `, `muerto`, `last_heartbeat=`. Match **solo** `error_trace`. Evaluar tras `heartbeat_starvation` y **antes** del cubo genómico / catch-all.

Sustituir `has_any(["orphan","huérfan",…])` por firma genómica (`audit-entity-eda-coverage` | `entity-manager` | `Domain_Entity_Created` | `eda genómica`) && `!is_orphan_lock_trace`.

## Fase 3 — Tests (CA-6)

```text
cd SddIA && cargo test -p execute-process -- daemon_heartbeat enrich_fracture_pbi_kaizen
```

| Test | Aserción |
|------|----------|
| `orphan_lock_dead_pid_emits_fracture_once` | intacto (L3) |
| `orphan_lock_pre_boot_does_not_emit` | 0 fracturas; classification `host_reboot_stale_lock` |
| `analyze_fracture_kaizen_orphan_lock_not_eda` | traza canónica email-watcher 13215; sin `Domain_Entity_Created` |
| `analyze_fracture_kaizen_orphan_not_from_action_name` | `timeout in worker` + `daemon-heartbeat-audit` → no cubo orphan_lock |

## Fase 4 — Genoma (CA-7)

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/em-daemon-heartbeat-audit-1.2.0.json
./sddia-run.sh --process entity-manager --inputs-file .tmp/em-enrich-fracture-pbi-kaizen-1.3.0.json
```

DA-2. Tras acuse: DA-5. Prohibido `StrReplace` sobre `{name}.md` de process/action.

`daemon-heartbeat-audit` 1.1.0 → 1.2.0: `btime`, `host_reboot_stale_lock`, emisión huérfana solo post-boot.
`enrich-fracture-pbi-kaizen` 1.2.0 → 1.3.0: cubo `orphan_lock`; acote genómico.

Evolution bajo `SddIA/evolution/` vinculando este `document_id` (no DA-2).

## Fase 5 — Documental + archivo PBI (CA-8)

`implementation.md`, `execution.md`. Mover 4 satélites + PBI madre a `docs/todos/done/`. `validacion.md`: CA locales APTO; CA-9 `PENDIENTE-CI` hasta run verde (`features-documentation-pattern` v1.2.1). `global` no APTO mientras CA-9 sea gate.

## Fase 6 — DCC

```text
./sddia-run.sh --process delivery-close-cycle --inputs '{
  "source_process": "bug-fix",
  "persist_ref": "docs/fixes/thermal-resilience-heartbeat-mayeuta",
  "branch_name": "fix/thermal-resilience-heartbeat-mayeuta",
  ...
}'
```

No `gh pr create` raw.

## Fase 7 — CI (CA-9, mandato Racso)

Un chequeo de checks del PR (no bucle post-rojo). Si rojo: un parche + un push (DA-6). Si verde: completar CA-9 en `validacion.md` con URL/`run_id` si hace falta commit documental en la misma rama.

## Fase 8 — `accept-pr`

Solo con CA-9 verde.

```text
./sddia-run.sh --process accept-pr --inputs '{
  "source_branch": "fix/thermal-resilience-heartbeat-mayeuta",
  "author": "tekton",
  "correlation_id": "<uuid>"
}'
```

## Fuera de alcance

Umbrales, keepalive, `start-sddia.sh`, CLOCK_BOOTTIME como timestamp, fusionar `document_id`, genoma dirty ajeno (`event-sweeper.md`, tools telegram/iota).
