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
| append-mayeuta-hypothesis | d5c792b5-33a1-4287-a66f-ea3b2ad5716d | 1.0.0 | knowledge-management | Suscriptor de `Fracture_Clarification_Requested`. Inyecta hipótesis consultiva (`skill:mayeuta-llm` `SYNTHESIZE`) en el PBI de fractura inédita. No altera el sello determinista de Cúmulo/enrich. Handler nativo en `exe... | ``, `` |
| crypto-broker | 9b3259be-e7a0-4fb1-b5d9-620a46fbc18b | 1.0.0 | quality-assurance | Ser la **única puerta autorizada** hacia `skill:cryptography-manager` cuando el orquestador principal (p. ej. `execute-process` bajo contexto `ecosystem-evolution`) no posee en su `allowed_policies` el contexto `quali... | ``, `` |
| dispatch-aiua-intent | a1086194-e19c-49b1-88e1-bd828211b3a8 | 2.0.0 | ecosystem-evolution | Traduce un tendón Aiúa (`function_call.name` + `args`) a ECST en `eda_fractal.domain`. / Tendón / Destino / /--------/---------/ | `` |
| download-remote-asset | 6175f5cd-7844-4d0c-aa93-d2ce3a41d18e | 1.0.0 | knowledge-management | Reclamación de activos del repositorio maestro con declaración de integridad. La acción no nombra ni inspecciona al proveedor concreto de `asset:fetch`; la abstracción es total (pivote DLT G7). / Campo / Obligatorio /... | ``, `` |
| emit-domain-mutation | 7e4a9c2b-1d3f-4a8e-9b6c-0f1e2d3a4b5c | 2.0.2 | ecosystem-evolution | Sello universal Domain_Entity_* tras mutación del genoma. | ``, ``, ``, ``, `` |
| emit-pr-audited-event | 46dc0c14-d83f-40b5-a0e2-1fc58dd0871d | 2.0.6 | ecosystem-evolution | Entidad `emit-pr-audited-event` del Core SddIA. | ``, ``, ``, `` |
| emit-pr-merged-event | c0d71f2b-c1c1-4c56-8f74-2f4f41b24c4f | 2.0.3 | ecosystem-evolution | Emite PullRequest_Merged tras accept-pr | ``, ``, ``, ``, `` |
| emit-pr-presented-event | a1b2c3d4-e5f6-4789-a012-3456789abcde | 2.0.2 | ecosystem-evolution | Emite PullRequest_Presented en eda_bus.pending vía EventManagerPort. | ``, ``, ``, `` |
| emit-pr-resealed-event | e8f9a0b1-c2d3-4e5f-9a0b-1c2d3e4f5a6b | 2.0.2 | ecosystem-evolution | Emite evento de resellado diff/validación en revisión. | ``, ``, ``, `` |
| emit-suite-execution-requested | d5e6f7a8-b9c0-4d1e-8f2a-3b4c5d6e7f8a | 2.0.2 | ecosystem-evolution | Encola Suite_Execution_Requested desde despacho Aiúa. | ``, ``, ``, `` |
| emit-tracker-sync-failed | b0c1d2e3-f4a5-4678-9012-3456789abcde | 2.0.1 | ecosystem-evolution | Registra Tracker_Sync_Failed para tracker-sync-replay. | ``, ``, ``, `` |
| emit-user-preference-change-requested | c8e6a89d-3dff-486a-8239-3e37a6c60b6e | 2.0.7 | ecosystem-evolution | Entidad `emit-user-preference-change-requested` del Core SddIA. | ``, `` |
| emit-validation-started-event | e84218a8-7e40-4436-8ae7-a3fcc3396ad7 | 2.0.1 | ecosystem-evolution | Emite Validation_Started (fail-soft) al entrar en Verificación. | ``, ``, ``, `` |
| emit-work-initiated-event | c8d9e0f1-a2b3-4c5d-8e7f-6a5b4c3d2e1f | 2.0.1 | ecosystem-evolution | Emite Work_Initiated al cerrar workspace-init en feature, bug-fix y refactorization. | ``, ``, ``, `` |
| enrich-fracture-pbi-kaizen | 5c9850ad-38ad-4bc4-8807-26db1105dea7 | 1.5.2 | ecosystem-evolution | Entidad `enrich-fracture-pbi-kaizen` del Core SddIA. | ``, ``, ``, `` |
| execute-process | f1e2d3c4-b5a6-4789-b012-cdef34567890 | 1.2.0 | ecosystem-evolution | Orquestar de extremo a extremo la ejecución autónoma de un **proceso** del Core SddIA: resolver su ley vía Cúmulo, recorrer sus fases con Tekton, someter cada intención ejecutiva a Cerbero y delegar la física exclusiv... | ``, ``, ``, ``, ``, `` |
| invoke-aiua-core | 2edc7ef4-57e5-4d10-8753-bab8b0053cca | 1.1.0 | ecosystem-evolution | Leer `aiua_core.md` vía `directories.conscience` (Cúmulo). Ensamblar request.prompt: prefacio de identidad (frontmatter `name`/`entity_type`, primera persona) + genoma + contexto activo + ` | `` |
| materialize-ci-chronic-failure-pbi | a6eb7f0c-8b2f-4c7d-ae5e-6c1b589f3c92 | 1.0.1 | ecosystem-evolution | Materializa PBI kaizen ante CI crónico. | `` |
| materialize-fracture-pbi | b2c3d4e5-f6a7-4890-b123-4567890abcde | 1.1.0 | ecosystem-evolution | Acción canónica del Agente **Cúmulo** ante `System_Fracture_Detected`. Elige el sumidero según `tracker.provider` del proyecto resuelto (slug del payload o índice auto-hospedado). `linear` crea la issue por `tracker:l... | ``, ``, ``, `` |
| materialize-kaizen-alert-doc | d7e6f5a4-b3c2-4109-8765-43210abcdef0 | 1.2.0 | quality-assurance | Acción canónica del Agente **Cúmulo** ante `Kaizen_Alert_Required`. Materializa la cicatriz Kaizen con política **dual** (paridad `materialize_kaizen_alert_doc.rs`): / Modo / Destino / /------/---------/ | ``, ``, `` |
| notify-humanized-pr-merged | 1cd7bd40-b72f-4114-ac44-68b912774aa6 | 1.1.0 | ecosystem-evolution | Suscriptor Argos de `PullRequest_Merged`. Ensambla el bloque estático (compositor ola 1) con una síntesis de valor (≤2 líneas) obtenida vía `gemini-http-infer`. Antes de Gemini, resuelve `git-manager commit_summary` (... | ``, ``, ``, `` |
| persist-pec-correlation-proof | accb4de7-bb1e-4f88-b5cd-b8775a8ff5a4 | 1.0.0 | ecosystem-evolution | Suscriptor Cúmulo de `Process_Execution_Completed`. Persiste una **proyección durable** del cierre táctico indexada por `correlation_id` bajo `{eda_instance.proofs}/pec-correlation/{correlation_id}.json`, **antes** de... | ``, ``, `` |
| persist-thought-record | a37f9f1d-8f2a-441f-9357-776d65553362 | 1.1.0 | ecosystem-evolution | Persistir el par estímulo/respuesta en el grafo de pensamiento. **No** emite `Thought_Persisted`: el adaptador `lancedb-thought-repo` es el emisor autorizado. | `` |
| policy-validator | 3f8c2b1a-9d0e-4f7a-b2c1-0e9d8c7b6a50 | 1.0.0 | ecosystem-evolution | Emitir un **dictamen normativo** contrastando entradas de forja (agentes, tools) con la matriz de contextos S+ definida en `execution-contexts.md`, sin rutas hardcodeadas: la norma se localiza exclusivamente vÃ­a topo... | ``, ``, ``, `` |
| purge-repo-retention | e4b84b41-c804-4c34-8fea-8b0f6bbcb573 | 1.0.1 | ecosystem-evolution | Orquesta repo-retention-purger simulate o apply. | `` |
| purge-sandbox-cache | 37454c3a-4590-4dfb-a439-2014876a0138 | 1.0.2 | ecosystem-evolution | Orquesta limpieza de caché sandbox autorizada. | ``, `` |
| retrieve-active-context | afa0424b-cdd4-4810-9a29-d8e7c06d6a1f | 1.1.0 | ecosystem-evolution | Recuperar recuerdos KNN del grafo de pensamiento previo a la inyección genómica. `memories` vacío no es error. No toca el adaptador ni emite ECST. | `` |
| switch-llm-provider | a6f5e0a4-ccc5-4076-bf8a-b60cfe1fa684 | 1.0.2 | ecosystem-evolution | Conmutación explícita del proveedor LLM activo en .SddIA/llm-runtime.local.json, validando roles en capability-bindings.md sin auto-failover. | `` |
| sync-entity-index | 15b6f438-184c-4078-a483-22faa77447fa | 1.0.2 | ecosystem-evolution | Entidad `sync-entity-index` del Core SddIA. | ``, ``, `` |

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico (success, exitCode, data, error).

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
Test.

## 2. Orquestación
Paso.

## 3. Límites
Ninguno.
 |  |
| purge-repo-retention | e4b84b41-c804-4c34-8fea-8b0f6bbcb573 | 1.0.1 | ecosystem-evolution | ## 1. Propósito

Orquesta repo-retention-purger simulate o apply.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| dispatch-aiua-intent | a1086194-e19c-49b1-88e1-bd828211b3a8 | 2.0.0 | ecosystem-evolution | Acción dispatch-aiua-intent | `dispatch_aiua_intent` |
| purge-sandbox-cache | 37454c3a-4590-4dfb-a439-2014876a0138 | 1.0.2 | ecosystem-evolution | ## 1. Propósito

Orquesta limpieza de caché sandbox autorizada.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| append-mayeuta-hypothesis | d5c792b5-33a1-4287-a66f-ea3b2ad5716d | 1.0.0 | knowledge-management | Hipótesis semántica asíncrona (mayeuta-llm SYNTHESIZE); upsert H2; fail-open. | `fracture-semantic-hypothesis`, `delegate-mayeuta-llm` |
| persist-thought-record | a37f9f1d-8f2a-441f-9357-776d65553362 | 1.1.0 | ecosystem-evolution | Acción persist-thought-record | `persist_thought_record` |
| invoke-aiua-core | 2edc7ef4-57e5-4d10-8753-bab8b0053cca | 1.1.0 | ecosystem-evolution | Acción invoke-aiua-core | `invoke_aiua_core` |
| retrieve-active-context | afa0424b-cdd4-4810-9a29-d8e7c06d6a1f | 1.1.0 | ecosystem-evolution | Acción retrieve-active-context | `retrieve_active_context` |
| notify-humanized-pr-merged | 1cd7bd40-b72f-4114-ac44-68b912774aa6 | 1.1.0 | ecosystem-evolution | Estático + commit_summary fail-soft + gemini-http-infer + send-telegram-notification. | `notify-humanized-pr-merged`, `delegate-gemini-http-infer`, `delegate-send-telegram-notification`, `delegate-git-manager` |
| materialize-ci-chronic-failure-pbi | a6eb7f0c-8b2f-4c7d-ae5e-6c1b589f3c92 | 1.0.1 | ecosystem-evolution | ## 1. Propósito

Materializa PBI kaizen ante CI crónico.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| persist-pec-correlation-proof | `accb4de7-bb1e-4f88-b5cd-b8775a8ff5a4` | 1.0.0 | ecosystem-evolution | Proyección durable PEC por correlation_id bajo eda_instance.proofs/pec-correlation/. | `persist-pec-correlation-proof` |
| execute-process | f1e2d3c4-b5a6-4789-b012-cdef34567890 | 1.2.0 | ecosystem-evolution | Orquestación maestra para que Tekton ejecute un proceso del Core con resolución SSOT, mapa de identidad (canónico + aliases v1.3.0), fases ordenadas, `phase_invocations`, delegación a crypto-broker y gate Cerbero antes de cada cápsula. | `process-load-ssot`, `phase-graph-resolution`, `phase-invocation-binding`, `cerbero-policy-gate`, `capsule-fanout-skills-tools`, `crypto-broker-delegation` |
| crypto-broker | 9b3259be-e7a0-4fb1-b5d9-620a46fbc18b | 1.0.0 | quality-assurance | Puerta RBAC hacia operaciones deterministas de `cryptography-manager` (UUID, SHA-256, validación de hash) sin exponer `quality-assurance` al orquestador de forja. | `cryptography-broker`, `delegate-cryptography-manager` |
| policy-validator | 3f8c2b1a-9d0e-4f7a-b2c1-0e9d8c7b6a50 | 1.0.0 | ecosystem-evolution | Dictamen normativo: contrasta `allowed_policies` / `tool_context` / declaración de secretos con `execution-contexts.md` vía SSOT; usada en fases de auditoría de `agent-creator` y `tool-creator`. | `execution-contexts-validation`, `allowed-policies-audit`, `tool-context-compliance`, `secrets-declaration-audit` |
| emit-pr-merged-event | c0d71f2b-c1c1-4c56-8f74-2f4f41b24c4f | 2.0.3 | ecosystem-evolution | ## 1. Propósito

Emite PullRequest_Merged tras accept-pr

## 2. Orquestación

1. Validar entradas.
2. Delegar vía Cúmulo.
3. Envelope JSON canónico.

## 3. Límites

Sin SO directo. |  |

Emite PullRequest_Merged tras fusión soberana (accept-pr).

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| emit-pr-presented-event | a1b2c3d4-e5f6-4789-a012-3456789abcde | 2.0.2 | ecosystem-evolution | ## 1. Propósito

Emite PullRequest_Presented en eda_bus.pending vía EventManagerPort.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico (success, exitCode, data, error).

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| emit-work-initiated-event | c8d9e0f1-a2b3-4c5d-8e7f-6a5b4c3d2e1f | 2.0.1 | ecosystem-evolution | ## 1. Propósito

Emite Work_Initiated al cerrar workspace-init en feature, bug-fix y refactorization.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico (success, exitCode, data, error).

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| emit-validation-started-event | e84218a8-7e40-4436-8ae7-a3fcc3396ad7 | 2.0.1 | ecosystem-evolution | ## 1. Propósito

Emite Validation_Started (fail-soft) al entrar en Verificación.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| emit-tracker-sync-failed | b0c1d2e3-f4a5-4678-9012-3456789abcde | 2.0.1 | ecosystem-evolution | ## 1. Propósito

Registra Tracker_Sync_Failed para tracker-sync-replay.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| emit-pr-audited-event | 46dc0c14-d83f-40b5-a0e2-1fc58dd0871d | 2.0.6 | ecosystem-evolution | Acción emit-pr-audited-event

## Propósito

Entidad `emit-pr-audited-event` del Core SddIA.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
 |  |

Emisión ECST de dominio para emit-pr-audited-event.

## 2. Orquestación

1. Validar entradas.
2. Delegar vía Cúmulo y EventManagerPort.
3. Envelope JSON canónico.

## 3. Límites

Sin SO directo. |  |
| emit-pr-resealed-event | e8f9a0b1-c2d3-4e5f-9a0b-1c2d3e4f5a6b | 2.0.2 | ecosystem-evolution | ## 1. Propósito

Emite evento de resellado diff/validación en revisión.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| emit-suite-execution-requested | d5e6f7a8-b9c0-4d1e-8f2a-3b4c5d6e7f8a | 2.0.2 | ecosystem-evolution | ## 1. Propósito

Encola Suite_Execution_Requested desde despacho Aiúa.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| emit-domain-mutation | 7e4a9c2b-1d3f-4a8e-9b6c-0f1e2d3a4b5c | 2.0.2 | ecosystem-evolution | ## 1. Propósito

Sello universal Domain_Entity_* tras mutación del genoma.

## 2. Orquestación

1. Validar entradas frente a actions-contract.md.
2. Resolver cápsulas vía Cúmulo; delegar física en skills/tools autorizados.
3. Devolver envelope JSON canónico.

## 3. Límites

Sin terminal nativa. Sin bypass de Cerbero ni del bus EDA. |  |
| emit-user-preference-change-requested | c8e6a89d-3dff-486a-8239-3e37a6c60b6e | 2.0.7 | ecosystem-evolution | Acción emit-user-preference-change-requested

## Propósito

Entidad `emit-user-preference-change-requested` del Core SddIA.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
 |  |
| sync-entity-index | 15b6f438-184c-4078-a483-22faa77447fa | 1.0.2 | ecosystem-evolution | Acción sync-entity-index

## Propósito

Entidad `sync-entity-index` del Core SddIA.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
 |  |
| materialize-fracture-pbi | b2c3d4e5-f6a7-4890-b123-4567890abcde | 1.1.0 | ecosystem-evolution | Materializa PBI de fractura con resolutor Core por genoma YAML (ceguera nominal). | `fracture-pbi-materialization`, `fracture-pbi-resolver`, `delegate-filesystem-manager`, `cumulo-debt-ledger` |
| materialize-kaizen-alert-doc | d7e6f5a4-b3c2-4109-8765-43210abcdef0 | 1.1.0 | quality-assurance | Materializa TODO PENDING_AUDIT_DOC_* ante Kaizen_Alert_Required (plantilla por alert_kind). Cúmulo vía bus EDA. | `kaizen-alert-doc-materialization`, `delegate-filesystem-manager`, `cumulo-debt-ledger` |
| enrich-fracture-pbi-kaizen | 5c9850ad-38ad-4bc4-8807-26db1105dea7 | 1.5.2 | ecosystem-evolution | Acción enrich-fracture-pbi-kaizen

## Propósito

Entidad `enrich-fracture-pbi-kaizen` del Core SddIA.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
 |  |
| download-remote-asset | 6175f5cd-7844-4d0c-aa93-d2ce3a41d18e | 1.0.0 | knowledge-management | Reclamación de activos del repositorio maestro con declaración de integridad SHA-256. Abstracción de origen opaca (pivote DLT G7). | `remote-asset-reclamation`, `asset-integrity-declaration` |

## Archivos en carpeta no catalogados como acción

Ninguno. `actions-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** diez definiciones de acción con identidad atómica; reflejadas en sendas filas del catálogo.
- **Metadatos:** valores de la tabla (incl. **Capabilities**) copiados desde el YAML de cada `{name}.md` al momento de indexación.
