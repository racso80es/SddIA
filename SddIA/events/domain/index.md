---
family: domain
index_version: "1.3.0"
maintained_by_agent: cumulo
indexed_at: "2026-07-19"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo en esta carpeta. Incluye Domain_Entity_Telemetry_Captured (telemetria-activa)."
---

# Códice de Familia — `domain`

## Propósito

Chispas ontológicas (Nivel 3): verdad objetiva del ecosistema (PR, mutaciones genómicas, Self-Healing, snapshots de telemetría, fracturas, Kaizen).

| Campo | Valor |
|-------|-------|
| **Emisores autorizados** | Agentes Core: Cúmulo, Cerbero, **Radamanto** (Self-Healing + Telemetry_Captured); acciones `emit-*` indexadas |
| **Consumidor runtime** | `./.events/domain/` → `route-domain` + pipeline V3+ `pending/` (coexistencia D0.2) |
| **Ingesta vectorial** | `Domain_Entity_Telemetry_Captured` → `memory-evolution-ingest` (no es CRUD `Domain_Entity_Updated`) |

## Catálogo de Clases ECST

| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |
|----------------|------|------|------------|---------|----------|---------|--------------|
| `email-triaged.md` | `6a4b0e9a-42e1-425c-8a16-9344eae4f246` | email-triaged | Email_Triaged | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `email_triaged` |
| `email-received.md` | `574fe330-137f-4f3a-b72d-dba189c6c406` | email-received | Email_Received | 1.0.0 | events-contract v1.1.0 | peripheral-sensing | `email_received` |
| `domain-entity-telemetry-captured.md` | `54a49fa7-8d45-4376-9aa1-deeebeb301ea` | domain-entity-telemetry-captured | Domain_Entity_Telemetry_Captured | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `domain_entity_telemetry_captured` |
| `vector-memory-indexed.md` | `5fc8293d-d853-4b20-8387-b039c9eb5438` | vector-memory-indexed | Vector_Memory_Indexed | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `vector_memory_indexed` |
| `thought-persisted.md` | `612a8b69-23fc-48d0-950e-28722ab084b9` | thought-persisted | Thought_Persisted | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `thought_persisted` |
| `domain-entity-deleted.md` | `a7c81b2f-b466-4b18-82c5-84ef0a5941b8` | domain-entity-deleted | Domain_Entity_Deleted | 1.1.0 | events-contract v1.1.0 | ecosystem-evolution | `domain_entity_deleted` |
| `domain-entity-updated.md` | `65dcff67-d392-4ab1-9977-2e320d3c8c34` | domain-entity-updated | Domain_Entity_Updated | 1.1.0 | events-contract v1.1.0 | ecosystem-evolution | `domain_entity_updated` |
| `domain-entity-created.md` | `1f518278-7a3d-4160-b757-a3661d263ec3` | domain-entity-created | Domain_Entity_Created | 1.1.0 | events-contract v1.1.0 | ecosystem-evolution | `domain_entity_created` |
| `pull-request-presented.md` | `5e488ae6-7cb2-4a2c-9725-4a7d4ce239ea` | pull-request-presented | PullRequest_Presented | 1.2.0 | events-contract v1.1.0 | ecosystem-evolution | `pull_request_presented`, `dlt_oracle_route` |
| `pull-request-merged.md` | `cfb8ce66-784e-4826-8a0a-a20c671e3a60` | pull-request-merged | PullRequest_Merged | 1.0.0 | events-contract v1.1.0 | dlt-auditing | `pull_request_merged` |
| `system-fracture-detected.md` | `f8e3a1b2-c4d5-4e6f-9a0b-1c2d3e4f5a6b` | system-fracture-detected | System_Fracture_Detected | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `system_fracture_detected` |
| `kaizen-alert-required.md` | `a9b8c7d6-e5f4-4321-a987-6543210fedcb` | kaizen-alert-required | Kaizen_Alert_Required | 1.0.0 | events-contract v1.1.0 | quality-assurance | `kaizen_alert_required`, `doc_parity_debt` |
| `domain-entity-degraded.md` | `7a1b2c3d-4e5f-4a6b-8c9d-0e1f2a3b4c5d` | domain-entity-degraded | Domain_Entity_Degraded | 1.0.0 | events-contract v1.1.0 | quality-assurance | `domain_entity_degraded`, `self_healing_trigger` |
| `domain-entity-restored.md` | `8b2c3d4e-5f6a-4b7c-9d0e-1f2a3b4c5d6e` | domain-entity-restored | Domain_Entity_Restored | 1.0.0 | events-contract v1.1.0 | quality-assurance | `domain_entity_restored`, `self_healing_redemption` |
| `domain-entity-deprecated.md` | `9c3d4e5f-6a7b-4c8d-0e1f-2a3b4c5d6e7f` | domain-entity-deprecated | Domain_Entity_Deprecated | 1.0.0 | events-contract v1.1.0 | quality-assurance | `domain_entity_deprecated`, `self_healing_death` |
| `telemetry-compliance-breached.md` | `a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5e` | telemetry-compliance-breached | Telemetry_Compliance_Breached | 1.0.0 | events-contract v1.1.0 | quality-assurance | `telemetry_compliance_breached`, `contract_audit_alert` |
| `suite-execution-requested.md` | `b3c4d5e6-f7a8-4b9c-8d0e-1f2a3b4c5d6f` | suite-execution-requested | Suite_Execution_Requested | 1.0.0 | events-contract v1.1.0 | chaos-engineering | `suite_execution_requested`, `chaos_campaign_stimulus` |
| `system-immunity-certified.md` | `c4d5e6f7-a8b9-4c0d-9e1f-2a3b4c5d6e7f` | system-immunity-certified | System_Immunity_Certified | 1.0.0 | events-contract v1.1.0 | quality-assurance | `system_immunity_certified`, `chaos_immunity_dlt` |
| `manual-task-requested.md` | `a6b7c8d9-e0f1-4a2b-c3d4-e5f6a7b8c9d0` | manual-task-requested | Manual_Task_Requested | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `manual_task_requested` |
| `kaizen-idea-captured.md` | `b7c8d9e0-f1a2-4b3c-d4e5-f6a7b8c9d0e1` | kaizen-idea-captured | Kaizen_Idea_Captured | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `kaizen_idea_captured` |
| `telegram-message-received.md` | `e1f2a3b4-c5d6-47e8-f9a0-b1c2d3e4f5a6` | telegram-message-received | TelegramMessage_Received | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `telegram_message_received` |
| `pull-request-audited.md` | `b21e89f7-66a8-4235-950c-d9c9efbd6359` | pull-request-audited | PullRequest_Audited | 1.0.0 | events-contract v1.2.0 | quality-assurance | `pull_request_audited`, `argos_verdict_ecst` |
| `kalma2-process-requested.md` | `458c34a8-9ad5-4a40-88c4-0be1e5d9598e` | kalma2-process-requested | Kalma2_Process_Requested | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `kalma2_process_requested` |

## Integridad

- **Clases:** 22 ECST en `SddIA/events/domain/`.
- **Regla de oro:** no mezclar telemetría cruda ni orquestación táctica en esta familia.
