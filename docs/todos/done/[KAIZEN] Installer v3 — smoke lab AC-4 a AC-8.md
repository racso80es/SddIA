---
document_id: PBI-KAIZEN-INSTALLER-V3-SMOKE-LAB-AC4-8
uuid: "b93ca305-01e5-44ba-ab59-d4a70188d2e9"
title: "[KAIZEN] Installer v3 — smoke lab AC-4 a AC-8"
format: markdown
version: "1.0.0"
status: done
priority: media
type: kaizen
process: bug-fix
dispatch: false
feature_name: sddia-installer-v3-smoke-lab-ac4-8
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
  - SddIA/scripts/qa/test-sddia-installer.sh
  - SddIA/scripts/installer/installer_io.py
  - SddIA/scripts/sddia-installer.sh
---

# Installer v3 — automatizar matriz lab AC-4..AC-8

Residual del cierre del PBI contrato: AC-4..AC-8 quedaron en **inspección manual**; la historia §8 los exige en smoke o test negativo.

## Alcance

| AC | Objetivo del test |
|----|-------------------|
| AC-4 | Deploy lab `--skip-build`, root `/tmp/…`: stdout = un JSON; sin `cargo`/`systemctl`/JSON hijo en stdout; contenido en `result.log_ref`. |
| AC-5 | Forzar fallo en `build_bundle` (perfil inválido o stub) → exit 6, `STEP_FAILED`, `error.step`, `error.child_exit`. |
| AC-6 | Parser JSONL: cada paso ejecutado emite `begin`/`end` coherentes; sin secretos en líneas. |
| AC-7 | Con `correlation_id` en request → ficheros bajo `.events/progress/{cid}/`, `source_agent=sddia-installer`. |
| AC-8 | Test negativo R-VAULT-2: 0 valores de claves sensibles en envelope + progreso + log (nombres de clave sí). |

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-L1 | Casos AC-4..AC-8 integrados en `test-sddia-installer.sh` o script hermano invocado por el job CI. | `sddia-installer-smoke` verde |
| AC-L2 | AC-4/AC-5 acotados a lab (sin enable real en CI si ya es política del smoke). | revisión workflow |
| AC-L3 | Fallo de un caso lab → mensaje acotado (sin volcar bóveda). | smoke |

## Fuera de alcance

- Deploy live en instancia de producción del operador.
- Cambiar semántica de códigos 1–7.
