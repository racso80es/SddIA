---
feature_name: async-fracture-clarification-mayeuta
created: "2026-09-09"
process: feature
branch_name: feat/async-fracture-clarification-mayeuta
persist_ref: docs/features/async-fracture-clarification-mayeuta
pbi_ref: docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md
document_id: PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION
pbi_uuid: "b2c3d4e5-f6a7-4890-b1c2-d3e4f5a6b7c8"
pbi_version: "1.2.0"
execution_id: "5f792c9f-722b-4135-bded-dfb0c34db92e"
---

# Objetivos — async-fracture-clarification-mayeuta

## Objetivo

Inyectar triaje semántico asíncrono (`skill:mayeuta-llm` / `llm:interact` `SYNTHESIZE`) sobre fracturas **inéditas**, sin romper el Laudo `L-ENRICH-KINTSUGI-DETERMINISTA`.

## Alcance

- Tiempo 1: `enrich-fracture-pbi-kaizen` sella el PBI y, si `unclassified`, emite `Fracture_Clarification_Requested` a `eda_fractal.orchestration`. Cero LLM en ese handler.
- Tiempo 2: `action:append-mayeuta-hypothesis` (nativo, `knowledge-management`) upsert H2 consultivo. Fail-open.
- Clase ECST + suscripción fractal. Genoma vía `entity-manager`.

## Fuera de alcance

- Mutar contrato `mayeuta-llm`.
- LLM en `analyze_fracture_kaizen`.
- `gemini-http-infer` / temperatura.
- Special-case en `route_domain_core` / `route_fractal_core`.
- Reabrir el PBI `60db1db67e49`.

## Ley aplicada

- `CONSTITUTION_CORE` Filtros C/A/B; C3 local-subprocess.
- `external-ai-constraints` DA-2/DA-3/DA-4: genoma vía `entity-manager`; prefijo Raw Kernel.
- `paths-via-cumulo`; `events-contract` v1.1.0; `actions-contract` v1.3.0.
- Git vía `skill:git-manager` / procesos oficiales (`delivery-close-cycle`, `accept-pr`).
