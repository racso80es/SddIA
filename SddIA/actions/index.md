---
index_version: "1.0.0"
entity_family: "actions"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "actions"
indexed_at: "2026-05-06"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado."
---

# Índice de actions (Core SddIA)

Contrato normativo de la familia: `actions-contract.md` (no constituye una acción catalogada en esta tabla).

## Catálogo de definiciones (`{name}.md`)

| Name | UUID | Versión | Context | Descripción | Capabilities |
|------|------|---------|---------|-------------|--------------|
| persist-pec-correlation-proof | `accb4de7-bb1e-4f88-b5cd-b8775a8ff5a4` | 1.0.0 | ecosystem-evolution | Proyección durable PEC por correlation_id bajo eda_instance.proofs/pec-correlation/. | `persist-pec-correlation-proof` |
| execute-process | f1e2d3c4-b5a6-4789-b012-cdef34567890 | 1.2.0 | ecosystem-evolution | Orquestación maestra para que Tekton ejecute un proceso del Core con resolución SSOT, mapa de identidad (canónico + aliases v1.3.0), fases ordenadas, `phase_invocations`, delegación a crypto-broker y gate Cerbero antes de cada cápsula. | `process-load-ssot`, `phase-graph-resolution`, `phase-invocation-binding`, `cerbero-policy-gate`, `capsule-fanout-skills-tools`, `crypto-broker-delegation` |
| crypto-broker | 9b3259be-e7a0-4fb1-b5d9-620a46fbc18b | 1.0.0 | quality-assurance | Puerta RBAC hacia operaciones deterministas de `cryptography-manager` (UUID, SHA-256, validación de hash) sin exponer `quality-assurance` al orquestador de forja. | `cryptography-broker`, `delegate-cryptography-manager` |
| policy-validator | 3f8c2b1a-9d0e-4f7a-b2c1-0e9d8c7b6a50 | 1.0.0 | ecosystem-evolution | Dictamen normativo: contrasta `allowed_policies` / `tool_context` / declaración de secretos con `execution-contexts.md` vía SSOT; usada en fases de auditoría de `agent-creator` y `tool-creator`. | `execution-contexts-validation`, `allowed-policies-audit`, `tool-context-compliance`, `secrets-declaration-audit` |
| emit-pr-merged-event | c0d71f2b-c1c1-4c56-8f74-2f4f41b24c4f | 1.1.0 | dlt-auditing | Extrae hash post-merge en main y emite PullRequest_Merged en eda_bus.pending (cumulo.paths.json). | `pr-merged-event-emission`, `event-bus-pending-write`, `delegate-git-manager`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-pr-presented-event | a1b2c3d4-e5f6-4789-a012-3456789abcde | 1.1.1 | ecosystem-evolution | Emite PullRequest_Presented en pending/ (branch, status, signer_identity_rbac) tras presentación de PR. | `pr-presented-event-emission`, `event-bus-pending-write`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-pr-audited-event | d4e5f6a7-b8c9-4d0e-1f2a-3b4c5d6e7f8a | 1.0.0 | quality-assurance | Emite PullRequest_Audited en pending/ tras veredicto Argos en aduana pull-request-review. | `pr-audited-event-emission`, `event-bus-pending-write`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-suite-execution-requested | d5e6f7a8-b9c0-4d1e-8f2a-3b4c5d6e7f8a | 1.0.0 | chaos-engineering | Emite Suite_Execution_Requested en ./.events/domain/ para orquestación reactiva execute-suite. | `suite-execution-requested-emission`, `event-bus-domain-write`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-domain-mutation | 7e4a9c2b-1d3f-4a8e-9b6c-0f1e2d3a4b5c | 1.0.0 | ecosystem-evolution | Emite eventos ECST Domain_Entity_* en pending/ tras mutación de entidades estructurales del genoma; valida hashes por lifecycle_operation; mintea event_id vía crypto-broker. | `domain-mutation-emission`, `event-bus-pending-write`, `delegate-filesystem-manager`, `delegate-crypto-broker`, `domain-event-type-translation` |
| emit-user-preference-change-requested | c3d4e5f6-a7b8-4901-c234-567890ab004 | 1.0.0 | ecosystem-evolution | Emite User_Preference_Change_Requested en eda_fractal.domain. | `user-preference-change-emission`, `event-bus-domain-write` |
| sync-entity-index | a3f8c2e1-4b5d-6a7e-8f90-1a2b3c4d5e6f | 1.0.0 | ecosystem-evolution | Reconciliación asíncrona de index.md tras Domain_Entity_*: auditoría idempotente en create/update; purga de fila en delete. Cúmulo vía bus EDA. | `entity-index-reconciliation`, `delegate-filesystem-manager`, `cumulo-catalog-sync` |
| materialize-fracture-pbi | b2c3d4e5-f6a7-4890-b123-4567890abcde | 1.1.0 | ecosystem-evolution | Materializa PBI de fractura con resolutor Core por genoma YAML (ceguera nominal). | `fracture-pbi-materialization`, `fracture-pbi-resolver`, `delegate-filesystem-manager`, `cumulo-debt-ledger` |
| materialize-kaizen-alert-doc | d7e6f5a4-b3c2-4109-8765-43210abcdef0 | 1.0.0 | quality-assurance | Materializa TODO PENDING_AUDIT_DOC_* ante Kaizen_Alert_Required (cicatriz DIA). Cúmulo vía bus EDA. | `kaizen-alert-doc-materialization`, `delegate-filesystem-manager`, `cumulo-debt-ledger` |
| enrich-fracture-pbi-kaizen | c4d5e6f7-a8b9-4012-c345-678901234567 | 1.2.0 | knowledge-management | Enriquece PBI de fractura vía resolutor Core; cubo heartbeat_starvation; no_target sin dead-letter. | `fracture-root-cause-analysis`, `kaizen-evolution-proposal`, `fracture-pbi-resolver`, `delegate-filesystem-manager` |
| download-remote-asset | 6175f5cd-7844-4d0c-aa93-d2ce3a41d18e | 1.0.0 | knowledge-management | Reclamación de activos del repositorio maestro con declaración de integridad SHA-256. Abstracción de origen opaca (pivote DLT G7). | `remote-asset-reclamation`, `asset-integrity-declaration` |

## Archivos en carpeta no catalogados como acción

Ninguno. `actions-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** diez definiciones de acción con identidad atómica; reflejadas en sendas filas del catálogo.
- **Metadatos:** valores de la tabla (incl. **Capabilities**) copiados desde el YAML de cada `{name}.md` al momento de indexación.

