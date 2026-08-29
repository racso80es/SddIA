---
feature_name: route-domain-event-fracture-b3a715381787
created: "2026-08-29"
process: bug-fix
phases:
  - diagnose-instance-500
  - forge-publisher-prefixes
  - classify-publish-error
  - verify-unit
  - document-and-stop-for-laudo
branch_name: fix/route-domain-event-fracture-b3a715381787
persist_ref: docs/fixes/route-domain-event-fracture-b3a715381787
---

# Plan — fractura `b3a715381787`

Corte de esta entrega: **Diseño (spec + plan) + commit**. Sin implementación de código, sin `delivery-close-cycle`.

## Fase 0 — Diagnóstico de instancia (post-laudo, no raw de entrega)

1. Log del hijo Node `iota-publish-relay`: mensaje `error`/`feedback` del 500.
2. Cruzar `/health` OK vs publish KO.
3. Auditar presencia (no volcar secretos) de `IOTA_WALLET_SECRET`, `IOTA_ANCHOR_PACKAGE_ID`, `SDDIA_LAB_SIMULATE_IOTA` en bóveda.
4. Inspeccionar cola `eda_instance.dlt_reanchor` (Cúmulo). Confirmar candidato 1–4 en `execution.md`.

## Fase 1 — Prefijos en `iota-immutable-publisher`

Vía `./sddia-run.sh --process entity-manager` (DA-2). No editar `SddIA/tools/` a mano.

En `publish_via_relay`:

- Si `ureq` falla por transporte (refused, timeout, DNS) → `iota-relay-unreachable: {e}`.
- Si `ureq` expone respuesta HTTP no-2xx → leer cuerpo JSON; prefijo `iota-relay-publish-error: status={n} {error|feedback}`.
- Mantener rama 2xx + `success != true` y mapearla a `F-DLT-PUBLISH-ERROR` en el clasificador.

## Fase 2 — Clasificador en `route_domain_core.rs`

Motor (no genoma DA-2):

1. Rama nueva: `iota-relay-publish-error` → `F-DLT-PUBLISH-ERROR`.
2. Conservar `iota-relay-unreachable` / health → `F-DLT-RELAY-SIN-SUPERVISOR`.
3. Tests: caso 500 → `F-DLT-PUBLISH-ERROR`; caso refused (existente) intacto.

## Fase 3 — Verificación

```text
cargo test -p iota-immutable-publisher
cargo test -p execute-process emit_dlt_batch_fracture classify_batch
```

Argos: `validacion.md` con CA-1…CA-6. Archivo PBI a `docs/todos/done/` en la **misma** rama (cierre documental pre-merge).

## Fase 4 — Cierre de entrega

Solo tras CA de código + `validacion.md` APTO. `delivery-close-cycle` con fases de agente **ejecutadas**, no `simulated`. No reutilizar el snapshot `f8273a5` como Done.

## Orden y dependencias

```text
spec/plan (este commit)
  → laudo Vértice Biológico
    → Fase 0 (instancia) en paralelo con Fase 1 (entity-manager)
      → Fase 2 (motor)
        → Fase 3 tests + validacion
          → Fase 4 delivery-close
```

Fase 0 puede ir en paralelo a 1–2: la taxonomía no espera al 500 resuelto; el criterio de cierre del PBI sí exige ambos.
