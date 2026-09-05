---
uuid: "31fce110-1622-489c-a816-112849e22adb"
name: "event-bus-audit"
version: "1.2.0"
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
hash_signature: "sha256:e6126a24d4f370a368f4fc4bbb5addcd2d4a19f92c21f06ca5d17e50e2960e09"
implementation_path_ref: "SddIA/tools/event-bus-audit"
---

# event-bus-audit

Auditoría empírica del bus EDA: escaneo .events, validación ECST, informe y emisión `Kaizen_Alert_Required` **solo** si `needs_kaizen` es accionable.

`needs_kaizen = circuit_alert || actionable_stale_pending_count > 0`. No dispara por volumen histórico de dead-letter, dumps no-ECST (`github-bridge` / `FALLBACK_LOCAL_SIGNATURE`), testigos huérfanos ni pending `System_Fracture_Detected` (ya materializados). Flag `emit_kaizen_alert` (default `true`) no se duplica.

Cubre además el cruce catálogo `SddIA/events/{family}/` ↔ registros Cúmulo (`EMPTY_SUBSCRIBERS`, `FAMILY_MISMATCH`, `ORPHAN_REGISTRY_KEY`, `PURGE_BLACKHOLE`). Umbral orchestration/blackhole → `Kaizen_Alert_Required` sin fallar el audit (`success` se mantiene).

Proveedor canónico de capacidad `qa:probe` en bindings (PBI-043 H9).
