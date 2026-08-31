---
feature_name: dcc-hook-evol-overescalation-0c5268362b9a
created: "2026-08-31"
process: bug-fix
phases:
  - suppress-hook-evol-fracture
  - stamp-friction-id
  - tighten-kaizen-hook-cube
  - fix-is-delete-push
  - export-hook-guard-on-dcc-push
  - verify-unit
  - document-close-dcc
branch_name: fix/dcc-hook-evol-overescalation-0c5268362b9a
persist_ref: docs/fixes/dcc-hook-evol-overescalation-0c5268362b9a
---

# Plan — fractura `0c5268362b9a`

Corte de **esta** sesión: diseño (spec + plan) → commit → implementación → tests verdes → cierre documental → `delivery-close-cycle`.

## Fase 1 — Suppress F2 (CA-1/CA-2/CA-3)

`delivery_close.rs`:

```text
fn dcc_hook_evol_gate_trace(trace: &str) -> bool
fn dcc_hook_evol_block_suppresses_fracture(phase_name, status, error_trace) -> bool
```

True iff fase `Publicación remota`, status `failed|blocked`, traza contiene `evolution gate (--range --if-touched) failed`.

En `emit_dcc_phase_fractures`: `continue` si F4b **o** F4c DNS **o** este predicado.

Tests junto a `dcc_fracture_suppressed_on_evolution_gate_block`:

| Fixture | Pending `System_Fracture_Detected` |
|---------|-------------------------------------|
| Publicación remota + literal hook evolution gate | vacío |
| Apertura en forja + `pr_url` opaco | **emite** |
| Publicación remota + DNS | vacío (F4c intacto) |
| Aduana evolution blocked | vacío (F4b) |

## Fase 2 — Sello accionable (CA-3)

`stamp_dcc_hook_evol_block` en el `Err(e)` de fase DCC (hermano de `stamp_dcc_network_block`):

- `status: blocked`
- `friction_id: F-DCC-HOOK-EVOL-OVERESCALATION`
- conservar `error`

Agregador: `success: false`. No `fail_soft`.

## Fase 3 — Mayeuta F3 (CA-4/CA-5/CA-6)

`analyze_fracture_kaizen` cubo hook: tokens `delivery-close-cycle failed for` / `recurs` / `re-entrada`. Quitar `pre-push` y `hook` unarios.

Tests:

- Nuevo: `analyze_fracture_kaizen_prepush_evol_gate_not_hook_recursion` — traza canónica PBI → sin recursión.
- Actualizar: `analyze_fracture_kaizen_recursion_verdict` — fixture `SddIA pre-push: BLOCKED — delivery-close-cycle failed for feat/x`.
- Existentes DNS / heartbeat / bypass intactos.

## Fase 4 — `is_delete_push` (CA-7)

`pre_push_gate.sh`: `is_delete_push "$local_sha"`.

Test bash en `phase_capsules.rs` (patrón `pre_push_hook_runs_evolution_gate_only_when_no_branches_ca7`): source `hook_common.sh`; SHA 40 ceros como local → true; mismos ceros como argumento de «remoto» no se usan en el call site.

## Fase 5 — Guarda en push DCC (F1)

`capsule_delivery_remote_push`: set `SDDIA_HOOK_DELIVERY_CLOSE=1` con restore (`Drop` / `prev.ok()`) alrededor de `invoke_git_manager` push. No `SDDIA_SKIP_HOOKS`.

## Fase 6 — Verificación

```bash
cd SddIA && cargo test -p execute-process -- dcc_
cd SddIA && cargo test -p execute-process -- analyze_fracture_kaizen
cd SddIA && cargo test -p execute-process -- is_delete_push
cd SddIA && cargo test -p execute-process -- pre_push_hook_runs_evolution
```

## Fase 7 — Cierre documental + DCC

PBI → `docs/todos/done/`; `validacion.md` APTO `pbi_archived: true`; registro evolution; `./sddia-run.sh --process delivery-close-cycle`.
