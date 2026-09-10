---
uuid: "c9a6db76-5ae2-43d3-8b63-2d6ea08a5e8b"
name: "aiua-process-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Aiua_Process_Requested"
context: "ecosystem-evolution"
capabilities:
  - "aiua_process_requested"
hash_signature: "sha256:a76e798b0878cf81bfc349b98bfdd90f891c7fd19cbfc021f754a4060090946a"
---

# Event: Aiua_Process_Requested

Solicitud de ciclo SDLC emitida por el latido aiua-stimulus-processing. Hermano de Kalma2_Process_Requested. Destino runtime: eda_fractal.domain. No es CRUD genómico.

## Payload ECST

### REQUIRED
- `process`
- `raw_text`

### OPTIONAL
- `pbi_ref`
- `process_inputs`
- `intent_name`

### FORBIDDEN
- `host_path`
- `script`
- `secret`
- `agy_tool_payload`

## Emisores autorizados

- `aiua-stimulus-processing`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Aiua_Process_Requested`.
