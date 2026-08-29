---
feature_name: arch-immunological-system
created: "2026-08-29"
process: feature
document_id: PBI-ARCH-IMMUNOLOGICAL-SYSTEM
uuid: "056ac6a1-02fc-4988-a704-1f5b648d0e40"
execution_id: "987e1747-bd08-4c80-ad41-648f09cc4b12"
---

# Implementation — arch-immunological-system

## Touchpoints

| Área | Archivo |
|------|---------|
| SSOT umbrales | `SddIA/daemons/heartbeat-audit.thresholds.json`, `SddIA/core/cumulo.paths.json` (`argos.heartbeat_audit_thresholds`) |
| Handler audit | `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs` |
| Loader umbrales | `SddIA/engine/execute-process/src/engine/handlers/heartbeat_audit_thresholds.rs` |
| Fagocitosis | `SddIA/engine/execute-process/src/engine/handlers/phagocyte_recovered_fracture_pbis.rs` |
| Routing | `SddIA/engine/execute-process/src/engine/mod.rs` |
| Procesos | `SddIA/process/daemon-heartbeat-audit.md` v1.1.0, `SddIA/process/phagocyte-recovered-fracture-pbis.md` v1.0.0 |
| Evolution | `SddIA/evolution/7f3a9e2b-1c4d-4f8a-9b6e-0d5c8a1f3e72.md` |

## Decisiones

- Sin `Anomaly_Detected`; estado en `heartbeat-audit.json`.
- Radamanto excluido del sondeo PID.
- `apply` documental default false; env `SDDIA_PHAGOCYTE_APPLY=1` en forja.
