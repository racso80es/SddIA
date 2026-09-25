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

## Laudos propuestos (pendientes de ratificación del Vértice)

| ID | Decisión propuesta | Alternativa descartada |
|----|--------------------|------------------------|
| **L-LOCUS** | Router = cápsula Rust `tool:llm-router` (stdin/stdout, `capsule-json-io` 2.0). El motor no cambia su resolver DI en Fase 1. | Router en `capability_di_resolver` / capa de inyección con `affinity` en `requires_capability`: nueva semántica de contrato de proceso, reintento en motor, ×3 trabajo, fuera del principio «cápsulas ejecutan». |
| **L-TERM** | Nuevo término `llm:infer` / contrato `llm.infer` (inferencia cruda de proveedor). `llm:interact` intacto (mayeuta-llm). Alta en `capability-taxonomy` v1.0.8 vía entity-manager (AC-NO-INVENT con este laudo). | Bump `llm.interact` 1.1.0: mezcla sobre de skill gobernada con sobre de adaptador; forzaría `provides llm:interact` en adaptadores → ambigüedad del resolver. |
| **L-REGISTRY** | Esquema `llm-registry.schema.json` en Core (`library_norms/capability-contracts/`); datos en `.SddIA/llm-registry.json` (Cúmulo `instance.llm_registry`; override `SDDIA_LLM_REGISTRY_PATH`). Starter-kit: `.SddIA/llm-registry.example.json`. | `SddIA/core/llm-registry.json` con slugs (F5 del PBI). |
| **L-EMIT** | El router devuelve `attempts[]` tipados y `telemetry_receipt`; no emite ECST. Degradación de entidad = decisión Radamanto por telemetría (Fase 1b). | Router emisor de `Domain_Entity_Degraded`: falsos positivos por un 429 aislado; acoplamiento a bus. |
| **L-SALTO** | Salto solo ante `rate_limited | timeout | upstream_unavailable | network`. `auth`, `malformed_response`, `unknown` no saltan (fallo de configuración/contrato, no de disponibilidad). | Saltar siempre: enmascara errores de bóveda. |
| **L-AIUA** | `aiua-stimulus-processing` v1.3.0: `Combustion-Inferencia` → `tool:llm-router`, `affinity: "aiua"`. `inputs.model`/`effort` viajan como override opcional al oráculo elegido. `SDDIA_GEMINI_MODEL` deja de ser default del latido. | Mantener agy cableado y poner el router «detrás» del skill: duplica lógica en la skill. |
| **L-LEGACY** | Adaptadores aceptan request legacy (`request.prompt` + `parameters`) y request `llm.infer`; tests de ambos. | Romper compatibilidad: afecta `notify-humanized-pr-merged` y smokes. |
| **L-ORTHO** | Sin diff en `agent_runtime.rs`, `resolve_phase_model`, `capability-bindings` (`llm:interact`), `SddIA/agents/*.md`, `mayeuta-llm`, `kalma2-bridge`. | — |

## Preguntas abiertas para el Vértice (no bloquean la planificación)

1. ¿Ratifica **L-TERM** (`llm:infer`) o prefiere bump de `llm.interact`? Cambia T0 y el `provides` de los adaptadores.
2. ¿Ratifica **L-LOCUS** (cápsula) frente al motor propuesto en el anexo de Tormentosa?
3. ¿Debe la instancia actual quedar con registro `agy → gemini-http` (primario agy, fallback Gemini HTTP) o al revés? Es dato de bóveda, no de genoma; solo afecta al smoke live.

## Fuera de este ciclo

Fases `agent:` (tiers done). Rebind de `llm:interact`. `mayeuta-llm` consumidor del router (Fase 1b). Migración de `notify-humanized-pr-merged` (Fase 1b). Circuit breaker de estado Cerbero (Fase 1b). LLM local (Fase 2). Adaptador Cursor como oráculo (laudo aparte). `kalma2-bridge`.
