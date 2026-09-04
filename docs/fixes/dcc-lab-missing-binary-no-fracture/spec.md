---
feature_name: dcc-lab-missing-binary-no-fracture
created: "2026-09-04"
process: bug-fix
base: main
scope: dcc-f4b-lab-binary
version_spec: "1.0.0"
branch_name: fix/ignition-pre-push-guard
persist_ref: docs/fixes/dcc-lab-missing-binary-no-fracture
pbi_ref: docs/todos/done/[FIX] delivery-close-cycle — Ola 3 binario ausente no fractura (ca3d901fdc9a).md
document_id: PBI-FIX-FRACTURE-ca3d901fdc9a-OLA3
fracture_hash: ca3d901fdc9a
incident_ref: "System_Fracture_Detected — ca3d901fdc9a"
laudo: C
---

# Especificación — Ola 3 binario lab ausente no fractura

## Problema

`emit_dcc_phase_fractures` escala `failed` por receta de compile (`sddia-qa no encontrado`, `cápsula skill '…' no encontrada bajo SddIA/target`) a `System_Fracture_Detected`. F4b no cubre `failed` de binario ausente. Lab incompleto ≠ colapso ontológico.

## Corrección

Predicado `dcc_lab_binary_missing_suppresses_fracture` en `delivery_close.rs`, simétrico a F4b. Match case-insensitive de los dos literales. Cualquier fase. **No** `fail_soft`: el acuse sigue `success: false`.

Negativos: gate evolution real (`evolution gate (--range --if-touched) failed`); revocación RBAC (`revocado en revoked_entities`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| DCC-NF-CA1 | Traza `sddia-qa no encontrado` → cero `System_Fracture_Detected` |
| DCC-NF-CA2 | Traza cápsula `git-manager` ausente → igual |
| DCC-NF-CA3 | Gate evolution real y RBAC siguen emitiendo |
