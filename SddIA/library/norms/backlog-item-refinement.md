---
uuid: "028fb30f-66e7-4811-bfc6-960c808b9517"
name: "backlog-item-refinement"
version: "1.0.0"
nature: "tactical-norm"
author: "tekton"
scope: "software-engineering"
category: "workflow"
dependencies: []
hash_signature: "sha256:b3dd1d7beed3d4f49ec30265e5eb5ff6efe938e8f24fdea973a2e444ecdd157f"
---

## Propósito

Refinamiento unificado de ítems de backlog (PBI/HU/fix/kaizen/deuda/spike) vía proceso refine-item: cabecera YAML canónica, fase Mayeuta con verdict.json, Definition of Ready por tipo antes de sellado.

## Obligaciones de aduana

- Prohibido refine-pbi y refine-hu como procesos activos; usar refine-item con item_ref opaco.
- Prohibido sellar refined sin DoR del tipo y verdict refined sin open_questions.
- Con tracker.provider linear, persistencia en description del issue, no en docs/todos/pending.
