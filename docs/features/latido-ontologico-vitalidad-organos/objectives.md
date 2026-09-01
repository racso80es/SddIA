---
feature_name: latido-ontologico-vitalidad-organos
created: "2026-08-31"
process: feature
branch_name: feat/latido-ontologico-vitalidad-organos
persist_ref: docs/features/latido-ontologico-vitalidad-organos
pbi_ref: docs/todos/done/[OPERATIVO] Latido Ontológico (System Heartbeat).md
document_id: PBI-OPER-LATIDO-ONTOLOGICO-001
uuid: "cafd87eb-817f-4eee-a169-f9cd6019e931"
execution_id: "cb141830-b5e3-4b9e-904d-014922254734"
mayeuta_verdict: ok
---

# Objetivos — latido-ontologico-vitalidad-organos

## Misión

Cerrar la ceguera residual de vitalidad: el puente Kalma2 no entra en el censo de Argos; el auditor ignora PID muerto; el contrato `Daemon_Heartbeat` declara 3 emisores de 6; los invariantes sin PID no fracturan.

## Punto objetivo

> **O-LATIDO-VITALIDAD:** `audit_staleness` ve `kalma2-bridge` (genoma + `DaemonRuntime`); lock huérfano / PID muerto emite `System_Fracture_Detected`; `daemon-heartbeat.md` lista los 6 stems del índice; `system-vitality-probe` emite `System_Vitality_Probed` y fractura en rojo. Sin `System_Heartbeat_Emitted` ni `System_Degraded`.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Emisores `Daemon_Heartbeat` = índice de daemons | Re-forjar launcher/systemd de Kalma2 |
| `{name}.md` + `DaemonRuntime` en crate `interfaces/kalma2-bridge` | Mover crate a `SddIA/daemons/` |
| Auditor: PID muerto + lock → fractura | Panel Espejo / `System_Vitality_Probed` como Read Model UI |
| Proceso `system-vitality-probe` + clase telemetry | Supervisión IOTA (ya cerrada) |
| Sweep `event-sweeper` + `SDDIA_VITALITY_PROBE_SECONDS` | `verify-cumulo-indices` universal |
| | Metabolismo Adaptativo (Fase 4 condicionada) |

## Ley aplicada

- DA-2: eventos, process, daemons vía `entity-manager`.
- `daemons-contract` §2: Kalma2 es órgano de interfaz (excepción documentada), no sensor ciego.
- Un SSOT de intervalo: `execution.heartbeat_interval_seconds`. Cero `SDDIA_HEARTBEAT_*`.
- Cierre documental en rama (un PR).
