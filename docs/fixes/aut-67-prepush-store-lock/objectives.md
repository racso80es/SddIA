---
feature_name: aut-67-prepush-store-lock
created: "2026-11-02"
process: bug-fix
branch_name: Fix_Events_Multiple_02
sighting_branch: Fix_Events_Multiple_02
tracker_ref: AUT-67
document_id: PBI-3d736171
uuid: e46d8e0c-b9fa-4d18-a035-99e9cf66ac07
persist_ref: docs/fixes/aut-67-prepush-store-lock
pbi_ref: AUT-67
execution_id: "e5c2418a-ce87-45ae-a522-2246568fd632"
---

# Objetivos — aut-67-prepush-store-lock

## Misión

AUT-67. El pre-push con PR abierto trata StoreError::Locked de events.redb como fallo de Local_QA_Requested. Distinguir contención del watcher de rechazo de QA. No subir SDDIA_EVENT_STORE_OPEN_BUDGET_MS. Fuera: atestación tree_sha desfasada.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
