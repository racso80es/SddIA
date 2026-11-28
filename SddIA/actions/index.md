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
| switch-llm-provider | `a6f5e0a4-ccc5-4076-bf8a-b60cfe1fa684` | 1.0.1 | ecosystem-evolution | Conmuta de forma explícita el proveedor LLM activo de la instancia escribiendo `active_provider` en el fichero de runtime local (`instance.llm_runtime_local` de `cumulo.paths.json`). No existe auto-failover: el cambio... | `switch_llm_provider` |
| purge-repo-retention | `e4b84b41-c804-4c34-8fea-8b0f6bbcb573` | 1.0.1 | filesystem-ops | Retención repo: dry-run + purga `repo-retention-purger` con jail de candidatos. | `purge_repo_retention` |
| dispatch-aiua-intent | a1086194-e19c-49b1-88e1-bd828211b3a8 | 2.0.1 | ecosystem-evolution | Traduce un tendón Aiúa (`function_call.name` + `args`) a ECST en `eda_fractal.domain`. | `dispatch_aiua_intent` |
| purge-sandbox-cache | 37454c3a-4590-4dfb-a439-2014876a0138 | 1.0.2 | filesystem-ops | Purgar `/tmp/cursor-sandbox-cache/**` tras simulación con `tool:ephemeral-cache-purger`. | `purge_sandbox_cache`, `delegate-ephemeral-cache-purger` |
| append-mayeuta-hypothesis | d5c792b5-33a1-4287-a66f-ea3b2ad5716d | 1.0.0 | knowledge-management | Hipótesis semántica asíncrona (mayeuta-llm SYNTHESIZE); upsert H2; fail-open. | `fracture-semantic-hypothesis`, `delegate-mayeuta-llm` |
| persist-thought-record | a37f9f1d-8f2a-441f-9357-776d65553362 | 1.1.0 | ecosystem-evolution | Acción persist-thought-record | `persist_thought_record` |
| invoke-aiua-core | 2edc7ef4-57e5-4d10-8753-bab8b0053cca | 1.1.0 | ecosystem-evolution | Acción invoke-aiua-core | `invoke_aiua_core` |
| retrieve-active-context | afa0424b-cdd4-4810-9a29-d8e7c06d6a1f | 1.1.0 | ecosystem-evolution | Acción retrieve-active-context | `retrieve_active_context` |
| notify-humanized-pr-merged | 1cd7bd40-b72f-4114-ac44-68b912774aa6 | 1.1.0 | ecosystem-evolution | Estático + commit_summary fail-soft + gemini-http-infer + send-telegram-notification. | `notify-humanized-pr-merged`, `delegate-gemini-http-infer`, `delegate-send-telegram-notification`, `delegate-git-manager` |
| materialize-ci-chronic-failure-pbi | `a6eb7f0c-8b2f-4c7d-ae5e-6c1b589f3c92` | 1.0.1 | ecosystem-evolution | Materializar PBI Kaizen ante `CI_Chronic_Failure_Detected`. | `materialize_ci_chronic_failure_p` |
| persist-pec-correlation-proof | `accb4de7-bb1e-4f88-b5cd-b8775a8ff5a4` | 1.0.0 | ecosystem-evolution | Proyección durable PEC por correlation_id bajo eda_instance.proofs/pec-correlation/. | `persist-pec-correlation-proof` |
| execute-process | f1e2d3c4-b5a6-4789-b012-cdef34567890 | 1.2.0 | ecosystem-evolution | Orquestación maestra para que Tekton ejecute un proceso del Core con resolución SSOT, mapa de identidad (canónico + aliases v1.3.0), fases ordenadas, `phase_invocations`, delegación a crypto-broker y gate Cerbero antes de cada cápsula. | `process-load-ssot`, `phase-graph-resolution`, `phase-invocation-binding`, `cerbero-policy-gate`, `capsule-fanout-skills-tools`, `crypto-broker-delegation` |
| crypto-broker | 9b3259be-e7a0-4fb1-b5d9-620a46fbc18b | 1.0.1 | quality-assurance | Única puerta Cerbero hacia `skill:cryptography-manager` cuando el orquestador carece de `quality-assurance` en `allowed_policies`; pipe JSON stdin/stdout sin criptografía en prosa. | `cryptography-broker`, `delegate-cryptography-manager` |
| policy-validator | 3f8c2b1a-9d0e-4f7a-b2c1-0e9d8c7b6a50 | 1.0.1 | ecosystem-evolution | Dictamen Cerbero vs `execution-contexts.md` vía Cúmulo (`policy_validator.rs`). | `execution-contexts-validation`, `allowed-policies-audit`, `tool-context-compliance`, `secrets-declaration-audit` |
| emit-pr-merged-event | c0d71f2b-c1c1-4c56-8f74-2f4f41b24c4f | 2.0.2 | ecosystem-evolution | Emite `PullRequest_Merged` en `eda_bus.pending` tras merge (`accept-pr`). Handler `actions::emit_pr_merged`. | `pr-merged-event-emission`, `event-bus-pending-write`, `delegate-git-manager`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-pr-presented-event | a1b2c3d4-e5f6-4789-a012-3456789abcde | 2.0.2 | ecosystem-evolution | Sella `PullRequest_Presented` (DCC / `delivery-emit-pr-presented`). | `pr-presented-event-emission`, `event-bus-pending-write`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-work-initiated-event | c8d9e0f1-a2b3-4c5d-8e7f-6a5b4c3d2e1f | 2.0.1 | ecosystem-evolution | Emitir ECST `Work_Initiated` al abrir workspace-init (feature/bug-fix/refactorization). | `work-initiated-event-emission`, `event-bus-pending-write`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-validation-started-event | e84218a8-7e40-4436-8ae7-a3fcc3396ad7 | 2.0.1 | ecosystem-evolution | Sellar `Validation_Started` tras fases Tekton previas a validación Argos. | `validation-started-event-emission`, `event-bus-pending-write`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-tracker-sync-failed | b0c1d2e3-f4a5-4678-9012-3456789abcde | 2.0.1 | tracker-operations | Fallo tracker → Tracker_Sync_Failed sin secretos HTTP en ECST. | `tracker-sync-failed-emission`, `event-bus-pending-write`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-pr-audited-event | `b504fe38-3519-4943-bfcd-5137c3a00f01` | 2.0.2 | ecosystem-evolution | Emitir `PullRequest_Audited` tras fase de auditoría técnica en `pull-request-review` / Argos. | `pr-audited-event-emission`, `event-bus-pending-write`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-pr-resealed-event | e8f9a0b1-c2d3-4e5f-9a0b-1c2d3e4f5a6b | 2.0.2 | ecosystem-evolution | Emite `PullRequest_Resealed` tras sello diff (`git-hook-pre-push`). | `pr-resealed-event-emission`, `event-bus-pending-write`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-suite-execution-requested | d5e6f7a8-b9c0-4d1e-8f2a-3b4c5d6e7f8a | 2.0.2 | ecosystem-evolution | Publica `Suite_Execution_Requested` para disparo de suite (AIUA). | `suite-execution-requested-emission`, `event-bus-domain-write`, `delegate-crypto-broker`, `delegate-filesystem-manager` |
| emit-domain-mutation | 7e4a9c2b-1d3f-4a8e-9b6c-0f1e2d3a4b5c | 2.0.2 | ecosystem-evolution | Sello universal EDA tras forja gobernada: publica `Domain_Entity_Created/Updated/Deleted` en `pending`, valida ECST y sincroniza `SddIA/core/eda-coverage.json`. | `domain-mutation-emission`, `event-bus-pending-write`, `delegate-filesystem-manager`, `delegate-crypto-broker`, `domain-event-type-translation` |
| emit-user-preference-change-requested | `4548fdd5-7c77-4589-a289-73e622bf63df` | 2.0.2 | ecosystem-evolution | `User_Preference_Change_Requested` (domain) para cambios de preferencia Kalma2/AIUA. | `user-preference-change-emission`, `event-bus-domain-write` |
| sync-entity-index | `d6636f55-7268-48b2-93c8-cf52178c07a1` | 1.0.1 | ecosystem-evolution | Acción canónica del Agente **Cúmulo** para reconciliación asíncrona de catálogos (`index.md`) tras eventos `Domain_Entity_*` en el bus EDA. | `entity-index-reconciliation`, `delegate-filesystem-manager`, `cumulo-catalog-sync` |
| materialize-fracture-pbi | b2c3d4e5-f6a7-4890-b123-4567890abcde | 1.1.0 | ecosystem-evolution | Materializa PBI de fractura con resolutor Core por genoma YAML (ceguera nominal). | `fracture-pbi-materialization`, `fracture-pbi-resolver`, `delegate-filesystem-manager`, `cumulo-debt-ledger` |
| materialize-kaizen-alert-doc | d7e6f5a4-b3c2-4109-8765-43210abcdef0 | 1.1.0 | quality-assurance | Materializa TODO PENDING_AUDIT_DOC_* ante Kaizen_Alert_Required (plantilla por alert_kind). Cúmulo vía bus EDA. | `kaizen-alert-doc-materialization`, `delegate-filesystem-manager`, `cumulo-debt-ledger` |
| enrich-fracture-pbi-kaizen | `f830d7a4-aa71-4107-b05d-88918bd40fc8` | 1.5.1 | knowledge-management | Acción canónica del Agente **Mayeuta** ante `System_Fracture_Detected`. **No** crea el PBI (eso es Cúmulo); localiza el PBI abierto vía resolutor Core (`fracture_hash` / `fracture_process` en genoma YAML) y añade la s... | `fracture-root-cause-analysis`, `kaizen-evolution-proposal`, `fracture-pbi-resolver`, `delegate-filesystem-manager` |
| download-remote-asset | 6175f5cd-7844-4d0c-aa93-d2ce3a41d18e | 1.0.1 | knowledge-management | Reclamar activos `library_codexes` / `library_norms` del origen remoto vía DI `asset:fetch` sin nombrar proveedor (G7). Retorna `content`, `declared_hash`, `origin_kind`. | `remote-asset-reclamation`, `asset-integrity-declaration` |

## Archivos en carpeta no catalogados como acción

Ninguno. `actions-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** diez definiciones de acción con identidad atómica; reflejadas en sendas filas del catálogo.
- **Metadatos:** valores de la tabla (incl. **Capabilities**) copiados desde el YAML de cada `{name}.md` al momento de indexación.

