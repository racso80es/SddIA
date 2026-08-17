---
uuid: "accb4de7-bb1e-4f88-b5cd-b8775a8ff5a4"
name: "persist-pec-correlation-proof"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "persist-pec-correlation-proof"
  - "delegate-filesystem-manager"
  - "cumulo-pec-proof"
inputs:
  - "correlation_id": "string; UUID del Kalma2_Process_Requested (L3)"
  - "process_name": "string"
  - "status": "string; payload.status del PEC"
  - "cycle_phase": "string; opcional (initialized|awaiting_agents|completed)"
  - "event_id": "string; event_id del PEC (opcional)"
  - "timestamp": "string; RFC3339 del PEC (opcional)"
outputs:
  - "success": "boolean"
  - "proof_path": "string; ruta relativa bajo eda_instance.proofs"
  - "skipped": "boolean; true si falta correlation_id"
hash_signature: "sha256:b18ea811d9ed8efbc61a9a1e39ff44b27b7fcf79a4cea89bf9379fb6de9c1f72"
---

# Acción: persist-pec-correlation-proof

## 1. Propósito

Suscriptor Cúmulo de `Process_Execution_Completed`. Persiste una **proyección durable** del cierre táctico indexada por `correlation_id` bajo `{eda_instance.proofs}/pec-correlation/{correlation_id}.json`, **antes** del `purge_after` del padre.

El bridge Kalma2 **lee** este testigo; no se suscribe (Filtro C).

## 2. Orquestación

### Paso 1 — Validación

Si `correlation_id` ausente o vacío → `skipped: true`, `success: true` (no tumba el fan-out).

### Paso 2 — Persistencia

JSON `kind: pec-correlation-proof` con `payload.process_name`, `status`, `cycle_phase`. Idempotente: overwrite del mismo cid.

### Paso 3 — Cierre

Envelope `{success, proof_path, skipped}`.

## 3. Límites

* No escribe en `eda_fractal.*`.
* No ancla IOTA ni alimenta Radamanto.
* Namespace `pec-correlation/` — no colisionar con Merkle `{uuid}.json` de Ocean DPP.
