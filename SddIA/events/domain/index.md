---
family: domain
index_version: "1.0.0"
maintained_by_agent: cumulo
indexed_at: "2026-05-27"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo en esta carpeta."
---

# Códice de Familia — `domain`

## Propósito

Chispas ontológicas (Nivel 3): verdad objetiva del ecosistema (PR, mutaciones genómicas, fracturas, Kaizen).

| Campo | Valor |
|-------|-------|
| **Emisores autorizados** | Agentes Core: Cúmulo, Cerbero, **Radamanto** (Self-Healing); acciones `emit-*` indexadas |
| **Consumidor runtime (Fase 3)** | `./.events/domain/` → `route-domain` + pipeline V3+ `pending/` (coexistencia D0.2) |

## Catálogo de Clases ECST

| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |
|----------------|------|------|------------|---------|----------|---------|--------------|
| `domain-entity-deleted.md` | `a7c81b2f-b466-4b18-82c5-84ef0a5941b8` | domain-entity-deleted | Domain_Entity_Deleted | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `domain_entity_deleted` |
| `domain-entity-updated.md` | `65dcff67-d392-4ab1-9977-2e320d3c8c34` | domain-entity-updated | Domain_Entity_Updated | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `domain_entity_updated` |
| `domain-entity-created.md` | `1f518278-7a3d-4160-b757-a3661d263ec3` | domain-entity-created | Domain_Entity_Created | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `domain_entity_created` |
| `pull-request-presented.md` | `5e488ae6-7cb2-4a2c-9725-4a7d4ce239ea` | pull-request-presented | PullRequest_Presented | 1.2.0 | events-contract v1.1.0 | ecosystem-evolution | `pull_request_presented`, `dlt_oracle_route` |
| `pull-request-merged.md` | `cfb8ce66-784e-4826-8a0a-a20c671e3a60` | pull-request-merged | PullRequest_Merged | 1.0.0 | events-contract v1.1.0 | dlt-auditing | `pull_request_merged` |
| `system-fracture-detected.md` | `f8e3a1b2-c4d5-4e6f-9a0b-1c2d3e4f5a6b` | system-fracture-detected | System_Fracture_Detected | 1.0.0 | events-contract v1.1.0 | ecosystem-evolution | `system_fracture_detected` |
| `kaizen-alert-required.md` | `a9b8c7d6-e5f4-4321-a987-6543210fedcb` | kaizen-alert-required | Kaizen_Alert_Required | 1.0.0 | events-contract v1.1.0 | quality-assurance | `kaizen_alert_required`, `doc_parity_debt` |
| `tool-degraded.md` | `7a1b2c3d-4e5f-4a6b-8c9d-0e1f2a3b4c5d` | tool-degraded | Tool_Degraded | 1.0.0 | events-contract v1.1.0 | quality-assurance | `tool_degraded`, `self_healing_trigger` |
| `status-restored.md` | `8b2c3d4e-5f6a-4b7c-9d0e-1f2a3b4c5d6e` | status-restored | Status_Restored | 1.0.0 | events-contract v1.1.0 | quality-assurance | `status_restored`, `self_healing_redemption` |
| `tool-deprecated.md` | `9c3d4e5f-6a7b-4c8d-0e1f-2a3b4c5d6e7f` | tool-deprecated | Tool_Deprecated | 1.0.0 | events-contract v1.1.0 | quality-assurance | `tool_deprecated`, `self_healing_death` |

## Integridad

- **Clases:** 10 ECST en `SddIA/events/domain/`.
- **Regla de oro:** no mezclar telemetría cruda ni orquestación táctica en esta familia.
