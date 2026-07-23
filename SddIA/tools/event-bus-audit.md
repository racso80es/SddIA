---
uuid: "31fce110-1622-489c-a816-112849e22adb"
name: "event-bus-audit"
version: "1.1.0"
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

Auditoría empírica del bus EDA: escaneo .events, validación ECST, informe y emisión Kaizen_Alert_Required.

Proveedor canónico de capacidad `qa:probe` en bindings (PBI-043 H9).
