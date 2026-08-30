---
feature_name: ael-ca9-dcc-evolution-phase
created: "2026-08-30"
process: bug-fix
base: main
scope: hook-delega-sync-base-genoma-entity-manager
branch_name: fix/ael-ca9-dcc-evolution-phase
persist_ref: docs/fixes/ael-ca9-dcc-evolution-phase
pbi_ref: docs/todos/done/[KAIZEN] AEL-CA9 — fase gate-evolution SSOT en delivery-close-cycle.md
document_id: PBI-KAIZEN-AEL-CA9-DCC-EVOLUTION-PHASE
execution_id: "8a2e80d1-39ad-4ca5-aeea-b665a77121df"
---

# Especificación — AEL-CA9 residual (hook delega)

## Problema

La fase «Aduana evolution» **ya existe** en `delivery-close-cycle` v1.3.0 y en `capsule_evolution_audit_gate`. El residual de AEL-CA9/CA10 (`6d64bcc7` L6 + diferido `c51acf014c0f`) son tres huecos:

| ID | Hueco |
|----|--------|
| R1 | `pre_push_gate.sh` corre `run_evolution_gate` **siempre**; si hay ramas nuevas, DCC lo vuelve a ejecutar → duplicado (rompe «hook delega») |
| R2 | Fase DCC invoca `gate-evolution --json --range` **sin** `--sync-base`; hook y CI sí alinean base |
| R3 | Notas del genoma desalineadas (`SDDIA_SKIP_HOOKS` vs `SDDIA_HOOK_DELIVERY_CLOSE`; Impacto solo `feature`; lab sin skip de evolution) |

No reimplementar L6 motor. No reabrir F1–F4c de `c51acf014c0f`.

## Reproducción (esta sesión)

`./sddia-run.sh --process bug-fix` + `SDDIA_AGENT_RELAY_IDE=1` para este PBI:

- Init: rama `fix/ael-ca9-dcc-evolution-phase`, `objectives.md`
- Fases agente: `simulated`
- **DCC encadenado** pese a F1/F2 en fuente: snapshot `9e24dc8`, [PR #232](https://github.com/racso80es/SddIA/pull/232) — binario `execute-process` de invocación **sin** barrera (fósil vs árbol). No es alcance de este PBI; no tratar #232 como Done.

## Cambio requerido

### R1 — Topología hook (`pre_push_gate.sh`)

Tras parsear stdin y guarda `main`:

| Condición | `run_evolution_gate` |
|-----------|----------------------|
| `in_delivery_close_cycle` | No (skip total; DCC es SSOT) |
| `#branches > 0` (se invocará DCC) | **No** — la fase lo hará |
| `#branches == 0` (PR OPEN / skip presentación) | **Sí** — única capa |

Conservar F4c para push incremental de PR abierto. Cero doble `gate-evolution` en presentación.

### R2 — Paridad de fase (`capsule_evolution_audit_gate`)

Args: `gate-evolution --json --range --sync-base`. Sin `--if-touched` (DCC ya cierra entrega). Sin `--require-synced-base` (CI, AEL-CA13). Conservar `base_resolution` en envelope; `mode != synced` no falla la fase; el veredicto de material sí.

### R3 — Genoma vía `entity-manager` (DA-2 / AEL-CA10)

Prohibido editar `delivery-close-cycle.md` a mano. Forja:

- Nota Aduana evolution: `--range --sync-base`; bloqueo `EVOL_*`; no `System_Fracture_Detected` (F4b).
- Anti-recursión: `SDDIA_HOOK_DELIVERY_CLOSE=1` como guarda real.
- Lab: `SDDIA_LAB_SKIP_EVOLUTION_GATE` (o equivalente) en tabla.
- Impacto: `feature` \| `bug-fix` \| `refactorization`.
- `hash_signature` + SemVer 1.3.0 → 1.4.0.

R3 exige laudo del Vértice Biológico antes de ejecutar.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | Presentación (rama nueva): `gate-evolution` **una** vez (fase DCC) |
| CA-2 | PR OPEN (`#branches==0`): hook corre gate; no DCC |
| CA-3 | `SDDIA_HOOK_DELIVERY_CLOSE=1`: hook skip; fase DCC sigue bloqueando material sin correlato |
| CA-4 | Cápsula usa `--sync-base`; envelope con `base_resolution` |
| CA-5 | Genoma solo vía `entity-manager`; VPI OK; `hash_signature` recalculado |
| CA-6 | Notas anti-recursión nombran `SDDIA_HOOK_DELIVERY_CLOSE` |
| CA-7 | Test/fixture: ramas nuevas no spawnean `sddia-qa gate-evolution` en el hook antes de DCC |
| CA-8 | Cascada spec/plan/implementation/execution/validacion APTO; PBI en `done/` en el mismo PR |

## Fuera de alcance

- Mutación manual del `.md` de proceso.
- Reabrir F1–F4c / `c51acf014c0f`.
- `--require-synced-base` en DCC local.
- Fracturas DLT `701c77ebeab8` / `b3a715381787`.
- Higienizar PR #232 / snapshot `9e24dc8` (deuda operativa del init; no mezclar en R1–R3).
- Re-lanzar DCC sobre este ciclo mientras no haya `validacion.md`.
