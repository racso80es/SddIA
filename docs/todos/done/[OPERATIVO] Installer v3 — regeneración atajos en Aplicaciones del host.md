---
document_id: PBI-OPERATIVO-INSTALLER-V3-ATAJOS-HOST
uuid: "ab610ab9-9ce8-4efd-a8d1-1138b1aa73d1"
title: "[OPERATIVO] Installer v3 — regeneración atajos en Aplicaciones del host"
format: markdown
version: "1.0.0"
status: done
priority: baja
type: operativo
process: feature
dispatch: false
feature_name: sddia-installer-v3-host-shortcuts
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
  - SddIA/library/norms/sddia-installer-contract.md
  - SddIA/scripts/installer/shortcuts/
---

# Installer v3 — atajos en `~/Aplicaciones/SddIA/`

Residual operativo: la forja ya materializa plantillas con `shortcuts --dest`; el directorio del escritorio del Vértice sigue pudiendo llevar scripts legacy (`exec` directo al motor, teardown sin confirmación).

## Intención

Runbook + ejecución documentada en el host de ingeniería: regenerar `SddIA_Deploy.sh` y `SddIA_Eliminar_Cliente.sh` que invocan `sddia-installer-ui.sh`. Verificación humana o checklist en acta.

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-H1 | Comando canónico documentado: `{FORGE}/sddia-installer.sh shortcuts --dest ~/Aplicaciones/SddIA`. | runbook en feature o norma 1.3.0 §operador |
| AC-H2 | Tras regeneración, ambos `.sh` ejecutables y contienen ruta a `sddia-installer-ui.sh` de la forja actual. | `grep` + `sha256` |
| AC-H3 | Eliminar Cliente ya no hace `teardown --force` directo al motor sin UI. | lectura script |
| AC-H4 | Acta breve en `docs/audits/` o nota en PBI al cerrar (fecha, hash plantilla). | cierre documental |

## Fuera de alcance

- Mutar `~/Aplicaciones/` desde CI.
- `.desktop` opcionales (solo si el operador los pide).

## Nota

No requiere cambio de genoma salvo runbook; puede cerrarse como **operativo** sin PR de código si solo hay acta + scripts host actualizados.
