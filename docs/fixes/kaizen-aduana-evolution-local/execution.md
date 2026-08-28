---
feature_name: kaizen-aduana-evolution-local
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-aduana-evolution-local
persist_ref: docs/fixes/kaizen-aduana-evolution-local
pbi_ref: docs/todos/pending/[KAIZEN] Aduana evolution local inexistente — hooks sin instalar, --if-touched invertido y fase de impacto stub.md
correlation_id: "6b617fa0-ced7-49d5-9e1f-48356f3f26d3"
agents: tekton
phase: Ejecución
uuid: fedb9597-a2a3-4c5b-825c-e3c7f3186b1b
---

# Ejecución — kaizen-aduana-evolution-local

## Contexto

| Campo | Valor |
|-------|-------|
| execution_id | `6b617fa0-ced7-49d5-9e1f-48356f3f26d3` |
| Rama | `fix/kaizen-aduana-evolution-local` |
| Diseño | `1018a3a` (spec + plan) |

## Fases Tekton

| Fase | Estado | Evidencia |
|------|--------|-----------|
| L2 — predicado material | executed | 3 tests `gate_evolution::tests` OK |
| L3 — base_resolution | executed | flags + JSON; test timeout CA14 pendiente (mock git) |
| L4 — pre-push PR OPEN | executed | `pre_push_gate.sh` `run_evolution_gate` |
| L1 — hooksPath + verify-hooks | executed | `verify_hooks.rs`; `start-sddia.sh` |
| L5 — impact assessment | executed | diff real en `phase_capsules.rs` |
| L7 — retirar gate pre-commit | executed | `pre_commit_gate.sh` sin evolution |
| L6 — DCC fase evolution | executed | `entity-manager` → DCC v1.2.0; handlers Rust |
| tests-ci | executed | workflow `--require-synced-base` |
| smoke CA12 | executed | `evolution_audit_ca12` + `evolution_phase_blocks_unregistered_material_ca12` |
| CA14 timeout | executed | `git_timed_kills_process_within_budget_ca14` + `resolve_base_sync_fetch_timeout_declares_outcome_ca14` |
| evolution-register | executed | `6d64bcc7-b677-4c43-b239-928e279d2a04` vía `sddia-qa evolution-register` |

## Comandos de verificación

```bash
cd SddIA && unset CARGO_TARGET_DIR
cargo test -p sddia-qa gate_evolution::tests          # 3 passed
cargo test -p execute-process delivery_close            # 16 passed
cargo build -p sddia-qa -p execute-process

cd ..  # raíz repo
./SddIA/target/debug/sddia-qa verify-hooks --json     # finding si hooksPath unset
./SddIA/target/debug/sddia-qa gate-evolution --json --range
```

## Fricción operativa

| Fricción | Mitigación |
|----------|------------|
| `CARGO_TARGET_DIR` del sandbox escribía binario distinto al de hooks | `unset CARGO_TARGET_DIR` antes de `cargo build`; binario en `SddIA/target/debug/sddia-qa` |
| Gate en rango sin commits de código | EVOL_OK sobre docs-only; evolution-register tras commit material |

## Laudo CA6 (pre-commit)

El gate evolution sobre **staged** se retiró de `pre_commit_gate.sh`. La aduana de rama queda en: pre-push (PR abierto), fase DCC «Aduana evolution», y CI `--range --require-synced-base`.

## Smoke CA12

Mutación material sin evolution debe bloquear con `EVOL_MATERIAL_UNREGISTERED`:

```bash
# En worktree limpio (stash WIP si aplica)
printf 'probe\n' > SddIA/tools/_smoke_ca12_probe.txt
git add SddIA/tools/_smoke_ca12_probe.txt
git commit --no-verify -m "temp: smoke ca12"
./SddIA/target/debug/sddia-qa gate-evolution --json --range
# Esperado: success=false, EVOL_MATERIAL_UNREGISTERED
git reset --hard HEAD~1
```

## entity-manager (L6)

- Proceso: `delivery-close-cycle` v1.2.0
- Evento: `Domain_Entity_Updated` `2094024e-8318-4c87-929c-1a57528e8dd5`
- Hash: `sha256:b26d16f7bb9144d6d2e01cf9d89b196285fb9e043178915ae990cc51af184cb4`

## Siguiente paso (cierre)

1. Commit de implementación en rama (código + evolution `6d64bcc7-…` + docs).
2. `validacion.md` APTO + PBI → `docs/todos/done/`.
3. `delivery-close-cycle` vía hook o proceso.
