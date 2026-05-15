---
index_version: "1.0.0"
entity_family: "library-codexes"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "library_codexes"
indexed_at: "2026-05-15"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado."
---

# Índice de códices de dominio (Librería SddIA)

Contrato normativo de la familia: `codex-contract.md` (no constituye un códice de dominio catalogado).

## Catálogo de definiciones (`{slug}.md`)

| Archivo fuente | uuid | name | version | target_environment | certification_grade |
|----------------|------|------|---------|-------------------|---------------------|
| `codex-backend-admin-splus.md` | `04f0bc4e-ef1f-4431-a445-398f1820db07` | SddIA Codex Backend Admin S+ | 1.0.0 | backend, dotnet, admin | Pendiente |
| `codex-frontend-admin-splus.md` | `30dc1742-aba2-4c30-8a3a-eea0e256194b` | SddIA Codex Frontend Admin S+ | 1.0.0 | frontend, nextjs, admin | Pendiente |
| `codex-frontend-product-splus.md` | `0b681575-8d20-413c-bc0e-a5ef1a378f7b` | SddIA Codex Frontend Product S+ | 1.0.0 | frontend, nextjs, product | Pendiente |

## Archivos en carpeta no catalogados como códice de dominio

Ninguno. `codex-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** tres archivos `domain-codex` con identidad atómica; reflejados en sendas filas del catálogo.
- **Metadatos:** valores de la tabla copiados desde el YAML de cada códice al momento de indexación.
