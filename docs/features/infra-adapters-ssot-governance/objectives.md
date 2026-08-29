---
feature_name: infra-adapters-ssot-governance
created: "2026-08-29"
process: feature
branch_name: feat/infra-adapters-ssot-governance
persist_ref: docs/features/infra-adapters-ssot-governance
pbi_ref: docs/todos/pending/PBI-ARCH-INFRA-ADAPTERS-SSOT-001.md
document_id: PBI-ARCH-INFRA-ADAPTERS-SSOT-001
uuid: b7e4c1a9-2f83-4d6e-9a15-3c8f0d2b6e47
execution_id: "eb646386-6dc9-43d8-9b08-630de228a192"
status: blueprint_locked
mayeuta_verdict: ok
---

# Objetivos — infra-adapters-ssot-governance

## Misión

Gobernar `SddIA/infrastructure/**` en el SSOT de Cúmulo: ruta indexada, contrato de familia, censo de adaptadores con `uuid` y `status`, para que un consumidor (Espejo de Consciencia, Fase 2) resuelva topología **sin walk a ciegas**.

## Punto objetivo

> **O-INFRA-SSOT:** `cumulo.paths.json` declara `directories.infrastructure` y `directories.infrastructure_adapters`. El índice lista los dos adaptadores LanceDB como `placeholder`. Cúmulo no marca ruido por fichas huérfanas. El Espejo puede apuntar a este censo.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Alta SSOT 1.7.0 + contrato + índice + 2 fichas | Driver LanceDB real / workspace Cargo |
| `status` explícito | `type: adapter` + entity-manager |
| Evolution + backlink Espejo DD-7 | Panel Kalma2 / IOTA como fila de infra |
| Auditoría estática índice ↔ YAML | Extender `sync_entity_index.rs` |

## Ley aplicada

- DA-1…DA-5 (clarify D2). Hechos I1–I6.
- Soberanía de rutas: Cúmulo propone; Tekton aplica el JSON (no hay creator de paths).
- DA-2 obrera: `SddIA/infrastructure/` no está en genoma protegido; no forjar `SddIA/{tools,skills,actions,process,agents,events,norms}/`.
- Anti-Alucinación Espacial: prohibido listar adaptadores que no tengan `stat` físico del crate.
