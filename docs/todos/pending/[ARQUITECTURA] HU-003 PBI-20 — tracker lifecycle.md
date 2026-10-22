---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-20
uuid: "e1d4445d-20ad-400f-b23e-fc115e2cdd84"
title: "[ARQUITECTURA] Tracker lifecycle — Core-self y Paciente 0 en linear"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-manifests
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-14
unblocks: []
files:
  - .SddIA/project.md
---

# [ARQUITECTURA] Tracker lifecycle — Core-self y Paciente 0 en linear

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.20.

## 1. Intención

Core-self permanece en Linear. Paciente 0 se declara Linear y no hereda el default `file`.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| El default nuevo convierte a Core-self en `file` | `.SddIA/project.md` ya tiene `provider: linear` y `team_key: OSC`. Se conserva. No se borra el objeto `tracker`. |
| El checkout de Paciente 0 está en este workspace | No. La ruta documentada es `/home/racso/Proyectos/SddIA_AP`. No hay ficha en `.SddIA/projects/`. Este PBI no inventa el contenido actual. |
| `done_gate: linear` junto a `provider: linear` pasa a ser inválido | Es coherente. Puede quedarse. Un valor contradictorio no. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | `.SddIA/project.md` de Core-self mantiene `provider: linear` y `team_key: OSC`. El binding del PBI 14 lo acepta. |
| R-2 | Si `/home/racso/Proyectos/SddIA_AP/.SddIA/project.md` existe, `tracker.provider` es `linear` con `team_key` y `state_map`. Si el checkout no está, el PBI no se da por cerrado: queda bloqueado a la presencia del manifiesto, sin fabricar un fichero dentro de este repo. |
| R-3 | Ninguno de los dos manifiestos omite `provider` (eso los pasaría a `file`). |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | `project_binding` acepta el manifiesto Core-self tras el PBI 14. |
| AC-2 | Core-self no queda con provider efectivo `file`. |
| AC-3 | Paciente 0, cuando el fichero está en disco, valida como `linear` y no como default `file`. |

## 5. Fuera de alcance

- Borrar `docs/todos/` histórico del Core.
- Crear `fs-tracker-adapter`.
- Dar de alta otro provider.

## 6. Ficheros

- `.SddIA/project.md`
