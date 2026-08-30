---
document_id: PBI-KAIZEN-AEL-CA9-DCC-EVOLUTION-PHASE
uuid: "9ca90ae1-6d64-4bcc-8009-c51acf014ca9"
title: "[KAIZEN] AEL-CA9 — fase gate-evolution SSOT en delivery-close-cycle (hook delega)"
format: markdown
version: "1.0.0"
created: "2026-08-30"
updated: "2026-08-30"
status: "abierto"
priority: media
process: bug-fix
type: kaizen
fracture_hash: null
origin:
  - PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL
  - PBI-FIX-FRACTURE-c51acf014c0f
suggested_branch: fix/ael-ca9-dcc-evolution-phase
persist_ref_suggested: docs/fixes/ael-ca9-dcc-evolution-phase
depends_on:
  - PBI-FIX-FRACTURE-c51acf014c0f
architectural_constraints:
  - A-GENOMA-VIA-ENTITY-MANAGER
  - A-HOOK-DELEGA-NO-DUPLICA
  - DA-2
related:
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/scripts/qa/git-hooks/pre_push_gate.sh
  - SddIA/tools/sddia-qa/src/gate_evolution.rs
  - docs/todos/done/[KAIZEN] Aduana evolution local inexistente — hooks sin instalar, --if-touched invertido y fase de impacto stub.md
  - docs/todos/done/[FIX] delivery-close-cycle — barrera de fase simulated (c51acf014c0f).md
  - docs/fixes/kaizen-aduana-evolution-local/spec.md
  - docs/fixes/dcc-simulated-barrier-c51acf014c0f/spec.md
---

# [KAIZEN] AEL-CA9 — fase `gate-evolution` SSOT en `delivery-close-cycle`

Diferido de `PBI-FIX-FRACTURE-c51acf014c0f` (F4 / AEL-CA9) y residual de `PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL` (`6d64bcc7`, L6 / AEL-CA9+CA10). **No** reabre F1–F4c de `c51acf014c0f`.

## 1. Contrato original (AEL-CA9 / AEL-CA10)

| ID | Criterio |
|----|----------|
| **AEL-CA9** | `delivery-close-cycle` declara la verificación evolution como **fase del proceso** (junto a «Aduana EDA genómica»). Toda ruta de entrega la atraviesa. **El hook delega en el proceso; no duplica el gate.** |
| **AEL-CA10** | Mutación del genoma (`delivery-close-cycle.md`, `hash_signature`) vía `execute-process` → `entity-manager` + `recalc-process-hash-signatures`. Cero edición manual del `.md`. |

Laudo `6d64bcc7` §5.6 (`A-GENOMA-VIA-ENTITY-MANAGER`): añadir/ajustar la fase exige forja; edición manual la detecta `verify-process-integrity`.

## 2. Estado auditado (2026-08-30) — no reimplementar L6 motor

La fase **ya existe** en genoma y motor. Este PBI no es «añadir Aduana evolution desde cero».

| Capa | Estado |
|------|--------|
| Genoma `delivery-close-cycle.md` v1.3.0 | Fase «Aduana evolution» (`delegates_to: agent:argos`, intent `sddia-qa gate-evolution --json --range`) entre Impacto y Aduana EDA |
| Motor `delivery_close.rs` / `residual_runner.rs` | Handler `capsule_evolution_audit_gate` |
| Cápsula | `gate-evolution --json --range` **sin** `--sync-base` ni `--if-touched` |
| Test | `evolution_phase_blocks_unregistered_material_ca12` / `evolution_audit_gate_blocks_unregistered_material_ca12` |
| F4b (`c51acf014c0f`) | `blocked` de esta fase **no** emite `System_Fracture_Detected` |
| Push de DCC | `in_delivery_close_cycle` exime el hook → la fase del proceso es la **única** aduana evolution de esa ruta |

**Hecho (no reabrir):** L6 motor + genoma de `6d64bcc7`. F4a/F4c de `c51acf014c0f` (dispatchers `+x`, `verify-hooks --fix`, `run_evolution_gate` en pre-push).

## 3. Residual (causa de este PBI)

Tres huecos, no uno.

### R1 — Hook duplica el gate (rompe «hook delega»)

`pre_push_gate.sh` (F4c) corre `run_evolution_gate` **siempre** (tras parsear stdin, antes del bucle DCC), con `--range --if-touched --sync-base`.

Luego, si hay ramas nuevas, invoca `delivery-close-cycle`, que vuelve a ejecutar «Aduana evolution» (`--range` solo).

| Ruta | Gate hook | Fase DCC |
|------|:---------:|:--------:|
| Push de rama **nueva** (presentación) | Sí | Sí → **duplicado** |
| Push de rama con PR **OPEN** (`#branches==0`) | Sí | No → hook es la única capa (legítimo) |
| Push **dentro** de DCC (`SDDIA_HOOK_DELIVERY_CLOSE=1`) | No (exento) | Sí → fase es SSOT (correcto) |
| `git push` operador sin DCC | Sí | No |

El laudo L6 de `6d64bcc7` (§ L6 spec): *«Tras la fase: retirar invocación duplicada en `pre_push_gate.sh`»*. F4c reintrodujo el duplicado a sabiendas para cubrir el push de PR OPEN. **Homologar, no borrar a ciegas:** el hook solo dispara el binario cuando DCC **no** va a correr.

### R2 — Paridad de flags DCC vs hook vs CI

| Disparador | Flags |
|------------|-------|
| Hook `run_evolution_gate` | `--json --range --if-touched --sync-base` |
| Fase DCC `capsule_evolution_audit_gate` | `--json --range` |
| CI `wasi-runtime-smoke` | `--json --range --require-synced-base` |

DCC no sincroniza base (`--sync-base`) ni declara degradación. Un clon con `origin/main` stale puede **pasar** la fase y **fallar** CI (AEL-CA5 / CA13). El hook ya pide `--sync-base`; la fase SSOT no.

### R3 — Genoma desalineado del mecanismo real (AEL-CA10)

`delivery-close-cycle.md` § Notas / anti-recursión documenta `SDDIA_SKIP_HOOKS=1` en `git-manager`; el código usa `SDDIA_HOOK_DELIVERY_CLOSE=1` (`in_delivery_close_cycle`). Deriva §3.7 de `6d64bcc7`. Tabla de lab **no** lista skip de Aduana evolution. Nota de Impacto sigue diciendo `source_process == feature` (obsoleto tras L5).

Cualquier alineación del `.md` es **mutación de genoma** → `entity-manager` + `recalc-process-hash-signatures`. Prohibido parche manual (DA-2).

## 4. Cambio requerido

### 4.1 Topología hook (R1)

`pre_push_gate.sh`:

1. Conservar `in_delivery_close_cycle` → skip total (DCC es SSOT).
2. Si `#branches > 0` (se va a invocar DCC) → **no** llamar `run_evolution_gate`; el proceso lo hará.
3. Si `#branches == 0` (PR ya presentado / skip de presentación) → **sí** `run_evolution_gate` (única capa).
4. Guarda `main` y parseo de stdin **antes** de cualquier gate.

Resultado: cero doble ejecución en presentación; F4c sigue cubriendo push incremental de PR OPEN.

### 4.2 Paridad de la fase (R2)

`capsule_evolution_audit_gate`: invocar `gate-evolution --json --range --sync-base` (mismo predicado de material que L2; **sin** `--if-touched` ciego — DCC ya sabe que cierra entrega). Conservar `--require-synced-base` solo en CI.

Propagar `base_resolution` (ya lo hace). Si `mode != synced`, no fallar la fase por eso (AEL-CA5); el veredicto de material sí.

### 4.3 Forja de genoma (R3 / AEL-CA10)

Vía `./sddia-run.sh --process entity-manager` sobre `delivery-close-cycle` (domain `codex-software-engineering/process/`):

- Nota operativa **Aduana evolution**: flags `--range --sync-base`; bloqueo `EVOL_MATERIAL_UNREGISTERED` / `EVOL_HASH_MISMATCH`; no emitir `System_Fracture_Detected` (F4b).
- Anti-recursión: documentar `SDDIA_HOOK_DELIVERY_CLOSE=1` como guarda real; `SDDIA_SKIP_HOOKS=1` solo si sigue existiendo en subproceso `git-manager`.
- Lab: fila `SDDIA_LAB_SKIP_EVOLUTION_GATE` (o equivalente ya existente) en la tabla.
- Corregir nota Impacto (`feature` \| `bug-fix` \| `refactorization`).
- Recalc `hash_signature`. Bump SemVer de proceso (p. ej. 1.3.0 → 1.4.0).

## 5. Fuera de alcance

- Reabrir F1/F2/F3/F4a/F4b/F4c de `c51acf014c0f`.
- Mutar `delivery-close-cycle.md` a mano.
- Armado de `core.hooksPath` / bit `+x` (hecho).
- Fracturas DLT `b3a715381787` / `701c77ebeab8`.
- `--require-synced-base` dentro de DCC local (sigue siendo privilegio de CI, AEL-CA13).

## 6. Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | Push de rama nueva (presentación) ejecuta `gate-evolution` **una** vez (fase DCC), no en el hook + fase |
| CA-2 | Push de rama con PR OPEN (`#branches==0`) ejecuta `run_evolution_gate` en el hook; no invoca DCC |
| CA-3 | Push con `SDDIA_HOOK_DELIVERY_CLOSE=1` no corre el hook; la fase DCC sigue bloqueando material sin correlato |
| CA-4 | `capsule_evolution_audit_gate` usa `--sync-base`; envelope incluye `base_resolution` |
| CA-5 | Genoma actualizado **solo** vía `entity-manager`; `verify-process-integrity` OK; `hash_signature` recalculado |
| CA-6 | Notas anti-recursión del `.md` nombran `SDDIA_HOOK_DELIVERY_CLOSE` |
| CA-7 | Test: pre-push con ramas nuevas no spawnea `sddia-qa gate-evolution` antes de DCC (o fixture equivalente del bucle) |
| CA-8 | Cascada spec/plan/implementation/execution/validacion APTO; este PBI en `docs/todos/done/` en el mismo PR |

## 7. Orden de implementación

```text
spec/plan
  → Fase R2 (flags cápsula; tests CA-4)     [no genoma]
    → Fase R1 (pre_push_gate; tests CA-1/CA-2/CA-3/CA-7)
      → Fase R3 (entity-manager + hash_signature; CA-5/CA-6)
        → validacion + PBI done/ + correlato evolution
```

R2 y R1 son código de motor/scripts (no DA-2). R3 es la única mutación de genoma.

> El Vértice Biológico lauda antes de ejecutar R3 (forja de proceso oficial).
