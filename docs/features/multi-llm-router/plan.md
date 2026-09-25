---
feature_name: multi-llm-router
created: "2026-09-25"
process: feature
phases: "T0-contract T1-registry T2-adapters T3-router T4-aiua T5-vault-docs T6-close"
branch_name: feat/multi-llm-router
persist_ref: docs/features/multi-llm-router
document_id: PBI-MULTI-LLM-ROUTER
agents: dedalo
---

# Plan — multi-llm-router

Blueprint de ejecución. Fuentes Rust (`SddIA/tools/*/src`, `SddIA/skills/*/src`, `SddIA/engine/`) editables directo. Entidades `.md` de `tools/`, `skills/`, `process/`, `library/norms/` solo vía `./sddia-run.sh --process entity-manager`. Cúmulo (`SddIA/core/cumulo.paths.json`) editable directo + `sddia-qa`.

| ID | Fase | Touchpoints | Done |
|----|------|-------------|------|
| **T0** | Contrato | `library/norms/capability-contracts/llm.infer.schema.json`; `capability-taxonomy.md` v1.0.8 (término `llm:infer`) — ambos gobernados | Schema válido draft 2020-12; término presente; `sddia-qa` índices OK | 
| **T1** | Registro | `llm-registry.schema.json`; Cúmulo `instance.llm_registry`; starter-kit `.SddIA/llm-registry.example.json` (`model: ""`); verificar `.gitignore` | Example valida contra schema; `rg` slugs en `SddIA/` = 0 nuevos (CA-REGISTRY) |
| **T2** | Adaptadores | `tools/gemini-http-infer/src/main.rs` (request `llm.infer` + legacy, `error_code`, `telemetry_receipt.provider`); `skills/antigravity-cli-executor/src/main.rs` (ídem); `.md` v1.1.0 de ambos con `provides llm:infer` vía entity-manager update | Tests unitarios de mapeo `error_code` (429/503/401/timeout/vacío; network/timeout/quota agy); tests legacy verdes (CA-ADAPTERS) |
| **T3** | Router | `tools/llm-router/{Cargo.toml,src/main.rs}`; `tools/llm-router.md` vía entity-manager create | Tests: selección `oracle_id`/`affinity`/default; salto solo L-SALTO; ciclo → `llm-registry-invalid`; registro ausente → fail-soft; `attempts[]` completo; `max_hops` (CA-ROUTER) |
| **T4** | Aiúa | `process/aiua-stimulus-processing.md` v1.3.0 (entity-manager update: `Combustion-Inferencia` → `tool:llm-router`); `engine/.../handlers/aiua_stimulus.rs` (`infer_via_router`, telemetría `provider`/`routing_attempts`/`cognitive-degraded`); tests de genoma y lab con stubs | `cargo test -p execute-process aiua` verde; genoma sin literales de adaptador (CA-AIUA) |
| **T5** | Bóveda + docs | Ambos `.env.example` (bloque registro); README § DI fila `llm:infer`; evolution uuid `d2e44083-…` | `sddia-qa gate-evolution --json --range` exitCode 0 (CA-GOV) |
| **T6** | Cierre | `implementation.md`, `execution.md`, `validacion.md` (`PENDIENTE-CI` hasta run verde), PBI → `done/` solo con `global: APTO`; `delivery-close-cycle` | Un PR; CI verde; CA-DOC |

## Orden y dependencias

`T0 → T1 → T2 → T3 → T4 → T5 → T6`. T2 y T3 pueden solaparse tras T0 (el router solo necesita el contrato). T4 exige T3 compilado (`cargo build -p llm-router`) y T2 para el smoke live.

## Gates por fase

- Tras cada entity-manager: acuse JSON `success:true`; DA-5 (sin polling).
- `cargo build --workspace` y `cargo test -p llm-router -p gemini-http-infer -p antigravity-cli-executor -p execute-process` antes de T5.
- `SddIA/target/debug/sddia-qa` (índices, EDA) antes del push.
- CA-ORTHO: `git diff --stat main..HEAD` sin `agent_runtime.rs`, `kalma2-agent-runtime-cursor.py`, `capability-bindings.md`, `SddIA/agents/`, `mayeuta-llm*`, `kalma2-bridge/`, `notify_humanized_pr_merged.rs`.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Creator de `norm` no cubre `capability-contracts/*.schema.json` | Alta gobernada mínima (norm creator para la taxonomía) + evolution que declare el schema; documentar en `implementation.md` |
| Dos `provides llm:infer` disparan `CAPABILITY_PROVIDER_AMBIGUOUS` | Ningún process declara `requires_capability: llm:infer`; test negativo en `capability_di_resolver` fixture opcional |
| Smoke live sin sesión `agy` o sin `GEMINI_API_KEY` | CA-AIUA se valida con stubs; smoke live = evidencia adicional no bloqueante |
| WASI CI compila `tools/*` | El router no usa red ni FS fuera de lectura; si el job WASI lo rechaza, excluir con paridad `gemini-http-infer` y anotar en evolution |

## Laudos pendientes que alteran el plan

- `L-TERM` rechazado → T0 pasa a bump `llm.interact` 1.1.0 y T2 cambia `provides`. 
- `L-LOCUS` rechazado → T3/T4 se sustituyen por trabajo en `capability_di_resolver` + capa de inyección (re-planificar).
