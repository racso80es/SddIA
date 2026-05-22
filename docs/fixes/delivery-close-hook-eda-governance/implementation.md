---
feature_name: delivery-close-hook-eda-governance
created: "2026-05-22"
process: bug-fix
version_implementation: "1.0.0"
---

# Implementación — delivery-close-hook-eda-governance

## Hito 1 — Anti-recursión

### `SddIA/scripts/qa/git-hooks/hook_common.py`

- Constante `HOOK_DELIVERY_CLOSE_ENV` (`SDDIA_HOOK_DELIVERY_CLOSE`).
- `in_delivery_close_cycle()` — guarda de re-entrada.
- `invoke_process()` — inyecta `SDDIA_HOOK_DELIVERY_CLOSE=1` en `env` del subproceso.
- `resolve_persist_ref()` — resuelve `docs/features/{slug}` **y** `docs/fixes/{slug}`.
- `should_skip_pre_push_present()` — skip si PR `OPEN` o `MERGED`, o si existe `PullRequest_Presented` en bus.

### `SddIA/scripts/qa/git-hooks/pre_push_gate.py`

- Skip temprano cuando `in_delivery_close_cycle()`.

### `SddIA/scripts/qa/execute_process_capsules.py`

- `invoke_git_manager(..., extra_env=)` — propagación de entorno acotada al subproceso.
- `capsule_delivery_remote_push` — si `source_process == git-hook-pre-push`, push con `SDDIA_SKIP_HOOKS=1` solo en hijo.

## Hito 2 — Retroactivo PR #20

Emisión vía `execute-action.py` con payloads en `tmp/retroactive-pr20-*.json`. Watcher `--once` → `processed/`.

## Hito 3 — Gobernanza

- `SddIA/norms/obediencia-procesos.md` v1.1 — Ley de Jurisdicción Delegada + protocolo Kintsugi.
- `SddIA/norms/pull-request-orchestration.md` §7 — cross-ref escalado.

## Hito 4 — Kintsugi EDA + Autoconocimiento

- `SddIA/events/system-fracture-detected.md` — Clase ECST.
- `SddIA/actions/materialize-fracture-pbi.md` — Cúmulo (Qué).
- `SddIA/actions/enrich-fracture-pbi-kaizen.md` — Mayeuta (Por Qué).
- `SddIA/scripts/qa/execute-action.py` — handlers `_run_materialize_fracture_pbi`, `_run_enrich_fracture_pbi_kaizen`.
- `SddIA/core/event-subscriptions.json` — suscripción dual ordenada.
- `SddIA/agents/mayeuta.md` §6 — reacción Kintsugi async.
- Backfill Fase C: `backfill-manifest.json` en persist_ref (`orphan_count_after: 0` lote 1).

## Hito 5 — Idempotencia Ola B

Consolidado en Hito 1 (`should_skip_pre_push_present` + `resolve_persist_ref`).
