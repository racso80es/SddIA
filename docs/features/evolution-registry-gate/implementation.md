---
feature_name: evolution-registry-gate
created: "2026-08-13"
process: feature
branch_name: feat/evolution-registry-gate
persist_ref: docs/features/evolution-registry-gate
document_id: 70f78d23-e209-4e41-9292-cb7421a934f6
execution_id: 0bceeb41-64d1-4920-af9d-46a11c0455a2
phase: implementation
agents: tekton
items:
  - evolution_contract.md v1.1.1
  - skill sddia-evolution-register (WASI)
  - sddia-qa gate-evolution / evolution-register
  - pre_commit_gate.sh detonador
  - CI sddia-index-qa.yml
  - registro 0bceeb41-64d1-4920-af9d-46a11c0455a2
---

# Implementation — evolution-registry-gate

## Touchpoints

| Artefacto | Acción |
|-----------|--------|
| `SddIA/evolution/evolution_contract.md` | Bump 1.1.0 → **1.1.1**: §7 exclusiones, §8 reason-codes, política hash, gate vs validador |
| `SddIA/skills/sddia-evolution-register.md` | Alta vía `entity-manager` (`uuid` `f9d6ad5c-…`, `ecosystem-evolution`) |
| `SddIA/skills/sddia-evolution-register/` | Crate WASI `wasm32-wasip1`: `verdict` + `alta`/`modificacion`/`baja`; **cero Git** |
| `SddIA/skills/index.md` | Fila skill + capabilities `evolution-verdict`, `evolution-record-compute` |
| `SddIA/core/eda-coverage.json` | Cobertura `f9d6ad5c-…` (`Domain_Entity_Created`) |
| `SddIA/tools/sddia-qa/src/gate_evolution.rs` | Captura Git nativa, inyección `diff`+`registry`, `wasmtime`/nativo, persistencia atómica |
| `SddIA/tools/sddia-qa/src/main.rs` | Wire `gate-evolution` / `evolution-register` |
| `SddIA/scripts/qa/git-hooks/hook_common.sh` | `resolve_sddia_qa` |
| `SddIA/scripts/qa/git-hooks/pre_commit_gate.sh` | Detonador: `gate-evolution --json`; abort iff `success==false` ∧ `exitCode>0` |
| `.github/workflows/sddia-index-qa.yml` | `cargo test -p sddia-evolution-register`; CI `gate-evolution --json --range` |
| `SddIA/evolution/0bceeb41-64d1-4920-af9d-46a11c0455a2.md` | Alta canónica v1.1.1 (cápsula calculó `{detail,index,hash}`) |
| `SddIA/evolution/Evolution_log.md` | Fila `CANONICO` del hito (host aplicó JSON emitido) |

## No tocado

- `cumulo.paths.json`
- `validate-evolution-contract` (solo usage CLI)
- Universo 61 (cuerpos históricos)
- Enum `git-manager`
- Env de bypass obrero
- WIP ajeno (fixes EV-AUD-005 / seeds OPERATIVO)

## Notas de implementación

- Persistencia: cápsula emite JSON; CLI `persist()` = backup índice → write detalle → write índice → rollback. Este hito: host aplicó el mismo JSON (`result.detail` + `result.index`) porque el invocador CLI persistente quedó bloqueado por aduana de sesión; hash verificado `sha256:e275fc41c5710c6db5eb88ef08dbd26034bfbd75e0061818373d500f8673881c`.
- `alta` duplicada vs idempotente: si el estado propuesto ≡ persistido inyectado → `EVOL_OK` + `idempotent: true`; si el `id_cambio` existe con payload distinto → `EVOL_DUPLICATE`.
- Hook: `git diff --cached` preexistente queda **solo** para EDA `staged_touches_genome`. El bloque evolution no inventaría paths ni coteja.
