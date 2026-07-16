---
contrato_version: "1.0.0"
id_cambio: "fa7b3fc4-2bfd-4c3a-9cce-52bc47b09ee9"
fecha: "2026-07-16T18:50:00+02:00"
autor: "dedalo"
proyecto_origen_cambio: "SddIA"
contexto: "Ola 7 PENDING_AUDIT_DOC event-bus-audit — spam por review_id en hash."
descripcion_breve: "materialize-kaizen-alert-doc idempotente por alert_kind+implicated_files; consolidación documental."
tipo_operacion: "bug-fix"
cambios_realizados:
  - anterior: "Cada Kaizen_Alert_Required con review_id distinto creaba PENDING_AUDIT_DOC nuevo"
    nuevo: "Reutiliza TODO abierto con misma huella alert_kind+files"
impacto: "Cola DIA deja de acumular satélites event-bus-audit."
replicacion:
  instrucciones: "cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process materialize_kaizen"
relacionado:
  - "PBI-KAIZEN-AUDIT-DOC-DEDUPE-OLA-20260716"
  - "docs/fixes/kaizen-audit-doc-dedupe-ola-20260716/"
---

# Kaizen audit-doc — dedupe ola event-bus-audit
