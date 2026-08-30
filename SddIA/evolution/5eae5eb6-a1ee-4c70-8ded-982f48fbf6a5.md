---
contrato_version: "1.1.2"
id_cambio: "5eae5eb6-a1ee-4c70-8ded-982f48fbf6a5"
fecha: "2026-08-30"
tipo_operacion: modificacion
descripcion_breve: "Mayeuta: cubo heartbeat_starvation en analyze_fracture_kaizen; enrich-fracture-pbi-kaizen 1.2.0."
hash_integrity: "sha256:8a8131a1d37584fb80497acbaa6c64f52ec988cc2bf5b682f5aaa750a5f275c3"
relacionado:
  - PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER
  - docs/fixes/mayeuta-heartbeat-kaizen-classifier/
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/actions/index.md
  - SddIA/core/eda-coverage.json
  - SddIA/evolution/Evolution_log.md
autor: tekton
contexto: "F-MAYEUTA-HB-BLIND: traza canónica Argos caía al fallback process_fix. F-MAYEUTA-HB-TOKEN-TRAP: prohibido token heartbeat en blob general."
impacto: "analyze_fracture_kaizen clasifica omitió N ciclos de Daemon_Heartbeat como refactor_tool (inanición, no muerte); match solo error_trace."
proyecto_origen_cambio: SddIA
source_feature: mayeuta-heartbeat-kaizen-classifier
document_id: PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER
---

# Mayeuta — cubo latido en clasificador Kaizen

- `is_heartbeat_starvation_trace`: anclas literales de `emit_system_fracture`.
- Veredicto `refactor_tool`; prohibido «Auditar proceso `{daemon_id}`».
- Acción `enrich-fracture-pbi-kaizen` v1.2.0 vía `entity-manager`.