---
feature_name: kaizen-aduana-evolution-local
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-aduana-evolution-local
persist_ref: docs/fixes/kaizen-aduana-evolution-local
pbi_ref: docs/todos/pending/[KAIZEN] Aduana evolution local inexistente — hooks sin instalar, --if-touched invertido y fase de impacto stub.md
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL
uuid: 1464cf34-77bf-4ac7-91b1-b1d19df98a00
phases:
  - l1-hooks-path-verify
  - l2-if-touched-material
  - l3-base-resolution-degraded
  - l4-pre-push-gate-order
  - l5-impact-assessment-real
  - l6-dcc-phase-entity-manager
  - l7-retire-precommit-evol-gate
  - tests-unit
  - smoke-ca12
---

# Plan — Aduana evolution local

Orden: L2 (predicado, desbloquea tests) → L3 (base, tests de timeout) → L4 (tres líneas, cierra hueco PR OPEN) → L1 (hooksPath + verify-hooks) → L5 (impacto) → L7 (retirar gate pre-commit) → L6 (forja DCC + quitar duplicado pre-push) → tests unitarios consolidados → smoke CA12. Este commit **solo** sella Diseño (`spec.md` + `plan.md`). Cierre documental y `delivery-close-cycle` **después** de Ejecución/Argos.

## Fase L2 — Predicado `--if-touched`

Archivo: `SddIA/tools/sddia-qa/src/gate_evolution.rs`.

1. `range_touches_material(paths, prefixes)` — prefijos desde `load_paths_config` (`directories` bajo `SddIA/` que son genoma; no hardcodear `GENOME_PREFIXES` del shell).
2. Sustituir el corte L365: skip solo si `--if-touched` y **no** hay material.
3. Tests (CA11 a–c) **antes** de cablear hooks: fixture de `paths` sin Git si es posible; si el gate exige repo, tmp git mínimo.

## Fase L3 — `base_resolution`

Mismo crate. `range_diff_spec` → struct + JSON en envelope.

1. `git_timed(args, budget)`: `Command::spawn` + `try_wait` + `kill`; env `GIT_TERMINAL_PROMPT=0`.
2. Flag `--sync-base` (opt-in). Flag `--require-synced-base` (CI).
3. `age_seconds` sin red: commit date / reflog de `refs/remotes/origin/main`.
4. stderr warning si `stale`/`local`; stdout intacto.
5. Test CA14: remoto inalcanzable → retorno ≤ presupuesto + margen, `fetch_outcome: timeout`.
6. Workflow: añadir `--require-synced-base` al job delta; **no** `--sync-base`. `fetch-depth: 0` + `git fetch origin main` se quedan.

## Fase L4 — Orden pre-push

`SddIA/scripts/qa/git-hooks/pre_push_gate.sh`: invocar `gate-evolution --json --range --if-touched` (predicado ya L2) **antes** del `exit 0` por `#branches == 0`. La guarda de presentación sigue omitiendo DCC, no el gate.

## Fase L1 — `core.hooksPath` + `verify-hooks`

1. Bootstrap instancia: `git config core.hooksPath SddIA/scripts/qa/git-hooks` (vía script de arranque existente, no `~/.gitconfig` global). Idempotente.
2. Comando `sddia-qa verify-hooks --json`. Finding + remedio si el path no coincide.
3. `install-hooks.sh`: comentario de compatibilidad; no nuevo instalador.

## Fase L5 — Impacto

`SddIA/engine/execute-process/src/engine/phase_capsules.rs`: calcular diff; quitar stub y filtro `feature`. Compartir lógica de paths/base con L2/L3 (extraer helper en `sddia-qa` o duplicar el mínimo de parseo name-status — preferir invocar el mismo `diff_paths` si el crate es dependencia; `execute-process` ya es dep de `sddia-qa`, no al revés: extraer a módulo compartido **solo** si el acoplamiento es trivial; si no, `git diff` vía `git-manager` en el handler).

## Fase L7 — Pre-commit

`pre_commit_gate.sh`: eliminar bloque `gate-evolution`. Conservar `--verify-process-integrity` + `audit-eda-coverage`. Documentar en `execution.md` el laudo CA6.

## Fase L6 — Genoma DCC

1. `./sddia-run.sh --process entity-manager` — fase evolution en `delivery-close-cycle.md` (domain root). Prohibido Write directo al `.md`.
2. `sddia-qa recalc-process-hash-signatures` según contrato.
3. Handler nativo de la fase: invocar `sddia-qa gate-evolution --json --range` (sin `--if-touched` ciego).
4. Quitar el gate de `pre_push_gate.sh` (el proceso es la autoridad).
5. Alinear nota anti-recursión `SDDIA_HOOK_DELIVERY_CLOSE`.

## Fase tests unitarios

- CA11: (a) material sin evolution → `EVOL_MATERIAL_UNREGISTERED`; (b) solo evolution → skip OK; (c) sin material → skip; (d) lista de ramas vacía (PR open) → gate igual se corre (L4, test de script o de orden).
- CA14: timeout de fetch.
- CA5b: JSON contiene `base_resolution.mode`.

Comando: `cd SddIA && cargo test -p sddia-qa` (y `execute-process` si L5 tiene tests).

## Fase smoke CA12

Tras binarios: mutación `SddIA/` sin evolution → DCC (o hook) bloquea **antes** de push con el mismo `reason_code` que CI. No en este commit.

## Cierre (fuera de esta parada)

`implementation.md` + `execution.md` → Argos `validacion.md` → PBI a `done/` → `delivery-close-cycle`.
