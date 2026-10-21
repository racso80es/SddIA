---
feature_name: eda-redb-06-semantic-layer
created: "2026-10-21"
process: feature
branch_name: feat/eda-redb-06-semantic-layer
persist_ref: docs/features/eda-redb-06-semantic-layer
pbi_ref: docs/todos/pending/[ARQUITECTURA] EDA Redb 06 — Doble capa semántica.md
document_id: PBI-EDA-REDB-06-SEMANTIC-LAYER
execution_id: "8aa2812e-0456-4679-ae2d-18956c57c085"
---

# Objetivos — eda-redb-06-semantic-layer

## Misión

Doble capa semántica, laudos L-REDB-08 y L-REDB-09. `agents-contract` 1.2.0 declara `cognitive_role_descriptor`. Los siete agentes del catálogo lo llevan sin cambiar `uuid`, `allowed_policies` ni `llm_profile.tier`.

## Alcance

- AC-11: descriptor y tier coinciden con la tabla del PBI. El contrato es 1.2.0. `CONSTITUTION_CORE.md` sigue nombrando los Filtros C, A y B.
- CA-08: `agents/index.md` no gana filas. La Aiúa no entra en el catálogo.

## Fuera de alcance

Implementar el store, retirar `event-sweeper` y cambiar el tier de Argos o autorizar LLM en Cerbero, Cúmulo o Radamanto.
