---
feature_name: dcc-hook-evol-overescalation-0c5268362b9a
created: "2026-08-31"
process: bug-fix
base: main
scope: hook-evol-overescalation-mayeuta-prepush
branch_name: fix/dcc-hook-evol-overescalation-0c5268362b9a
persist_ref: docs/fixes/dcc-hook-evol-overescalation-0c5268362b9a
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (0c5268362b9a).md
document_id: PBI-FIX-FRACTURE-0c5268362b9a
execution_id: "0b5db925-9f68-4698-9afa-9f68b698f418"
---

# Especificación — fractura `0c5268362b9a` (pre-push evolution ≠ colapso)

## Problema

`delivery-close-cycle` fase **Publicación remota** abortó con:

```
SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed
error: falló el empuje de algunas referencias a 'https://github.com/racso80es/SddIA.git'
```

El bloqueo del hook es **aduana determinista**. Defectos:

| ID | Defecto |
|----|---------|
| F0 | Detonante del especimen: `actions/index.md` sin correlato. Cerrado en PR #236. Fuera de alcance. |
| F1 | Hook re-aduana el push de DCC operador; `is_delete_push` trata SHA remoto cero (ref nueva) como delete. |
| F2 | `emit_dcc_phase_fractures` escala ese `failed` a `System_Fracture_Detected` (hueco F4b). |
| F3 | `analyze_fracture_kaizen` dispara recursión hook por el token `pre-push` en la traza. |

La guarda `SDDIA_HOOK_DELIVERY_CLOSE` **ya existe**. Prohibido reimplementarla. Prohibido `SDDIA_SKIP_HOOKS=1` global.

## Reproducción

1. DCC original (ciclo `fix/mayeuta-heartbeat-kaizen-classifier`, correlato incompleto) → hash `0c5268362b9a`.
2. `./sddia-run.sh --process bug-fix` + `SDDIA_AGENT_RELAY_IDE=1`: workspace-init `0b5db925`; Diseño `simulated`; Ejecución…Cierre `skipped` (`prior_agent_phase_not_executed`).

## Laudo F1 (SSOT única)

DCC es SSOT de `gate-evolution` durante entrega (AEL-CA9 CA-1). El hook F4c corre **solo** cuando DCC no va a invocarse (`#branches == 0` por skip presentación / PR OPEN, CA-2).

Dos cambios **complementarios**, no dos gates:

1. **Protocolo git:** `is_delete_push` inspecciona SHA **local** (ceros = delete). Ref nueva (`remote_sha` cero) permanece en `branches[]`.
2. **Guarda existente en push DCC:** `capsule_delivery_remote_push` exporta `SDDIA_HOOK_DELIVERY_CLOSE=1` en el subproceso `git-manager` (paridad `accept-pr` con skip acotado). El hook hace skip temprano (`in_delivery_close_cycle`). No es reimplementar la guarda; es **usarla** en la ruta operador.

## Cambio requerido

### F2 — Suppress simétrico F4b (motor, no DA-2)

`SddIA/engine/execute-process/src/engine/delivery_close.rs`.

**Predicado de traza** (`dcc_hook_evol_gate_trace`), case-insensitive:

| Token |
|-------|
| `evolution gate (--range --if-touched) failed` |

**Supresión** (`dcc_hook_evol_block_suppresses_fracture`):

| Condición | Emite `System_Fracture_Detected` |
|-----------|----------------------------------|
| F4b: aduana evolution/EDA `blocked` | **No** (ya) |
| F4c DNS: Publicación remota / forja + red | **No** (ya) |
| `Publicación remota` + `failed`/`blocked` + traza hook evolution gate | **No** (nuevo) |
| `failed` de forja/cápsula no-gate (p. ej. `pr_url` opaco) | **Sí** |
| DNS / `git-manager` ajeno al gate | **Sí** (no tragarse) |

Al clasificar: `stamp_dcc_hook_evol_block` → `status: blocked`, `friction_id: F-DCC-HOOK-EVOL-OVERESCALATION`. Conservar `error`. Envelope DCC `success: false`. **Prohibido** `fail_soft`.

### F3 — Cubo hook estricto

`enrich_fracture_pbi_kaizen.rs` `analyze_fracture_kaizen`:

Tokens de recursión **solo** sobre `hook_blob` (`error_trace` + `attempted_action`):

| Incluir (re-entrada real) | Excluir |
|---------------------------|---------|
| `delivery-close-cycle failed for` | `pre-push` unario |
| `recurs` | `hook` unario |
| `re-entrada` | `block` / `BLOCKED` (sigue en cubo catch-all operativo) |

Traza canónica de este PBI → **no** «Recursión o re-entrada»; **no** proponer reimplementar la guarda.

Actualizar `analyze_fracture_kaizen_recursion_verdict`: fixture = literal hook de re-entrada (`BLOCKED — delivery-close-cycle failed for …`).

### F1 — Topología hook + guarda en push

| Artefacto | Cambio |
|-----------|--------|
| `hook_common.sh` `is_delete_push` | Sin cambio de semántica (ceros = delete). |
| `pre_push_gate.sh` | Pasar `$local_sha`, no `$remote_sha`. |
| `capsule_delivery_remote_push` | Guardia env `SDDIA_HOOK_DELIVERY_CLOSE=1` alrededor de `invoke_git_manager("push", …)`; restaurar al salir. |

No mutar genoma `delivery-close-cycle.md` (nota anti-recursión ya nombra la guarda).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | Traza hook evolution gate en `Publicación remota` `failed`/`blocked` **no** materializa `System_Fracture_Detected` |
| CA-2 | Mismo caso **no** tragarse: `Apertura en forja` + `pr_url` opaco **sigue** emitiendo |
| CA-3 | Phase report `friction_id: F-DCC-HOOK-EVOL-OVERESCALATION`; status `blocked`; F4b/F4c DNS intactos |
| CA-4 | `analyze_fracture_kaizen` con traza canónica `0c5268362b9a` **sin** «Recursión o re-entrada» |
| CA-5 | Traza `BLOCKED — delivery-close-cycle failed for` sigue `refactor_tool` + recursión |
| CA-6 | DNS / heartbeat tests existentes verdes |
| CA-7 | `is_delete_push` con SHA remoto cero → **falso**; SHA local cero → **verdadero** (bash, patrón CA-7 AEL) |
| CA-8 | Cascada spec/plan/implementation/execution/validacion APTO; PBI en `done/` en el mismo PR |

## Fuera de alcance

- Reabrir F0 / PR #236 / correlato `actions/index.md`.
- Reimplementar `SDDIA_HOOK_DELIVERY_CLOSE` / `SDDIA_SKIP_HOOKS` global.
- Mutar genoma `delivery-close-cycle.md` vía `entity-manager`.
- Resolver `EVOL_MATERIAL_UNREGISTERED` de un ciclo ajeno.
- `--require-synced-base` en DCC local.
