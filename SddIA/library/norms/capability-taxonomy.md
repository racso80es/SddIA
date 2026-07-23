---
uuid: "e9c66ec6-5b59-4aae-b9f2-91cc313fe295"
name: "capability-taxonomy"
version: "1.0.4"
nature: "tactical-norm"
author: "tekton"
scope: "agnostic"
category: "architecture"
hash_signature: "sha256:e0fa06d7b8b9a299d0afb9ac35cc35317d5e5746415bf47d20c84749a8358523"
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
  - id: "qa:probe"
    contract: "qa.probe"
    version: "1.0.0"
    description: "Sonda de caos / auditoría empírica vía tools (sandbox-breacher, schema-corruptor, io-choke, event-bus-audit)."
  - id: "audit:compliance"
    contract: "audit.compliance"
    version: "1.0.0"
    description: "Auditoría de cumplimiento termodinámico/gobernanza (exclusiva telemetry-compliance-audit); distinta de qa:probe (Caos)."
  - id: "llm:interact"
    contract: "llm.interact"
    version: "1.0.0"
    description: "Interacción LLM gobernada vía skill mayeuta-llm (síntesis / clasificación de intención)."
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
| `qa:probe` | `qa.probe` | 1.0.0 | Sonda Caos / auditoría empírica vía tools |
| `audit:compliance` | `audit.compliance` | 1.0.0 | Cumplimiento termodinámico (Gobernanza; ≠ Caos) |
| `llm:interact` | `llm.interact` | 1.0.0 | Interacción LLM vía mayeuta-llm |

## Restricciones Duras

- Prohibido declarar capacidades no indexadas en `catalog`.
- Prohibido usar `spec.json` como soporte de `provides`/`requires_capability`; SSOT = `{name}.md`.
- El contrato I/O de cada capacidad vive bajo el path Cúmulo `capability_contracts` (`{contract}.schema.json`).
- **Rigor taxonómico:** `qa:probe` (Caos/sonda) ≠ `audit:compliance` (Gobernanza/cumplimiento). Prohibido reuso cruzado.
- Referencia: PBI-042/043 features DI; H9 `inyeccion-dependencias-h9-auditorias`.
