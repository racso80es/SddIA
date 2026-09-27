---
document_id: PBI-KAIZEN-INSTALLER-V3-UI-TTY-EXPECT
uuid: "5fc72fba-4c31-4c2d-8342-7c84cafb59a3"
title: "[KAIZEN] Installer v3 — smoke TTY presentador (AC-9 y AC-11)"
format: markdown
version: "1.0.0"
status: done
priority: baja
type: kaizen
process: bug-fix
dispatch: false
feature_name: sddia-installer-v3-ui-tty-expect
historia_ref: docs/todos/done/historias/[ARQUITECTURA] Installer v3 — contrato de entrada-salida por comando y UX de Deploy - Eliminar Cliente.md
historia_document_id: HU-INSTALLER-V3-IO-CONTRACT-UX
historia_uuid: "489f5b85-fa2f-4f0e-ae52-7278e039dff1"
created: "2026-09-27"
author: tekton
residual_de: audit_cierre HU-INSTALLER-V3-IO-CONTRACT-UX (2026-09-27)
pbi_antecesores:
  - document_id: PBI-ARQUITECTURA-INSTALLER-V3-UX-EJECUTABLES
    uuid: "8cb95b4a-52f0-4030-91c1-8b937a425589"
related:
  - SddIA/scripts/installer/sddia-installer-ui.sh
  - SddIA/scripts/qa/test-sddia-installer.sh
---

# Installer v3 — smoke TTY con expect/script

Residual **AC-9** y **AC-11**: el presentador se validó manualmente; CI solo cubre passthrough sin TTY (AC-10).

## Intención

Tests automatizados con pseudo-TTY (`script`, `expect` o equivalente en Ubuntu CI) que no bloqueen el job (timeouts, `--no-hold` donde aplique).

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-T1 | Deploy dry-run bajo TTY: al menos una línea `k/N — título`; resumen con Resultado, Puerto, Log, Duración; con `SDDIA_INSTALLER_HOLD=0` termina sin bloqueo. | script/expect en CI o job opcional |
| AC-T2 | Teardown TTY sin `--yes`: entrada distinta de `ELIMINAR` → exit 3, envelope `TEARDOWN_REQUIRES_FORCE`, motor no invocado (mock o contador). | expect |
| AC-T3 | Teardown TTY con `--yes` + dry-run: invoca fachada con `--force` (envelope ok). | expect |
| AC-T4 | Sin regresión AC-10 (pipe a `cat`). | smoke existente |

## Fuera de alcance

- Probar hold de 600 s en CI.
- zenity/GTK.

## Plan

1. Añadir `SddIA/scripts/qa/test-sddia-installer-ui-tty.sh` (o sección condicional si `script` disponible).
2. Documentar en `docs/features/sddia-installer-v3-ux-executables/` si el job CI gana un step opcional.
