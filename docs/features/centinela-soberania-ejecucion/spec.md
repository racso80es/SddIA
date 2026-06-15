---
feature_name: centinela-soberania-ejecucion
created: "2026-06-15"
process: feature
branch_name: feat/centinela-soberania-ejecucion
persist_ref: docs/features/centinela-soberania-ejecucion
---

# Especificación — Centinela Soberanía de Ejecución

## Alcance

Materializar ED kitchen CEN-01…CEN-05: contrato `daemons`, actuador OS, kill-switch, definiciones legacy, auditor Argos heartbeat, runtime `Daemon_Heartbeat` en scripts.

## Entregables

| ID | Entregable |
|----|------------|
| CEN-01 | `SddIA/daemons/daemons-contract.md`, `daemon-creator`, Cúmulo |
| CEN-02 | `governance-daemon-manager` + handler lab |
| CEN-03 | `daemon-kill-switch` + hooks CLI |
| CEN-04 | 3 Centinelas indexados + `Daemon_Heartbeat` ECST |
| CEN-05 | `daemon-heartbeat-audit` + suscripción telemetry |
| Post | `daemon_centinel_runtime.py` en watchers legacy |
