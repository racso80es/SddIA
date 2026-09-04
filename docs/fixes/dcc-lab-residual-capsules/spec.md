---
feature_name: dcc-lab-residual-capsules
created: "2026-09-04"
process: bug-fix
base: main
scope: ignition-dcc-skill-elf
version_spec: "1.0.0"
branch_name: fix/ignition-pre-push-guard
persist_ref: docs/fixes/dcc-lab-residual-capsules
pbi_ref: docs/todos/done/[FIX] delivery-close-cycle — residual cápsulas DCC (ca3d901fdc9a).md
document_id: PBI-FIX-FRACTURE-ca3d901fdc9a-RESIDUAL
fracture_hash: ca3d901fdc9a
incident_ref: "System_Fracture_Detected — ca3d901fdc9a"
laudo: C
---

# Especificación — residual cápsulas DCC en ignición

## Diagnóstico

DCC `fa1e88a6` (post Ola 2): Aduana evolution `blocked` `EVOL_CUMULO: cápsula sddia-evolution-register ausente`. Apertura en forja `failed` `cápsula skill 'shell-executor' no encontrada bajo SddIA/target`. `start-sddia.sh` no listaba esos crates. `gate-evolution` sin `--if-touched` invoca la cápsula siempre (`phase_capsules::evolution_gate_args`).

## Corrección

1. `-p shell-executor` y `-p sddia-evolution-register` en lote release de `_ensure_orchestrator`.
2. Mismos crates en el lote debug.
3. Sello DA-2 (junto a `git-manager`): `write_genome: false`, `write_witness: true`.

## Fuera de alcance

- Compilar `wasm32-wasip1` (fallback nativo en `invoke_register` / `resolve_capsule`).
- Mutar `SddIA/skills/shell-executor.md` ni `sddia-evolution-register.md`.
- Reabrir Ola 1–3.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| DCC-RES-CA1 | ELF PIE en `SddIA/target/{release,debug}/` para ambos nombres |
| DCC-RES-CA2 | `gate-evolution --json --range --sync-base` sin literal de cápsula ausente |
| DCC-RES-CA3 | `--tool shell-executor` sin `no encontrada bajo SddIA/target` |
