---
document_id: PBI-ARQUITECTURA-INSTALLER-V3-REQUEST-STDIN-ENV
uuid: "f7b237b0-dd48-4b85-9834-d85067123115"
title: "[ARQUITECTURA] Installer v3 — entrada stdin y SDDIA_CAPSULE_REQUEST"
format: markdown
version: "1.0.0"
status: pending
priority: media
type: arquitectura
process: refactorization
dispatch: false
feature_name: sddia-installer-v3-request-stdin-env
historia_ref: docs/todos/done/historias/[ARQUITECTURA] Installer v3 — contrato de entrada-salida por comando y UX de Deploy - Eliminar Cliente.md
historia_document_id: HU-INSTALLER-V3-IO-CONTRACT-UX
historia_uuid: "489f5b85-fa2f-4f0e-ae52-7278e039dff1"
created: "2026-09-27"
author: tekton
residual_de: audit_cierre HU-INSTALLER-V3-IO-CONTRACT-UX (2026-09-27)
pbi_antecesores:
  - document_id: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
    uuid: "cb5483e7-4e39-4fb6-9c11-4a8285b957b7"
related:
  - sddia-installer.sh
  - SddIA/scripts/sddia-installer.sh
  - SddIA/scripts/installer/installer_io.py
  - SddIA/library/norms/sddia-installer-contract.md
  - SddIA/library/norms/sddia-installer-request.schema.json
---

# Installer v3 — entrada stdin y SDDIA_CAPSULE_REQUEST

Cierre del residual **AC-2** (historia §8): hoy solo `--request-file` y `argv`; la norma 1.3.0 cita también stdin JSON y `SDDIA_CAPSULE_REQUEST`.

## Intención

Misma precedencia que §4.1 de la historia: `--request-file` > stdin JSON (si TTY no consume stdin) > `SDDIA_CAPSULE_REQUEST` > `argv`. Petición inválida → exit 7 `REQUEST_INVALID`.

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-R1 | Request equivalente por stdin (pipe/heredoc) y por `--request-file` → mismo `result.plan` en dry-run (diff salvo timestamps). | smoke |
| AC-R2 | `SDDIA_CAPSULE_REQUEST` con JSON válido equivale a stdin cuando no hay `--request-file`. | smoke |
| AC-R3 | JSON malformado o campo prohibido → exit 7, `error.code=REQUEST_INVALID`. | smoke |
| AC-R4 | Sin regresión de casos 1–17 actuales de `test-sddia-installer.sh`. | CI |

## Fuera de alcance

- Cambiar envelope, pasos o presentador.
- Forjar entidad `tool` (D7).

## Plan

1. Fachada y/o motor: leer stdin cuando no hay argv de comando pendiente y no hay `--request-file`.
2. Extender `parse-request` / wiring en `installer_io.py` si hace falta.
3. Smoke §AC-R1..R3; cierre documental en rama única.
