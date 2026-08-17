---
feature_name: kaizen-pec-subscribers-circuit-audit
created: "2026-08-17"
process: feature
base: main
scope: kaizen-pec-subscribers-circuit-audit
version_spec: "1.0.0"
document_id: PBI-KAIZEN-PEC-SUBSCRIBERS-CIRCUIT-AUDIT
uuid: fe8d3d21-ebeb-4a83-8b53-f2d7f0c19b16
laudo: S2-pec-correlation-proof
---

# Especificación — kaizen-pec-subscribers-circuit-audit

## 1. Laudos Dedalo

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L-O1-XOR** | ¿S2 testigo o S3 no-unlink PEC? | **S2.** `purge_after` de `route-orchestration` **no** se relaja. | Orquestación es línea de montaje táctica. Conservar PEC en el bus viola la termodinámica. El testigo vive en instancia (`eda_instance.proofs`), no en el bus. |
| **L-O1-S1** | ¿Telegram? | **Sí.** Suscriptor `argos` / `tool: send-telegram-notification`. | Circuito humano. Fallo de Telegram no anula el testigo (S2 va **primero** en el array). `skipped-*` es terminal-ok. |
| **L-O1-S2** | ¿Nombre y locus del testigo? | Acción **`persist-pec-correlation-proof`**. Fichero `{eda_instance.proofs}/pec-correlation/{correlation_id}.json`. | Namespace `pec-correlation/` evita colisión con Merkle Ocean DPP (`proofs/{event_uuid}.json`). |
| **L-CID** | ¿Clave de lectura status? | `correlation_id ≡ event_id` del `Kalma2_Process_Requested` (L3 vigente). | Bridge sigue ciego: **lee** el testigo; no se suscribe. |
| **L-O2** | ¿Dónde vive la auditoría de cobertura? | Cápsula `event-bus-audit` (src). | `tool-creator` **update** regenera UUID (forge no patch-safe). No invocar update. Códigos nuevos en la cápsula existente; uuid `31fce110-…` intacto. |
| **L-O2-THRESH** | ¿El audit falla? | **No.** `success` del tool se mantiene. `PURGE_BLACKHOLE` o `EMPTY_SUBSCRIBERS` en registro orchestration → emite `Kaizen_Alert_Required`. | Hallazgo accionable ≠ `delivery_state: failed` del auditor. |
| **L-FORGE** | ¿Genoma? | CREATE `persist-pec-correlation-proof` vía `entity-manager`. JSON de suscripciones = SSOT Cúmulo (no DA-2). Motor y bridge no son genoma. | Filtro C: no inventar segundo evento de cierre; no suscribir el bridge. |

### Rechazados

- **S3** (no-unlink / archivo PEC) — dualidad con S2 prohibida; ensucia el bus táctico.
- Bridge como suscriptor.
- IOTA / Radamanto-on-PEC.
- Telegram en `Kalma2_Process_Requested`.

## 2. Circuito objetivo

```text
execute-process peaje
  → PEC @ eda_fractal.orchestration (correlation_id plumbed)
  → route-orchestration
       ├─ persist-pec-correlation-proof  → .SddIA/proofs/pec-correlation/{cid}.json
       └─ send-telegram-notification     → humano (process_name + correlation_id + status)
  → purge padre (all_ok)
  → GET /api/status?event_id={cid}
       1. domain / dead_letter
       2. PEC vivo en orchestration
       3. testigo pec-correlation
       → 200 completed|failed|initialized|awaiting_agents
       → 404 solo si no hay domain ni PEC ni testigo
```

PEC **sin** `payload.correlation_id` → `skipped-no-correlation` (no escribe; no tumba el fan-out).

## 3. Contrato del testigo

```json
{
  "kind": "pec-correlation-proof",
  "correlation_id": "<uuid>",
  "pec_event_id": "<uuid>",
  "timestamp": "<RFC3339>",
  "payload": {
    "process_name": "<string>",
    "status": "success|failed|…",
    "cycle_phase": "completed|initialized|awaiting_agents"
  }
}
```

Idempotente: reescritura del mismo `correlation_id` (gana el PEC más reciente enrutado).

## 4. Registro orquestación

`SddIA/core/event-orchestration-subscriptions.json`:

1. `cumulo` / `action: persist-pec-correlation-proof`
2. `argos` / `tool: send-telegram-notification`

Orden innegociable: testigo antes que Telegram.

## 5. `GET /api/status` (bridge, ceguera espacial)

Tras miss de PEC en `eda_fractal.orchestration`, leer `{proofs}/pec-correlation/{event_id}.json`. Proyectar con el mismo `project_status` (ciclo `cycle_phase`). El bridge **no** escribe.

## 6. Auditoría O2 (`event-bus-audit`)

Cruce catálogo `SddIA/events/{telemetry,orchestration,domain}/` ↔ tres JSON Cúmulo.

| Código | Condición |
|--------|-----------|
| `EMPTY_SUBSCRIBERS` | Clase catalogada con array vacío **o** clave ausente en el registro de su familia. |
| `FAMILY_MISMATCH` | Clave presente en registro de familia ≠ `event_family` de la clase. |
| `ORPHAN_REGISTRY_KEY` | Clave en JSON sin clase catalogada. |
| `PURGE_BLACKHOLE` | `EMPTY_SUBSCRIBERS` ∧ router de esa familia con `purge_after=true` (orchestration y domain fractal). Telemetría: no (purge_after=false en `route_telemetry_event`). |

No sustituye ECST/staleness. H2–H5 quedan como hallazgos hasta un PBI de cableado.

## 7. Fuera de alcance

SSE progreso Kalma2; PPR #174/#177; reabrir PBI-044; mutar `route-orchestration.md` (S3).
