---
uuid: "e9c66ec6-5b59-4aae-b9f2-91cc313fe295"
name: "capability-taxonomy"
version: "1.0.0"
nature: "tactical-norm"
author: "tekton"
scope: "agnostic"
category: "architecture"
hash_signature: "sha256:5637306f1df71c69672369a2894c14ac7d1731ac68e1433972f3f6edba28bb1a"
catalog:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
    description: "Cierre documental en rama (archivo PBI / validacion pbi_archived)."
---

## Directriz Core

**Códice de la Lengua** — Taxonomía Universal de Capacidades SddIA.

Ninguna Entidad de Dominio puede declarar `provides` o `requires_capability` con un `id` que no figure en el catálogo machine-readable de esta norma (`catalog` en frontmatter). La invención libre de términos (`doc:close` vs `document:closure`, etc.) es **Entropía Semántica** y falla el Filtro A.

Alta de términos: únicamente vía `entity-manager` (`lifecycle_operation: update`) sobre esta norma + registro en `SddIA/evolution/`.

## Catálogo vigente (legible)

| id | contract | version | Descripción |
|----|----------|---------|-------------|
| `doc:closure` | `doc.closure` | 1.0.0 | Cierre documental en rama |

## Restricciones Duras

- Prohibido declarar capacidades no indexadas en `catalog`.
- Prohibido usar `spec.json` como soporte de `provides`/`requires_capability`; SSOT = `{name}.md`.
- El contrato I/O de cada capacidad vive bajo el path Cúmulo `capability_contracts` (`{contract}.schema.json`).
- Referencia feature: `docs/features/inyeccion-dependencias-capacidades` (PBI-042).
