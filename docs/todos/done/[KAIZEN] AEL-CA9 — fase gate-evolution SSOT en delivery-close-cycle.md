---
document_id: PBI-KAIZEN-AEL-CA9-DCC-EVOLUTION-PHASE
uuid: "9ca90ae1-6d64-4bcc-8009-c51acf014ca9"
title: "[KAIZEN] AEL-CA9 — fase gate-evolution SSOT en delivery-close-cycle (hook delega)"
format: markdown
version: "1.1.0"
created: "2026-08-30"
updated: "2026-08-30"
status: "cerrado"
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
  - SddIA/scripts/qa/git-hooks/hook_common.sh
  - SddIA/tools/sddia-qa/src/gate_evolution.rs
  - docs/todos/done/[KAIZEN] Aduana evolution local inexistente — hooks sin instalar, --if-touched invertido y fase de impacto stub.md
  - docs/todos/done/[FIX] delivery-close-cycle — barrera de fase simulated (c51acf014c0f).md
  - docs/fixes/kaizen-aduana-evolution-local/spec.md
  - docs/fixes/dcc-simulated-barrier-c51acf014c0f/spec.md
  - docs/fixes/ael-ca9-dcc-evolution-phase/validacion.md
---

# [KAIZEN] AEL-CA9 — fase `gate-evolution` SSOT en `delivery-close-cycle`

Diferido de `PBI-FIX-FRACTURE-c51acf014c0f` (F4 / AEL-CA9) y residual de `PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL` (`6d64bcc7`, L6 / AEL-CA9+CA10). **No** reabre F1–F4c de `c51acf014c0f`.

**Cerrado 2026-08-30** — R1 hook delega, R2 `--sync-base` en cápsula, R3 genoma v1.4.0 vía `entity-manager`. Validación APTO. Rama `fix/ael-ca9-dcc-evolution-phase`. PR #232.

## 1. Contrato original (AEL-CA9 / AEL-CA10)

| ID | Criterio |
|----|----------|
| **AEL-CA9** | `delivery-close-cycle` declara la verificación evolution como **fase del proceso** (junto a «Aduana EDA genómica»). Toda ruta de entrega la atraviesa. **El hook delega en el proceso; no duplica el gate.** |
| **AEL-CA10** | Mutación del genoma (`delivery-close-cycle.md`, `hash_signature`) vía `execute-process` → `entity-manager` + `recalc-process-hash-signatures`. Cero edición manual del `.md`. |

Laudo `6d64bcc7` §5.6 (`A-GENOMA-VIA-ENTITY-MANAGER`): añadir/ajustar la fase exige forja; edición manual la detecta `verify-process-integrity`.

## 2. Residual cubierto

| ID | Hueco | Resolución |
|----|-------|------------|
| R1 | Hook duplicaba gate en presentación | `pre_push_hook_runs_evolution_gate n` → gate solo si `n==0` |
| R2 | DCC sin `--sync-base` | `evolution_gate_args()` = `--json --range --sync-base` |
| R3 | Notas genoma desalineadas | `entity-manager` → DCC v1.4.0; VPI OK |

## 3. Criterios de aceptación

| ID | Criterio | Estado |
|----|----------|--------|
| CA-1 | Presentación: `gate-evolution` una vez (fase DCC) | APTO |
| CA-2 | PR OPEN (`#branches==0`): hook corre gate; no DCC | APTO |
| CA-3 | `SDDIA_HOOK_DELIVERY_CLOSE=1`: hook skip | APTO |
| CA-4 | Cápsula `--sync-base`; envelope `base_resolution` | APTO |
| CA-5 | Genoma vía `entity-manager`; VPI OK | APTO |
| CA-6 | Notas nombran `SDDIA_HOOK_DELIVERY_CLOSE` | APTO |
| CA-7 | Test predicado `pre_push_hook_runs_evolution_gate` | APTO |
| CA-8 | Cascada + este PBI en `done/` | APTO |

## 4. Fuera de alcance (respetado)

- Reabrir F1–F4c de `c51acf014c0f`.
- Mutar `delivery-close-cycle.md` a mano.
- Fracturas DLT `b3a715381787` / `701c77ebeab8`.
- `--require-synced-base` en DCC local.
