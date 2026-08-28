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
| `tqm-dispatch-discarded.md` | `be28d7c5-18fd-4e08-9e53-e5e2bec63f16` | tqm-dispatch-discarded | TQM_Dispatch_Discarded | 1.0.0 | events-contract v1.1.0 | system-operations | `tqm_dispatch_discarded` |
| process-execution-completed.md | a8f3c2e1-9b4d-4a7c-8e6f-1d2b3c4d5e6f | process-execution-completed | Process_Execution_Completed | 1.0.0 | events-contract v1.1.0 | system-operations | process_execution_completed, orchestration_handoff |
| local-qa-requested.md | e7f1a2b3-c4d5-4e6f-9a0b-8c7d6e5f4a3b | local-qa-requested | Local_QA_Requested | 1.0.0 | events-contract v1.1.0 | local-quality-gate | local_qa_requested, pre_push_blocking |

## Integridad

- 2 clases ECST catalogadas en `SddIA/events/orchestration/`.
