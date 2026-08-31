---
feature_name: latido-ontologico-vitalidad-organos
created: "2026-08-31"
process: feature
branch_name: feat/latido-ontologico-vitalidad-organos
persist_ref: docs/features/latido-ontologico-vitalidad-organos
execution_id: "cb141830-b5e3-4b9e-904d-014922254734"
pbi_ref: docs/todos/done/[OPERATIVO] Latido Ontológico (System Heartbeat).md
document_id: PBI-OPER-LATIDO-ONTOLOGICO-001
uuid: "cafd87eb-817f-4eee-a169-f9cd6019e931"
items:
  - forja/daemon-entity-class
  - forja/markdown_body_replacements-fence
  - genoma/kalma2-bridge
  - genoma/system-vitality-probed
  - genoma/system-vitality-probe
  - contrato/daemon-heartbeat-emisores
  - runtime/kalma2-daemon-runtime
  - auditor/orphan-lock
  - handler/system-vitality-probe
  - sweeper/vitality-probe-sweep
---

# Implementation — latido-ontologico-vitalidad-organos

## Touchpoints

| Artefacto | Cambio | Vía |
|-----------|--------|-----|
| `entity_manager.rs` / `domain_mutation.rs` | Brazo `daemon`; `ENTITY_CLASSES` + `suite`/`daemon` | IDE |
| `forges/factory.rs` + `common.rs` | Update event/daemon con replacements; merge capabilities; censo ECST; fence YAML `\n---` | IDE |
| `kalma2-bridge.md` | Create genoma + índice (7 centinelas) | entity-manager |
| `system-vitality-probed.md` | Create telemetry | entity-manager |
| `system-vitality-probe.md` | Create process core + body | entity-manager |
| `daemon-heartbeat.md` | Emisores = 7 stems; uuid `9c5190ac-…` inmutable | entity-manager |
| `event-sweeper.md` | Body + capability `vitality-probe-sweep` | entity-manager |
| `kalma2-bridge` crate | `DaemonRuntime` + `ctrlc` → `shutdown` | IDE |
| `daemon_heartbeat.rs` | Lock + PID muerto → fractura idempotente | IDE |
| `system_vitality.rs` | 4 sondas; ECST; fractura por `probe_id` | IDE |
| `event-sweeper/src/main.rs` | `SDDIA_VITALITY_PROBE_SECONDS` (300/piso 30) | IDE |

## Contrato

- Prohibido `System_Heartbeat_Emitted` / `System_Degraded`.
- Kalma2 no se mueve de `interfaces/`; no entra en `REQUIRED_DAEMONS`.
- Espejo no consume `System_Vitality_Probed`.
- SIGTERM limpio quita lock; `kill -9` deja huérfano → fractura.
