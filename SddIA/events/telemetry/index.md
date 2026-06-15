---
family: telemetry
index_version: "1.0.0"
maintained_by_agent: cumulo
indexed_at: "2026-05-27"
---

# Códice de Familia — `telemetry`

## Propósito

Ruido físico de infraestructura (Nivel 1): métricas de ejecución capturadas en el Peaje Termodinámico (CLI).

| Campo | Valor |
|-------|-------|
| **Emisores autorizados** | **Solo CLI** — `execute-process`, `execute-action`, cápsulas `execute_process_capsules` |
| **Consumidor runtime (Fase 3)** | `./.events/telemetry/` → `route-telemetry` → Radamanto (batch); purga tras consumo |

## Catálogo de Clases ECST

| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |
|----------------|------|------|------------|---------|----------|---------|--------------|
| `daemon-heartbeat.md` | `9c5190ac-ac8a-46b6-b61d-67d45ff7caf1` | daemon-heartbeat | Daemon_Heartbeat | 1.0.0 | events-contract v1.1.0 | system-operations | `daemon_heartbeat` |
| `raw-execution-finished.md` | `5a02d313-685d-4464-84c1-ffe16ef6ba6d` | raw-execution-finished | Raw_Execution_Finished | 1.0.0 | events-contract v1.1.0 | system-operations | `raw_execution_finished`, `thermodynamic_toll` |

## Integridad

- **Clases:** 1 ECST (Fase 1); ampliación prevista en Fases 3–5.
