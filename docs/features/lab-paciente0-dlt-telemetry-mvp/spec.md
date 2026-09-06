---
feature_name: lab-paciente0-dlt-telemetry-mvp
created: "2026-09-06"
process: feature
base: main
scope: core
document_id: PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
execution_id: "17ed4fb6-e729-4dbe-9813-cf9985aa9bce"
---

# Spec — lab-paciente0-dlt-telemetry-mvp

## Contrato de fan-out

`route-domain` carga `eda_fractal.domain_subscriptions` → `SddIA/core/event-domain-subscriptions.json`. Clave `Domain_Entity_Telemetry_Captured`:

1. `cumulo` + `process: memory-evolution-ingest` (existente).
2. `cumulo` + `tool: iota-immutable-publisher` (nuevo).

Despacho tool IOTA: rama existente en `dispatch_subscriber` (`publish_immutable_data`, `network: testnet`, payload = `serde_json::to_string(event)`).

## Skip de credenciales (este tipo solo)

Tras detectar tool `iota-immutable-publisher` y **antes** de tratar fallo como `failed`:

si `event_type == "Domain_Entity_Telemetry_Captured"` y el camino físico devolvería `config-missing: IOTA_WALLET_SECRET` o `iota-publish-unavailable` → `(sid, "skipped-config-missing", Some(reason), 0)`.

Añadir `"skipped-config-missing"` a `OK_STATUSES` en `route_fractal_core.rs`.

Simulación (`SDDIA_LAB_SIMULATE_IOTA`) y mock HTTP **no** usan este skip.

## Digest durable

En éxito IOTA del camino fractal, **antes** de `stamp_fractal_delivery_state` / `purge_after`:

1. Escribir `delivery_state.transaction_digest` y `delivery_state.cumulo.iota-immutable-publisher` (el stamp de status string se mantiene).
2. Escribir `{eda_instance.proofs}/dlt-telemetry/{event_id}.json`:

```json
{
  "event_id": "<uuid>",
  "event_type": "Domain_Entity_Telemetry_Captured",
  "entity_id": "<payload.entity_id>",
  "network": "testnet",
  "transaction_digest": "<digest>",
  "mode": "lab-simulated|relay"
}
```

Digest vacío o `"batched-digest"` → `failed`, no proof.

Resolución de proofs: `cumulo.paths.json` → `eda_instance.proofs` (default `.SddIA/proofs`).

## Suscripciones JSON

```json
{
  "agent": "cumulo",
  "tool": "iota-immutable-publisher",
  "intent": "Anclaje DLT inmutable de snapshot de telemetría en IOTA Rebased."
}
```

Ficheros: `event-domain-subscriptions.json` (SSOT) y `event-subscriptions.json` (paridad). Posición: segundo ítem de la clave.

## Clase ECST

`entity-manager` `entity_class: event` `lifecycle_operation: update` vía `markdown_body_replacements` sobre `domain-entity-telemetry-captured.md`.

Ampliar § Suscripciones: ingest + IOTA. UUID `54a49fa7-8d45-4376-9aa1-deeebeb301ea` inmutable. Versión 1.0.0. Hash/`eda-coverage` los sella el gestor.

## Tests (`execute-process`)

1. Fan-out: ingest + IOTA simulado → `delivery_status` ambos OK; proof con `lab-sim-*`; ≠ `batched-digest`.
2. Sin secreto y sin simulate → `skipped-config-missing`; ingest OK; `purged` o no-DLQ; cero proof o proof ausente.
3. `PullRequest_Presented` + config-missing **no** cambia a skip (regresión).
4. `email-triage-gateway` no invoca publisher (CA-1: ausencia de llamada en handler / grep de módulo).

## Fuera de spec

Anclaje físico CI. Mutar Peaje. Overlays instancia. Throttle.
