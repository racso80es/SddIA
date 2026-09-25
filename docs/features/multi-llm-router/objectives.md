---
feature_name: multi-llm-router
created: "2026-09-25"
process: feature
branch_name: feat/multi-llm-router
persist_ref: docs/features/multi-llm-router
pbi_ref: docs/todos/pending/PBI-MULTI-LLM-ROUTER.md
document_id: PBI-MULTI-LLM-ROUTER
uuid: "d2e44083-ccdf-45af-b477-f6c71833fc31"
execution_id: "31a62dff-7b1e-454f-9539-eccc0b16eb3a"
delivery_mode: branch_pr
delivery_mode_source: inputs
phase: mayeuta-stabilization
agents: mayeuta
---

# Objetivos — multi-llm-router

## Misión

Que el latido de la Aiúa (y, después, cualquier consumidor de inferencia) no dependa de un proveedor único: un contrato de adaptador tipado, un registro de oráculos de **instancia** y una cápsula Core `tool:llm-router` que selecciona oráculo y salta al siguiente ante cuota/timeout/caída, devolviendo telemetría de cada intento. Conmutar proveedor = editar `.SddIA/llm-registry.json`, no genoma.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Contrato `llm.infer` (schema + término en taxonomía) vía entity-manager | Bump/rebind de `llm:interact`; `mayeuta-llm` |
| `llm-registry.schema.json` (Core) + Cúmulo `instance.llm_registry` + example starter-kit | Registro con slugs en `SddIA/core/` |
| Cápsula Rust `tool:llm-router` (selección, fallback encadenado, `attempts[]`, `telemetry_receipt`) | Router dentro de `capability_di_resolver` / motor |
| `gemini-http-infer` v1.1.0 y `antigravity-cli-executor` v1.1.0 como adaptadores con `error_code` tipado, compat legacy | Nuevos adaptadores (local, Cursor) |
| `aiua-stimulus-processing` v1.3.0 → router; handler `aiua_stimulus.rs`; tests | `kalma2-bridge`, `notify-humanized-pr-merged` |
| Bóveda: `SDDIA_LLM_REGISTRY_PATH` comentada; notas en ambos `.env.example` | Fases `agent:` / tiers / `SDDIA_LLM_TIER_*` |
| Evolution con uuid del PBI + cascada documental | Emisión de `Domain_Entity_Degraded` desde el router (L-EMIT); circuit breaker Cerbero |

## Criterios de aceptación (este ciclo)

Los CA del PBI v1.2.0 §6: `CA-CONTRACT`, `CA-REGISTRY`, `CA-ROUTER`, `CA-ADAPTERS`, `CA-AIUA`, `CA-ORTHO`, `CA-FAILSOFT`, `CA-GOV`, `CA-DOC`. Laudos aplicados: `L-LOCUS`, `L-TERM`, `L-REGISTRY`, `L-EMIT`, `L-SALTO`, `L-AIUA`, `L-LEGACY`, `L-ORTHO` (clarify.md). `global: APTO` solo con CI verde.

## Ley aplicada

- Rutas vía `SddIA/core/cumulo.paths.json`; sin rutas cableadas.
- Git vía `skill:git-manager`.
- Mutación de `tools/`, `skills/`, `process/`, `library/norms/` vía entity-manager (DA-2). Motor Rust (`SddIA/engine/`) y fuentes de cápsulas (`SddIA/tools/{name}/src`, `SddIA/skills/{name}/src`) editables directo.
- `capsule-json-io` 2.0: `exitCode === 0 ⟺ success`.
- `features-documentation-pattern` v1.2.1; cierre documental en rama, un PR.
- DA-5 / DA-6 tras acuses CLI y CI.
- Jerarquía: Acción → Agente → Skill → Tools. El router no conoce dominio.
