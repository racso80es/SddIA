---
contrato_version: "1.1.2"
id_cambio: "c1f0fed3-0bf8-4bee-bf3b-c3a70fcf09b4"
fecha: "2026-10-23"
tipo_operacion: modificacion
descripcion_breve: "La fase Ejecución deja un commit por tarea y lo anota en el PBI de trabajo."
hash_integrity: "sha256:f1b6f4aacbab0f3a48130c2f8f8824f5d573ce07a61af18210b855dc949c310c"
relacionado:
  - "PBI-TRACKER-PROVIDER-CAPSULE"
  - "docs/features/tracker-provider-capsule/"
  - "SddIA/tools/linear-tracker-adapter"
  - "SddIA/library/codexes/codex-software-engineering/process/feature.md"
  - "SddIA/library/codexes/codex-software-engineering/process/bug-fix.md"
  - "SddIA/library/codexes/codex-software-engineering/process/refactorization.md"
document_id: PBI-TRACKER-PROVIDER-CAPSULE
source_feature: tracker-provider-capsule
---

# PBI-TRACKER-PROVIDER-CAPSULE — commit por tarea

La fase Ejecución de feature, bug-fix y refactorization fija un commit por tarea del plan y la marca de forja en el PBI de trabajo. El adaptador Linear conserva el cuerpo HTTP cuando el estado no es 2xx.
