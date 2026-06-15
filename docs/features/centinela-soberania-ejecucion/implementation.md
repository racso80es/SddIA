---
feature_name: centinela-soberania-ejecucion
created: "2026-06-15"
process: feature
branch_name: feat/centinela-soberania-ejecucion
persist_ref: docs/features/centinela-soberania-ejecucion
---

# Implementación — centinela-soberania-ejecucion

## CEN-01 (Done)

| Artefacto | Ruta |
|-----------|------|
| Contrato familia | `SddIA/daemons/daemons-contract.md` v1.0.0 |
| Índice soberano | `SddIA/daemons/index.md` |
| Topología Cúmulo | `directories.daemons`, `contracts.daemons`, `execution_capsules.daemons`, `daemons_instance` |
| Proceso creator | `SddIA/process/daemon-creator.md` |

## CEN-02 (Done)

| Artefacto | Ruta |
|-----------|------|
| Actuador OS | `SddIA/process/governance-daemon-manager.md` v1.0.0 |
| Handler lab | `SddIA/scripts/qa/governance_daemon_manager_core.py` |

## CEN-03 (Done)

| Artefacto | Ruta |
|-----------|------|
| Kill-Switch | `SddIA/process/daemon-kill-switch.md` v1.0.0 |
| Handler lab | `SddIA/scripts/qa/daemon_kill_switch_core.py` |
| Hooks CLI | `execute-process.py` → `register_kill_switch_hooks` |

## CEN-04 (Done)

| Artefacto | Ruta |
|-----------|------|
| event-watcher | `SddIA/daemons/event-watcher.md` |
| telegram-watcher | `SddIA/daemons/telegram-watcher.md` |
| github-bridge-watcher | `SddIA/daemons/github-bridge-watcher.md` |
| Clase ECST | `SddIA/events/telemetry/daemon-heartbeat.md` |

## CEN-05 (Done)

| Artefacto | Ruta |
|-----------|------|
| Auditor Argos | `SddIA/process/daemon-heartbeat-audit.md` v1.0.0 |
| Handler lab | `SddIA/scripts/qa/daemon_heartbeat_audit_core.py` |
| Suscripción | `event-telemetry-subscriptions.json` → `Daemon_Heartbeat` |
| Argos | `SddIA/agents/argos.md` §5 |

## Kitchen ED — cerrado

Todos los ítems CEN-01…CEN-05 materializados.

## Post-CEN — runtime legacy (Done)

| Artefacto | Ruta |
|-----------|------|
| Runtime compartido | `SddIA/scripts/qa/daemon_centinel_runtime.py` |
| Heartbeat + lock | `event-watcher.py`, `telegram-watcher.py`, `github_bridge_watcher.py` |
