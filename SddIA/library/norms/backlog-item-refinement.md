---
uuid: "028fb30f-66e7-4811-bfc6-960c808b9517"
name: "backlog-item-refinement"
version: "1.0.0"
nature: "tactical-norm"
author: "tekton"
scope: "software-engineering"
category: "workflow"
dependencies: []
hash_signature: "sha256:25b7bf51e3e29ec632df2c7ab0378487ab44fd0c852eaae39a1089c22410a629"
---

## Directriz Core

Refinamiento unificado de ítems de backlog (PBI/HU/fix/kaizen/deuda/spike) vía proceso refine-item: cabecera YAML canónica, fase Mayeuta con verdict.json, Definition of Ready por tipo antes de sellado.

## Restricciones Duras (Aduana de Fricción)

- Prohibido refine-pbi y refine-hu como procesos activos; usar refine-item con item_ref opaco.
- Prohibido sellar refined sin DoR del tipo y verdict refined sin open_questions.
- Con tracker.provider linear, persistencia en description del issue, no en docs/todos/pending.
