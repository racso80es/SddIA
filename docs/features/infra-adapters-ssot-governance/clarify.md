---
feature_name: infra-adapters-ssot-governance
created: "2026-08-29"
process: feature
purpose: Estabilización Mayeuta — gobernanza SSOT de infrastructure/adapters
branch_name: feat/infra-adapters-ssot-governance
persist_ref: docs/features/infra-adapters-ssot-governance
pbi_ref: docs/todos/pending/PBI-ARCH-INFRA-ADAPTERS-SSOT-001.md
document_id: PBI-ARCH-INFRA-ADAPTERS-SSOT-001
uuid: b7e4c1a9-2f83-4d6e-9a15-3c8f0d2b6e47
execution_id: "eb646386-6dc9-43d8-9b08-630de228a192"
status: blueprint_locked
mayeuta_verdict: ok
---

# Clarificación — infra-adapters-ssot-governance

Transcript Mayeuta (2026-08-29). Semilla `PBI-ARCH-INFRA-ADAPTERS-SSOT-001` (deuda DD-7 del Espejo). Init lab: `execution_id` `eb646386-6dc9-43d8-9b08-630de228a192`.

Fuentes: PBI; `SddIA/core/cumulo.paths.json` v1.6.5; `SddIA/agents/cumulo.md`; `SddIA/infrastructure/adapters/{lancedb_thought_repo,lancedb_evolution_repo}`; `tools/index.md` / `daemons/index.md` (patrón de censo); `sync_entity_index.rs`; `external-ai-constraints.md` DA-2.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` |
| `feature_name` | `infra-adapters-ssot-governance` |
| Rama | `feat/infra-adapters-ssot-governance` |
| `persist_ref` | `docs/features/infra-adapters-ssot-governance` |
| `document_id` | `PBI-ARCH-INFRA-ADAPTERS-SSOT-001` |
| Intención estable | Dar existencia soberana en el SSOT a `SddIA/infrastructure/**` para que Cúmulo (y el Espejo) resuelvan conectores **sin walk a ciegas** |

---

## D1 — Hechos SSOT (triage)

| ID | Afirmación | Hecho | Laudo |
|----|------------|-------|-------|
| **I1** | `directories` no lista infraestructura | `cumulo.paths.json` v1.6.5: no hay `infrastructure` ni `infrastructure_adapters` | Hueco real de topología |
| **I2** | Adaptadores = entidades atómicas | Crates host: `sddia-infrastructure-lancedb-thought` / `…-evolution`; **sin** `{name}.md`, `uuid`, `index.md` | Entropía documental, no cápsulas |
| **I3** | LanceDB es placeholder | `lancedb_thought_repo/src/lib.rs`: `Ok(None)` / `Ok(vec![])`; `Cargo.toml` **no** declara crate `lancedb`. Evolution adapter persiste JSON en `.SddIA/vector_store/evolution/` | `status: placeholder` (ambos) |
| **I4** | Cúmulo audita cualquier `index.md` | `sync_entity_index.rs::index_map` solo: process, agent, skill, tool, action, codex, suite. **No** daemons ni adapters | INF-CA2 ≠ handler vivo. Auditoría MVP = estática (índice ↔ YAML) |
| **I5** | `SddIA/infrastructure/` es genoma protegido DA-2 | Tabla DA-2: tools, skills, actions, process, agents, events, norms, library_* | **No protegido.** Tekton puede escribir contrato/índice/`{name}.md` |
| **I6** | Path del PBI | Título con `/adapters` creó un **directorio**. Reubicado a `docs/todos/pending/PBI-ARCH-INFRA-ADAPTERS-SSOT-001.md` | Nombre plano obligatorio |

---

## D2 — Laudos DA-1…DA-5 (cierran el refinamiento)

| ID | Laudo | Efecto |
|----|-------|--------|
| **DA-1** | `directories.infrastructure` = `SddIA/infrastructure`; `directories.infrastructure_adapters` = `SddIA/infrastructure/adapters`. **No** `execution_capsules` (no son WASI). **No** `products` (no son crates publicados) | SemVer SSOT **1.6.5 → 1.7.0** (clave nueva = minor) |
| **DA-2** | Mínimo viable: **censo tabular** `index.md` + ficha `{name}.md` por adaptador. **Prohibido** inventar `type: adapter` en Constitución / `entity-manager` este ciclo | Identidad sin nueva clase de forja |
| **DA-3** | Contrato de familia `SddIA/infrastructure/adapters/adapters-contract.md`; clave `contracts.infrastructure_adapters`. No vive en `directories.norms` | Homólogo a `daemons-contract.md` junto al índice |
| **DA-4** | Campo `status`: `placeholder` \| `active` \| `deprecated` en frontmatter **y** columna del índice | Hoy ambos LanceDB = `placeholder` |
| **DA-5** | Catálogo + `cumulo.paths.json` + evolution: **Tekton directo** (I5). Extender `entity-manager` / `sync_entity_index` = **fuera** (PBI hijo si se exige forja de clase) | Sin anidar creators |

---

## D3 — Alcance estabilizado

| Dentro | Fuera |
|--------|-------|
| Claves Cúmulo + bump 1.7.0 | Integración física LanceDB (`PBI-CORE-LANCEDB-REAL-001`) |
| `adapters-contract.md` + `index.md` + 2 fichas | Panel Espejo / `GET /api/system-health` |
| `status` coherente con código | `type: adapter` en Constitución |
| Evolution `{uuid}.md` | Extender `sync_entity_index.rs` / creator |
| Backlink Espejo DD-7 → este `document_id` | IOTA como “conector de infra” (sigue siendo `tool`) |

---

## D4 — Identidades previstas (Dedalo fija uuid)

| name (kebab) | crate / dir | uuid | status |
|--------------|-------------|------|--------|
| `lancedb-thought-repo` | `lancedb_thought_repo` | `0a22c260-2c5a-4aaa-a632-2c9a78e983e4` | placeholder |
| `lancedb-evolution-repo` | `lancedb_evolution_repo` | `ab9bef02-c2c1-426b-a2b2-ca1cc170f21c` | placeholder |
