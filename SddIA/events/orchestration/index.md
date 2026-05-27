---
family: orchestration
index_version: "1.0.0"
maintained_by_agent: cumulo
indexed_at: "2026-05-27"
---

# Códice de Familia — `orchestration`

## Propósito

Comunicación táctica entre Entidades de Dominio (Nivel 2): chispas de línea de montaje y relevo por artefactos.

| Campo | Valor |
|-------|-------|
| **Emisores autorizados** | CLI tras `status: success`; agentes auditores (p. ej. Argos → `Artifact_Validated` en Fase 3) |
| **Consumidor runtime (Fase 3)** | `./.events/orchestration/` → `route-orchestration` |

## Catálogo de Clases ECST

| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |
|----------------|------|------|------------|---------|----------|---------|--------------|
| process-execution-completed.md | a8f3c2e1-9b4d-4a7c-8e6f-1d2b3c4d5e6f | process-execution-completed | Process_Execution_Completed | 1.0.0 | events-contract v1.1.0 | system-operations | process_execution_completed, orchestration_handoff |

## Integridad

- Carpeta y Códice materializados; catálogo vacío hasta forja explícita.
