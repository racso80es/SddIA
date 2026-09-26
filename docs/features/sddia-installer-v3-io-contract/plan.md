---
feature_name: sddia-installer-v3-io-contract
created: "2026-09-26"
process: refactorization
branch_name: refactor/sddia-installer-v3-io-contract
persist_ref: docs/features/sddia-installer-v3-io-contract
pbi_ref: docs/todos/pending/[ARQUITECTURA] Installer v3 — contrato de entrada-salida del comando.md
document_id: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
uuid: "cb5483e7-4e39-4fb6-9c11-4a8285b957b7"
---

# Plan — sddia-installer-v3-io-contract

## L0 — Planificación (commit inicial)

clarify, objectives, spec, plan.

## L1 — F0 Contrato

Cúmulo `installer_logs`; schemas JSON; norma 1.2.0.

## L2 — F1–F3 Motor + IO

`installer_io.py`, integración motor, pasos, log hijos, PTC opcional.

## L3 — F2 Fachada

Merge verify/eventos; un stdout.

## L4 — QA + CI + cierre

`test-sddia-installer.sh`, `validacion.md`, PBI `done/`, PR, accept-pr tras CI verde.
