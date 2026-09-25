---
feature_name: multi-llm-router
created: "2026-09-25"
process: feature
purpose: clarificacion
branch_name: feat/multi-llm-router
persist_ref: docs/features/multi-llm-router
pbi_ref: docs/todos/pending/PBI-MULTI-LLM-ROUTER.md
document_id: PBI-MULTI-LLM-ROUTER
execution_id: "31a62dff-7b1e-454f-9539-eccc0b16eb3a"
agents: mayeuta
---

# Clarify — multi-llm-router

Relevo IDE de la fase Estabilización (`agent:mayeuta` simulated). Semilla: `.tmp/feature-multi-llm-router.json`. Init con `SDDIA_LAB_ALLOW_DIRTY=1`: único dirt = traslado kitchen→pending del PBI (dos paths; `pbi_ref` cubre uno).

## Hechos verificados que fijan el diseño

| Hecho | Fuente | Consecuencia |
|-------|--------|--------------|
| Combustión Aiúa = `skill:antigravity-cli-executor` (no Gemini HTTP) | `aiua-stimulus-processing` v1.2.0; `aiua_stimulus.rs::infer_antigravity_cli`; evolution `1dc4055c` | El consumidor primario del router es el handler Aiúa; el adaptador agy es el primario de facto. |
| `invoke_capsule_json(repo, name, payload, prefer_wasm)` invoca cápsulas por nombre (`SddIA/target/{profile}/{name}`) | `capsules.rs:303` | El router puede invocar adaptadores con el mismo helper (o `sddia-io` equivalente desde cápsula). |
| `llm.interact.schema.json` v1.0.0 solo valida sobre | `library/norms/capability-contracts/` | El contrato de adaptador es nuevo o bump; no «se crea» desde cero el de interact. |
| Resolver DI: 1 provider por `capability_id`; ambigüedad = error | `capability_di_resolver.rs` | Adaptadores no deben `provides llm:interact`. |
| `Domain_Entity_Degraded/Restored` canónicos; `Tool_*` purgados | `event-domain-subscriptions.json`; `adecuar-ed-telemetry` | Reutilizar pipeline Radamanto/Cerbero; no emitir desde el router. |
| `.SddIA/` = instancia (gitignored); Cúmulo `instance.projects` ya existe como patrón | `cumulo.paths.json` | `instance.llm_registry` sigue el mismo patrón. |
| Starter-kit exige claves comentadas y vacías | `.env.example` (ambas bóvedas) | Example de registro con `model: ""`. |
| `tools/*` son miembros automáticos del workspace Cargo | `SddIA/Cargo.toml` | `tool:llm-router` compila con `cargo build -p llm-router`. |

## Laudos ratificados (Vértice Biológico, 2026-09-25)

| ID | Dictamen | Alternativa descartada |
|----|----------|------------------------|
| **L-TERM** | Nuevo término `llm:infer` / contrato `llm.infer`. Adaptadores `provides: ["llm:infer"]`. `llm:interact` intacto (mayeuta-llm). AC-NO-INVENT cerrado por este laudo. | Bump `llm.interact` 1.1.0 → `CAPABILITY_PROVIDER_AMBIGUOUS`. |
| **L-LOCUS** | Cápsula Rust `tool:llm-router` (`capsule-json-io` 2.0). Motor / `capability_di_resolver` intacto. Invocable por Aiúa o como shim de `SDDIA_LLM_CLI_COMMAND`. | Router en el resolver (contrato process 1.5.0, ×3 esfuerzo). |
| **L-VAULT** | Instancia: `oracle-agy` (`skill:antigravity-cli-executor`, `affinity: ["aiua"]`) → `oracle-gemini` (`tool:gemini-http-infer`). Smoke live: agy primero; salto Gemini en caliente con `cognitive-degraded: true`. | Gemini primario. |
| **L-REGISTRY** | Esquema Core; datos `.SddIA/llm-registry.json` (Cúmulo `instance.llm_registry`). | Slugs en `SddIA/core/`. |
| **L-EMIT** | Router: `attempts[]` + `telemetry_receipt`. Sin ECST. | Router emisor de `Domain_Entity_Degraded`. |
| **L-SALTO** | Salto solo `rate_limited \| timeout \| upstream_unavailable \| network`. | Saltar siempre. |
| **L-AIUA** | `aiua-stimulus-processing` v1.3.0 → `tool:llm-router`, `affinity: "aiua"`. | Router detrás del skill agy. |
| **L-LEGACY** | Adaptadores: request `llm.infer` + legacy. | Romper compat. |
| **L-ORTHO** | Sin diff en harness de fases `agent:`, bindings `llm:interact`, agentes, mayeuta-llm, kalma2-bridge. | — |

## Fuera de este ciclo

Fases `agent:` (tiers done). Rebind de `llm:interact`. `mayeuta-llm` consumidor del router (Fase 1b). Migración de `notify-humanized-pr-merged` (Fase 1b). Circuit breaker de estado Cerbero (Fase 1b). LLM local (Fase 2). Adaptador Cursor como oráculo (laudo aparte). `kalma2-bridge`.
