---
feature_name: sddia-installer-v3-io-contract
created: "2026-09-26"
process: refactorization
purpose: Contrato capsule-json-io 2.0 en motor/fachada installer (sin UX presentador)
version_clarify: "1.0.0"
pbi_ref: docs/todos/pending/[ARQUITECTURA] Installer v3 — contrato de entrada-salida del comando.md
document_id: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
pbi_uuid: "cb5483e7-4e39-4fb6-9c11-4a8285b957b7"
---

# Clarificación — sddia-installer-v3-io-contract

## Decisiones

| ID | Laudo |
|----|-------|
| L-IO-PY | Estado y envelope en `SddIA/scripts/installer/installer_io.py` (cápsula auxiliar); motor bash orquesta. |
| L-FACADE-MERGE | Fachada live `deploy`/`teardown`: motor escribe envelope en `SDDIA_INSTALLER_RESULT_FILE`; fachada fusiona verify/eventos y emite un solo stdout. |
| L-NORM | `sddia-installer-contract` 1.2.0 en `SddIA/library/norms/`; schemas `sddia-installer-request.schema.json` / `result` junto a la norma. |
| L-CUMULO | `instance.installer_logs` → `.SddIA/logs/installer/`. |
| L-UX-OUT | Presentador y atajos fuera de alcance (PBI UX). |

## Fuera

`paciente0-deploy`, entidad `tool`, Rust motor, `plan-legacy`.
