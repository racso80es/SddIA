---
index_version: "1.0.0"
entity_family: "skills"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "skills"
indexed_at: "2026-05-07"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado."
---

# Índice de skills (Core SddIA)

Contrato normativo de la familia: `skills-contract.md` (no constituye una skill ejecutable).

## Catálogo de definiciones (`{name}.md`)

| Archivo fuente | uuid | name | version | contract | context | Capabilities |
|----------------|------|------|---------|------------|---------|--------------|
| `mayeuta-llm.md` | `80c96e96-3e03-4af4-bed3-0af46d3fcf7f` | mayeuta-llm | 1.1.0 | skills-contract v1.1.0 | ecosystem-evolution | `llm-synthesize`, `llm-classify-intent`, `local-subprocess-inference`, `llm:interact` |
| `filesystem-manager.md` | `f4a5b6c7-d8e9-4f0a-1b2c-3d4e5f6a7b8c` | filesystem-manager | 1.0.0 | skills-contract v1.1.0 | filesystem-ops | `file-read`, `file-write`, `list-directory`, `delete-file`, `create-directory`, `move-file` |
| `cryptography-manager.md` | `a1f2e3d4-c5b6-4789-a012-3456789abc0` | cryptography-manager | 1.0.0 | skills-contract v1.1.0 | quality-assurance | `sha256-generation`, `hash-validation`, `uuid-generation` |
| `git-manager.md` | `4dac18fc-4cd1-4aa4-bdc3-faeb3bf762fc` | git-manager | 1.2.0 | skills-contract v1.1.0 | source-control | `git-read-state`, `git-branching`, `git-commit`, `git-sync-remote` |
| `text-metrics.md` | `0c84d99b-aa67-4b27-abb6-7133867c5102` | text-metrics | 1.0.0 | skills-contract v1.1.0 | ecosystem-evolution | `text-metrics` |
| `intent-transpiler.md` | `4f0edfe0-4380-442b-962d-9e98f8ecf956` | intent-transpiler | 1.0.0 | skills-contract v1.3.0 | knowledge-management | `intent-structuring`, `ssot-path-resolution`, `feature-topology-gate` |
| `rbac-governor.md` | `131ca963-db42-43cd-ade3-a41c3b704147` | rbac-governor | 1.0.0 | skills-contract v1.1.0 | knowledge-management | `rbac_governor` |
| `sddia-evolution-register.md` | `f9d6ad5c-6f7a-49f6-89fb-60d6119776b4` | sddia-evolution-register | 1.0.0 | skills-contract v1.4.0 | ecosystem-evolution | `evolution-verdict`, `evolution-record-compute` |
| `agenda-manager.md` | `feb7314d-b86d-4653-a876-507c824ec9e2` | agenda-manager | 1.0.0 | skills-contract v1.1.0 | filesystem-ops | `agenda_manager` |
| `user-preference-store.md` | `f1a2b3c4-d5e6-4789-a012-3456789ab01` | user-preference-store | 1.0.0 | skills-contract v1.1.0 | knowledge-management | `user_preference_store`, `memory:pref-write`, `memory:pref-query` |
| `antigravity-cli-executor.md` | `d8b07e6f-1cc0-4b6f-a789-02ade10471f5` | antigravity-cli-executor | 1.0.0 | skills-contract v1.4.0 | system-operations | `antigravity_cli_executor` |
| `shell-executor.md` | `93d23720-d79a-412f-a85d-ab9b2d9862bd` | shell-executor | 1.0.0 | skills-contract v1.1.0 | system-operations | `execute-external-binary`, `orchestrator-bridge` |
| `bus-operator.md` | `c8e1f4a2-6b3d-4f9e-a1c0-2d7e8f9a0b1c` | bus-operator | 1.1.0 | skills-contract v1.1.0 | ecosystem-evolution | `eda-subscription-lookup`, `event-bus-transit`, `receipt-suffix-mutation`, `delegate-markdown-table-editor` |
| `mayeuta-llm.md` | `80c96e96-3e03-4af4-bed3-0af46d3fcf7f` | mayeuta-llm | 1.0.0 | skills-contract v1.1.0 | ecosystem-evolution | `llm-synthesize`, `llm-classify-intent`, `local-subprocess-inference` |

## Archivos en carpeta no catalogados como skill

Ninguno. `skills-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** cuatro archivos de definición de skill con identidad atómica; reflejados en sendas filas del catálogo.
- **Metadatos:** valores de la tabla (incl. **Capabilities**) copiados desde el YAML de `filesystem-manager.md`, `cryptography-manager.md`, `git-manager.md` y `shell-executor.md` al momento de indexación.

