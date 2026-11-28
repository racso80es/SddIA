---
uuid: "31fce110-1622-489c-a816-112849e22adb"
name: "event-bus-audit"
version: "1.2.1"
contract: "tools-contract v1.2.0"
domain_origin: "SddIA"
context: "quality-assurance"
capabilities:
  - "event_bus_audit"
provides:
  - id: "qa:probe"
    contract: "qa.probe"
    version: "1.0.0"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "error": "string; diagnóstico si aplica"
hash_signature: "sha256:1403075c9258460171d5059ca9551fce4796f91c2e2fedea7f7c9f222085f457"
implementation_path_ref: "SddIA/tools/event-bus-audit"
---

# event-bus-audit

## Propósito

Cápsula QA de auditoría EDA (store vía `EventManagerPort`). Emite Kaizen solo si `circuit_alert` o pending stale accionable.

## Operación

Binario bajo `SddIA/tools/event-bus-audit`; fase de proceso homónimo delega aquí con `qa:probe`.

`needs_kaizen = circuit_alert || actionable_stale_pending_count > 0`. No dispara por volumen histórico de dead-letter, dumps no-ECST (`github-bridge` / `FALLBACK_LOCAL_SIGNATURE`), testigos huérfanos ni pending `System_Fracture_Detected` (ya materializados). Flag `emit_kaizen_alert` (default `true`) no se duplica.

Emisión `Kaizen_Alert_Required`: `stamp_published_event` + `record_ecst` cuando `writes_store`; publicación exclusiva vía `EventManagerPort` (`eda://domain/<event_id>`), sin ficheros residuales en disco.

Cubre además el cruce catálogo `SddIA/events/{family}/` ↔ registros Cúmulo (`EMPTY_SUBSCRIBERS`, `FAMILY_MISMATCH`, `ORPHAN_REGISTRY_KEY`, `PURGE_BLACKHOLE`). Umbral orchestration/blackhole → `Kaizen_Alert_Required` sin fallar el audit (`success` se mantiene).

Proveedor canónico de capacidad `qa:probe` en bindings (PBI-043 H9).
