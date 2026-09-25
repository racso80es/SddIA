---
feature_name: object-lock-exit3-89b7-014259
created: "2026-09-25"
process: bug-fix
branch_name: fix/object-lock-exit3-89b7-014259
persist_ref: docs/fixes/object-lock-exit3-89b7-014259
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (89b7c8b105ec).md
sibling_pbi_ref: docs/todos/pending/[FIX] kalma2-bridge — fractura sistémica (0142599491fb).md
document_id: PBI-FIX-FRACTURE-89b7c8b105ec
execution_id: "2b45bcaf-60ea-46bc-ae41-cb2102d92925"
items:
  - dlt-object-lock-predicate
  - mayeuta-lock-and-exit3
  - kalma-exit3-suppress
  - genome-enrich-1.5.0
---

# Implementación — sellos `89b7c8b105ec` y `0142599491fb`

## Touchpoints

| Ítem | Path | Cambio |
|------|------|--------|
| DLT-LOCK-CA1/CA2 | `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | `dlt_transient_object_lock_trace`: `reserved for another transaction`. `emit_dlt_batch_fracture` no escribe pending. Gas, red, `config-missing` e «issues with transaction inputs» sin esta firma quedan como estaban. |
| MAYEUTA-LOCK-CA3 / MAYEUTA-EXIT3-CA2 | `SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs` | Subtipo DLT de reserva antes del opaco. Cubo literal `mayeuta-llm/prótesis exit 3`. |
| KALMA-EXIT3-CA1 | `SddIA/interfaces/kalma2-bridge/src/main.rs` | `prosthetic_exit_emits_fracture`: exit 3 no llama a `emit_system_fracture`. |
| Genoma | `SddIA/actions/enrich-fracture-pbi-kaizen.md` v1.5.0 | Vía `entity-manager`. `hash_signature: sha256:0ab82805973f62844946b7cdef5b6e183703ac83d29290adab942d4f9982818f`. |
| Evolution | `SddIA/evolution/c9c0206e-aed4-4e48-a17d-b28d1de43d46.md` | `alta` `EVOL_OK`. |

## Fuera

Cola serial del relay. Predicados de gas y de red. ELF ausente `64f37c7f7b34`. Exits de la prótesis distintos de 3. Canal LLM asíncrono.
