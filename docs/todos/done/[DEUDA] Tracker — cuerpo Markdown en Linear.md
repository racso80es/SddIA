---
document_id: PBI-DEUDA-LINEAR-CUERPO-MARKDOWN
uuid: "b7c85f38-e6b7-4724-9cba-ea47b306c8c7"
title: "[DEUDA] Tracker — descripción Linear = cuerpo Markdown de HU y PBI"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: deuda
process: feature
dispatch: true
feature_name: linear-issue-markdown-body
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
tracker_ref: OSC-13
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
execution_mode: autonomo
cola_ejecucion: docs/todos/done/
derived_from_pr: "https://github.com/racso80es/SddIA/pull/316"
blocked_by: []
unblocks: []
---

# Descripción Linear = Markdown del ítem

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` (issue OSC-5).

## 0. Hecho

Los issues creados el 2026-10-02 (OSC-5 y OSC-6…OSC-12) llevaban una descripción de tres líneas: `document_id`, ruta y PR. Linear parte el código inline cuando la ruta contiene corchetes (`[DEUDA]`), así que el texto se ve roto y **no** está el cuerpo del Markdown.

La cápsula `linear-tracker-adapter` no tiene operación de alta ni de actualización de descripción (crear issues está fuera de su contrato).

## 1. Intención

Quien abra la HU o un PBI en Linear debe leer el **mismo contenido** que el archivo Markdown local (frontmatter incluido), no un puntero al fichero.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | La descripción del issue de la HU es el texto íntegro de su `.md`. |
| R-2 | La descripción de cada issue PBI hijo es el texto íntegro de su `.md` en `docs/todos/`. |
| R-3 | Al cambiar el Markdown, la descripción Linear se vuelve a volcar (mismo `tracker_ref`). No se resume. |
| R-4 | Las rutas con `[` `]` no van en código inline suelto: o forman parte del cuerpo ya volcado, o se escriben sin cercar con un solo backtick. |
| R-5 | El volcado no envía `LINEAR_API_TOKEN` ni otros secretos. |

## 3. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-1 | OSC-5 muestra el Markdown de la HU en Realizado. |
| AC-2 | OSC-6…OSC-12 y este PBI muestran el Markdown de su archivo en `pending/`. |
| AC-3 | Un cambio local de una sección y un segundo volcado se ve en Linear sin perder el padre ni la label `hu` / `pbi`. |

## 4. Fuera de alcance

Añadir `issueCreate` a la cápsula. Comentarios de `tracker-stamp`. Estados del workflow.
