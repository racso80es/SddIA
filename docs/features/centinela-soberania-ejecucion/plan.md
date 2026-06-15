---
feature_name: centinela-soberania-ejecucion
created: "2026-06-15"
process: feature
branch_name: feat/centinela-soberania-ejecucion
persist_ref: docs/features/centinela-soberania-ejecucion
---

# Plan — Centinela Soberanía de Ejecución

1. Forja contrato + procesos vía `entity-manager`
2. Handlers lab (`governance_daemon_manager_core`, `daemon_kill_switch_core`, `daemon_heartbeat_audit_core`)
3. Definiciones Centinelas + evento telemetry
4. Runtime heartbeat en `SddIA/scripts/daemons/*.py`
5. Cierre documental + PR único
