---
document_id: PBI-FIX-FRACTURE-ca3d901fdc9a-RESIDUAL
uuid: "c01b2776-7023-40bc-91ee-d38357e0c607"
title: "[FIX] delivery-close-cycle — residual cápsulas DCC"
format: markdown
version: "1.0.0"
created: "2026-09-04"
updated: "2026-09-04"
status: cerrado
refinement_status: clarified
priority: alta
process: bug-fix
type: fix
dispatch: false
parent_pbi: PBI-FIX-FRACTURE-ca3d901fdc9a
parent_ref: docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (ca3d901fdc9a).md
suggested_branch: fix/ignition-pre-push-guard
persist_ref_suggested: docs/fixes/dcc-lab-residual-capsules
fracture_hash: ca3d901fdc9a
fracture_process: delivery-close-cycle
incident_ref: "System_Fracture_Detected — ca3d901fdc9a"
friction_ids:
  - F-DCC-EVOLUTION-GATE
  - F-DCC-APERTURA-FORJA
architectural_constraints:
  - A-IGNITION-PRODUCE-SKILL-ELF
  - A-NO-FORGE-GENOME-DA2
execution_file_lock:
  - start-sddia.sh
gates_this_wave:
  - DCC-RES-CA1
  - DCC-RES-CA2
  - DCC-RES-CA3
related:
  - start-sddia.sh
  - SddIA/skills/shell-executor.md
  - SddIA/skills/sddia-evolution-register.md
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (ca3d901fdc9a).md
source_audit: "2026-09-04. Tras Ola 1–3: DCC fa1e88a6 Aduana evolution blocked EVOL_CUMULO cápsula sddia-evolution-register ausente; Apertura en forja failed cápsula skill shell-executor no encontrada bajo SddIA/target. Ignición no construye esos crates."
review_notes: "Residual de laboratorio, no reapertura del padre. Mismo patrón Ola 1: ELF + sello write_genome=false. No mutar genoma skills/."
---

# [FIX] delivery-close-cycle — residual cápsulas DCC

Absorbe los dos binarios skill que Ola 1–2 dejaron fuera de `start-sddia.sh`.

## Linaje

| Campo | Valor |
|-------|--------|
| Padre | `PBI-FIX-FRACTURE-ca3d901fdc9a` (cerrado; índice) |
| Precedente | Ola 1 `git-manager` en ignición |

## 0. Laudo

**Intención:** ignición produce ELF nativo de `sddia-evolution-register` y `shell-executor` bajo `compiled_capsules.native_root`. DCC deja de fallar por receta de compile de esas dos cápsulas.

### Fuera de alcance

- Mutar `{name}.md` (`source_sha256`).
- Target WASI `wasm32-wasip1` (resolver nativo basta).
- Abrir PR / `delivery-close-cycle` (verificación local de CAs).

## 1. Alcance

| Archivo | Cambio |
|---------|--------|
| `start-sddia.sh` | ambos crates en `release_pkgs` y lote debug; sello DA-2 junto a `git-manager` |

## 2. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| DCC-RES-CA1 | ELF ejecutable `SddIA/target/{release,debug}/{sddia-evolution-register,shell-executor}` | `file` / mime |
| DCC-RES-CA2 | `sddia-qa gate-evolution --json --range --sync-base` no emite `cápsula sddia-evolution-register ausente` | CLI |
| DCC-RES-CA3 | `--tool shell-executor` no emite `no encontrada bajo SddIA/target` | orquestador |

## Criterio de cierre

- [x] DCC-RES-CA1…CA3
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO en `docs/todos/done/` en la rama del PR
