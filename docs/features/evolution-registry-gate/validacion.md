---
feature_name: evolution-registry-gate
branch: feat/evolution-registry-gate
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/172
created: "2026-08-13"
process: feature
execution_id: 0bceeb41-64d1-4920-af9d-46a11c0455a2
persist_ref: docs/features/evolution-registry-gate
pbi_ref: docs/todos/done/[FEATURE] Evolution — gate automático de registro y coherencia (EV-AUD-001-002).md
document_id: 70f78d23-e209-4e41-9292-cb7421a934f6
checks:
  AC-ATOMIC: APTO
  AC-MATERIAL: APTO
  AC-INVALID: APTO
  AC-SELF: APTO
  AC-TESTS: APTO
  AC-CUMULO: APTO
  AC-ADUANA: APTO
  AC-INJECT: APTO
  AC-HOOK-INERT: APTO
  AC-WASI: APTO
  AC-DIAG: APTO
  AC-DEP: APTO
  AC-PR: APTO
git_changes:
  - SddIA/evolution/evolution_contract.md
  - SddIA/evolution/Evolution_log.md
  - SddIA/evolution/0bceeb41-64d1-4920-af9d-46a11c0455a2.md
  - SddIA/skills/sddia-evolution-register.md
  - SddIA/skills/sddia-evolution-register/
  - SddIA/skills/index.md
  - SddIA/core/eda-coverage.json
  - SddIA/tools/sddia-qa/src/gate_evolution.rs
  - SddIA/tools/sddia-qa/src/main.rs
  - SddIA/scripts/qa/git-hooks/hook_common.sh
  - SddIA/scripts/qa/git-hooks/pre_commit_gate.sh
  - .github/workflows/sddia-index-qa.yml
  - SddIA/Cargo.lock
  - docs/features/evolution-registry-gate/
  - docs/todos/done/[FEATURE] Evolution — gate automático de registro y coherencia (EV-AUD-001-002).md
  - docs/todos/pending/[FEATURE] Evolution — gate automático de registro y coherencia (EV-AUD-001-002).md
---

# Validación — evolution-registry-gate

## Dictamen

**APTO** — EV-AUD-001/002 cubiertos en modo **delta**: cápsula WASI `sddia-evolution-register` coteja JSON inyectado; CLI nativo `sddia-qa gate-evolution` captura e invoca; pre-commit/CI abortan solo ante `success: false` ∧ `exitCode > 0`. Universo 61 no se certifica (L-DEP / L-ENFORCE-DELTA).

## Checks

| ID | Resultado | Evidencia |
|----|-----------|-----------|
| AC-ATOMIC | APTO | Cápsula emite `{detail,index}`; `persist()` backup→detalle→índice→rollback. Hito `0bceeb41-…` aplicado desde JSON de cápsula (hash `sha256:e275fc41…`). Residual: sin test de crash a mitad de write. |
| AC-MATERIAL | APTO | Test `material_unregistered` → `EVOL_MATERIAL_UNREGISTERED` |
| AC-INVALID | APTO | `hash_mismatch`, `not_indexed`, `alta_requires_fecha`, `baja_requires_rutas_eliminadas` |
| AC-SELF | APTO | Test `self_only_evolution_ok`; smoke `gate-evolution --json` → `EVOL_OK` (sin staged material) |
| AC-TESTS | APTO | 14 tests: alta/dup/idempotente, modificacion, baja, hash, inject, cero Git en prod |
| AC-CUMULO | APTO | CLI `load_paths_config`; `directories.evolution` + `evolution_log` + `evolution_contract`; falta → `EVOL_CUMULO` |
| AC-ADUANA | APTO | `pre_commit_gate.sh` + job `wasi-runtime-smoke` invocan el mismo CLI; sin env de skip obrero |
| AC-INJECT | APTO | Tests JSON stdin; `lib.rs` prod sin `Command::new("git")` / `git diff` |
| AC-HOOK-INERT | APTO | Bloque evolution: solo `gate-evolution --json` + parseo sobre. `git diff --cached` restante = EDA `staged_touches_genome` (preexistente; no inventario evolution) |
| AC-WASI | APTO | Crate `wasm32-wasip1`; CI `cargo build --workspace --target wasm32-wasip1` |
| AC-DIAG | APTO | Sobre v2.0 `reason_codes`; `exitCode === 0` ⟺ `success` |
| AC-DEP | APTO | Gate modo delta; 61 legacy no entran. `7bb37ff1-…` sigue abierto |
| AC-PR | APTO | Cascada + PBI en `docs/todos/done/` (`pbi_archived: true`) en esta rama |

## Notas

- `validate-evolution-contract` intacto (solo lectura del corte).
- WIP ajeno (fixes EV-AUD-005 / seeds OPERATIVO) **fuera** de este PR.
- `delivery-close-cycle` / PR: mandato operador. `pr_url` no es gate de Done.
