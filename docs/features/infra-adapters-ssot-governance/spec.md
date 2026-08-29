---
feature_name: infra-adapters-ssot-governance
created: "2026-08-29"
process: feature
base: main
scope: infra-adapters-ssot-governance
version_spec: "1.0.0"
uuid: b7e4c1a9-2f83-4d6e-9a15-3c8f0d2b6e47
status: dedalo_locked
agent: dedalo
branch_name: feat/infra-adapters-ssot-governance
persist_ref: docs/features/infra-adapters-ssot-governance
pbi_ref: docs/todos/pending/PBI-ARCH-INFRA-ADAPTERS-SSOT-001.md
document_id: PBI-ARCH-INFRA-ADAPTERS-SSOT-001
execution_id: "eb646386-6dc9-43d8-9b08-630de228a192"
---

# Especificación — infra-adapters-ssot-governance

## 1. Topología

```text
SddIA/core/cumulo.paths.json          ← evoluciona (directories + contracts; 1.7.0)
SddIA/infrastructure/                 ← INTANGIBLE árbol; se declara, no se mueve
SddIA/infrastructure/adapters/
  adapters-contract.md                ← nace (contrato de familia)
  index.md                            ← nace (censo Cúmulo)
  lancedb-thought-repo.md             ← nace (ficha)
  lancedb-evolution-repo.md           ← nace (ficha)
  lancedb_thought_repo/               ← INTANGIBLE crate (PBI LanceDB real)
  lancedb_evolution_repo/             ← INTANGIBLE crate
SddIA/evolution/{uuid}.md             ← nace (cicatriz)
docs/todos/pending/PBI-ARCH-…         ← evoluciona (refined)
Espejo PBI DD-7                       ← evoluciona (puntero a este document_id)
sync_entity_index.rs                  ← INTANGIBLE este ciclo
entity-manager / Constitución         ← INTANGIBLE (sin type:adapter)
```

## 2. Laudos Dedalo

| Ref | Pregunta | Laudo |
|-----|----------|-------|
| **L1** | Claves SSOT | `directories.infrastructure`, `directories.infrastructure_adapters`; `contracts.infrastructure_adapters` |
| **L2** | SemVer paths | `1.6.5` → `1.7.0` |
| **L3** | Identidad | kebab `{name}.md` + `index.md`. Sin clase nueva en forja |
| **L4** | Contrato | `adapters-contract.md` v1.0.0 junto al índice (patrón daemons) |
| **L5** | Frontmatter ficha | `id`, `uuid`, `type: infrastructure-adapter` (**etiqueta local**, no taxonomía Constitución), `version`, `status`, `crate_name`, `impl_dir`, `contract` |
| **L6** | `status` | `placeholder` \| `active` \| `deprecated`. Ambos LanceDB = `placeholder` hasta `PBI-CORE-LANCEDB-REAL-001` |
| **L7** | Auditoría INF-CA2 | Estática: cada fila del índice = ficha + `stat` de `impl_dir`. **No** extender `index_map` este ciclo |
| **L8** | Consumidor | Resolver `directories.infrastructure_adapters` + leer `index.md`. Prohibido glob `SddIA/infrastructure/adapters/**/src` |
| **L9** | IOTA | Sigue en `tools/index.md`. No duplicar como adaptador |
| **L10** | Evolution | Hito ↔ uuid `b7e4c1a9-2f83-4d6e-9a15-3c8f0d2b6e47` |
| **L11** | PBI path | SSOT documental = `PBI-ARCH-INFRA-ADAPTERS-SSOT-001.md` (sin `/` en el filename) |

## 3. Contrato de familia (mínimo)

Cabecera YAML de cada ficha:

```yaml
id: lancedb-thought-repo          # kebab-case
uuid: <v4>
type: infrastructure-adapter      # etiqueta de familia, no entity-manager
version: 1.0.0
status: placeholder               # placeholder | active | deprecated
crate_name: sddia-infrastructure-lancedb-thought
impl_dir: lancedb_thought_repo    # relativo a directories.infrastructure_adapters
contract: adapters-contract v1.0.0
```

Índice: frontmatter `entity_family: infrastructure-adapters`, `maintained_by_agent: cumulo`, `directories_key: infrastructure_adapters`, columnas `Archivo fuente | uuid | name | version | status | crate_name | impl_dir`.

## 4. Criterios (mapeo PBI)

| ID | Verificación Dedalo |
|----|---------------------|
| INF-CA1 | Diff `cumulo.paths.json` 1.7.0 + L1 |
| INF-CA2 | Índice + contrato presentes; Argos: filas = fichas + dirs; sin `sync_entity_index` |
| INF-CA3 | Dos fichas; `status: placeholder`; `src/lib.rs` sin crate `lancedb` |
| INF-CA4 | Consumidor lee clave Cúmulo + `index.md` |
| INF-CA5 | `SddIA/evolution/b7e4c1a9-2f83-4d6e-9a15-3c8f0d2b6e47.md` |
| INF-CA6 | Cierre en un PR (fase posterior) |
