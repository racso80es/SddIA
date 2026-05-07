---
index_version: "1.0.0"
entity_family: "agents"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "agents"
indexed_at: "2026-05-07"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado; columna Allowed policies obligatoria (agents-contract)."
---

# Índice de agents (Core SddIA)

Contrato normativo de la familia: `agents-contract.md` (no constituye un agente catalogado en esta tabla).

## Catálogo de definiciones (`{name}.md`)

| Archivo fuente | uuid | name | version | contract | Allowed policies |
|----------------|------|------|---------|----------|------------------|
| `cerbero.md` | `e1f2a3b4-c5d6-7e8f-9a0b-1c2d3e4f5a6b` | cerbero | 1.0.0 | agents-contract v1.0.0 | `knowledge-management` |
| `cumulo.md` | `8f7d6c5b-4a01-4e56-9a2b-e98e4d2a1c3f` | cumulo | 1.0.0 | agents-contract v1.0.0 | `knowledge-management`, `ecosystem-evolution` |
| `tekton.md` | `b3a4c5d6-7e8f-9a0b-1c2d-3e4f5a6b7c8d` | tekton | 1.1.0 | agents-contract v1.1.0 | `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations` |
| `argos.md` | `bd3b1d76-3734-4fbb-b447-ad5e4a5e4907` | argos | 1.0.0 | agents-contract v1.0.0 | `quality-assurance`, `filesystem-ops`, `source-control` |
| `dedalo.md` | `9e60e48e-43be-463d-999e-d3dbd83924af` | dedalo | 1.0.0 | agents-contract v1.0.0 | `ecosystem-evolution`, `knowledge-management`, `filesystem-ops` |
| `mayeuta.md` | `db1acdb5-23b9-490e-a339-dc511091e959` | mayeuta | 1.0.0 | agents-contract v1.0.0 | `knowledge-management`, `filesystem-ops` |

## Archivos en carpeta no catalogados como agente

Solo `agents-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** seis definiciones de agente con identidad atómica; reflejadas en sendas filas del catálogo.
- **Metadatos:** valores de la tabla (incl. **Allowed policies**) copiados desde el YAML de `cerbero.md`, `cumulo.md`, `tekton.md`, `argos.md`, `dedalo.md` y `mayeuta.md` al momento de indexación. Tekton incluye `ecosystem-evolution`, `filesystem-ops`, `source-control` (invocación de `skill:git-manager` tras gate Cerbero) y `system-operations` (invocación de `skill:shell-executor` para binarios terceros; `git` prohibido aquí). Criptografía solo vía `action:crypto-broker` y regla Cerbero en `execute-process`. El agente `argos` orquesta verificación vía `action:execute-process` con políticas `quality-assurance`, `filesystem-ops` y `source-control`. El agente `dedalo` diseña blueprints de proceso cruzando contextos de cápsulas contra `target_executor_rbac` inyectado; excepciones de delegación según `execute-process` y normas vigentes. El agente `mayeuta` estabiliza requisitos y transcript bajo `knowledge-management` + `filesystem-ops`, sin mutar SSOT ni emitir YAML de `process-contract`; al escalar a `dedalo`, el runtime reinyecta `cumulo_topology`, `active_norm_pack` y `target_executor_rbac` junto a `refined_requirements` derivado de `thermodynamic_stable_requirement_md`.
