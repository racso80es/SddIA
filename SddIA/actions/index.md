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
| execute-process | f1e2d3c4-b5a6-4789-b012-cdef34567890 | 1.2.0 | ecosystem-evolution | Orquestación maestra para que Tekton ejecute un proceso del Core con resolución SSOT, mapa de identidad (canónico + aliases v1.3.0), fases ordenadas, `phase_invocations`, delegación a crypto-broker y gate Cerbero antes de cada cápsula. | `process-load-ssot`, `phase-graph-resolution`, `phase-invocation-binding`, `cerbero-policy-gate`, `capsule-fanout-skills-tools`, `crypto-broker-delegation` |
| crypto-broker | 9b3259be-e7a0-4fb1-b5d9-620a46fbc18b | 1.0.0 | quality-assurance | Puerta RBAC hacia operaciones deterministas de `cryptography-manager` (UUID, SHA-256, validación de hash) sin exponer `quality-assurance` al orquestador de forja. | `cryptography-broker`, `delegate-cryptography-manager` |
| policy-validator | 3f8c2b1a-9d0e-4f7a-b2c1-0e9d8c7b6a50 | 1.0.0 | ecosystem-evolution | Dictamen normativo: contrasta `allowed_policies` / `tool_context` / declaración de secretos con `execution-contexts.md` vía SSOT; usada en fases de auditoría de `agent-creator` y `tool-creator`. | `execution-contexts-validation`, `allowed-policies-audit`, `tool-context-compliance`, `secrets-declaration-audit` |
| emit-pr-merged-event | c0d71f2b-c1c1-4c56-8f74-2f4f41b24c4f | 1.1.0 | dlt-auditing | Extrae hash post-merge en main y emite PullRequest_Merged (target_branch fijo main) en `.SddIA/events/pending/`. | `pr-merged-event-emission`, `event-bus-pending-write`, `delegate-git-manager`, `delegate-crypto-broker`, `delegate-filesystem-manager` |

## Archivos en carpeta no catalogados como acción

Ninguno. `actions-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** cuatro definiciones de acción con identidad atómica; reflejadas en sendas filas del catálogo.
- **Metadatos:** valores de la tabla (incl. **Capabilities**) copiados desde el YAML de `execute-process.md`, `crypto-broker.md`, `policy-validator.md` y `emit-pr-merged-event.md` al momento de indexación.
