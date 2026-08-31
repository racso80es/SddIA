---
feature_name: latido-ontologico-vitalidad-organos
created: "2026-08-31"
process: feature
purpose: decisiones-territorio-v2.1
pbi_ref: docs/todos/pending/[OPERATIVO] Latido Ontológico (System Heartbeat).md
execution_id: "cb141830-b5e3-4b9e-904d-014922254734"
---

# Clarificaciones — latido-ontologico-vitalidad-organos

## Decisiones

| ID | Pregunta | Laudo |
|----|----------|-------|
| L-INTERFAZ | ¿Kalma2 es centinela sensorial? | No. Crate en `interfaces/`. Catalogar para censo + `DaemonRuntime`. Jurisdicción: órgano de interfaz. `daemon_context`: `system-operations`. |
| L-MUERTE | ¿`kill` produce fractura hoy? | No. `audit_running_daemon` retorna `Ok(None)` si `!pid_alive`. Extender: lock + PID muerto → fractura idempotente. SIGTERM limpio sigue borrando lock (`shutdown`). |
| L-IOTA | ¿Forjar `iota-publish-relay.md`? | No. Ya existe. Solo incluirlo en emisores autorizados. |
| L-ESPEJO | ¿Este evento alimenta el panel? | No. Espejo lee `heartbeat-audit.json` × map-snapshot. `System_Vitality_Probed` es telemetría de invariantes. |
| L-HTTP | ¿Duplicar `_wait_http` de ignición? | No. Probe `kalma2.http` es runtime (sweeper). Ignición one-shot no se toca. |
| L-REQUIRED | ¿Kalma2 en `REQUIRED_DAEMONS`? | No. Gate HTTP ya existe. Censo Argos ≠ lista de ignición. |
| L-EMIT | ¿Nueva clase de latido? | Prohibido. Solo `System_Vitality_Probed`. |

## Fricciones cubiertas

`F-AUDITORIA-CENSO-GENOMA` · `F-AUDITOR-PID-MUERTO-SILENCIO` · `F-ORGANO-INTERFAZ-SIN-LATIDO` · `F-CONTRATO-HEARTBEAT-DRIFT` · `F-VITALIDAD-NO-PROCESO-INVISIBLE`
