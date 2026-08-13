---
feature_name: evolution-registry-gate
created: "2026-08-13"
process: feature
branch_name: feat/evolution-registry-gate
persist_ref: docs/features/evolution-registry-gate
document_id: 70f78d23-e209-4e41-9292-cb7421a934f6
execution_id: 0bceeb41-64d1-4920-af9d-46a11c0455a2
phase: verification
agents: argos
phases: "contract → forge-skill-wasi → register-crate-wasm → cli-inject → hook-inert → tests → self-record → docs → argos"
---

# Plan — evolution-registry-gate

## Fases

| # | Fase | Acciones | Done |
|---|------|----------|------|
| 1 | Contrato 1.1.1 | Extender `evolution_contract.md`: exclusiones, reason-codes, hash, gate vs validador; SemVer 1.1.1 | [x] |
| 2 | Forja skill | `entity-manager` → `skill-creator` (`sddia-evolution-register`, `ecosystem-evolution`, I/O `verdict`+mutaciones, sustrato WASI) | [x] |
| 3 | Crate WASI | `SddIA/skills/sddia-evolution-register/`: `wasm32-wasip1`, envelope v2, cotejo `diff`×`registry`, cálculo `{detail,index}`, **cero Git** | [x] |
| 4 | CLI nativo | `sddia-qa gate-evolution`: captura árbol, inyecta stdin, `wasmtime`, eco sobre, persistencia atómica de mutaciones | [x] |
| 5 | Hook inerte | `pre_commit_gate.sh`: solo invocar CLI; abort iff `success: false` ∧ `exitCode > 0`. CI = mismo CLI | [x] |
| 6 | Tests | Matriz L-TESTS + AC-INJECT (JSON fixtures, sin Git en cápsula) + AC-HOOK-INERT | [x] |
| 7 | Auto-registro hito | `operation: alta` vía cápsula WASI; host aplica `result.detail`+`result.index` **antes** del commit de genoma | [x] |
| 8 | Docs Tekton | `implementation.md`, `execution.md` | [x] |
| 9 | Argos + cierre | `validacion.md` APTO, PBI → `done/`, `pbi_archived: true` | [x] |

## Orden de mutación

1. **`directories.evolution`:** contrato 1.1.1.
2. **Genoma skill WASI:** `entity-manager` (DA-2). Crate bajo `execution_capsules.skills`; artefacto `compiled_capsules.wasm_root`.
3. **`directories.tools` / `sddia-qa`:** captura+inyección+persistencia (DA-4).
4. **Hook:** recorte a detonador inerte (quitar cualquier `diff --cached` añadido para evolution).
5. **CI:** job llama el CLI; no reimplementa cotejo.
6. **Registro del hito** (fase 7) antes del commit material.
7. **`persist_ref`:** cascada.

## Delegaciones

| Fase | Vía |
|------|-----|
| 1, 8 | Tekton, `directories.evolution` + docs |
| 2 | `entity-manager` / `skill-creator` |
| 3–4, 6 | Tekton + `cargo test` / `cargo build --target wasm32-wasip1 -p sddia-evolution-register` + `cargo build -p sddia-qa` |
| 5 | Tekton, hook + workflow |
| 7 | CLI → cápsula WASI (no edición manual del índice) |
| 9 | agent:argos + `doc:closure` |

RBAC: `ecosystem-evolution`, `filesystem-ops`, `quality-assurance`.

## Inputs entity-manager (fase 2)

```text
entity_class: skill
skill_name: sddia-evolution-register
skill_context: ecosystem-evolution
skill_version: 1.0.0
skills_contract_version: 1.4.0
```

I/O = spec `operation: verdict|alta|modificacion|baja` + `diff`/`registry` inyectados.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Encender gate sobre 61 legacy | L-ENFORCE-DELTA: solo `request.diff` inyectado |
| Claves `skillCapsules` ausentes | L-CUMULO-KEYS: `execution_capsules` + `compiled_capsules.wasm_root` |
| Cápsula con Git “por comodidad” | L-INJECT / AC-INJECT: tests sin proceso Git; crate WASI |
| Hook vuelve a listar paths | L-HOOK-INERT: abort solo por sobre; review Argos del script |
| WASI sin FS para write | Persistencia = CLI nativo (L-ATOMIC) |
| Primer commit genoma sin evo | Fase 7 obligatoria |
| WIP ajeno en working tree | Fuera de este PR |

## Handoff Tekton

Ejecutar §1–8. No migrar históricos. No mutar `cumulo.paths.json`. No alterar `validate-evolution-contract` salvo usage. Cápsula **sin Git**. Hook **sin diff**.
