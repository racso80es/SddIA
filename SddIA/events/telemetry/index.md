---
family: telemetry
index_version: "1.1.0"
maintained_by_agent: cumulo
indexed_at: "2026-07-19"
---

# Códice de Familia — `telemetry`

## Propósito

Ruido físico de infraestructura (Nivel 1): métricas de ejecución capturadas en el Peaje Termodinámico (CLI).

| Campo | Valor |
|-------|-------|
| **Emisores autorizados** | **Solo CLI** — `execute-process`, `execute-action`, cápsulas |
| **Consumidor runtime** | `./.events/telemetry/` → `route-telemetry` → `radamanto-batch` + `telemetry-compliance-audit` (+ `daemon-heartbeat-audit` para latidos) |
| **Chispa secundaria (domain)** | Tras consumo OK, Radamanto emite `Domain_Entity_Telemetry_Captured` → `memory-evolution-ingest` |

## Catálogo de Clases ECST

| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |
|----------------|------|------|------------|---------|----------|---------|--------------|
| `daemon-heartbeat.md` | `9c5190ac-ac8a-46b6-b61d-67d45ff7caf1` | daemon-heartbeat | Daemon_Heartbeat | 1.0.0 | events-contract v1.1.0 | system-operations | `daemon_heartbeat` |
| `raw-execution-finished.md` | `5a02d313-685d-4464-84c1-ffe16ef6ba6d` | raw-execution-finished | Raw_Execution_Finished | 1.0.0 | events-contract v1.1.0 | system-operations | `raw_execution_finished`, `thermodynamic_toll` |

## Integridad

- **Clases:** 2 ECST.
- **Purga:** fan-out sella `delivery_state`; purga física vía infraestructura (`route-telemetry` / sweeper), no el stub residual.
