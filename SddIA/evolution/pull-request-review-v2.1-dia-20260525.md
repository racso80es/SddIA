---
contrato_version: "1.0.0"
id_cambio: "d4e8f1a2-6b3c-4d5e-9f01-pull-review-v21-dia-20260525"
fecha: "2026-05-25T00:00:00+00:00"
autor: "feature/norma-paridad-documental"
proyecto_origen_cambio: "SddIA"
contexto: "DIA — sensor audit-doc-parity en aduana pull-request-review v2.1.0"
descripcion_breve: "Reglas paridad documental no bloqueantes; sensor Python con ceguera espacial EDA."
tipo_operacion: "evolucion-proceso"
cambios_realizados:
  - anterior: "pull-request-review v2.0.0 — triaje técnico sin DIA"
    nuevo: "v2.1.0 — intent triaje incluye audit-doc-parity.py; reglas DIA-1..3 en cuerpo"
impacto: "Medio — alerta Kaizen PENDING_AUDIT_DOC_* sin bloqueo merge."
replicacion:
  instrucciones: "python SddIA/scripts/qa/audit-doc-parity.py --persist-ref docs/features/<feature> --json"
  hash_integrity: "c0d8d748e7260e13"
---

# Evolution — pull-request-review v2.0.0 → v2.1.0 (DIA)

## Decisiones

* Sensor **audit-doc-parity.py** invocado en triaje técnico; **no** eleva `delivery_state: failed`.
* Persistencia Kaizen vía fase Cosecha; evento `Kaizen_Alert_Required` reservado para EDA v2.

## Evidencia

`docs/features/norma-paridad-documental/validacion.md` — smoke `smoke-dia-parity-20260525`.
