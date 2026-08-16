---
contrato_version: "1.1.1"
id_cambio: "ef2b0ef2-b792-4cb7-ac1b-bfea203f4bde"
fecha: "2026-08-16"
tipo_operacion: modificacion
descripcion_breve: "Umbrales Radamanto por entity_type process vs tool y fail-soft PPR/DCC (PPR #174+#177)."
hash_integrity: "sha256:4b94af621e00047acaff86cfc2ecf05623c493feffed96a9c7c6a82069938165"
relacionado:
  - "ba900e95-1a47-4185-b86c-bc7a251b4fe6"
  - "PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS"
  - "docs/features/radamanto-process-threshold-rehab/"
  - "SddIA/agents/radamanto.thresholds.json"
  - "SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs"
---

# Umbrales Radamanto por entity_type process vs tool y fail-soft PPR/DCC (PPR #174+#177).

Tabla `success_rate_min_by_entity_type` (process=0.70). `resolve_entity_type` vía catálogo process. Fail-soft ola 1 (PPR fricción no-aduana) y ola 2 (DCC higiene/impacto post `pr_url`). Rehab instancia DCC fuera del diff.
