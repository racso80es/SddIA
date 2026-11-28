---
contrato_version: "1.1.1"
id_cambio: "c9d4bfe5-d3be-4c36-b8af-ec0c114e9ec8"
fecha: "2026-11-28"
tipo_operacion: alta
descripcion_breve: "AUT-129: build_cycle_payload resuelve tracker_ref desde persist_ref (resolve_tracker_ref_for_lifecycle) en milestone PrOpened"
hash_integrity: "sha256:d44093705689c8045e91b1d36536c8bc37272aeb49236e0da7ae51ff36967115"
relacionado:
  - "AUT-129"
  - "AUT-120"
  - "AUT-75"
  - "SddIA/engine/execute-process/src/engine/lifecycle_phase_action.rs"
  - "docs/fixes/aut-129-dcc-pr-opened-tracker-ref-persist"
---

# AUT-129: PrOpened resuelve tracker_ref vía persist_ref

En `build_cycle_payload`, tras resolver `pbi_ref`, se enriquece `tracker_ref` con la misma semántica que `resolve_tracker_ref_for_lifecycle` sobre `persist_ref` únicamente, alineando el milestone `PrOpened` con `attach_tracker_ref_from_persist` usado en la emisión de `PullRequest_Presented`.

Test de regresión: `build_cycle_payload_resolves_tracker_ref_from_persist_objectives_only`.