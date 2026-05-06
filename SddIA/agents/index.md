---
index_version: "1.0.0"
entity_family: "agents"
maintained_by_agent: "cumulo"
paths_ref: "cumulo.paths.json"
directories_key: "agents"
indexed_at: "2026-05-06"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado; columna Allowed policies obligatoria (agents-contract)."
---

# Índice de agents (Core SddIA)

Contrato normativo de la familia: `agents-contract.md` (no constituye un agente catalogado en esta tabla).

## Catálogo de definiciones (`{name}.md`)

| Archivo fuente | uuid | name | version | contract | Allowed policies |
|----------------|------|------|---------|----------|------------------|
| `cerbero.md` | `e1f2a3b4-c5d6-7e8f-9a0b-1c2d3e4f5a6b` | cerbero | 1.0.0 | agents-contract v1.0.0 | `knowledge-management` |
| `cumulo.md` | `8f7d6c5b-4a01-4e56-9a2b-e98e4d2a1c3f` | cumulo | 1.0.0 | agents-contract v1.0.0 | `knowledge-management`, `ecosystem-evolution` |
| `tekton.md` | `b3a4c5d6-7e8f-9a0b-1c2d-3e4f5a6b7c8d` | tekton | 1.1.0 | agents-contract v1.1.0 | `ecosystem-evolution`, `filesystem-ops` |
| `auditor.md` | `bd3b1d76-3734-4fbb-b447-ad5e4a5e4907` | auditor | 1.0.0 | agents-contract v1.0.0 | `quality-assurance`, `filesystem-ops`, `source-control` |

## Archivos en carpeta no catalogados como agente

Solo `agents-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** cuatro definiciones de agente con identidad atómica; reflejadas en sendas filas del catálogo.
- **Metadatos:** valores de la tabla (incl. **Allowed policies**) copiados desde el YAML de `cerbero.md`, `cumulo.md`, `tekton.md` y `auditor.md` al momento de indexación. Políticas de Tekton acotadas a los contextos de los procesos Core catalogados (`ecosystem-evolution` + `filesystem-ops` vía `skill:filesystem-manager`; criptografía solo vía `action:crypto-broker` y regla Cerbero en `execute-process`). Si un proceso nuevo declara cápsulas de otro contexto (p. ej. `source-control`), ampliar `allowed_policies` de Tekton y esta fila en el mismo commit que el proceso. El agente `auditor` orquesta verificación vía `action:execute-process` con políticas `quality-assurance`, `filesystem-ops` y `source-control`.
