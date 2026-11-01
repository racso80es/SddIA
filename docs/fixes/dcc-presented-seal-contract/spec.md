---
feature_name: dcc-presented-seal-contract
created: "2026-11-01"
base: main
scope: contrato documental del sello presentación
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-presented-seal-contract
tracker_ref: AUT-48
execution_id: "9e85f016-c9be-46de-af20-92455e7817b8"
---

# Especificación — contrato del sello presentación

## Corte

HEAD `31ec7ac4`:

```text
capsule_delivery_emit_presented
  resolve_pr_url_for_presented_seal
    state / inputs
    gh pr view solo si forja blocked antes de gh
  sin URL → Ok(blocked) F-DCC-PR-URL-MISSING
  con URL → emit-pr-presented-event
fracture-signatures.json
  F-DCC-PR-URL-MISSING incluye 186c785103d8
delivery-close-cycle.md v1.6.0
  pr_url | salida de la fase anterior (opcional en payload ECST v1.1)
```

## Decisión

1. La fila `pr_url` de la fase Sello Presentación ECST deja de decir «opcional». El origen es `state.pr_url`, `inputs.pr_url` o `gh pr view` solo cuando la forja quedó `blocked` antes de invocar `gh`.
2. Una frase bajo la tabla: sin URL el envelope es `blocked`, `friction_id: F-DCC-PR-URL-MISSING`, y no se emite `PullRequest_Presented`.
3. Versión del proceso `1.6.0` → `1.6.1`. El creator reseña `hash_signature`. El resto del fichero no cambia.
4. La mutación entra por `entity-manager` (`entity_class: process`, `entity_name: delivery-close-cycle`, `lifecycle_operation: update`).

## Regresión

| Caso | Esperado |
|------|----------|
| Grep del proceso | no queda «opcional en payload ECST v1.1» en el sello |
| Fila `pr_url` | nombra `state`, `inputs` y el `gh pr view` acotado |
| `emit_presented_blocks_missing_pr_url` | sigue en verde, sin tocar el test |

## Fuera de alcance

Handler, firmas, corpus, reinyección del incidente original.

## Criterios de aceptación

Los del cuerpo refinado de AUT-48: genoma alineado, handler intacto, `validacion.md` APTO y AUT-48 en Done.
