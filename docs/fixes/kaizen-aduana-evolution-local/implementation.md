---
feature_name: kaizen-aduana-evolution-local
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-aduana-evolution-local
persist_ref: docs/fixes/kaizen-aduana-evolution-local
pbi_ref: docs/todos/pending/[KAIZEN] Aduana evolution local inexistente — hooks sin instalar, --if-touched invertido y fase de impacto stub.md
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL
uuid: fedb9597-a2a3-4c5b-825c-e3c7f3186b1b
execution_id: "6b617fa0-ced7-49d5-9e1f-48356f3f26d3"
---

# Implementación — Aduana evolution local

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/tools/sddia-qa/src/gate_evolution.rs` | L2: `range_touches_material` desde `cumulo.paths.json`; L3: `BaseResolution`, `git_timed`, flags `--sync-base` / `--require-synced-base`; JSON `base_resolution` |
| `SddIA/tools/sddia-qa/src/verify_hooks.rs` | L1: comando `verify-hooks --json` |
| `SddIA/tools/sddia-qa/src/main.rs` | Cableado `verify-hooks`; flags en usage |
| `SddIA/scripts/qa/git-hooks/pre_push_gate.sh` | L4: PR OPEN (`#branches==0`) ejecuta `gate-evolution --range --if-touched --sync-base`; ramas nuevas → DCC sin gate duplicado |
| `SddIA/scripts/qa/git-hooks/pre_commit_gate.sh` | L7: retirado bloque `gate-evolution` |
| `SddIA/scripts/qa/git-hooks/install-hooks.sh` | Comentario compatibilidad vs `core.hooksPath` |
| `start-sddia.sh` | L1: `_sddia_ensure_hooks_path()` idempotente |
| `.github/workflows/sddia-index-qa.yml` | Job delta: `--require-synced-base` |
| `SddIA/engine/execute-process/src/engine/phase_capsules.rs` | L5: `capsule_delivery_impact_assessment` con diff real; L6: `capsule_evolution_audit_gate` |
| `SddIA/engine/execute-process/src/engine/delivery_close.rs` | Handler fase «Aduana evolution» |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | Handler fase «Aduana evolution» |
| `SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md` | v1.2.0 vía `entity-manager`: fase «Aduana evolution» entre Impacto y Aduana EDA |
| `SddIA/core/eda-coverage.json` | Sello `Domain_Entity_Updated` DCC |

## L2 — Predicado material

- `material_prefixes_from_cfg`: prefijos de `directories` excluyendo `evolution/`.
- `--if-touched` salta solo si el rango **no** toca material genómico (corrige tautología D3).

## L3 — Base degradada

- Modos: `synced` | `stale` | `local`; `age_seconds`, `fetch_outcome` en envelope.
- Fetch acotado 3s con `GIT_TERMINAL_PROMPT=0` y SSH BatchMode.
- Warning en stderr si base no sincronizada; CI exige `--require-synced-base`.

## L4 — Pre-push PR abierto

- Lista de ramas vacía (PR ya presentado) → `run_evolution_gate` antes de `exit 0`.
- Rama con push nuevo → `delivery-close-cycle` (fase evolution dentro del proceso).

## L1 — Hooks vivos

- `start-sddia.sh` fija `core.hooksPath` si difiere.
- `verify-hooks` emite finding accionable con remedio literal.

## L5 — Impacto real

- `capsule_delivery_impact_assessment`: `git-manager diff_name_only` sobre `origin/{target}...HEAD`.
- Elegibles: `feature`, `bug-fix`, `refactorization` (elimina filtro solo `feature`).

## L6 — DCC

- Fase «Aduana evolution» invoca `sddia-qa gate-evolution --json --range`.
- Bloqueo propagado como `status: blocked` con `reason_codes` del gate.

## L7 — Pre-commit

- Sin `gate-evolution` local sobre staged; paridad de predicado con CI vía rango en pre-push/DCC/CI.

## Verificación local

```bash
cd SddIA && unset CARGO_TARGET_DIR
cargo test -p sddia-qa gate_evolution::tests
cargo test -p execute-process delivery_close
cargo build -p sddia-qa -p execute-process
./target/debug/sddia-qa verify-hooks --json
./target/debug/sddia-qa gate-evolution --json --range
```

## Pendiente pre-PR

- Ninguno (CA12/CA14 cerrados en tests).
