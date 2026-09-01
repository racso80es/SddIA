---
family: telemetry
index_version: "1.1.0"
maintained_by_agent: cumulo
indexed_at: "2026-07-19"
---

# Códice de Familia — `telemetry`

## Propósito

Ruido físico de infraestructura (Nivel 1): Peaje Termodinámico (CLI) y sensores periféricos catalogados (latido de centinelas, colapso de CI remota).

| Campo | Valor |
|-------|-------|
| **Emisores autorizados** | CLI (`execute-process`, `execute-action`, cápsulas) **y** centinelas listados en cada Clase (`Daemon_Heartbeat`, `CI_Job_Failed` → `github-bridge-watcher`) |
| **Consumidor runtime** | `./.events/telemetry/` → `route-telemetry` → `radamanto-batch` + `telemetry-compliance-audit` (+ `daemon-heartbeat-audit` para latidos; `CI_Job_Failed` solo `radamanto-batch` rama ledger) |
| **Chispa secundaria (domain)** | Tras consumo OK, Radamanto emite `Domain_Entity_Telemetry_Captured` → `memory-evolution-ingest` |

## Catálogo de Clases ECST

| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |
|----------------|------|------|------------|---------|----------|---------|--------------|
| `ci-job-failed.md` | `1c026b2b-5ee1-40ff-940d-e214ba98c494` | ci-job-failed | CI_Job_Failed | 1.0.0 | events-contract v1.1.0 | system-operations | `ci_job_failed` |
| `system-vitality-probed.md` | `380e11c3-49af-47d0-80b0-072575ae8f66` | system-vitality-probed | System_Vitality_Probed | 1.0.0 | events-contract v1.1.0 | system-operations | `system_vitality_probed` |
| `daemon-heartbeat.md` | `9c5190ac-ac8a-46b6-b61d-67d45ff7caf1` | daemon-heartbeat | Daemon_Heartbeat | 1.0.0 | events-contract v1.1.0 | system-operations | `daemon_heartbeat` |
| `raw-execution-finished.md` | `5a02d313-685d-4464-84c1-ffe16ef6ba6d` | raw-execution-finished | Raw_Execution_Finished | 1.0.0 | events-contract v1.1.0 | system-operations | `raw_execution_finished`, `thermodynamic_toll` |

## Integridad

- **Clases:** 4 ECST.
- **Purga:** fan-out sella `delivery_state`; purga física vía infraestructura (`route-telemetry` / sweeper), no el stub residual.
