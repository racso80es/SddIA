---
uuid: "d5e6f7a8-b9c0-4d1e-8f2a-3b4c5d6e7f8a"
name: "emit-suite-execution-requested"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "chaos-engineering"
capabilities:
  - "suite-execution-requested-emission"
  - "event-bus-domain-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs:
  - "suite_id": "string; identificador kebab-case de la Suite (required)"
  - "asset_id": "string; UUID de la ED Suite (optional)"
  - "execution_strategy": "string; override fail_fast | run_all (optional)"
  - "correlation_id": "string; UUID v4 de correlación (optional)"
outputs:
  - "success": "boolean"
  - "event_id": "string; UUID v4 del evento minteado"
  - "target_path": "string; ruta relativa del JSON en ./.events/domain/"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: emit-suite-execution-requested

## 1. Propósito

Emitir la instancia ECST **Suite_Execution_Requested** en el bus fractal **domain** (`./.events/domain/`). No ejecuta `execute-suite` ni enruta el bus; solo mintea `event_id` y persiste el JSON.

## 2. Orquestación

Gate **Cerbero** previo por `context`. Rutas vía `cumulo.paths.json`.

### Paso 1 — Validación

- `suite_id` obligatorio (string no vacío).
- La Suite debe existir bajo `directories.suites`.

### Paso 2 — Identidad (`action:crypto-broker`)

```json
{ "operation": "GENERATE_UUID", "target_type": "STRING", "target_payload": "" }
```

### Paso 3 — Cápsula de evento

```json
{
  "event_id": "<event_id>",
  "event_type": "Suite_Execution_Requested",
  "event_family": "domain",
  "timestamp": "<ISO-8601 UTC>",
  "emitter_agent": "emit-suite-execution-requested",
  "payload": {
    "suite_id": "<suite_id>",
    "asset_id": "<asset_id si aplica>",
    "execution_strategy": "<strategy si aplica>"
  },
  "delivery_state": {}
}
```

### Paso 4 — Persistencia

Escribir `{eda_fractal.domain}/<event_id>.json` vía `write_fractal_event`.

### Paso 5 — Cierre (stdout)

Envelope `actions-contract` con `success`, `event_id`, `target_path`.

## 3. Límites

* Sin invocación directa de `execute-suite` ni `route-domain-event` en esta acción.
* Sin sellado IOTA.
