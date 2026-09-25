---
feature_name: object-lock-exit3-89b7-014259
created: "2026-09-25"
process: bug-fix
phases:
  - dlt-object-lock-predicate
  - mayeuta-lock-and-exit3
  - kalma-exit3-suppress
  - verify-unit
  - genome-evolution
  - commit-plan
branch_name: fix/object-lock-exit3-89b7-014259
persist_ref: docs/fixes/object-lock-exit3-89b7-014259
execution_id: "2b45bcaf-60ea-46bc-ae41-cb2102d92925"
---

# Plan — sellos `89b7c8b105ec` y `0142599491fb`

## Fase 0 — Commit de planificación

PBI refinados, `objectives.md`, `spec.md`, `plan.md`. Sin código.

## Fase 1 — Predicado de reserva (DLT-LOCK-CA1, CA2)

`route_domain_core.rs`: `dlt_transient_object_lock_trace`. Tests: la traza del PBI no escribe pending y deja cola `dlt_reanchor`; gas y ENETUNREACH siguen suprimidos; `config-missing` y «issues with transaction inputs» sin la firma siguen emitiendo.

## Fase 2 — Mayeuta (MAYEUTA-LOCK-CA3, MAYEUTA-EXIT3-CA2)

`enrich_fracture_pbi_kaizen.rs`: subtipo reserva dentro del cubo DLT; cubo literal del exit 3. Tests `analyze_fracture_kaizen` con ambas trazas de PBI. El cubo gas y el `{acción} failed:` genérico siguen como están.

## Fase 3 — Puente (KALMA-EXIT3-CA1)

`kalma2-bridge` `main.rs`: predicado de exit. Test de unidad. Sin spawn de red.

## Fase 4 — Unitario

```text
cd SddIA && cargo test -p execute-process --lib -- dlt_transient emit_dlt_batch_fracture analyze_fracture_kaizen
cd SddIA && cargo test -p kalma2-bridge --lib -- prosthetic_exit
```

Si `kalma2-bridge` no es lib, el test vive en el binario: `cargo test -p kalma2-bridge -- prosthetic_exit`.

## Fase 5 — Genoma y evolution

`entity-manager` update de `enrich-fracture-pbi-kaizen` (texto de los dos cubos). `sddia-qa evolution-register`. Sin edición directa del `.md` de la acción.

## Fuera de este plan

E2E IOTA. Cierre documental y PR: después de la implementación, no en el commit de planificación.
