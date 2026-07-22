---
uuid: "e9c66ec6-5b59-4aae-b9f2-91cc313fe295"
name: "capability-taxonomy"
version: "1.0.3"
nature: "tactical-norm"
author: "tekton"
scope: "agnostic"
category: "architecture"
hash_signature: "sha256:a0d1b0c634ee85f7e0a97e68179e7da35cf5f563a7ad7c1f5808977bffbbe5d2"
catalog:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
    description: "Cierre documental en rama (archivo PBI / validacion pbi_archived)."
  - id: "proc:git-sync"
    contract: "proc.git_sync"
    version: "1.0.0"
    description: "Operaciones Git gobernadas (sync, branch, commit) vía cápsula git-manager."
  - id: "fs:persist"
    contract: "fs.persist"
    version: "1.0.0"
    description: "Persistencia de artefactos vía filesystem-manager (READ/WRITE/LIST/DELETE/CREATE/MOVE) en forja/índice/workspace."
  - id: "bus:route"
    contract: "bus.route"
    version: "1.0.0"
    description: "Enrutado / fan-out del bus EDA fractal (domain, orchestration, telemetry) vía cápsula bus-operator."
---

## Directriz Core

**Códice de la Lengua** — Taxonomía Universal de Capacidades SddIA.

Ninguna Entidad de Dominio puede declarar `provides` o `requires_capability` con un `id` que no figure en el catálogo machine-readable de esta norma (`catalog` en frontmatter). La invención libre de términos (`doc:close` vs `document:closure`, etc.) es **Entropía Semántica** y falla el Filtro A.

Alta de términos: únicamente vía mutación gobernada bajo topología feature/fix + registro en `SddIA/evolution/` + sello `Domain_Entity_Updated` (R11). El forge lab `norm-creator` en `update` **no** preserva `catalog`; la materialización canónica del Códice es Write atómico del `{name}.md` + sello EDA.

## Catálogo vigente (legible)

| id | contract | version | Descripción |
|----|----------|---------|-------------|
| `doc:closure` | `doc.closure` | 1.0.0 | Cierre documental en rama |
| `proc:git-sync` | `proc.git_sync` | 1.0.0 | Operaciones Git gobernadas vía git-manager |
| `fs:persist` | `fs.persist` | 1.0.0 | Persistencia artefactos vía filesystem-manager |
| `bus:route` | `bus.route` | 1.0.0 | Fan-out bus EDA fractal vía bus-operator |

## Restricciones Duras

- Prohibido declarar capacidades no indexadas en `catalog`.
- Prohibido usar `spec.json` como soporte de `provides`/`requires_capability`; SSOT = `{name}.md`.
- El contrato I/O de cada capacidad vive bajo el path Cúmulo `capability_contracts` (`{contract}.schema.json`).
- Referencia feature: `docs/features/inyeccion-dependencias-capacidades` (PBI-042); Hito 5 alta `fs:persist`: `docs/features/inyeccion-dependencias-migracion-catalogo`; H8 alta `bus:route`: `docs/features/inyeccion-dependencias-h8-familia-route`.
