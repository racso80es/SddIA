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
| `filesystem-manager.md` | `f4a5b6c7-d8e9-4f0a-1b2c-3d4e5f6a7b8c` | filesystem-manager | 1.0.0 | skills-contract v1.1.0 | filesystem-ops | `file-read`, `file-write`, `list-directory`, `delete-file`, `create-directory`, `move-file` |
| `cryptography-manager.md` | `a1f2e3d4-c5b6-4789-a012-3456789abc0` | cryptography-manager | 1.0.0 | skills-contract v1.1.0 | quality-assurance | `sha256-generation`, `hash-validation`, `uuid-generation` |
| `git-manager.md` | `4dac18fc-4cd1-4aa4-bdc3-faeb3bf762fc` | git-manager | 1.0.0 | skills-contract v1.1.0 | source-control | `git-read-state`, `git-branching`, `git-commit`, `git-sync-remote` |
| `shell-executor.md` | `93d23720-d79a-412f-a85d-ab9b2d9862bd` | shell-executor | 1.0.0 | skills-contract v1.1.0 | system-operations | `execute-external-binary`, `orchestrator-bridge` |

## Archivos en carpeta no catalogados como skill

Ninguno. `skills-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** cuatro archivos de definición de skill con identidad atómica; reflejados en sendas filas del catálogo.
- **Metadatos:** valores de la tabla (incl. **Capabilities**) copiados desde el YAML de `filesystem-manager.md`, `cryptography-manager.md`, `git-manager.md` y `shell-executor.md` al momento de indexación.
