---
feature_name: infra-adapters-ssot-governance
created: "2026-08-29"
process: feature
phase: planning
agents: dedalo
phases:
  - T0-ssot-cumulo
  - T1-contrato-indice
  - T2-fichas-lancedb
  - T3-evolution-backlink
  - T4-argos-static
  - T5-doc-closure
branch_name: feat/infra-adapters-ssot-governance
persist_ref: docs/features/infra-adapters-ssot-governance
pbi_ref: docs/todos/pending/PBI-ARCH-INFRA-ADAPTERS-SSOT-001.md
document_id: PBI-ARCH-INFRA-ADAPTERS-SSOT-001
uuid: b7e4c1a9-2f83-4d6e-9a15-3c8f0d2b6e47
execution_id: "eb646386-6dc9-43d8-9b08-630de228a192"
---

# Plan — infra-adapters-ssot-governance

Blueprint Tekton. Contratos: `spec.md` L1–L11. **Stop planning:** no ejecutar T0–T5 en esta sesión.

Init lab: `execution_id` `eb646386-6dc9-43d8-9b08-630de228a192` · vehículo `feature` · relevo IDE.

## T0 — SSOT Cúmulo (INF-CA1)

1. `SddIA/core/cumulo.paths.json`: `version` `1.6.5` → `1.7.0`.
2. `directories.infrastructure` = `SddIA/infrastructure`.
3. `directories.infrastructure_adapters` = `SddIA/infrastructure/adapters`.
4. `contracts.infrastructure_adapters` = `SddIA/infrastructure/adapters/adapters-contract.md`.
5. No tocar `execution_capsules` ni `products`.

**Gate:** JSON válido; claves resolubles.

## T1 — Contrato + índice (INF-CA2, L4)

1. Crear `adapters-contract.md` v1.0.0 (identidad, `status`, Anti-Alucinación: ficha ⇒ `stat(impl_dir)`).
2. Crear `index.md` con el censo (2 filas). Frontmatter `directories_key: infrastructure_adapters`.

**Prohibido:** mutar `SddIA/tools|skills|actions|process|agents|events|norms/`.

## T2 — Fichas LanceDB (INF-CA3, L5/L6)

1. `lancedb-thought-repo.md` — uuid `0a22c260-2c5a-4aaa-a632-2c9a78e983e4`, `status: placeholder`.
2. `lancedb-evolution-repo.md` — uuid `ab9bef02-c2c1-426b-a2b2-ca1cc170f21c`, `status: placeholder`.
3. Alinear columnas del índice. No editar `src/lib.rs` ni `Cargo.toml`.

## T3 — Evolution + backlink (INF-CA5)

1. `SddIA/evolution/b7e4c1a9-2f83-4d6e-9a15-3c8f0d2b6e47.md` — alta SSOT + familia adapters.
2. Espejo: DD-7 apunta a `PBI-ARCH-INFRA-ADAPTERS-SSOT-001` (path plano).
3. PBI: `status: refined`, DAs cerradas por spec.

## T4 — Argos estático (INF-CA2/CA4)

1. Cada fila: ficha existe + `impl_dir` es directorio.
2. Ningún `impl_dir` sin ficha (anti-huérfano documental).
3. Consumidor de prueba: leer `directories.infrastructure_adapters` desde `cumulo.paths.json` (script/test o revisión), **sin** glob de `src/`.
4. No invocar `sync-entity-index` con `entity_class` inventada.

## T5 — Cierre documental (INF-CA6)

`implementation.md` + `execution.md` + `validacion.md` APTO + PBI → `docs/todos/done/` en **esta** rama. Un PR.

## Fuera de esta parada

Ejecución Tekton (T0–T5). Panel Espejo. LanceDB físico. Extensión de `entity-manager` / `sync_entity_index`.
